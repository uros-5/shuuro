use itertools::Itertools;
use std::fmt;

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
            } => format!("{}{}{}", from, to, if promoted { "*" } else { "" }),
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
    color_bb: [BitBoard; 2],
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

    /// Checks if a player with the given color can declare winning.
    ///
    /// See the section 25 in http://www.computer-shuuro.org/wcsc26/rule.pdf for more detail.
    pub fn try_declare_winning(&self, c: Color) -> bool {
        if c != self.side_to_move {
            return false;
        }

        let king_pos = self.find_king(c);
        if king_pos.is_none() {
            return false;
        }

        let king_pos = king_pos.unwrap();
        if king_pos.relative_file(c) >= 3 {
            return false;
        }

        let (mut point, count) =
            PieceType::iter()
                .filter(|&pt| pt != PieceType::King)
                .fold((0, 0), |accum, pt| {
                    let unit = match pt {
                        PieceType::Rook | PieceType::Bishop => 3,
                        _ => 1,
                    };

                    let bb = &(&self.type_bb[pt.index()] & &self.color_bb[c.index()]);
                    let count = bb.count() as u8;
                    let point = count * unit;

                    (accum.0 + point, accum.1 + count)
                });

        if count < 10 {
            return false;
        }

        point += PieceType::iter()
            .filter(|pt| pt.is_hand_piece())
            .fold(0, |acc, pt| {
                let num = self.hand.get(Piece {
                    piece_type: pt,
                    color: c,
                });
                let pp = match pt {
                    PieceType::Rook | PieceType::Bishop => 5,
                    _ => 1,
                };

                acc + num * pp
            });

        let lowerbound = match c {
            Color::Blue => 28,
            Color::Red => 27,
            Color::NoColor => 0,
        };
        if point < lowerbound {
            return false;
        }

        if self.in_check(c) {
            return false;
        }

        true
    }

    /// Checks if the king with the given color is in check.
    pub fn in_check(&self, c: Color) -> bool {
        if let Some(king_sq) = self.find_king(c) {
            println!("{}", king_sq);
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
        promoted: bool,
    ) -> Result<MoveRecord, MoveError> {
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

    /// Returns a list of squares to where the given pieve at the given square can move.
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
                        return Err(MoveError::PerpetualCheckLose);
                    } else if prev.1 * 2 >= (i as u16) {
                        return Err(MoveError::PerpetualCheckWin);
                    } else {
                        return Err(MoveError::Repetition);
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
        let board = (0..12)
            .map(|row| {
                let mut s = String::new();
                let mut num_spaces = 0;
                for file in (0..12) {
                    match *self.piece_at(Square::new(file, row).unwrap()) {
                        Some(pc) => {
                            if num_spaces > 0 {
                                s.push_str(&num_spaces.to_string());
                                num_spaces = 0;
                            }

                            s.push_str(&pc.to_string());
                        }
                        None => num_spaces += 1,
                    }
                }

                if num_spaces > 0 {
                    s.push_str(&num_spaces.to_string());
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
    use crate::{init, BitBoard, Color, Piece, PieceType, Position, Square, EMPTY_BB, SQUARE_BB};
    pub const START_POS: &str = "KR10/12/12/12/12/12/12/12/12/12/12/kr10 b - 1";
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
                "KQR9/1PPP8/12/12/12/12/12/12/12/12/1ppp8/qkb9 b - 1",
                false,
                true,
            ),
            /*
            ("9/3r5/9/9/6B2/9/9/9/3K5 b P 1", true, false),
            (
                "ln2r1knl/2gb1+Rg2/4Pp1p1/p1pp1sp1p/1N2pN1P1/2P2PP2/PP1G1S2R/1SG6/LK6L w 2PSp 1",
                false,
                true,
            ),
            (
                "lnsg1gsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSG1GSNL b - 1",
                false,
                false,
            ),
            */
        ];

        let mut pos = Position::new();
        for case in test_cases.iter() {
            pos.set_sfen(case.0).expect("failed to parse SFEN string");
            assert_eq!(case.1, pos.in_check(Color::Blue));
            assert_eq!(case.2, pos.in_check(Color::Red));
        }
    }
}
