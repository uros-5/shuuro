use itertools::Itertools;
use std::{fmt, u8, vec};

use crate::{
    between, generate_plinths, get_non_sliding_attacks, get_sliding_attacks, square_bb, BitBoard,
    Color, Hand, Move, MoveError, MoveRecord, Piece, PieceType, SfenError, Square, EMPTY_BB,
    FILE_BB,
};

/// Outcome stores information about outcome after move.
#[derive(Debug, Clone, PartialEq)]
pub enum Outcome {
    Check { color: Color },
    Checkmate { color: Color },
    Draw,
    Nothing,
    DrawByRepetition,
    DrawByMaterial,
    Stalemate,
    MoveNotOk,
    MoveOk,
}

impl Outcome {
    pub fn to_string(&self) -> String {
        match &self {
            Outcome::Check { color } => format!("Check_{}", color.to_string()),
            Outcome::Checkmate { color } => format!("Checkmate_{}", color.to_string()),
            Outcome::Draw => format!("Draw"),
            Outcome::Nothing => format!("Live"),
            Outcome::DrawByRepetition => format!("RepetitionDraw"),
            Outcome::DrawByMaterial => format!("MaterialDraw"),
            Outcome::Stalemate => format!("Stalemate"),
            Outcome::MoveOk => format!("Live"),
            Outcome::MoveNotOk => format!("Illegal move"),
        }
    }
}

#[derive(Debug)]
pub enum MoveType {
    Empty,
    Plinth,
    NoKing { king: Square },
}

impl MoveType {
    pub fn blockers(&self, position: &Position, c: &Color) -> BitBoard {
        match self {
            MoveType::Empty => EMPTY_BB,
            MoveType::Plinth => &position.occupied_bb | &position.color_bb[2],
            MoveType::NoKing { king } => {
                &(&(&position.occupied_bb | &position.color_bb[2]) & &!&square_bb(king.to_owned()))
                    | &position.color_bb[c.index()]
            }
        }
    }

    pub fn moves(&self, position: &Position, bb: BitBoard, p: Piece, sq: Square) -> BitBoard {
        let primary_bb = &bb & &!&position.color_bb[p.color.index()];
        match self {
            MoveType::Empty => bb,
            MoveType::Plinth => {
                if p.piece_type != PieceType::Knight {
                    let bb = &(primary_bb) & &!&position.color_bb[2];
                    if p.piece_type == PieceType::Pawn {
                        let sq = self.get_pawn_square(sq, &p.color);
                        let primary_bb = &primary_bb & &!&square_bb(sq);
                        let sq = &square_bb(sq) & &!&position.color_bb[p.color.flip().index()];
                        let primary_bb = &primary_bb & &position.color_bb[p.color.flip().index()];
                        let moves = &(&primary_bb | &(&sq & &!&position.color_bb[2]))
                            & &!&position.color_bb[p.color.index()];
                        moves
                    } else {
                        bb
                    }
                } else {
                    primary_bb
                }
            }
            MoveType::NoKing { king } => {
                if p.piece_type != PieceType::Knight {
                    if p.piece_type == PieceType::Pawn {
                        let up_sq = self.get_pawn_square(sq, &p.color);
                        let up_sq = square_bb(up_sq);
                        return &primary_bb & &!&up_sq;
                    }
                    &(&(bb) & &!&position.color_bb[2]) | &(&bb & &square_bb(king.to_owned()))
                } else {
                    &bb | &(&bb & &square_bb(king.to_owned()))
                }
            }
        }
    }

    pub fn get_pawn_square(&self, sq: Square, color: &Color) -> Square {
        let sq = {
            match color {
                &Color::White => Square::from_index(sq.index() as u8 + 12 as u8).unwrap(),
                &Color::Black => Square::from_index(sq.index() as u8 - 12 as u8).unwrap(),
                &Color::NoColor => sq,
            }
        };

        sq
    }
}

pub struct Fixer {
    pub square: Option<Square>,
    pub fix: BitBoard,
}

impl Fixer {
    pub fn new(square: Square, fix: BitBoard) -> Self {
        Fixer {
            square: Some(square),
            fix,
        }
    }

    pub fn square(&self) -> Option<Square> {
        self.square
    }
    pub fn fix(&self) -> BitBoard {
        self.fix
    }
}

impl Default for Fixer {
    fn default() -> Fixer {
        Fixer {
            square: None,
            fix: EMPTY_BB,
        }
    }
}

#[derive(Clone)]
pub struct PieceGrid([Option<Piece>; 144]);

impl PieceGrid {
    pub fn get(&self, sq: Square) -> &Option<Piece> {
        &self.0[sq.index()]
    }

    pub fn set(&mut self, sq: Square, pc: Option<Piece>) {
        self.0[sq.index()] = pc;
    }
}

impl Default for PieceGrid {
    fn default() -> Self {
        PieceGrid([None; 144])
    }
}

impl fmt::Debug for PieceGrid {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "PieceGrid {{ ")?;

        for pc in self.0.iter() {
            write!(fmt, "{:?} ", pc)?;
        }
        write!(fmt, "}}")
    }
}

/// Represents a state of the game
#[derive(Debug, Clone)]
pub struct Position {
    board: PieceGrid,
    hand: Hand,
    ply: u16,
    side_to_move: Color,
    move_history: Vec<MoveRecord>,
    sfen_history: Vec<(String, u16)>,
    occupied_bb: BitBoard,
    color_bb: [BitBoard; 3],
    game_status: Outcome,
    pub type_bb: [BitBoard; 7],
}

impl Position {
    /// Creates a new instance of `Position` with an empty board.
    pub fn new() -> Position {
        Default::default()
    }

    /// Returns a piece at the given square.
    pub fn piece_at(&self, sq: Square) -> &Option<Piece> {
        self.board.get(sq)
    }

    /// Returns a bitboard containing pieces of the given player.
    pub fn player_bb(&self, c: Color) -> &BitBoard {
        &self.color_bb[c.index()]
    }

    /// Returns the number of the given piece in hand.
    pub fn hand(&self, p: Piece) -> u8 {
        self.hand.get(p)
    }

    /// Returns the side to make a move next.
    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    /// Returns the number of plies already completed by the current state.
    pub fn ply(&self) -> u16 {
        self.ply
    }

    /// Returns a history of all moves made since the beginning of the game.
    pub fn move_history(&self) -> &[MoveRecord] {
        &self.move_history
    }

    pub fn outcome(&self) -> &Outcome {
        return &self.game_status;
    }

    /// Returns all legal moves where piece can be moved.
    pub fn legal_moves(&self, square: &Square) -> BitBoard {
        let my_moves = self.non_legal_moves(&square);
        let pinned_moves = self.pinned_moves(&square);
        let check_moves = self.check_moves(&square);
        if check_moves.len() > 0 {
            let piece = self.piece_at(*square).unwrap();
            let king = self.find_king(piece.color).unwrap();
            if king == *square {
                let enemy_moves = self.enemy_moves(&square);
                return &my_moves & &!&enemy_moves;
            } else {
                return self.fix_pin(square, &pinned_moves, check_moves, my_moves);
            }
        }

        return self.fix_pin(square, &pinned_moves, check_moves, my_moves);
    }

    /// Returns all non-legal moves.
    fn non_legal_moves(&self, square: &Square) -> BitBoard {
        let piece = self.piece_at(*square);
        match piece {
            Some(i) => self.move_candidates(*square, *i, MoveType::Plinth),
            None => EMPTY_BB,
        }
    }

    /// Returns Fixer struct, who has fix for pin(if it exist).
    fn pinned_moves(&self, square: &Square) -> Fixer {
        let piece = self.piece_at(*square);
        match piece {
            Some(i) => {
                let ksq = self.find_king(i.color);
                if ksq.is_none() {
                    return Fixer::default();
                    //return BitBoard::empty();
                }
                let ksq = ksq.unwrap();
                let mut pin = Fixer::default();
                for s in [
                    (
                        PieceType::Queen,
                        get_sliding_attacks(PieceType::Queen, ksq, EMPTY_BB),
                    ),
                    (
                        PieceType::Rook,
                        get_sliding_attacks(PieceType::Rook, ksq, EMPTY_BB),
                    ),
                    (
                        PieceType::Bishop,
                        get_sliding_attacks(PieceType::Bishop, ksq, EMPTY_BB),
                    ),
                    /*(
                        PieceType::Pawn,
                        get_non_sliding_attacks(PieceType::Pawn, ksq, i.color.flip()),
                    )*/
                ]
                .iter()
                {
                    // this is enemy
                    let bb = &(&self.type_bb[s.0.index()] & &self.color_bb[i.color.flip().index()])
                        & &s.1;
                    for psq in bb {
                        // this piece is pinned
                        let mut pinned =
                            &(&between(ksq, psq) & &self.occupied_bb) & &!&self.color_bb[2];
                        // this is fix for pin
                        //let fixed = &(&between(psq, ksq) & &!&pinned) | &bb;
                        if pinned.count() == 1
                            && (&pinned & &self.color_bb[i.color.index()]).is_any()
                        {
                            pin.fix = &(&between(psq, ksq) & &!&pinned) | &bb;
                            pin.square = pinned.pop_reverse();
                            if &pin.square.unwrap() == square {
                                return pin;
                            } else {
                                pin.fix = EMPTY_BB;
                                pin.square = None;
                            }
                        }
                    }
                }
                pin
            }

            None => Fixer::default(),
        }
    }

