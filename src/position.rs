use itertools::Itertools;
use std::{fmt, vec};

use crate::{
    between, generate_plinths, get_non_sliding_attacks, get_sliding_attacks, square_bb, BitBoard,
    Color, Hand, Move, MoveError, Piece, PieceType, SfenError, Square, EMPTY_BB, FILE_BB,
};

/// Outcome stores information about outcome after move.
#[derive(Debug, PartialEq)]
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

/// MoveRecord stores information necessary to undo the move.
#[derive(Debug)]
pub enum MoveRecord {
    Normal {
        from: Square,
        to: Square,
        placed: Piece,
        captured: Option<Piece>,
        promoted: bool,
    },
}

impl MoveRecord {
    /// Converts the move into SFEN formatted string.
    pub fn to_sfen(&self) -> String {
        match *self {
            MoveRecord::Normal {
                from, to, promoted, ..
            } => format!("{}_{}{}", from, to, if promoted { "*" } else { "" }),
        }
    }
}

impl PartialEq<Move> for MoveRecord {
    fn eq(&self, other: &Move) -> bool {
        match (self, other) {
            (
                &MoveRecord::Normal {
                    from: f1,
                    to: t1,
                    promoted,
                    ..
                },
                &Move::Normal {
                    from: f2,
                    to: t2,
                    promote,
                },
            ) => f1 == f2 && t1 == t2 && promote == promoted,
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
                        let sq = {
                            match p.color {
                                Color::Red => {
                                    Square::from_index(sq.index() as u8 + 12 as u8).unwrap()
                                }
                                Color::Blue => {
                                    Square::from_index(sq.index() as u8 - 12 as u8).unwrap()
                                }
                                Color::NoColor => sq,
                            }
                        };
                        &(&bb & &position.color_bb[p.color.flip().index()]) | &square_bb(sq)
                    } else {
                        bb
                    }
                } else {
                    primary_bb
                }
            }
            MoveType::NoKing { king } => {
                if p.piece_type != PieceType::Knight {
                    &(&(bb) & &!&position.color_bb[2]) | &(&bb & &square_bb(king.to_owned()))
                } else {
                    &bb | &(&bb & &square_bb(king.to_owned()))
                }
            }
        }
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
#[derive(Debug)]
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
                return self.fix_pin(&pinned_moves, check_moves, my_moves);
            }
        }

        return self.fix_pin(&pinned_moves, check_moves, my_moves);
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
                        let mut pinned = &between(ksq, psq) & &self.occupied_bb;
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
        let check_moves = self.check_moves(&self.find_king(c).unwrap());
        if check_moves.len() > 0 {
            true
        } else {
            false
        }
    }

    /// Checks if given color is in checkmate.
    pub fn is_checkmate(&self, color: &Color) -> bool {
        let king = self.find_king(*color);
        match king {
            Some(k) => {
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
    fn fix_pin(&self, fixer: &Fixer, checks: Vec<BitBoard>, my_moves: BitBoard) -> BitBoard {
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
                if checks.len() > 1 {
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
        let sfen = self.generate_sfen().split(' ').take(3).join(" ");
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
            Some(_i) => {}
            None => {
                if moved.piece_type == PieceType::Pawn {
                    let free_sq = from.index() + 12;
                    if to.index() != free_sq {
                        return Err(MoveError::Inconsistent("The piece cannot move to there"));
                    } else if to.in_promotion_zone(moved.color) {
                        promoted = true;
                    }
                }
                ();
            }
        }

        if !(&self.legal_moves(&from) & to).is_any() {
            return Err(MoveError::Inconsistent("The piece cannot move to there"));
        }

        if !promoted && !moved.is_placeable_at(to) {
            return Err(MoveError::NonMovablePiece);
        }

        let placed = if promoted {
            match moved.promote() {
                Some(promoted) => promoted,
                None => return Err(MoveError::Inconsistent("This type of piece cannot promote")),
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
            let pc = cap.flip();
            let pc = match pc.unpromote() {
                Some(unpromoted) => unpromoted,
                None => pc,
            };
            self.hand.increment(pc);
        }

        self.side_to_move = opponent;
        self.ply += 1;

        self.log_position();
        self.detect_repetition()?;
        self.detect_insufficient_material()?;

        let move_record = MoveRecord::Normal {
            from,
            to,
            placed,
            captured,
            promoted,
        };

        self.move_history.push(move_record);
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
                    let unpromoted_cap = cap.unpromote().unwrap_or(*cap);
                    self.hand.decrement(unpromoted_cap.flip());
                }
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

    pub fn play(&mut self, from: &str, to: &str) {
        let m = Move::Normal {
            from: Square::from_sfen(from).expect("This square does not exist."),
            to: Square::from_sfen(to).expect("This square does not exist."),
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
                _ => (),
            },
        }
    }

    fn detect_repetition(&self) -> Result<(), MoveError> {
        if self.sfen_history.len() < 9 {
            return Ok(());
        }

        let cur = self.sfen_history.last().unwrap();

        let mut cnt = 0;
        for (i, entry) in self.sfen_history.iter().rev().enumerate() {
            if entry.0 == cur.0 {
                cnt += 1;

                if cnt == 4 {
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
        if self.in_check(self.side_to_move.flip()) && self.sfen_history.len() < 2 {
            return Err(SfenError::IllegalBoardState);
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
        for i in history {
            self.sfen_history.push(i);
        }
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
            "b" => Color::Blue,
            "r" => Color::Red,
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
                        num_pieces = num_pieces * 13 + (n as u8);
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

        let color = if self.side_to_move == Color::Blue {
            "b"
        } else {
            "r"
        };

        let mut hand = [Color::Blue, Color::Red]
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
    pub fn get_hand(&mut self, c: Color) -> String {
        return self.hand.to_sfen(c);
    }

    /////////////////////////////////////////////////////////////////////////////
    // Deploy part in Shuuro.
    /////////////////////////////////////////////////////////////////////////////

    /// Squares for king to be placed.
    fn king_squares(&self, c: &Color) -> BitBoard {
        let files = ['d', 'e', 'f', 'g', 'h', 'i'];
        let mut bb = EMPTY_BB;
        let mut all = |num: usize| -> BitBoard {
            for file in files {
                bb |= Square::from_sfen(&format!("{}{}", file, num)[..]).unwrap();
            }
            bb
        };
        match *c {
            Color::Blue => all(12),
            Color::Red => all(1),
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
            Color::Red => test(p, [0, 1, 2]),
            Color::Blue => test(p, [11, 10, 9]),
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
        for p in [PieceType::Queen, PieceType::Rook, PieceType::Bishop] {
            let bb = &self.type_bb[p.index()] & &self.color_bb[attacked_color.flip().index()];
            for i in bb {
                all = get_sliding_attacks(p, i, self.occupied_bb);
                if (&all & &king).is_any() {
                    match *attacked_color {
                        Color::Red => {
                            let files = &FILE_BB[1] | &FILE_BB[2];
                            return &(&files & &all) & &!&king;
                        }
                        Color::Blue => {
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
                if !self.is_hand_empty(&p.color.flip(), PieceType::Plinth) {
                    self.side_to_move = p.color.flip();
                }
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
}

/////////////////////////////////////////////////////////////////////////////
// Trait implementations
/////////////////////////////////////////////////////////////////////////////

impl Default for Position {
    fn default() -> Position {
        Position {
            side_to_move: Color::Blue,
            board: PieceGrid([None; 144]),
            hand: Default::default(),
            ply: 1,
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
                    write!(f, "{:>3}|", piece.to_string())?;
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
            if self.side_to_move == Color::Blue {
                "Blue"
            } else {
                "Red"
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
        write!(f, "Hand (Blue): ")?;
        fmt_hand(Color::Blue, f)?;
        writeln!(f)?;

        write!(f, "Hand (Red): ")?;
        fmt_hand(Color::Red, f)?;
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
            color: Color::Blue,
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
            let blue = pos.player_bb(Color::Blue);
            let red = pos.player_bb(Color::Red);

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
                "5NNQK3/8B3/57/57/57/8r3/57/57/pp55/1k55/57/57 r - 1",
                &[],
                &[I2],
            ),
            (
                "5NNQK3/8B3/8R3/57/57/8r3/57/57/pp55/1k55/57/57 r - 1",
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
            let blue = pos.pinned_bb(Color::Blue);
            let red = pos.pinned_bb(Color::Red);

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
            ("f2", PieceType::Rook, Color::Red, 13),
            ("e7", PieceType::Knight, Color::Blue, 7),
        ];
        let mut pos = Position::new();
        pos.set_sfen("57/5R5K/57/57/3b1L04k1/55b1/4n7/57/55R1/57/57/57 r - 1")
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
        }
    }

    #[test]
    fn in_check() {
        setup();

        let test_cases = [
            (
                "KQR9/1PPP8/57/57/57/57/57/57/57/57/1ppp8/qkb9 r - 1",
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
                "RR5K4/7L04/QP55/7L04/57/57/57/nbq9/7q4/57/57/56k r - 1",
                false,
                false,
            ),
            ("KQP8/2n8/57/57/57/57/57/k11/57/57/57/57 b - 1", false, true),
        ];

        let mut pos = Position::new();
        for case in test_cases.iter() {
            pos.set_sfen(case.0).expect("failed to parse SFEN string");
            assert_eq!(case.1, pos.in_check(Color::Blue));
            assert_eq!(case.2, pos.in_check(Color::Red));
        }
    }

    #[test]
    fn is_checkmate() {
        setup();
        let cases = [
            (
                "1K8r1/9rr1/57/57/57/57/57/57/57/k11/57/57 b - 1",
                true,
                Color::Red,
            ),
            (
                "5RNB4/5K4r1/6B5/57/57/57/57/57/ppppp7/57/57/9k2 r - 1",
                false,
                Color::Blue,
            ),
            (
                "12/57/7k3Q/57/57/KRn9/57/57/57/57/57/57 b - 1",
                false,
                Color::Red,
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
        pos.set_sfen("12/57/PPPQP4K2/7RR3/57/57/57/4pp6/2kr8/57/57/57 b 1r2R 1")
            .expect("failed to parse SFEN string");
        for _ in 0..2 {
            assert!(pos.make_move(Move::new(D9, I9, false)).is_ok());
            assert!(pos.make_move(Move::new(H4, A4, false)).is_ok());
            assert!(pos.make_move(Move::new(I9, D9, false)).is_ok());
            assert!(pos.make_move(Move::new(A4, H4, false)).is_ok());
        }

        assert!(pos.make_move(Move::new(D9, I9, false)).is_ok());
        assert!(pos.make_move(Move::new(H4, A4, false)).is_ok());
        assert!(pos.make_move(Move::new(I9, D9, false)).is_ok());

        assert_eq!(
            Some(MoveError::RepetitionDraw),
            pos.make_move(Move::new(A4, H4, false)).err()
        );
    }

    #[test]
    fn make_move() {
        setup();

        let base_sfen = "57/3KRRB5/5PP5/57/57/57/57/qbbn8/57/6k5/57/57 r K2RB2P 1";
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
    fn make_moves() {
        setup();
        let mut pos = Position::new();
        pos.set_sfen("6K5/57/57/6k5/57/PL055/57/p56/57/57/57/57 r - 1")
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
                to: E1,
                promote: false,
            },
            Move::Normal {
                from: A9,
                to: A7,
                promote: false,
            },
        ];

        for case in test_cases.iter() {
            pos.set_sfen(base_sfen)
                .expect("failed to parse SFEN string");
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
            (0, 2, PieceType::Rook, Color::Red),
            (0, 3, PieceType::Knight, Color::Red),
            (0, 4, PieceType::Bishop, Color::Red),
            (0, 5, PieceType::King, Color::Red),
            (0, 6, PieceType::Queen, Color::Red),
            (0, 7, PieceType::Bishop, Color::Red),
            (0, 8, PieceType::Knight, Color::Red),
            (0, 9, PieceType::Rook, Color::Red),
            (2, 2, PieceType::Pawn, Color::Red),
            (2, 3, PieceType::Pawn, Color::Red),
            (2, 4, PieceType::Pawn, Color::Red),
            (2, 5, PieceType::Pawn, Color::Red),
            (2, 6, PieceType::Pawn, Color::Red),
            (2, 7, PieceType::Pawn, Color::Red),
            (2, 8, PieceType::Pawn, Color::Red),
            (2, 9, PieceType::Pawn, Color::Red),
            (9, 2, PieceType::Pawn, Color::Blue),
            (9, 3, PieceType::Pawn, Color::Blue),
            (9, 4, PieceType::Pawn, Color::Blue),
            (9, 5, PieceType::Pawn, Color::Blue),
            (9, 6, PieceType::Pawn, Color::Blue),
            (9, 7, PieceType::Pawn, Color::Blue),
            (9, 8, PieceType::Pawn, Color::Blue),
            (9, 9, PieceType::Pawn, Color::Blue),
            (11, 2, PieceType::Rook, Color::Blue),
            (11, 3, PieceType::Knight, Color::Blue),
            (11, 4, PieceType::Bishop, Color::Blue),
            (11, 5, PieceType::King, Color::Blue),
            (11, 6, PieceType::Queen, Color::Blue),
            (11, 7, PieceType::Bishop, Color::Blue),
            (11, 8, PieceType::Knight, Color::Blue),
            (11, 9, PieceType::Rook, Color::Blue),
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
                    color: Color::Blue,
                })
            );
            assert_eq!(
                n,
                pos.hand(Piece {
                    piece_type: pt,
                    color: Color::Red,
                })
            );
        }

        assert_eq!(Color::Blue, pos.side_to_move());
        assert_eq!(1, pos.ply());
    }

    #[test]
    fn all_legal_moves() {
        setup();
        let cases = [
            ("57/7k4/57/5n6/57/57/57/1Q55/57/K56/57/57 b - 1", "f4", 0),
            ("57/7k4/57/5n6/57/57/57/2Q9/57/K56/57/57 b - 1", "f4", 8),
            (
                "8K3/3PPPPP4/6p5/8R3/6pp4/57/57/57/8r3/57/8k3/57 r - 1",
                "i4",
                7,
            ),
            (
                "8K3/3PPPPP4/6p5/55R1/6pp4/57/57/57/8r3/57/8k3/57 r - 1",
                "k4",
                1,
            ),
            ("1K55/1qq9/57/57/57/57/57/1R55/57/2k9/57/57 r - 1", "b8", 0),
            ("1K55/q1q9/57/57/57/57/57/1R55/57/2k9/57/57 r - 1", "b8", 0),
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
            (Color::Blue, 6, [D12, E12, F12, G12, H12, I12]),
            (Color::Red, 6, [D1, E1, F1, G1, H1, I1]),
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
            (PieceType::Knight, Color::Red, A1),
            (PieceType::Queen, Color::Blue, D12),
            (PieceType::Rook, Color::Red, C1),
            (PieceType::Queen, Color::Blue, I12),
            (PieceType::Knight, Color::Red, H1),
            (PieceType::Rook, Color::Blue, B12),
            (PieceType::Rook, Color::Blue, G12),
            (PieceType::Queen, Color::Blue, F12),
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
        assert!(position_set.is_hand_empty(&Color::Blue, PieceType::Plinth));
        assert!(position_set.is_hand_empty(&Color::Red, PieceType::Plinth));
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
            (PieceType::Knight, Color::Blue, 8),
            (PieceType::Bishop, Color::Blue, 7),
        ];
        for case in cases {
            let file = position_set.empty_squares(Piece {
                piece_type: case.0,
                color: case.1,
            });
            assert_eq!(file.count(), case.2);
        }
        assert_eq!(position_set.get_hand(Color::Blue), "rrbn");
    }
}
