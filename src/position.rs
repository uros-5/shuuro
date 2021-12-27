use itertools::Itertools;
use std::{fmt, vec};

use crate::{
    between, get_non_sliding_attacks, get_sliding_attacks, BitBoard, Color, Hand, Move, MoveError,
    Piece, PieceType, SfenError, Square, EMPTY_BB,
};

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

struct PieceGrid([Option<Piece>; 144]);

impl PieceGrid {
    pub fn get(&self, sq: Square) -> &Option<Piece> {
        &self.0[sq.index()]
    }

    pub fn set(&mut self, sq: Square, pc: Option<Piece>) {
        self.0[sq.index()] = pc;
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

    /// c is color that is supposed to win
    pub fn is_checkmate(&self, c: Color) -> bool {
        let (my_moves, my_checks) = self.my_moves(c, vec![], vec![], false);
        let king_moves = self.king_moves(c);
        let pieces_failed: bool = {
            let mut test: bool = true;
            if my_checks.len() == 1 {
                let (enemy_moves, _) = self.my_moves(c.flip(), vec![], vec![], true);
                for e_m in enemy_moves.iter() {
                    if (e_m & &my_checks[0]).is_any() {
                        test = false;
                        break;
                    }
                }
            }
            test
        };
        if pieces_failed && my_checks.len() > 0 {
            for check in my_checks.iter() {
                if (check & &king_moves).is_any() {
                    for my_move in my_moves.iter() {
                        if (my_move & &king_moves).is_any() {
                            return true;
                        }
                    }
                } else {
                    return true;
                }
            }
        } else if pieces_failed && my_checks.len() == 0 {
            return false;
        }
        return false;
    }

    fn my_moves(
        &self,
        c: Color,
        mut my_moves: Vec<BitBoard>,
        mut checks: Vec<BitBoard>,
        king_ignore: bool,
    ) -> (Vec<BitBoard>, Vec<BitBoard>) {
        let enemy_king = self.find_king(c.flip()).unwrap();
        for i in 0..144 {
            let sq = Square::from_index(i).unwrap();
            let pc = self.board.get(sq);
            match pc {
                Some(i) => {
                    if i.piece_type == PieceType::King && king_ignore == true {
                        continue;
                    }
                    if i.color == c {
                        let p_moves = self.move_candidates2(sq, *i);
                        my_moves.push(p_moves);
                        if (&p_moves & enemy_king).is_any() {
                            checks.push(p_moves);
                        }
                    }
                }
                None => (),
            }
        }
        (my_moves, checks)
    }

    fn king_moves(&self, c: Color) -> BitBoard {
        let king_sq = self.find_king(c).unwrap();
        let piece = self.board.get(king_sq).unwrap();
        self.move_candidates(king_sq, piece)
    }

    /// Checks if the king with the given color is in check.
    pub fn in_check(&self, c: Color) -> bool {
        if let Some(king_sq) = self.find_king(c) {
            self.is_attacked_by(king_sq, c.flip())
        } else {
            false
        }
    }

    /// Sets a piece at the given square.
    fn set_piece(&mut self, sq: Square, p: Option<Piece>) {
        self.board.set(sq, p);
    }

    fn is_attacked_by(&self, sq: Square, c: Color) -> bool {
        PieceType::iter().any(|pt| self.get_attackers_of_type(pt, sq, c).is_any())
    }

    fn get_attackers_of_type(&self, pt: PieceType, sq: Square, c: Color) -> BitBoard {
        let bb = &self.type_bb[pt.index()] & &self.color_bb[c.index()];

        if bb.is_empty() {
            return bb;
        }

        let attack_pc = Piece {
            piece_type: pt,
            color: c,
        };

        &bb & &self.move_candidates(sq, attack_pc.flip())
    }

    fn find_king(&self, c: Color) -> Option<Square> {
        let mut bb = &self.type_bb[PieceType::King.index()] & &self.color_bb[c.index()];
        if bb.is_any() {
            bb.pop_reverse()
        } else {
            None
        }
    }

    fn log_position(&mut self) {
        // TODO: SFEN string is used to represent a state of position, but any transformation which uniquely distinguish positions can be used here.
        // Consider light-weight option if generating SFEN string for each move is time-consuming.
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
    pub fn make_move(&mut self, m: Move) -> Result<(), MoveError> {
        let res = match m {
            Move::Normal { from, to, promote } => self.make_normal_move(from, to, promote)?,
        };

        self.move_history.push(res);
        Ok(())
    }

    fn make_normal_move(
        &mut self,
        from: Square,
        to: Square,
        _promoted: bool,
    ) -> Result<MoveRecord, MoveError> {
        let mut promoted = false;
        let stm = self.side_to_move();
        let opponent = stm.flip();

        let moved = self
            .piece_at(from)
            .ok_or(MoveError::Inconsistent("No piece found"))?;

        let captured = *self.piece_at(to);

        if moved.color != stm {
            return Err(MoveError::Inconsistent(
                "The piece is not for the side to move",
            ));
        }

        match captured {
            Some(i) => {
                if i.piece_type == PieceType::Plynth {
                    if moved.piece_type != PieceType::Knight {
                        return Err(MoveError::Inconsistent("The piece cannot move to there"));
                    }
                } else {
                    ();
                }
            }
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

        if !self.move_candidates(from, moved).any(|sq| sq == to) {
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

        if self.in_check(stm) {
            // Undo-ing the move.
            self.set_piece(from, Some(moved));
            self.set_piece(to, captured);
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
                self.hand.decrement(pc);
            }

            return Err(MoveError::InCheck);
        }

        self.side_to_move = opponent;
        self.ply += 1;

        self.log_position();
        self.detect_repetition()?;

        Ok(MoveRecord::Normal {
            from,
            to,
            placed,
            captured,
            promoted,
        })
    }

    /// Returns a list of squares at which a piece of the given color is pinned.
    pub fn pinned_bb(&self, c: Color) -> BitBoard {
        let ksq = self.find_king(c);
        if ksq.is_none() {
            return BitBoard::empty();
        }
        let ksq = ksq.unwrap();

        [
            (
                PieceType::Rook,
                get_sliding_attacks(PieceType::Rook, ksq, EMPTY_BB),
            ),
            (
                PieceType::Bishop,
                get_sliding_attacks(PieceType::Bishop, ksq, EMPTY_BB),
            ),
        ]
        .iter()
        .fold(BitBoard::empty(), |mut accum, &(pt, ref mask)| {
            let bb = &(&self.type_bb[pt.index()] & &self.color_bb[c.flip().index()]) & mask;

            for psq in bb {
                let between = &between(ksq, psq) & &self.occupied_bb;
                if between.count() == 1 && (&between & &self.color_bb[c.index()]).is_any() {
                    accum |= &between;
                }
            }

            accum
        })
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
    pub fn move_candidates(&self, sq: Square, p: Piece) -> BitBoard {
        let bb = match p.piece_type {
            PieceType::Rook => get_sliding_attacks(PieceType::Rook, sq, self.occupied_bb),
            PieceType::Bishop => get_sliding_attacks(PieceType::Bishop, sq, self.occupied_bb),
            PieceType::Queen => get_sliding_attacks(PieceType::Queen, sq, self.occupied_bb),
            PieceType::Knight => get_non_sliding_attacks(PieceType::Knight, sq, p.color),
            PieceType::Pawn => get_non_sliding_attacks(PieceType::Pawn, sq, p.color),
            PieceType::King => get_non_sliding_attacks(PieceType::King, sq, p.color),
            _ => EMPTY_BB,
        };
        &bb & &!&self.color_bb[p.color.index()]
    }

    pub fn move_candidates2(&self, sq: Square, p: Piece) -> BitBoard {
        let bb = match p.piece_type {
            PieceType::Rook => get_sliding_attacks(PieceType::Rook, sq, self.occupied_bb),
            PieceType::Bishop => get_sliding_attacks(PieceType::Bishop, sq, self.occupied_bb),
            PieceType::Queen => get_sliding_attacks(PieceType::Queen, sq, self.occupied_bb),
            PieceType::Knight => get_non_sliding_attacks(PieceType::Knight, sq, p.color),
            PieceType::Pawn => get_non_sliding_attacks(PieceType::Pawn, sq, p.color),
            PieceType::King => get_non_sliding_attacks(PieceType::King, sq, p.color),
            _ => EMPTY_BB,
        };
        bb
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

    fn generate_sfen(&self) -> String {
        fn add_num_space(num_spaces: i32, mut s: String) -> String {
            if num_spaces == 10 {
                s.push_str("55");
            } else if num_spaces == 11 {
                s.push_str("56");
            } else if num_spaces == 12 {
                s.push_str("57");
            } else {
                s.push_str(&num_spaces.to_string());
            }
            s
        }
        let board = (0..12)
            .map(|row| {
                let mut s = String::new();
                let mut num_spaces = 0;
                for file in 0..12 {
                    match *self.piece_at(Square::new(file, row).unwrap()) {
                        Some(pc) => {
                            if num_spaces > 0 {
                                let mut _s = add_num_space(num_spaces, s);
                                s = _s;
                                num_spaces = 0;
                            }

                            s.push_str(&pc.to_string());
                        }
                        None => num_spaces += 1,
                    }
                }

                if num_spaces > 0 {
                    let _s = add_num_space(num_spaces, s);
                    s = _s;
                    num_spaces = 0;
                }

                s
            })
            .join("/");

        let color = if self.side_to_move == Color::Blue {
            "b"
        } else {
            "w"
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
                    write!(f, "   |")?;
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
                "RR5K4/7L4/QP55/7L4/57/57/57/nbq9/7q4/57/57/56k r - 1",
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
                Color::Blue,
            ),
            (
                "5RNB4/5K4r1/6B5/57/57/57/57/57/ppppp7/57/57/9k2 r - 1",
                false,
                Color::Red,
            ),
            (
                "12/57/7k3Q/57/57/KRn9/57/57/57/57/57/57 b - 1",
                false,
                Color::Blue,
            ),
        ];
        for case in cases.iter() {
            let mut pos = Position::new();
            pos.set_sfen(case.0).expect("failed to parse SFEN string");
            println!("{}", pos);
            assert_eq!(case.1, pos.is_checkmate(case.2));
        }
    }

    #[test]
    fn player_bb() {
        setup();

        let cases: &[(&str, &[Square], &[Square])] = &[
            (
                "BBQ9/57/57/4R7/57/57/57/5ppp4/nnq/57/57/57 b - 1",
                &[A9, B9, C9, F8, G8, H8],
                &[A1, B1, C1, E4],
            ),
            (
                "12/57/6PPPP2/3Q6N1/57/57/57/5ppp4/7qk3/57/57/57 b P 1",
                &[H9, I9, F8, G8, H8],
                &[G3, H3, I3, J3, D4, K4],
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
                        pos.move_candidates(sq, pc).count(),
                    );
                    sum += pos.move_candidates(sq, pc).count();
                }
            }
        }

        assert_eq!(27, sum);
    }

    #[test]
    fn make_normal_move() {
        setup();

        let base_sfen = "12/3KRRB5/5PP5/57/57/57/57/qbbn8/57/6k5/57/57 r 1K2R1B2P 1";
        let test_cases = [
            (D2, E1, false, true),
            (E2, E7, false, true),
            (G2, I4, false, true),
            (F2, F1, false, true),
            (G3, H4, false, false),
        ];

        for case in test_cases.iter() {
            let mut pos = Position::new();
            pos.set_sfen(base_sfen)
                .expect("failed to parse SFEN string");
            println!("{}", pos);
            assert_eq!(case.3, pos.make_normal_move(case.0, case.1, case.2).is_ok());
        }

        let mut pos = Position::new();
        // Leaving the checked king is illegal.
        pos.set_sfen("12/1K8RR/57/57/57/r9k1/57/57/57/57/57/57 b kr 1")
            .expect("failed to parse SFEN string");
        assert!(pos.make_normal_move(A6, A1, false).is_err());

        pos.set_sfen("12/1RR9/57/57/57/r9k1/57/57/57/57/57/57 b kr 1")
            .expect("failed to parse SFEN string");
        assert!(pos.make_normal_move(K6, K5, false).is_ok());
    }

    #[test]
    fn repetition() {
        setup();

        let mut pos = Position::new();
        pos.set_sfen("12/57/PPPQP4K2/7RR3/57/57/57/4pp6/2kr8/57/57/57 b 1r2R 1")
            .expect("failed to parse SFEN string");
        for _ in 0..2 {
            assert!(pos.make_normal_move(D9, I9, false).is_ok());
            assert!(pos.make_normal_move(H4, A4, false).is_ok());
            assert!(pos.make_normal_move(I9, D9, false).is_ok());
            assert!(pos.make_normal_move(A4, H4, false).is_ok());
        }

        assert!(pos.make_normal_move(D9, I9, false).is_ok());
        assert!(pos.make_normal_move(H4, A4, false).is_ok());
        assert!(pos.make_normal_move(I9, D9, false).is_ok());

        assert_eq!(
            Some(MoveError::RepetitionDraw),
            pos.make_normal_move(A4, H4, false).err()
        );
    }

    #[test]
    fn unmake_move() {
        setup();

        let mut pos = Position::new();
        let base_sfen = "RRQNN3K3/PP1P4PP2/2P9/57/57/B56/1n3q4PP/4qq6/rq3r5r/57/4k7/57 b 3q3r 1";
        pos.set_sfen(base_sfen)
            .expect("failed to parse SFEN string");
        let base_state = format!("{}", pos);
        println!("{}", base_state);
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
}