    /// Returns a BitBoard of all squares at which a piece of the given color is pinned.
    pub fn pinned_bb(&self, c: Color) -> BitBoard {
        let mut bb = EMPTY_BB;
        for sq in self.color_bb[c.index()] {
            let pinned = self.pinned_moves(&sq);
            match pinned.square {
                Some(p) => {
                    bb |= p;
                }
                None => (),
            }
        }
        bb
    }

    /// Returns Vector of all checks.
    fn check_moves(&self, square: &Square) -> Vec<BitBoard> {
        let piece = self.piece_at(*square);
        match piece {
            Some(i) => {
                let ksq = self.find_king(i.color);
                if ksq.is_none() {
                    return vec![];
                }
                let ksq = ksq.unwrap();
                let mut all: Vec<BitBoard> = vec![];
                for s in [
                    (
                        PieceType::Queen,
                        self.move_candidates(
                            ksq,
                            Piece {
                                piece_type: PieceType::Queen,
                                color: i.color,
                            },
                            MoveType::Plinth,
                        ),
                    ),
                    (
                        PieceType::Rook,
                        self.move_candidates(
                            ksq,
                            Piece {
                                piece_type: PieceType::Rook,
                                color: i.color,
                            },
                            MoveType::Plinth,
                        ),
                    ),
                    (
                        PieceType::Bishop,
                        self.move_candidates(
                            ksq,
                            Piece {
                                piece_type: PieceType::Bishop,
                                color: i.color,
                            },
                            MoveType::Plinth,
                        ),
                    ),
                    (
                        PieceType::Knight,
                        self.move_candidates(
                            ksq,
                            Piece {
                                piece_type: PieceType::Knight,
                                color: i.color,
                            },
                            MoveType::Plinth,
                        ),
                    ),
                ]
                .iter()
                {
                    // this is enemy
                    let bb = &(&self.type_bb[s.0.index()] & &self.color_bb[i.color.flip().index()])
                        & &s.1;
                    for psq in bb {
                        // this is fix for check
                        let fix = &between(ksq, psq);
                        all.push(fix | &bb);
                    }
                }
                all
            }

            None => vec![],
        }
    }

    /// Checks if the king with the given color is in check.
    pub fn in_check(&self, c: Color) -> bool {
        let king = &self.find_king(c);
        if let Some(k) = king {
            let check_moves = self.check_moves(k);
            if check_moves.len() > 0 {
                return true;
            } else {
                return false;
            }
        }
        false
    }

    /// Checks if given color is in checkmate.
    pub fn is_checkmate(&self, color: &Color) -> bool {
        let king = self.find_king(*color);
        match king {
            Some(k) => {
                if !self.in_check(*color) {
                    return false;
                }
                let king_moves = self.legal_moves(&k);
                if !king_moves.is_any() {
                    let mut final_moves = EMPTY_BB;
                    for sq in self.color_bb[color.index()] {
                        final_moves |= &self.legal_moves(&sq);
                    }
                    if final_moves.is_any() {
                        return false;
                    }
                    return true;
                }
                return false;
            }
            None => {
                return false;
            }
        }
    }

    /// Returns BitBoard of all moves after fixing pin.
    fn fix_pin(
        &self,
        sq: &Square,
        fixer: &Fixer,
        checks: Vec<BitBoard>,
        my_moves: BitBoard,
    ) -> BitBoard {
        let piece = self.piece_at(*sq).unwrap();
        match fixer.square {
            Some(_square) => {
                if checks.len() == 1 {
                    let checks = checks.get(0).unwrap();
                    return checks & &fixer.fix;
                } else if checks.len() > 1 {
                    return EMPTY_BB;
                }
                return &fixer.fix & &my_moves;
            }
            None => {
                let mut my_moves = my_moves;
                let enemy_moves = self.enemy_moves(&self.find_king(piece.color).unwrap());
                if piece.piece_type == PieceType::King {
                    my_moves = &my_moves & &!&enemy_moves;
                    return my_moves;
                } else if checks.len() > 1 {
                    return EMPTY_BB;
                }
                for bb in checks.iter() {
                    my_moves &= bb;
                }
                return my_moves;
            }
        }
    }

    /// Returns BitBoard of all moves by color.
    fn color_moves(&self, c: &Color) -> BitBoard {
        let mut all = EMPTY_BB;
        for sq in self.color_bb[c.index()] {
            let piece = self.piece_at(sq);
            let moves = self.move_candidates(
                sq,
                piece.unwrap(),
                MoveType::NoKing {
                    king: self.find_king(c.flip()).unwrap(),
                },
            );
            all |= &moves;
        }
        all
    }

    /// Returns BitBoard of all moves by opponent.
    fn enemy_moves(&self, square: &Square) -> BitBoard {
        let piece = self.piece_at(*square);
        match piece {
            Some(i) => self.color_moves(&i.color.flip()),
            None => EMPTY_BB,
        }
    }

    /// Sets a piece at the given square.
    fn set_piece(&mut self, sq: Square, p: Option<Piece>) {
        self.board.set(sq, p);
    }

    fn find_king(&self, c: Color) -> Option<Square> {
        let mut bb = &self.type_bb[PieceType::King.index()] & &self.color_bb[c.index()];
        if bb.is_any() {
            bb.pop_reverse()
        } else {
            None
        }
    }

    /// Saves position in sfen_history
    fn log_position(&mut self) {
        let mut sfen = self.generate_sfen().split(' ').take(3).join(" ");
        let in_check = self.in_check(self.side_to_move());
        let continuous_check = if in_check {
            let past = if self.sfen_history.len() >= 2 {
                let record = self.sfen_history.get(self.sfen_history.len() - 2).unwrap();
                record.1
            } else {
                0
            };
            past + 1
        } else {
            0
        };
        if self.move_history.len() > 0 {
            sfen.push_str(format!(" {} ", self.ply()).as_str());
            sfen.push_str(&self.move_history.last().unwrap().to_sfen());
        }

        self.sfen_history.push((sfen, continuous_check));
    }

    /////////////////////////////////////////////////////////////////////////
    // Making a move
    /////////////////////////////////////////////////////////////////////////

    /// Makes the given move. Returns `Err` if the move is invalid or any special condition is met.

    pub fn make_move(&mut self, m: Move) -> Result<Outcome, MoveError> {
        let mut promoted = false;
        let stm = self.side_to_move();
        let opponent = stm.flip();
        let (from, to) = m.info();
        let moved = self
            .piece_at(from)
            .ok_or(MoveError::Inconsistent("No piece found"))?;
        let captured = *self.piece_at(to);
        let outcome = Outcome::Checkmate { color: opponent };

        if moved.color != stm {
            return Err(MoveError::Inconsistent(
                "The piece is not for the side to move",
            ));
        } else if self.game_status == outcome {
            return Err(MoveError::Inconsistent("Match is over."));
        }

        match captured {
            Some(_i) => {
                if moved.piece_type == PieceType::Pawn {
                    if to.in_promotion_zone(moved.color) {
                        promoted = true;
                    }
                }
            }
            None => {
                if moved.piece_type == PieceType::Pawn {
                    if to.in_promotion_zone(moved.color) {
                        promoted = true;
                    }
                }
                ();
            }
        }

        if !(&self.legal_moves(&from) & to).is_any() {
            return Err(MoveError::Inconsistent("The piece cannot move to there"));
        }

        let placed = if promoted {
            match moved.promote() {
                Some(promoted) => promoted,
                None => {
                    return Err(MoveError::Inconsistent("This type of piece cannot promote"));
                }
            }
        } else {
            moved
        };

        self.set_piece(from, None);
        self.set_piece(to, Some(placed));
        self.occupied_bb ^= from;
        self.occupied_bb ^= to;
        self.type_bb[moved.piece_type.index()] ^= from;
        self.type_bb[placed.piece_type.index()] ^= to;
        self.color_bb[moved.color.index()] ^= from;
        self.color_bb[placed.color.index()] ^= to;

        if let Some(ref cap) = captured {
            self.occupied_bb ^= to;
            self.type_bb[cap.piece_type.index()] ^= to;
            self.color_bb[cap.color.index()] ^= to;

            //self.hand.increment(pc);
        }

        self.side_to_move = opponent;
        self.ply += 1;

        let move_record = MoveRecord::Normal {
            from,
            to,
            placed,
            captured,
            promoted,
        };

        self.move_history.push(move_record);

        self.log_position();
        self.detect_repetition()?;
        self.detect_insufficient_material()?;

        if self.is_checkmate(&self.side_to_move) {
            return Ok(Outcome::Checkmate {
                color: self.side_to_move.flip(),
            });
        } else if self.in_check(self.side_to_move) {
            return Ok(Outcome::Check {
                color: self.side_to_move,
            });
        } else if (&self.color_bb[self.side_to_move.flip().index()]
            & &self.type_bb[PieceType::King.index()])
            .count()
            == 0
        {
            return Ok(Outcome::Checkmate {
                color: self.side_to_move.flip(),
            });
        }
        self.is_stalemate(&self.side_to_move)?;
        Ok(Outcome::MoveOk)
    }

    pub fn is_stalemate(&self, color: &Color) -> Result<(), MoveError> {
        for pt in PieceType::iter() {
            let pieces = &self.color_bb[color.index()] & &self.type_bb[pt.index()];
            for p in pieces {
                let moves = self.legal_moves(&p);
                if moves.count() > 0 {
                    return Ok(());
                }
            }
        }
        Err(MoveError::DrawByStalemate)
    }

    /// Detecting insufficient material.
    pub fn detect_insufficient_material(&self) -> Result<(), MoveError> {
        let major = [PieceType::Rook, PieceType::Queen];
        let minor = [PieceType::Knight, PieceType::Bishop];
        if self.occupied_bb.count() == 2 {
            return Err(MoveError::DrawByInsufficientMaterial);
        }
        for c in Color::iter() {
            let mut bb = EMPTY_BB;
            for i in major {
                bb |= &(&self.color_bb[c.index()] & &self.type_bb[i.index()])
            }
            if bb.is_any() {
                return Ok(());
            }
            for i in minor {
                bb |= &(&self.color_bb[c.index()] & &self.type_bb[i.index()])
            }
            if bb.count() == 1 && bb.count() == 0 {
                continue;
            }

            return Ok(());
        }
        Err(MoveError::DrawByInsufficientMaterial)
    }

    /// Undoes the last move.
    pub fn unmake_move(&mut self) -> Result<(), MoveError> {
        if self.move_history.is_empty() {
            // TODO: error?
            return Ok(());
        }

        let last = self.move_history.pop().unwrap();
        match last {
            MoveRecord::Normal {
                from,
                to,
                ref placed,
                ref captured,
                promoted,
            } => {
                if *self.piece_at(from) != None {
                    return Err(MoveError::Inconsistent(
                        "`from` of the move is filled by another piece",
                    ));
                }

                let moved = if promoted {
                    match placed.unpromote() {
                        Some(unpromoted) => unpromoted,
                        None => return Err(MoveError::Inconsistent("Cannot unpromoted the piece")),
                    }
                } else {
                    *placed
                };
                if *self.piece_at(to) != Some(*placed) {
                    return Err(MoveError::Inconsistent(
                        "Expected piece is not found in `to`",
                    ));
                }

                self.set_piece(from, Some(moved));
                self.set_piece(to, *captured);
                self.occupied_bb ^= from;
                self.occupied_bb ^= to;
                self.type_bb[moved.piece_type.index()] ^= from;
                self.type_bb[placed.piece_type.index()] ^= to;
                self.color_bb[moved.color.index()] ^= from;
                self.color_bb[placed.color.index()] ^= to;

                if let Some(ref cap) = *captured {
                    self.occupied_bb ^= to;
                    self.type_bb[cap.piece_type.index()] ^= to;
                    self.color_bb[cap.color.index()] ^= to;
                }
            }
            _ => {
                return Ok(());
            }
        };

        self.side_to_move = self.side_to_move.flip();
        self.ply -= 1;
        self.sfen_history.pop();

        Ok(())
    }

    /// Returns a list of squares to where the given piece at the given square can move.
    pub fn move_candidates(&self, sq: Square, p: Piece, move_list: MoveType) -> BitBoard {
        let blockers = move_list.blockers(&self, &p.color);
        let bb = match p.piece_type {
            PieceType::Rook => get_sliding_attacks(PieceType::Rook, sq, blockers),
            PieceType::Bishop => get_sliding_attacks(PieceType::Bishop, sq, blockers),
            PieceType::Queen => get_sliding_attacks(PieceType::Queen, sq, blockers),
            PieceType::Knight => get_non_sliding_attacks(PieceType::Knight, sq, p.color),
            PieceType::Pawn => get_non_sliding_attacks(PieceType::Pawn, sq, p.color),
            PieceType::King => get_non_sliding_attacks(PieceType::King, sq, p.color),
            _ => EMPTY_BB,
        };
        move_list.moves(&self, bb, p, sq)
    }

    pub fn play(&mut self, from: &str, to: &str) -> Result<&Outcome, SfenError> {
        let from_: Square;
        let to_: Square;
        match Square::from_sfen(from) {
            Some(i) => from_ = i,
            None => {
                return Err(SfenError::IllegalPieceFound);
            }
        };
        match Square::from_sfen(to) {
            Some(i) => to_ = i,
            None => {
                return Err(SfenError::IllegalPieceFound);
            }
        };
        let m = Move::Normal {
            from: from_,
            to: to_,
            promote: false,
        };
        let outcome = self.make_move(m);
        match outcome {
            Ok(i) => {
                self.game_status = i;
            }
            Err(error) => match error {
                MoveError::RepetitionDraw => self.game_status = Outcome::DrawByRepetition,
                MoveError::Draw => self.game_status = Outcome::Draw,
                MoveError::DrawByInsufficientMaterial => self.game_status = Outcome::DrawByMaterial,
                MoveError::DrawByStalemate => self.game_status = Outcome::Stalemate,
                _ => {
                    return Err(SfenError::IllegalMove);
                }
            },
        }
        return Ok(self.outcome());
    }

    fn detect_repetition(&self) -> Result<(), MoveError> {
        if self.sfen_history.len() < 9 {
            return Ok(());
        }

        let cur = self.sfen_history.last().unwrap();
        let cur_s = cur.0.split(" ").last();

        let mut cnt = 0;
        for (i, entry) in self.sfen_history.iter().rev().enumerate() {
            let entry_s = entry.0.split(" ").last();
            if entry_s == cur_s {
                cnt += 1;

                if cnt == 3 {
                    let prev = self.sfen_history.get(self.sfen_history.len() - 2).unwrap();

                    if cur.1 * 2 >= (i as u16) {
                        return Err(MoveError::Draw);
                    } else if prev.1 * 2 >= (i as u16) {
                        return Err(MoveError::Draw);
                    } else {
                        return Err(MoveError::RepetitionDraw);
                    }
                }
            }
        }

        Ok(())
    }

    /////////////////////////////////////////////////////////////////////////
    // SFEN serialization / deserialization
    /////////////////////////////////////////////////////////////////////////

    /// Parses the given SFEN string and updates the game state.
    pub fn set_sfen(&mut self, sfen_str: &str) -> Result<(), SfenError> {
        let mut parts = sfen_str.split_whitespace();

        // Build the initial position, all parts are required.
        parts
            .next()
            .ok_or(SfenError::MissingDataFields)
            .and_then(|s| self.parse_sfen_board(s))?;
        parts
            .next()
            .ok_or(SfenError::MissingDataFields)
            .and_then(|s| self.parse_sfen_stm(s))?;
        parts
            .next()
            .ok_or(SfenError::MissingDataFields)
            .and_then(|s| self.parse_sfen_hand(s))?;
        parts
            .next()
            .ok_or(SfenError::MissingDataFields)
            .and_then(|s| self.parse_sfen_ply(s))?;
        self.sfen_history.clear();
        self.log_position();
        /*
        if self.in_check(self.side_to_move.flip()) {
            return Err(SfenError::IllegalFirstMove);
        }
        */
        // Make moves following the initial position, optional.
        if let Some("moves") = parts.next() {
            for m in parts {
                if let Some(m) = Move::from_sfen(m) {
                    // Stop if any error occurrs.
                    match self.make_move(m) {
                        Ok(_) => {
                            self.log_position();
                        }
                        Err(_) => break,
                    }
                } else {
                    return Err(SfenError::IllegalMove);
                }
            }
        }

        Ok(())
    }

    pub fn set_sfen_history(&mut self, history: Vec<(String, u16)>) {
        self.sfen_history = history;
    }

    pub fn set_move_history(&mut self, history: Vec<MoveRecord>) {
        self.move_history = history;
    }

    /// Converts the current state into SFEN formatted string.
    pub fn to_sfen(&self) -> String {
        if self.sfen_history.is_empty() {
            return self.generate_sfen();
        }

        if self.move_history.is_empty() {
            return format!("{} {}", self.sfen_history.first().unwrap().0, self.ply);
        }

        let mut sfen = format!(
            "{} {} moves",
            &self.sfen_history.first().unwrap().0,
            self.ply - self.move_history.len() as u16
        );

        for m in self.move_history.iter() {
            sfen.push_str(&format!(" {}", &m.to_sfen()));
        }
        sfen
    }

    fn parse_sfen_stm(&mut self, s: &str) -> Result<(), SfenError> {
        self.side_to_move = match s {
            "b" => Color::Black,
            "w" => Color::White,
            _ => return Err(SfenError::IllegalSideToMove),
        };
        Ok(())
    }

    fn parse_sfen_hand(&mut self, s: &str) -> Result<(), SfenError> {
        if s == "-" {
            self.hand.clear();
            return Ok(());
        }

        let mut num_pieces: u8 = 0;
        for c in s.chars() {
            match c {
                n if n.is_digit(11) => {
                    if let Some(n) = n.to_digit(11) {
                        if num_pieces != 0 {
                            let num2 = format!("{}{}", num_pieces, n as u8).parse::<u8>().unwrap();
                            num_pieces = num2;
                            continue;
                        }
                        num_pieces = n as u8;
                    }
                }
                s => {
                    match Piece::from_sfen(s) {
                        Some(p) => self
                            .hand
                            .set(p, if num_pieces == 0 { 1 } else { num_pieces }),
                        None => return Err(SfenError::IllegalPieceType),
                    };
                    num_pieces = 0;
                }
            }
        }

        Ok(())
    }

    fn parse_sfen_ply(&mut self, s: &str) -> Result<(), SfenError> {
        self.ply = s.parse()?;
        Ok(())
    }
    fn parse_sfen_board(&mut self, s: &str) -> Result<(), SfenError> {
        let rows = s.split('/');

        self.occupied_bb = BitBoard::empty();
        self.color_bb = Default::default();
        self.type_bb = Default::default();

        for (i, row) in rows.enumerate() {
            if i >= 12 {
                return Err(SfenError::IllegalBoardState);
            }

            let mut j = 0;

            let mut is_promoted = false;
            for c in row.chars() {
                match c {
                    '+' => {
                        is_promoted = true;
                    }
                    '0' => {
                        let sq = Square::new(j, i as u8).unwrap();
                        self.set_piece(sq, None);
                        j += 1;
                    }
                    n if n.is_digit(11) => {
                        if let Some(n) = n.to_digit(11) {
                            for _ in 0..n {
                                if j >= 12 {
                                    return Err(SfenError::IllegalBoardState);
                                }

                                let sq = Square::new(j, i as u8).unwrap();

                                self.set_piece(sq, None);

                                j += 1;
                            }
                        }
                    }
                    s => match Piece::from_sfen(s) {
                        Some(mut piece) => {
                            if j >= 12 {
                                return Err(SfenError::IllegalBoardState);
                            }

                            if is_promoted {
                                if let Some(promoted) = piece.piece_type.promote() {
                                    piece.piece_type = promoted;
                                } else {
                                    return Err(SfenError::IllegalPieceType);
                                }
                            }

                            let sq = Square::new(j, i as u8).unwrap();
                            if piece.piece_type == PieceType::Plinth {
                                self.color_bb[piece.color.index()] |= sq;
                                continue;
                            }
                            self.set_piece(sq, Some(piece));
                            self.occupied_bb |= sq;
                            self.color_bb[piece.color.index()] |= sq;
                            self.type_bb[piece.piece_type.index()] |= sq;
                            j += 1;
                            is_promoted = false;
                        }
                        None => return Err(SfenError::IllegalPieceType),
                    },
                }
            }
        }

        Ok(())
    }

    pub fn generate_sfen(&self) -> String {
        fn add_num_space(num_spaces: i32, mut s: String) -> String {
            if num_spaces == 10 {
                s.push_str("55");
            } else if num_spaces == 11 {
                s.push_str("56");
            } else if num_spaces == 12 {
                s.push_str("57");
            } else if num_spaces > 0 {
                s.push_str(&num_spaces.to_string());
            }
            s
        }
        let board = (0..12)
            .map(|row| {
                let mut s = String::new();
                let mut num_spaces = 0;
                for file in 0..12 {
                    let sq = Square::new(file, row).unwrap();
                    match *self.piece_at(sq) {
                        Some(pc) => {
                            if num_spaces > 0 {
                                let mut _s = add_num_space(num_spaces, s);
                                s = _s;
                                num_spaces = 0;
                            }

                            if (&self.color_bb[2] & sq).is_any() {
                                if pc.piece_type == PieceType::Knight {
                                    s.push_str("L");
                                } else {
                                    ()
                                    //return Err(SfenError::IllegalPieceTypeOnPlinth);
                                }
                            }

                            s.push_str(&pc.to_string());
                        }
                        None => {
                            if (&self.color_bb[2] & sq).is_any() {
                                let mut _s = add_num_space(num_spaces, s);
                                s = _s;
                                num_spaces = 0;
                                s.push_str("L0");
                            } else {
                                num_spaces += 1;
                            }
                        }
                    }
                }

                if num_spaces > 0 {
                    let _s = add_num_space(num_spaces, s);
                    s = _s;
                    //num_spaces = 0;
                }

                s
            })
            .join("/");

        let color = if self.side_to_move == Color::Black {
            "b"
        } else {
            "w"
        };

        let mut hand = [Color::Black, Color::White]
            .iter()
            .map(|c| {
                PieceType::iter()
                    .filter(|pt| pt.is_hand_piece())
                    .map(|pt| {
                        let pc = Piece {
                            piece_type: pt,
                            color: *c,
                        };
                        let n = self.hand.get(pc);

                        if n == 0 {
                            "".to_string()
                        } else if n == 1 {
                            format!("{}", pc)
                        } else {
                            format!("{}{}", n, pc)
                        }
                    })
                    .join("")
            })
            .join("");

        if hand.is_empty() {
            hand = "-".to_string();
        }

        format!("{} {} {} {}", board, color, hand, self.ply)
    }
    //////////////////////////////////////////////
    /// Hand from shop is recommended to call this function.
    pub fn set_hand(&mut self, s: &str) {
        self.hand.set_hand(&s);
    }

    /// Get hand for specific color (in order K..P).
    pub fn get_hand(&self, c: Color) -> String {
        return self.hand.to_sfen(c);
    }

    /////////////////////////////////////////////////////////////////////////////
    // Deploy part in Shuuro.
    /////////////////////////////////////////////////////////////////////////////

    /// Squares for king to be placed.
    fn king_squares(&self, c: &Color) -> BitBoard {
        let files = ['d', 'e', 'f', 'g', 'h', 'i'];
        let mut bb = EMPTY_BB;
        let plinths = self.color_bb[Color::NoColor.index()];
        let mut all = |num: usize| -> BitBoard {
            for file in files {
                bb |= Square::from_sfen(&format!("{}{}", file, num)[..]).unwrap();
            }
            bb &= &!&plinths;
            bb
        };
        match *c {
            Color::Black => all(12),
            Color::White => all(1),
            Color::NoColor => EMPTY_BB,
        }
    }

    /// Available squares for selected piece.
    pub fn empty_squares(&self, p: Piece) -> BitBoard {
        let test = |p: Piece, list: [usize; 3]| -> BitBoard {
            for file in list {
                let mut bb = FILE_BB[file];
                bb &= &!&self.color_bb[p.color.index()];
                let plinths = self.color_bb[Color::NoColor.index()];
                if bb.is_empty() {
                    continue;
                }
                match p.piece_type {
                    PieceType::Knight => {
                        return bb;
                    }
                    PieceType::King => {
                        return self.king_squares(&p.color);
                    }
                    PieceType::Pawn => {
                        bb &= &!&plinths;
                        if bb.is_empty() {
                            continue;
                        } else if self.can_pawn_move(p) {
                            if file == 0 || file == 11 {
                                continue;
                            }
                            return bb;
                        } else {
                            return EMPTY_BB;
                        }
                    }
                    _ => {
                        bb &= &!&plinths;
                        if bb.is_empty() {
                            continue;
                        }
                        return bb;
                    }
                }
            }
            EMPTY_BB
        };
        let checks = self.checks(&p.color);
        if checks.is_any() {
            return checks;
        } else if !self.is_king_placed(p.color) && p.piece_type != PieceType::King {
            return EMPTY_BB;
        }
        match p.color {
            Color::White => test(p, [0, 1, 2]),
            Color::Black => test(p, [11, 10, 9]),
            Color::NoColor => EMPTY_BB,
        }
    }

    fn is_king_placed(&self, c: Color) -> bool {
        let king = &self.color_bb[c.index()] & &self.type_bb[PieceType::King.index()];
        if king.count() == 1 {
            return true;
        }
        false
    }

    /// Returns BitBoard for all safe squares for selected side.
    fn checks(&self, attacked_color: &Color) -> BitBoard {
        let king = &self.type_bb[PieceType::King.index()] & &self.color_bb[attacked_color.index()];
        if king.is_empty() {
            return EMPTY_BB;
        }
        let mut all;
        let occupied_bb = &self.occupied_bb | self.player_bb(Color::NoColor);
        for p in [PieceType::Queen, PieceType::Rook, PieceType::Bishop] {
            let bb = &self.type_bb[p.index()] & &self.color_bb[attacked_color.flip().index()];
            for i in bb {
                all = get_sliding_attacks(p, i, occupied_bb);
                if (&all & &king).is_any() {
                    match *attacked_color {
                        Color::White => {
                            let files = &FILE_BB[1] | &FILE_BB[2];
                            return &(&files & &all) & &!&king;
                        }
                        Color::Black => {
                            let files = &FILE_BB[9] | &FILE_BB[10];
                            return &(&files & &all) & &!&king;
                        }
                        Color::NoColor => {
                            return EMPTY_BB;
                        }
                    }
                }
            }
        }
        EMPTY_BB
    }

    /// Returns true if pawns can be placed on board.
    fn can_pawn_move(&self, p: Piece) -> bool {
        if self.is_hand_empty(&p.color, PieceType::Pawn) {
            true
        } else {
            false
        }
    }

    /// Returns true if hand with excluded piece is empty.
    pub fn is_hand_empty(&self, c: &Color, excluded: PieceType) -> bool {
        for pt in PieceType::iter() {
            if pt != excluded {
                let counter = self.hand.get(Piece {
                    piece_type: pt,
                    color: *c,
                });
                if counter != 0 {
                    return false;
                }
            }
        }
        true
    }

    /// Placing piece on square.
    pub fn place(&mut self, p: Piece, sq: Square) {
        if self.hand.get(p) > 0 {
            if (&self.empty_squares(p) & sq).is_any() {
                self.update_bb(p, sq);
                self.hand.decrement(p);
                let move_record = MoveRecord::Put { to: sq, piece: p };
                let sfen = self.generate_sfen().split(" ").next().unwrap().to_string();
                let hand = {
                    let s = self.get_hand(Color::White) + &self.get_hand(Color::Black)[..];
                    if s.len() == 0 {
                        String::from(" ")
                    } else {
                        s
                    }
                };
                self.ply += 1;
                let ply = self.ply();

                self.move_history.push(move_record.clone());
                if !self.is_hand_empty(&p.color.flip(), PieceType::Plinth) {
                    self.side_to_move = p.color.flip();
                }
                self.sfen_history.push((
                    format!(
                        "{}_{}_{}_{}_{}",
                        &move_record.to_sfen(),
                        &sfen.clone(),
                        hand,
                        self.side_to_move.to_string(),
                        ply
                    ),
                    1,
                ));
            }
        }
    }

    /// Generating random plinths.
    pub fn generate_plinths(&mut self) {
        self.color_bb[Color::NoColor.index()] = generate_plinths();
    }

    fn update_bb(&mut self, p: Piece, sq: Square) {
        self.set_piece(sq, Some(p));
        self.occupied_bb |= sq;
        self.color_bb[p.color.index()] |= sq;
        self.type_bb[p.piece_type.index()] |= sq;
    }

    /// Getting sfen_history
    pub fn get_sfen_history(&self) -> &Vec<(String, u16)> {
        &self.sfen_history
    }

    /// Getting move_history
    pub fn get_move_history(&self) -> &Vec<MoveRecord> {
        &self.move_history
    }
}

/////////////////////////////////////////////////////////////////////////////
// Trait implementations
/////////////////////////////////////////////////////////////////////////////

impl Default for Position {
    fn default() -> Position {
        Position {
            side_to_move: Color::Black,
            board: PieceGrid([None; 144]),
            hand: Default::default(),
            ply: 0,
            move_history: Default::default(),
            sfen_history: Default::default(),
            occupied_bb: Default::default(),
            color_bb: Default::default(),
            type_bb: Default::default(),
            game_status: Outcome::MoveOk,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "+---+---+---+---+---+---+---+---+---+---+---+---+")?;

        for file in (0..12).rev() {
            write!(f, "|")?;
            for row in 0..12 {
                if let Some(ref piece) = *self.piece_at(Square::new(row, file).unwrap()) {
                    write!(f, "{}", piece.to_string())?;
                    if (&self.color_bb[2] & Square::new(row, file).unwrap()).is_any() {
                        write!(f, " L|")?;
                    } else {
                        write!(f, "  |")?;
                    }
                } else {
                    if (&self.color_bb[2] & Square::new(row, file).unwrap()).is_any() {
                        write!(f, "{:>3}|", "L")?;
                    } else {
                        write!(f, "   |")?;
                    }
                }
            }

            //writeln!(f, " {}", (('a' as usize + row as usize) as u8) as char)?;
            writeln!(f, "\n+---+---+---+---+---+---+---+---+---+---+---+---+")?;
        }
        writeln!(f, "a   b   c   d   e   f   g   h   i   j   k   l")?;
        writeln!(
            f,
            "Side to move: {}",
            if self.side_to_move == Color::Black {
                "Black"
            } else {
                "White"
            }
        )?;

        let fmt_hand = |color: Color, f: &mut fmt::Formatter| -> fmt::Result {
            for pt in PieceType::iter().filter(|pt| pt.is_hand_piece()) {
                let pc = Piece {
                    piece_type: pt,
                    color,
                };
                let n = self.hand.get(pc);

                if n > 0 {
                    write!(f, "{}{} ", pc, n)?;
                }
            }
            Ok(())
        };
        write!(f, "Hand (Black): ")?;
        fmt_hand(Color::Black, f)?;
        writeln!(f)?;

        write!(f, "Hand (White): ")?;
        fmt_hand(Color::White, f)?;
        writeln!(f)?;

        write!(f, "Ply: {}", self.ply)?;

        Ok(())
    }
}
#[cfg(test)]
pub mod tests {
    use crate::{consts::*, Move, MoveError};
    use crate::{init, Color, Piece, PieceType, Position, Square};
    pub const START_POS: &str = "KR55/57/57/57/57/57/57/57/57/57/57/kr55 b - 1";

    fn setup() {
        init();
    }

    #[test]
    fn piece_exist() {
        setup();
        let mut pos = Position::new();
        pos.set_sfen(START_POS).unwrap();
        let sq = Square::from_index(132).unwrap();
        let piece = Piece {
            piece_type: PieceType::King,
            color: Color::Black,
        };

        assert_eq!(Some(piece), *pos.piece_at(sq));
    }

    #[test]
    fn player_bb() {
        setup();

        let cases: &[(&str, &[Square], &[Square], &[Square])] = &[
            (
                "BBQ8K/57/2L03L05/4R7/3L08/57/57/5ppp4/nnq/L0L0L09/57/1L05k4 b - 1",
                &[A9, B9, C9, F8, G8, H8, H12],
                &[A1, B1, C1, L1, E4],
                &[G3, B12, C3, D5],
            ),
            (
                "9K2/57/6PPPP2/3Q6N1/57/57/57/5ppp4/7qk3/57/57/57 b P 1",
                &[H9, I9, F8, G8, H8],
                &[G3, H3, I3, J3, D4, K4, J1],
                &[],
            ),
        ];

        let mut pos = Position::new();
        for case in cases {
            pos.set_sfen(case.0).expect("faled to parse SFEN string");
            let blue = pos.player_bb(Color::Black);
            let red = pos.player_bb(Color::White);

            assert_eq!(case.1.len(), blue.count() as usize);
            for sq in case.1 {
                assert!((blue & *sq).is_any());
            }

            assert_eq!(case.2.len(), red.count() as usize);
            for sq in case.2 {
                assert!((red & *sq).is_any());
            }

            for sq in case.3 {
                assert!((&pos.color_bb[2] & *sq).is_any())
            }
        }
    }

    #[test]
    fn pinned_bb() {
        setup();

        let cases: &[(&str, &[Square], &[Square])] = &[
            (
                "5NNQK3/8B3/57/57/57/8r3/57/57/pp55/1k55/57/57 w - 1",
                &[],
                &[I2],
            ),
            (
                "5NNQK3/8B3/8R3/57/57/8r3/57/57/pp55/1k55/57/57 w - 1",
                &[],
                &[],
            ),
            (
                "12/57/2K9/PPPPPP/57/57/2R2B6/57/2rn8/2k3q2R2/57/57 b - 1",
                &[C9, D9, G10],
                &[],
            ),
        ];

        let mut pos = Position::new();
        for case in cases {
            pos.set_sfen(case.0).expect("faled to parse SFEN string");
            let blue = pos.pinned_bb(Color::Black);
            let red = pos.pinned_bb(Color::White);

            assert_eq!(case.1.len(), blue.count());
            for sq in case.1 {
                assert!((&blue & *sq).is_any());
            }

            assert_eq!(case.2.len(), red.count());
            for sq in case.2 {
                assert!((&red & *sq).is_any());
            }
        }
    }

    #[test]
    fn legal_moves_pawn() {
        setup();
        let cases = [(
            "4K1Q4LN/4L07/2L09/6P5/6q5/55L01/57/9L02/6L05/L01L09/5p6/6k5 w - 11",
            G4,
            0,
        )];
        for case in cases {
            let mut pos = Position::default();
            pos.set_sfen(case.0).expect("failed to parse sfen string");
            let legal_moves = pos.legal_moves(&case.1);
            assert_eq!(legal_moves.count(), case.2);
        }
    }

    #[test]
    fn king_moves() {
        setup();
        let cases = [
            (
                "4K6LN/4L07/2L09/57/57/6Q3L01/6P5/5k3L02/6L05/L01L09/5p6/57 b - 19",
                4,
            ),
            (
                "4K6LN/4L07/2L09/57/57/6Q3L01/6P5/R4k3L02/6L05/L01L09/5p6/57 b - 19",
                3,
            ),
        ];
        for i in cases {
            let mut pos = Position::default();
            pos.set_sfen(i.0).expect("failed to parse sfen string");
            let legal_moves = pos.legal_moves(&F8);
            assert_eq!(legal_moves.count(), i.1);
        }
    }

    #[test]
    fn parse_sfen_hand() {
        setup();
        let mut pos = Position::new();
        pos.set_sfen("6KL04/3L08/57/57/9L02/4L07/6L04L0/3L08/57/57/1L055/57 b kr12pQ9P 1")
            .expect("failed to parse sfen string");
        assert_eq!(pos.hand(Piece::from_sfen('p').unwrap()), 12);
    }

    #[test]
    fn move_candidates() {
        setup();

        let mut pos = Position::new();
        pos.set_sfen("R3N7/4K7/57/57/57/57/57/bppp8/4k7/57/57/57 b - 1")
            .expect("failed to parse SFEN string");

        let mut sum = 0;
        for sq in Square::iter() {
            let pc = pos.piece_at(sq);

            if let Some(pc) = *pc {
                if pc.color == pos.side_to_move() {
                    println!(
                        "piece: {}, count: {}",
                        pc,
                        pos.move_candidates(sq, pc, crate::position::MoveType::Plinth)
                            .count(),
                    );
                    sum += pos
                        .move_candidates(sq, pc, crate::position::MoveType::Plinth)
                        .count();
                }
            }
        }

        assert_eq!(21, sum);
    }

    #[test]
    fn move_candidates_plinth() {
        setup();
        let cases = [
            ("f2", PieceType::Rook, Color::White, 13, "f3", "Live"),
            ("e7", PieceType::Knight, Color::Black, 7, "g8", "Live"),
            ("f6", PieceType::Pawn, Color::Black, 0, "f7", ""),
        ];
        let mut pos = Position::new();
        pos.set_sfen("57/5R5K/57/57/3b1L04k1/5p4b1/4n7/57/55R1/57/57/57 w - 1")
            .expect("failed to parse SFEN string");
        for case in cases {
            let bb = pos.move_candidates(
                Square::from_sfen(case.0).unwrap(),
                Piece {
                    piece_type: case.1,
                    color: case.2,
                },
                crate::position::MoveType::Plinth,
            );
            assert_eq!(case.3, bb.count());
            let result = pos.play(case.0, case.4);
            if let Ok(result) = result {
                assert_eq!(result.to_string(), case.5);
            } else {
                assert_eq!(result.is_ok(), false);
            }
        }
    }

    #[test]
    fn check_while_knight_on_plinth() {
        setup();
        let sfen = "4K5B1/5P2L03/57/2L02Q6/4L04L02/57/56L0/1L055/57/5Ln5L0/3p8/1q3kn5 b - 11";
        let mut pos = Position::new();
        pos.set_sfen(sfen).expect("failed to parse SFEN string");
        let legal_moves = pos.legal_moves(&F10);
        assert_eq!(legal_moves.count(), 6);
    }

    #[test]
    fn pawn_moves() {
        setup();
        let cases = [
            (
                "4K1Q4LN/4L07/2L09/57/6P5/55L01/57/9L02/6L05/L01L09/5p6/6k5 b - 12",
                "f11",
                "f10",
            ),
            (
                "2N1L0QKRL01N1/2P4P3P/55L01/57/57/L056/57/57/3L08/8L03/2pp2L05/4qLnk2r1b b - 15",
                "c11",
                "c10",
            ),
            (
                "2N1L0QKRL01N1/2P4P3P/55L01/57/57/L056/57/57/3L08/8L03/2pp2L05/4qLnk2r1b b - 15",
                "d11",
                "d10",
            ),
            (
                "4KN1Q4/4L0P1P1PP1/1P55/3L08/56L0/6L05/4L01L05/57/57/6L05/p7ppp1/L02q1k3r2 b - 16",
                "a11",
                "a10",
            ),
            ("2LNN2KNBB2/6L03P1/57/7L04/1L055/57/9L02/57/6L05/ppp2p2pQ2/pppL0p1ppp1pp/L02kq7 b - 29",
             "i11",
             "j10")
        ];
        let ng_cases = [(
            "4K1Q4LN/2P1L07/2L09/57/6P5/55L01/57/9L02/6L05/L01L09/5p6/6k5 w - 12",
            "c2",
            "c3",
        )];

        for case in cases {
            let mut position = Position::new();
            position
                .set_sfen(case.0)
                .expect("failed to parse sfen string");

            let played = position.play(case.1, case.2);
            assert!(played.is_ok());
        }

        for case in ng_cases {
            let mut position = Position::new();
            position
                .set_sfen(case.0)
                .expect("failed to parse sfen string");

            let played = position.play(case.1, case.2);
            assert!(played.is_err());
        }
    }

    #[test]
    fn pawn_captures_last_rank() {
        setup();
        let sfen =
            "4KN1Q4/4L0P1P1PP1/8r3/3L08/56L0/6L05/4L01L01q3/57/57/6L05/pP3k2ppp1/L01r9 w - 33";
        let mut position = Position::new();
        position
            .set_sfen(sfen)
            .expect("failed to parse sfen string");
        let pawn_moves = position.legal_moves(&B11);
        assert_eq!(pawn_moves.count(), 2);
        let result = position.play("b11", "c12");
        assert!(result.is_ok());
        assert_eq!(position.piece_at(C12).unwrap().piece_type, PieceType::Queen);
    }

    #[test]
    fn queen_moves_through() {
        setup();
        let cases = [
            (
                "2q1L0QKRL01N1/2P8P/7P2L01/57/57/L056/57/57/3L08/8L03/2pp2L05/5Lnk2r1b b - 20",
                C1,
                15,
            ),
            (
                "2B2KQ5/5R3P2/L06LN4/57/57/2L06L02/57/3L06L01/6L05/3p8/L04p1pp3/2n1rqk3b1 b - 17",
                F12,
                7,
            ),
        ];
        for case in cases {
            let mut position = Position::new();
            position
                .set_sfen(case.0)
                .expect("failed to parse sfen string");
            let queen_moves = position.legal_moves(&case.1);
            assert_eq!(queen_moves.count(), case.2);
        }
    }

    #[test]
    fn knight_jumps_move() {
        setup();
        let sfen = "4K1Q4LN/4L07/2L09/57/57/55L01/6P5/9L02/5kL05/L01L09/5p6/57 w - 17";
        let mut position = Position::new();
        position
            .set_sfen(sfen)
            .expect("failed to parse sfen string");

        let result = position.play("l1", "j2");
        assert!(result.is_ok());
    }

    #[test]
    fn in_check() {
        setup();

        let test_cases = [
            (
                "KQR9/1PPP8/57/57/57/57/57/57/57/57/1ppp8/qkb9 w - 1",
                false,
                true,
            ),
            (
                "5QR5/1K55/57/57/57/57/57/57/57/57/57/5k6 b - 1",
                true,
                false,
            ),
            (
                "2RNBKQBNR2/57/2PPPPPPPP2/57/57/57/57/57/57/2pppppppp2/57/2rnbkqbnr2 b - 1",
                false,
                false,
            ),
            (
                "RR5K4/7L04/QP55/7L04/57/57/57/nbq9/7q4/57/57/56k w - 1",
                false,
                false,
            ),
            ("KQP8/2n8/57/57/57/57/57/k11/57/57/57/57 w - 1", false, true),
        ];

        let mut pos = Position::new();
        for case in test_cases.iter() {
            pos.set_sfen(case.0).expect("failed to parse sfen string");
            assert_eq!(case.1, pos.in_check(Color::Black));
            assert_eq!(case.2, pos.in_check(Color::White));
        }
    }

    #[test]
    fn is_stalemate() {
        setup();
        let sfen = "57/2r9/K56/3q8/57/57/57/57/56k/57/57/57 b - 1";
        let mut pos = Position::new();
        pos.set_sfen(sfen).expect("failed to parse sfen string");
        let res = pos.play("d4", "c4");
        if let Ok(res) = res {
            assert_eq!(res.to_string(), "Stalemate");
        }
    }

    #[test]
    fn is_checkmate() {
        setup();
        let cases = [
            (
                "1K8r1/9rr1/57/57/57/57/57/57/57/k11/57/57 b - 1",
                true,
                Color::White,
            ),
            (
                "5RNB4/5K4r1/6B5/57/57/57/57/57/ppppp7/57/57/9k2 w - 1",
                false,
                Color::Black,
            ),
            (
                "12/57/7k3Q/57/57/KRn9/57/57/57/57/57/57 b - 1",
                false,
                Color::White,
            ),
            (
                "57/9K2/L06L04/57/57/2L06L02/57/3L05BL01/6LN5/4Qk6/L056/6n5 b - 69",
                true,
                Color::Black,
            ),
        ];
        for case in cases.iter() {
            let mut pos = Position::new();
            pos.set_sfen(case.0).expect("failed to parse SFEN string");
            assert_eq!(case.1, pos.is_checkmate(&case.2));
        }
    }

    #[test]
    fn repetition() {
        setup();

        let mut pos = Position::new();
        pos.set_sfen("57/57/PPPQP4K2/7RR3/57/57/57/4pp6/2kr8/57/57/57 b - 1")
            .expect("failed to parse SFEN string");
        for _ in 0..2 {
            assert!(pos.make_move(Move::new(D9, I9, false)).is_ok());
            assert!(pos.make_move(Move::new(H4, A4, false)).is_ok());
            assert!(pos.make_move(Move::new(I9, D9, false)).is_ok());
            assert!(pos.make_move(Move::new(A4, H4, false)).is_ok());
        }
        assert_eq!(
            Some(MoveError::RepetitionDraw),
            pos.make_move(Move::new(D9, I9, false)).err()
        );
    }

    #[test]
    fn make_move() {
        setup();

        let base_sfen = "57/3KRRB5/5PP5/57/57/57/57/qbbn8/57/6k5/57/57 w K2RB2P 1";
        let test_cases = [
            (
                D2,
                E1,
                false,
                true,
                "4K7/4RRB5/5PP5/57/57/57/57/qbbn8/57/6k5/57/57 b K2RB2P 2",
            ),
            (
                E2,
                E7,
                false,
                true,
                "57/3K1RB5/5PP5/57/57/57/4R7/qbbn8/57/6k5/57/57 b K2RB2P 2",
            ),
            (
                G2,
                I4,
                false,
                true,
                "57/3KRR6/5PP5/8B3/57/57/57/qbbn8/57/6k5/57/57 b K2RB2P 2",
            ),
            (
                F2,
                F1,
                false,
                true,
                "5R6/3KR1B5/5PP5/57/57/57/57/qbbn8/57/6k5/57/57 b K2RB2P 2",
            ),
            (G3, H3, false, false, base_sfen),
        ];

        for case in test_cases.iter() {
            let mut pos = Position::new();
            pos.set_sfen(base_sfen)
                .expect("failed to parse SFEN string");
            let move_ = Move::Normal {
                from: case.0,
                to: case.1,
                promote: case.2,
            };
            assert_eq!(case.3, pos.make_move(move_).is_ok());
            assert_eq!(case.4, pos.generate_sfen());
        }

        let mut pos = Position::new();
        // Leaving the checked king is illegal.
        pos.set_sfen("57/1K8RR/57/57/57/r9k1/57/57/57/57/57/57 b kr 1")
            .expect("failed to parse SFEN string");
        let move_ = Move::Normal {
            from: A6,
            to: A1,
            promote: false,
        };
        assert!(pos.make_move(move_).is_err());

        pos.set_sfen("7K4/1RR9/57/57/57/r9k1/57/57/57/57/57/57 b kr 1")
            .expect("failed to parse SFEN string");
        let move_ = Move::Normal {
            from: K6,
            to: K5,
            promote: false,
        };
        assert!(pos.make_move(move_).is_ok());
    }

    #[test]
    fn pawn_promoted() {
        setup();
        let mut pos = Position::new();
        pos.set_sfen("7K4/1L01p1N6/57/5R2B3/1L05L04/9L02/57/5L06/57/7L04/5L04L01/2r2k1n4 b - 28")
            .expect("failed to parse SFEN string");
        let move_ = Move::from_sfen("d2_d1").unwrap();
        assert!(pos.make_move(move_).is_ok());
    }

    #[test]
    fn make_moves() {
        setup();
        let mut pos = Position::new();
        pos.set_sfen("6K5/57/57/6k5/57/PL055/57/p56/57/57/57/57 w - 1")
            .expect("err");
        let m = Move::Normal {
            from: G1,
            to: G2,
            promote: false,
        };
        let m2 = Move::Normal {
            from: G4,
            to: G5,
            promote: false,
        };
        let m3 = Move::Normal {
            from: G5,
            to: G4,
            promote: false,
        };
        assert_eq!(true, pos.make_move(m).is_ok());
        assert_eq!(true, pos.make_move(m2).is_ok());
        assert_eq!(false, pos.make_move(m3).is_ok());
    }

    #[test]
    fn unmake_move() {
        setup();

        let mut pos = Position::new();
        let base_sfen = "RRQNN3K3/PP1P4PP2/2P9/57/57/B56/1n3q4PP/4qq6/rq3r5r/57/4k7/57 b 3q3r 1";
        pos.set_sfen(base_sfen)
            .expect("failed to parse SFEN string");
        let base_state = format!("{}", pos);
        let test_cases = [
            Move::Normal {
                from: E8,
                to: E5,
                promote: false,
            },
            Move::Normal {
                from: L9,
                to: I9,
                promote: false,
            },
            Move::Normal {
                from: B7,
                to: D6,
                promote: false,
            },
            Move::Normal {
                from: F7,
                to: F4,
                promote: false,
            },
            Move::Normal {
                from: E8,
                to: E2,
                promote: false,
            },
            Move::Normal {
                from: A9,
                to: A7,
                promote: false,
            },
        ];

        for case in test_cases.iter() {
            pos.make_move(*case)
                .unwrap_or_else(|_| panic!("failed to make a move: {}", case));
            pos.unmake_move()
                .unwrap_or_else(|_| panic!("failed to unmake a move: {}", case));
            assert_eq!(
                base_sfen,
                pos.to_sfen(),
                "{}",
                format!("sfen unmatch for {}", case).as_str()
            );
            assert_eq!(
                base_state,
                format!("{}", pos),
                "{}",
                format!("state unmatch for {}", case).as_str()
            );
        }
    }

    #[test]
    fn set_sfen_normal() {
        setup();

        let mut pos = Position::new();

        pos.set_sfen("2RNBKQBNR2/57/2PPPPPPPP2/57/57/57/57/57/57/2pppppppp2/57/2rnbkqbnr2 b - 1")
            .expect("failed to parse SFEN string");
        let filled_squares = [
            (0, 2, PieceType::Rook, Color::White),
            (0, 3, PieceType::Knight, Color::White),
            (0, 4, PieceType::Bishop, Color::White),
            (0, 5, PieceType::King, Color::White),
            (0, 6, PieceType::Queen, Color::White),
            (0, 7, PieceType::Bishop, Color::White),
            (0, 8, PieceType::Knight, Color::White),
            (0, 9, PieceType::Rook, Color::White),
            (2, 2, PieceType::Pawn, Color::White),
            (2, 3, PieceType::Pawn, Color::White),
            (2, 4, PieceType::Pawn, Color::White),
            (2, 5, PieceType::Pawn, Color::White),
            (2, 6, PieceType::Pawn, Color::White),
            (2, 7, PieceType::Pawn, Color::White),
            (2, 8, PieceType::Pawn, Color::White),
            (2, 9, PieceType::Pawn, Color::White),
            (9, 2, PieceType::Pawn, Color::Black),
            (9, 3, PieceType::Pawn, Color::Black),
            (9, 4, PieceType::Pawn, Color::Black),
            (9, 5, PieceType::Pawn, Color::Black),
            (9, 6, PieceType::Pawn, Color::Black),
            (9, 7, PieceType::Pawn, Color::Black),
            (9, 8, PieceType::Pawn, Color::Black),
            (9, 9, PieceType::Pawn, Color::Black),
            (11, 2, PieceType::Rook, Color::Black),
            (11, 3, PieceType::Knight, Color::Black),
            (11, 4, PieceType::Bishop, Color::Black),
            (11, 5, PieceType::King, Color::Black),
            (11, 6, PieceType::Queen, Color::Black),
            (11, 7, PieceType::Bishop, Color::Black),
            (11, 8, PieceType::Knight, Color::Black),
            (11, 9, PieceType::Rook, Color::Black),
        ];

        let empty_squares = [(0, 0, 12), (0, 11, 12), (5, 5, 4)];

        let hand_pieces = [
            (PieceType::Pawn, 0),
            (PieceType::Queen, 0),
            (PieceType::Knight, 0),
            (PieceType::Rook, 0),
            (PieceType::Bishop, 0),
        ];

        for case in filled_squares.iter() {
            let (row, file, pt, c) = *case;
            assert_eq!(
                Some(Piece {
                    piece_type: pt,
                    color: c,
                }),
                *pos.piece_at(Square::new(file, row).unwrap())
            );
        }

        for case in empty_squares.iter() {
            let (row, file, len) = *case;
            for i in row..(row + len) {
                assert_eq!(None, *pos.piece_at(Square::new(file, i).unwrap()));
            }
        }

        for case in hand_pieces.iter() {
            let (pt, n) = *case;
            assert_eq!(
                n,
                pos.hand(Piece {
                    piece_type: pt,
                    color: Color::Black,
                })
            );
            assert_eq!(
                n,
                pos.hand(Piece {
                    piece_type: pt,
                    color: Color::White,
                })
            );
        }

        assert_eq!(Color::Black, pos.side_to_move());
        assert_eq!(1, pos.ply());
    }

    #[test]
    fn all_legal_moves() {
        setup();
        let cases = [
            ("57/7k4/57/5n6/57/57/57/1Q55/57/K56/57/57 b - 1", "f4", 0),
            ("57/7k4/57/5n6/57/57/57/2Q9/57/K56/57/57 b - 1", "f4", 8),
            (
                "8K3/3PPPPP4/6p5/8R3/6pp4/57/57/57/8r3/57/8k3/57 w - 1",
                "i4",
                7,
            ),
            (
                "8K3/3PPPPP4/6p5/55R1/6pp4/57/57/57/8r3/57/8k3/57 w - 1",
                "k4",
                1,
            ),
            ("1K55/1qq9/57/57/57/57/57/1R55/57/2k9/57/57 w - 1", "b8", 0),
            ("1K55/q1q9/57/57/57/57/57/1R55/57/2k9/57/57 w - 1", "b8", 0),
        ];
        for case in cases {
            let mut pos = Position::new();
            pos.set_sfen(case.0).expect("error while parsing sfen");
            let moves_count = pos.legal_moves(&Square::from_sfen(case.1).unwrap()).count();
            assert_eq!(case.2, moves_count);
        }
    }

    #[test]
    fn king_squares() {
        let position_set = Position::default();
        let cases = [
            (Color::Black, 6, [D12, E12, F12, G12, H12, I12]),
            (Color::White, 6, [D1, E1, F1, G1, H1, I1]),
        ];
        for case in cases {
            let bb = position_set.king_squares(&case.0);
            assert_eq!(bb.count(), case.1);
            for sq in case.2 {
                assert!((&bb & sq).is_any());
            }
        }
    }

    #[test]
    fn is_hand_empty() {
        setup();
        let mut position_set = Position::default();
        position_set
            .parse_sfen_board("6K5/57/57/57/57/57/57/57/57/57/57/7k4")
            .expect("error while parsing sfen");
        position_set.set_hand("rrRqNNqq");
        let cases = [
            (PieceType::Knight, Color::White, A1),
            (PieceType::Queen, Color::Black, D12),
            (PieceType::Rook, Color::White, C1),
            (PieceType::Queen, Color::Black, I12),
            (PieceType::Knight, Color::White, H1),
            (PieceType::Rook, Color::Black, B12),
            (PieceType::Rook, Color::Black, G12),
            (PieceType::Queen, Color::Black, F12),
        ];
        for case in cases {
            position_set.place(
                Piece {
                    piece_type: case.0,
                    color: case.1,
                },
                case.2,
            );
        }
        assert!(position_set.is_hand_empty(&Color::Black, PieceType::Plinth));
        assert!(position_set.is_hand_empty(&Color::White, PieceType::Plinth));
    }

    #[test]
    fn place_king() {
        setup();
        let mut position_set = Position::default();
        position_set
            .set_sfen("7L04/57/57/57/57/57/57/57/57/57/57/57 w K 1")
            .expect("error");
        let cases = [(A1, 0), (B3, 0), (H1, 0), (G1, 1)];
        for case in cases {
            let piece = Piece {
                piece_type: PieceType::King,
                color: Color::White,
            };
            position_set.place(piece, case.0);
            assert_eq!(position_set.player_bb(Color::White).count(), case.1);
        }
    }

    #[test]
    fn generate_plinths() {
        setup();
        let mut position_set = Position::default();
        position_set.generate_plinths();
        assert_eq!(position_set.color_bb[Color::NoColor.index()].count(), 8);
    }

    #[test]
    fn empty_squares() {
        setup();
        let mut position_set = Position::default();
        position_set
            .parse_sfen_board("5KRRR3/4PPPP4/57/5L06/57/57/57/57/57/57/57/L04kqqLn3")
            .expect("error while parsing sfen");
        position_set.set_hand("NrNNbrn");
        let cases = [
            (PieceType::Knight, Color::Black, 8),
            (PieceType::Bishop, Color::Black, 7),
        ];
        for case in cases {
            let file = position_set.empty_squares(Piece {
                piece_type: case.0,
                color: case.1,
            });
            assert_eq!(file.count(), case.2);
        }
        assert_eq!(position_set.get_hand(Color::Black), "rrbn");
    }

    #[test]
    fn place_in_check() {
        setup();
        let black_fen = "5KQ2L02/9L02/57/57/3L08/5L06/2L09/2L09/8L03/57/9L02/6k5 b qrn2pN2P 3";
        let cases = [PieceType::Queen, PieceType::Pawn];
        for case in cases {
            let mut position_set = Position::default();
            position_set
                .set_sfen(black_fen)
                .expect("failed to parse sfen string");

            position_set.place(
                Piece {
                    piece_type: case,
                    color: Color::Black,
                },
                G11,
            );
            assert_eq!(position_set.ply(), 4);
        }
    }

    #[test]
    fn empty_squares_with_plinths() {
        setup();
        let cases = [
            (
                "5K1Q4/L056/5L05L0/57/57/7L04/57/2L09/57/8L03/4L02L04/7k4 b q2rb4n3pQ2R3BN3P 3",
                11,
                Color::Black,
            ),
            (
                "4LN2K4/5L06/9L02/57/55L01/57/57/57/1L01L02L05/57/57/6kq3L0 w rnQNRBP 4",
                2,
                Color::White,
            ),
        ];
        for case in cases {
            let mut position_set = Position::default();
            position_set
                .set_sfen(case.0)
                .expect("error while parsing sfen");
            let file = position_set.empty_squares(Piece {
                piece_type: PieceType::Knight,
                color: case.2,
            });
            assert_eq!(file.count(), case.1);
        }
    }
}
