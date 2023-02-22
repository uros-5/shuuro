use std::{
    cmp::Ordering,
    collections::HashMap,
    hash::Hash,
    ops::{BitAnd, BitOr, BitOrAssign, Not},
};

use itertools::Itertools;

use crate::{
    attacks::Attacks, bitboard::BitBoard, Color, Move, MoveError, MoveRecord,
    Piece, PieceType, SfenError, Square, Variant,
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

impl ToString for Outcome {
    fn to_string(&self) -> String {
        match &self {
            Outcome::Check { color } => format!("Check_{}", color.to_string()),
            Outcome::Checkmate { color } => {
                format!("Checkmate_{}", color.to_string())
            }
            Outcome::Draw => "Draw".to_string(),
            Outcome::Nothing => "Live".to_string(),
            Outcome::DrawByRepetition => "RepetitionDraw".to_string(),
            Outcome::DrawByMaterial => "MaterialDraw".to_string(),
            Outcome::Stalemate => "Stalemate".to_string(),
            Outcome::MoveOk => "Live".to_string(),
            Outcome::MoveNotOk => "Illegal move".to_string(),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<i32> for Outcome {
    fn into(self) -> i32 {
        match self {
            Outcome::MoveNotOk => -2,
            Outcome::MoveOk => -1,
            Outcome::Nothing => -1,
            Outcome::Check { color: _ } => -1,
            Outcome::Checkmate { color: _ } => 1,
            Outcome::Stalemate => 3,
            Outcome::DrawByRepetition => 4,
            Outcome::Draw => 5,
            Outcome::DrawByMaterial => 6,
        }
    }
}

pub trait Position<S, B, A>
where
    S: Square + Hash,
    B: BitBoard<S>,
    A: Attacks<S, B>,
    Self: Sized
        + Board<S, B, A>
        + Sfen<S, B, A>
        + Placement<S, B, A>
        + Play<S, B, A>,
    for<'a> &'a B: BitOr<&'a B, Output = B>,
    for<'a> &'a B: BitAnd<&'a B, Output = B>,
    for<'a> &'a B: Not<Output = B>,
    for<'a> &'a B: BitOr<&'a S, Output = B>,
    for<'a> &'a B: BitAnd<&'a S, Output = B>,
    for<'a> B: BitOrAssign<&'a S>,
{
}

pub trait Board<S, B, A>
where
    S: Square + Hash,
    B: BitBoard<S>,
    Self: Sized,
    A: Attacks<S, B>,
    for<'a> &'a B: BitOr<&'a B, Output = B>,
    for<'a> &'a B: BitAnd<&'a B, Output = B>,
    for<'a> &'a B: Not<Output = B>,
    for<'a> &'a B: BitOr<&'a S, Output = B>,
    for<'a> &'a B: BitAnd<&'a S, Output = B>,
    for<'a> B: BitOrAssign<&'a S>,
{
    //

    /// Creates a new instance of `Position` with an empty board.
    fn new() -> Self;
    /// Sets a piece at the given square.
    fn set_piece(&mut self, sq: S, p: Option<Piece>);
    /// Returns a piece at the given square.
    fn piece_at(&self, sq: S) -> &Option<Piece>;
    /// Returns a bitboard containing pieces of the given player.
    fn player_bb(&self, c: Color) -> B;
    /// Returns occupied bitboard, all pieces except plinths.
    fn occupied_bb(&self) -> B;
    /// Returns `BitBoard` of all `PieceType`.
    fn type_bb(&self, pt: &PieceType) -> B;
    /// Mutate player BitBoard(XOR).
    fn xor_player_bb(&mut self, color: Color, sq: S);
    /// Mutate PieceType BitBoard(XOR).
    fn xor_type_bb(&mut self, piece_type: PieceType, sq: S);
    /// Mutate occupied BitBoard(XOR).
    fn xor_occupied(&mut self, sq: S);
    /// Returns the side to make a move next.
    fn side_to_move(&self) -> Color;
    /// All BitBoards are empty.
    fn empty_all_bb(&mut self);
    /// Generate BitBoards from sfen
    fn sfen_to_bb(&mut self, piece: Piece, sq: &S);
    /// Returns a history of all moves made since the beginning of the game.
    fn ply(&self) -> u16;
    /// Increment ply
    fn increment_ply(&mut self);
    /// Change side to move.
    fn flip_side_to_move(&mut self);
    /// Set new stm
    fn update_side_to_move(&mut self, c: Color);
    /// Returns current status of the game.
    fn outcome(&self) -> &Outcome;
    /// Set new outcome
    fn update_outcome(&mut self, outcome: Outcome);
    /// Returns current variant.
    fn variant(&self) -> Variant;
    /// Changing to other variant.
    fn update_variant(&mut self, variant: Variant);
    /// Insert new sfen to sfen history.
    fn insert_sfen(&mut self, sfen: &str);
    /// Insert new MoveRecord to move_history.
    fn insert_move(&mut self, m: MoveRecord<S>);
    /// Clear sfen_history
    fn clear_sfen_history(&mut self);
    /// Set sfen history.
    fn set_sfen_history(&mut self, history: Vec<String>);
    /// Set history of previous moves.
    fn set_move_history(&mut self, history: Vec<MoveRecord<S>>);
    /// Returns history of all moves in `MoveRecord` format.
    fn move_history(&self) -> &[MoveRecord<S>];
    fn get_move_history(&self) -> &Vec<MoveRecord<S>>;
    /// Returns history of all moves in `Vec` format.
    fn get_sfen_history(&self) -> &Vec<String>;
    /// Get hand count for Piece.
    fn hand(&self, p: Piece) -> u8;
    /// Get hand in form of String
    fn get_hand(&self, c: Color) -> String;
    /// Set hand from str.
    fn set_hand(&mut self, s: &str);
    /// Decrement player hand.
    fn decrement_hand(&mut self, p: Piece);
    /// Dimensions of board.
    fn dimensions(&self) -> u8;
    /// Returns `Square` if King is available.
    fn find_king(&self, c: &Color) -> Option<S> {
        let mut bb = &self.type_bb(&PieceType::King) & &self.player_bb(*c);
        if bb.is_any() {
            bb.pop_reverse()
        } else {
            None
        }
    }
}

pub trait Sfen<S, B, A>
where
    S: Square + Hash,
    B: BitBoard<S>,
    A: Attacks<S, B>,
    Self: Sized + Board<S, B, A>,
    for<'a> &'a B: BitOr<&'a B, Output = B>,
    for<'a> &'a B: BitAnd<&'a B, Output = B>,
    for<'a> &'a B: Not<Output = B>,
    for<'a> &'a B: BitOr<&'a S, Output = B>,
    for<'a> &'a B: BitAnd<&'a S, Output = B>,
    for<'a> B: BitOrAssign<&'a S>,
{
    // SFEN PART
    /// Convert current position to sfen.
    fn to_sfen(&self) -> String {
        let sfen_history = self.get_sfen_history();
        let move_history = self.get_move_history();
        let ply = self.ply();
        if sfen_history.is_empty() {
            return self.generate_sfen();
        }
        if move_history.is_empty() {
            format!("{} {}", sfen_history.first().unwrap(), ply);
        }
        format!(
            "{} {}",
            &sfen_history.first().unwrap(),
            ply - move_history.len() as u16
        )
    }

    /// Generate sfen.
    fn generate_sfen(&self) -> String {
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
        let knights = [
            PieceType::Knight,
            PieceType::Chancellor,
            PieceType::ArchBishop,
            PieceType::Giraffe,
        ];

        let dimension = self.dimensions();

        let board = (0..dimension)
            .map(|row| {
                let mut s = String::new();
                let mut num_spaces = 0;
                for file in 0..dimension {
                    let sq = S::new(file, row).unwrap();
                    match *self.piece_at(sq) {
                        Some(pc) => {
                            if num_spaces > 0 {
                                let mut _s = add_num_space(num_spaces, s);
                                s = _s;
                                num_spaces = 0;
                            }
                            if (&self.player_bb(Color::NoColor) & &sq).is_any()
                            {
                                if knights.contains(&pc.piece_type) {
                                    s.push('L');
                                } else {
                                    //return Err(SfenError::IllegalPieceTypeOnPlinth);
                                }
                            }

                            s.push_str(&pc.to_string());
                        }
                        None => {
                            if (&self.player_bb(Color::NoColor) & &sq).is_any()
                            {
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

        let color = if self.side_to_move() == Color::Black {
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
                        let n = self.hand(pc);

                        if n == 0 {
                            "".to_string()
                        } else if n == 1 {
                            format!("{pc}")
                        } else {
                            format!("{n}{pc}")
                        }
                    })
                    .join("")
            })
            .join("");

        if hand.is_empty() {
            hand = "-".to_string();
        }

        format!("{} {} {} {}", board, color, hand, self.ply())
    }
    fn clear_hand(&mut self);

    fn insert_in_hand(&mut self, p: Piece, num: u8);

    fn parse_sfen_ply(&mut self, s: &str) -> Result<(), SfenError>;

    fn update_player(&mut self, piece: Piece, sq: &S);

    fn parse_sfen_stm(&mut self, s: &str) -> Result<(), SfenError> {
        let stm = match s {
            "b" => Color::Black,
            "w" => Color::White,
            _ => return Err(SfenError::IllegalSideToMove),
        };
        self.update_side_to_move(stm);
        Ok(())
    }

    fn parse_sfen_board(&mut self, s: &str) -> Result<(), SfenError> {
        let ranks = s.split('/');
        let dimension = self.dimensions();
        self.empty_all_bb();
        for (rank, row) in ranks.enumerate() {
            if rank >= dimension as usize {
                return Err(SfenError::IllegalBoardState);
            }

            let mut file = 0;

            let mut is_promoted = false;
            for ch in row.chars() {
                match ch {
                    '+' => {
                        is_promoted = true;
                    }
                    '0' => {
                        let sq = Square::new(file, rank as u8).unwrap();
                        self.set_piece(sq, None);
                        file += 1;
                    }
                    n if n.is_numeric() => {
                        if let Some(n) = n.to_digit(11) {
                            for _ in 0..n {
                                if file >= dimension {
                                    return Err(SfenError::IllegalBoardState);
                                }

                                let sq = Square::new(file, rank as u8).unwrap();

                                self.set_piece(sq, None);

                                file += 1;
                            }
                        }
                    }
                    s => match Piece::from_sfen(s) {
                        Some(mut piece) => {
                            if file >= dimension {
                                return Err(SfenError::IllegalBoardState);
                            }

                            if is_promoted {
                                if let Some(promoted) =
                                    piece.piece_type.promote()
                                {
                                    piece.piece_type = promoted;
                                } else {
                                    return Err(SfenError::IllegalPieceType);
                                }
                            }

                            let sq = Square::new(file, rank as u8).unwrap();
                            match piece.piece_type {
                                PieceType::Plinth => {
                                    self.update_player(piece, &sq);
                                    continue;
                                }
                                _ => {
                                    self.sfen_to_bb(piece, &sq);
                                    file += 1;
                                    is_promoted = false;
                                }
                            }
                        }
                        None => return Err(SfenError::IllegalPieceType),
                    },
                }
            }
        }

        Ok(())
    }

    fn parse_sfen_hand(&mut self, s: &str) -> Result<(), SfenError> {
        if s == "-" {
            self.clear_hand();
            return Ok(());
        }

        let mut num_pieces: u8 = 0;
        for c in s.chars() {
            match c {
                n if n.is_numeric() => {
                    if let Some(n) = n.to_digit(9) {
                        if num_pieces != 0 {
                            let num2 = format!("{}{}", num_pieces, n as u8)
                                .parse::<u8>()
                                .unwrap();
                            num_pieces = num2;
                            continue;
                        }
                        num_pieces = n as u8;
                    }
                }
                s => {
                    match Piece::from_sfen(s) {
                        Some(p) => self.insert_in_hand(
                            p,
                            if num_pieces == 0 { 1 } else { num_pieces },
                        ),
                        None => return Err(SfenError::IllegalPieceType),
                    };
                    num_pieces = 0;
                }
            }
        }

        Ok(())
    }

    /// Saves position in sfen_history.
    fn log_position(&mut self) {
        let mut sfen = self.generate_sfen().split(' ').take(3).join(" ");
        let move_history = self.get_move_history();
        if !move_history.is_empty() {
            sfen.push_str(format!(" {} ", self.ply()).as_str());
            sfen.push_str(&move_history.last().unwrap().to_sfen());
        }
        self.insert_sfen(&sfen);
    }

    fn is_hand_empty(&self, c: Color, excluded: PieceType) -> bool {
        for pt in PieceType::iter() {
            if pt != excluded {
                let counter = self.hand(Piece {
                    piece_type: pt,
                    color: c,
                });
                if counter != 0 {
                    return false;
                }
            }
        }
        true
    }
}

pub trait Placement<S, B, A>
where
    S: Square + Hash,
    B: BitBoard<S>,
    A: Attacks<S, B>,
    Self: Sized + Board<S, B, A> + Sfen<S, B, A>,
    for<'a> &'a B: BitOr<&'a B, Output = B>,
    for<'a> &'a B: BitAnd<&'a B, Output = B>,
    for<'a> &'a B: Not<Output = B>,
    for<'a> &'a B: BitOr<&'a S, Output = B>,
    for<'a> &'a B: BitAnd<&'a S, Output = B>,
    for<'a> B: BitOrAssign<&'a S>,
{
    // PLACEMENT PART

    /// Generate random plinths.
    fn generate_plinths(&mut self);

    /// BitBoard with all available squares for white.
    fn white_placement_attacked_ranks(&self) -> B;

    /// BitBoard with all available squares for black.
    fn black_placement_attacked_ranks(&self) -> B;

    /// All ranks for black. White rank is 0,1,2.
    fn black_ranks(&self) -> [usize; 3];

    /// Returns array of files where king can be placed.
    fn king_files<const K: usize>(&self) -> [&str; K];

    /// Returns BitBoard with file. Panics if file is bigger than expected.
    fn rank_bb(&self, file: usize) -> B;

    /// Check if king is placed.
    fn is_king_placed(&self, c: Color) -> bool {
        let king = &self.player_bb(c) & &self.type_bb(&PieceType::King);
        if king.count() == 1 {
            return true;
        }
        false
    }

    /// Returns BitBoard with all empty squares.
    fn king_squares<const K: usize>(&self, c: &Color) -> B {
        let files: [&str; K] = self.king_files();
        let mut bb = B::empty();
        let plinths = self.player_bb(Color::NoColor);
        let mut all = |num: usize| -> B {
            for file in files {
                bb |= &S::from_sfen(&format!("{file}{num}")[..]).unwrap();
            }
            bb &= &!&plinths;
            bb
        };
        match *c {
            Color::Black => all(self.dimensions() as usize),
            Color::White => all(1),
            Color::NoColor => B::empty(),
        }
    }

    fn can_pawn_move(&self, p: Piece) -> bool {
        self.is_hand_empty(p.color, PieceType::Pawn)
    }

    fn empty_squares(&self, p: Piece) -> B {
        let calc = |p: Piece, list: [usize; 3]| -> B {
            for file in list {
                let mut bb = self.rank_bb(file);
                bb &= &!&self.player_bb(p.color);
                let plinths = self.player_bb(Color::NoColor);
                if bb.is_empty() {
                    continue;
                }
                match p.piece_type {
                    PieceType::Knight
                    | PieceType::Chancellor
                    | PieceType::ArchBishop => {
                        return bb;
                    }
                    PieceType::King => {
                        return self.king_squares::<6>(&p.color);
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
                            return B::empty();
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
            B::empty()
        };
        let checks = self.checks(&p.color);
        if checks.is_any() {
            return checks;
        } else if !self.is_king_placed(p.color)
            && p.piece_type != PieceType::King
        {
            return B::empty();
        }
        match p.color {
            Color::White => calc(p, [0, 1, 2]),
            Color::Black => calc(p, self.black_ranks()),
            Color::NoColor => B::empty(),
        }
    }

    fn checks(&self, attacked_color: &Color) -> B {
        let king =
            &self.type_bb(&PieceType::King) & &self.player_bb(*attacked_color);
        if king.is_empty() {
            return B::empty();
        }
        let mut all;
        let occupied_bb = &self.occupied_bb() | &self.player_bb(Color::NoColor);
        for p in [
            PieceType::Queen,
            PieceType::Rook,
            PieceType::Bishop,
            PieceType::Chancellor,
            PieceType::ArchBishop,
        ] {
            if !self.variant().can_buy(&p) {
                continue;
            }
            let them =
                &self.type_bb(&p) & &self.player_bb(attacked_color.flip());
            for i in them {
                all = A::get_sliding_attacks(p, &i, occupied_bb);
                if (&all & &king).is_any() {
                    match *attacked_color {
                        Color::White => {
                            let ranks = self.white_placement_attacked_ranks();
                            return &(&ranks & &all) & &!&king;
                        }
                        Color::Black => {
                            let ranks = self.black_placement_attacked_ranks();
                            return &(&ranks & &all) & &!&king;
                        }
                        Color::NoColor => {
                            return B::empty();
                        }
                    }
                }
            }
        }
        B::empty()
    }

    fn update_bb(&mut self, p: Piece, sq: S);

    fn place(&mut self, p: Piece, sq: S) -> Option<String> {
        if p.color != self.side_to_move() {
            return None;
        } else if self.hand(p) > 0 && (&self.empty_squares(p) & &sq).is_any() {
            self.update_bb(p, sq);
            self.decrement_hand(p);
            let move_record = MoveRecord::Put { to: sq, piece: p };
            let sfen =
                self.generate_sfen().split(' ').next().unwrap().to_string();
            let hand = {
                let s = self.get_hand(Color::White)
                    + &self.get_hand(Color::Black)[..];
                if s.is_empty() {
                    String::from(" ")
                } else {
                    s
                }
            };
            self.increment_ply();
            let ply = self.ply();

            self.insert_move(move_record.clone());
            if !self.is_hand_empty(p.color.flip(), PieceType::Plinth) {
                self.update_side_to_move(p.color.flip());
            }
            let record = format!(
                "{}_{}_{}_{}_{}",
                &move_record.to_sfen(),
                &sfen,
                hand,
                self.side_to_move().to_string(),
                ply
            );
            self.insert_sfen(&record);
            return Some(record);
        }
        None
    }
}

pub trait Play<S, B, A>
where
    S: Square + Hash,
    B: BitBoard<S>,
    A: Attacks<S, B>,
    Self: Sized + Board<S, B, A> + Sfen<S, B, A>,
    for<'a> &'a B: BitOr<&'a B, Output = B>,
    for<'a> &'a B: BitAnd<&'a B, Output = B>,
    for<'a> &'a B: Not<Output = B>,
    for<'a> &'a B: BitOr<&'a S, Output = B>,
    for<'a> &'a B: BitAnd<&'a S, Output = B>,
    for<'a> B: BitOrAssign<&'a S>,
{
    // Play part.

    /// Create move from `&str`.
    fn play(&mut self, from: &str, to: &str) -> Result<&Outcome, SfenError> {
        let from = match S::from_sfen(from) {
            Some(i) => i,
            None => {
                return Err(SfenError::IllegalPieceFound);
            }
        };
        let to = match S::from_sfen(to) {
            Some(i) => i,
            None => {
                return Err(SfenError::IllegalPieceFound);
            }
        };
        let m = Move::Normal {
            from,
            to,
            promote: false,
        };
        let outcome = self.make_move(m);
        match outcome {
            Ok(i) => {
                self.update_outcome(i);
            }
            Err(error) => match error {
                MoveError::RepetitionDraw => {
                    self.update_outcome(Outcome::DrawByRepetition)
                }
                MoveError::Draw => self.update_outcome(Outcome::Draw),
                MoveError::DrawByInsufficientMaterial => {
                    self.update_outcome(Outcome::DrawByMaterial)
                }
                MoveError::DrawByStalemate => {
                    self.update_outcome(Outcome::Stalemate)
                }
                _ => {
                    return Err(SfenError::IllegalMove);
                }
            },
        }
        return Ok(self.outcome());
    }

    /// If last position has appeared three times then it's draw.
    fn detect_repetition(&self) -> Result<(), MoveError> {
        let sfen_history = self.get_sfen_history();

        if sfen_history.len() < 9 {
            return Ok(());
        }

        let sfen_history: Vec<&String> =
            sfen_history.iter().rev().take(15).collect();

        let cur = sfen_history.last().unwrap();
        let last_sfen = cur.split_whitespace().rev().last().unwrap();
        let mut cnt = 0;
        for (_i, entry) in sfen_history.iter().rev().enumerate() {
            let s = entry.split_whitespace().rev().last().unwrap();
            if s == last_sfen {
                cnt += 1;
                if cnt == 3 {
                    return Err(MoveError::RepetitionDraw);
                }
            }
        }
        Ok(())
    }

    /// Check if one of the players don't have enough pieces.
    fn detect_insufficient_material(&self) -> Result<(), MoveError> {
        let major = [
            PieceType::Rook,
            PieceType::Queen,
            PieceType::Chancellor,
            PieceType::ArchBishop,
        ];
        let minor = [PieceType::Knight, PieceType::Bishop];
        if self.occupied_bb().count() == 2 {
            return Err(MoveError::DrawByInsufficientMaterial);
        }
        for c in Color::iter() {
            if c == Color::NoColor {
                continue;
            }
            let mut bb = B::empty();
            for i in major {
                bb |= &(&self.player_bb(c) & &self.type_bb(&i));
            }
            if bb.is_any() {
                return Ok(());
            }
            for i in minor {
                bb |= &(&self.player_bb(c) & &self.type_bb(&i));
            }
            if bb.count() == 1 {
                continue;
            }
            for pawn in &self.player_bb(c) & &self.type_bb(&PieceType::Pawn) {
                let file = pawn.file();
                let file = self.file_bb(file as usize);
                let mut file_with_plinths =
                    &file & &self.player_bb(Color::NoColor);
                if file_with_plinths.is_empty() {
                    return Ok(());
                } else if c == Color::White {
                    if let Some(sq) = file_with_plinths.pop_reverse() {
                        if sq.index() <= pawn.index() {
                            bb |= &pawn;
                            // return Ok(());
                        }
                    }
                    continue;
                } else if c == Color::Black {
                    if let Some(sq) = file_with_plinths.pop() {
                        println!("{}, {}", sq.index(), pawn.index());
                        if sq.index() >= pawn.index() {
                            bb |= &pawn;
                            // return Ok(());
                        }
                    }
                    continue;
                }
            }
            if bb.count() == 0 {
                continue;
            }

            return Ok(());
        }
        Err(MoveError::DrawByInsufficientMaterial)
    }

    /// Returns all legal moves where piece can be moved.
    fn legal_moves(&self, color: &Color) -> HashMap<S, B> {
        let mut map = HashMap::new();
        let pinned_moves = self.pinned_moves(*color);
        let check_moves = self.check_moves(color);
        let enemy_moves = self.enemy_moves(color);
        let king = self.find_king(color).unwrap();
        for sq in self.player_bb(*color) {
            let my_moves = self.non_legal_moves(&sq);
            if !check_moves.is_empty() {
                if king == sq {
                    map.insert(king, &my_moves & &!&enemy_moves);
                } else {
                    let moves = self.fix_pin(
                        &sq,
                        &pinned_moves,
                        &check_moves,
                        my_moves,
                    );
                    map.insert(sq, moves);
                }
            } else {
                let moves =
                    self.fix_pin(&sq, &pinned_moves, &check_moves, my_moves);
                map.insert(sq, moves);
            }
        }
        map
    }

    /// Returns `BitBoard` of all moves by `Color`.
    fn color_moves(&self, c: &Color) -> B {
        let mut all = B::empty();
        for sq in self.player_bb(*c) {
            let piece = self.piece_at(sq);
            let moves = self.move_candidates(
                &sq,
                piece.unwrap(),
                MoveType::NoKing {
                    king: self.find_king(&c.flip()).unwrap(),
                },
            );
            all |= &moves;
        }
        all
    }

    /// Returns  `BitBoard` of all moves after fixing pin.
    fn fix_pin(
        &self,
        sq: &S,
        pins: &HashMap<S, B>,
        checks: &Vec<B>,
        my_moves: B,
    ) -> B {
        let piece = self.piece_at(*sq).unwrap();
        if let Some(pin) = pins.get(sq) {
            match (1).cmp(&checks.len()) {
                Ordering::Equal => {
                    let checks = checks.get(0).unwrap();
                    &(checks & pin) & &my_moves
                }
                Ordering::Greater => pin & &my_moves,
                Ordering::Less => B::empty(),
            }
        } else {
            let mut my_moves = my_moves;
            let enemy_moves = self.enemy_moves(&piece.color);
            if piece.piece_type == PieceType::King {
                my_moves = &my_moves & &!&enemy_moves;
                return my_moves;
            } else if checks.len() > 1 {
                return B::empty();
            }
            for bb in checks.iter() {
                my_moves &= bb;
            }
            my_moves
        }
    }

    /// Returns `BitBoard` of all moves by opponent.
    fn enemy_moves(&self, color: &Color) -> B {
        match color {
            Color::Black | Color::White => self.color_moves(&color.flip()),
            Color::NoColor => B::empty(),
        }
    }
    /// Returns all non-legal moves.
    fn non_legal_moves(&self, square: &S) -> B {
        let piece = self.piece_at(*square);
        match piece {
            Some(i) => self.move_candidates(square, *i, MoveType::Plinth),
            None => B::empty(),
        }
    }

    /// Returns `Pin` struct, who has unpin `BitBoard`(if pin exists).
    fn pinned_moves(&self, color: Color) -> HashMap<S, B> {
        let mut pins = HashMap::new();
        if color == Color::NoColor {
            return pins;
        }
        let ksq = self.find_king(&color);
        if ksq.is_none() {
            return pins;
        }
        let ksq = ksq.unwrap();
        let plinths = self.player_bb(Color::NoColor);

        for s in [
            PieceType::Queen,
            PieceType::Rook,
            PieceType::Bishop,
            PieceType::Chancellor,
            PieceType::ArchBishop,
        ]
        .iter()
        {
            if !self.variant().can_buy(s) {
                continue;
            }
            let piece_attacks = A::get_sliding_attacks(*s, &ksq, plinths);
            // this is enemy
            let enemy_bb = &(&self.type_bb(s) & &self.player_bb(color.flip()))
                & &piece_attacks;
            for psq in enemy_bb {
                // this piece is pinned
                let mut pinned = &(&A::between(ksq, psq) & &self.occupied_bb())
                    & &!&self.player_bb(Color::NoColor);
                // this is unpin
                let my_piece = &pinned & &self.player_bb(color);
                if pinned.count() == 1 && my_piece.is_any() {
                    let fix = &(&A::between(psq, ksq) & &!&pinned) | &enemy_bb;
                    let my_square = pinned.pop_reverse();
                    pins.insert(my_square.unwrap(), fix);
                }
            }
        }
        pins
    }

    /// Returns a `BitBoard` of all squares at which a piece of the given color is pinned.
    fn pinned_bb(&self, c: Color) -> B {
        let mut bb = B::empty();
        let pinned = self.pinned_moves(c);
        for sq in self.player_bb(c) {
            if let Some(_p) = pinned.get(&sq) {
                bb |= &sq;
            }
        }
        bb
    }

    fn set_sfen(&mut self, sfen_str: &str) -> Result<Outcome, SfenError> {
        let mut parts = sfen_str.split_whitespace();
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
        self.clear_sfen_history();
        self.log_position();
        if self.in_check(self.side_to_move().flip()) {
            let checkmate = Outcome::Checkmate {
                color: self.side_to_move(),
            };
            self.update_outcome(checkmate.clone());
            return Ok(checkmate);
        }
        Ok(Outcome::Nothing)
    }

    fn in_check(&self, c: Color) -> bool {
        let king = &self.find_king(&c);
        if let Some(_k) = king {
            let check_moves = self.check_moves(&c);
            return !check_moves.is_empty();
        }
        false
    }

    /// Checks if given color is in checkmate.
    fn is_checkmate(&self, c: &Color) -> bool {
        let king = self.find_king(c);
        match king {
            Some(k) => {
                if !self.in_check(*c) {
                    return false;
                }
                let all = self.legal_moves(c);
                if let Some(king_moves) = all.get(&k) {
                    if !king_moves.is_any() {
                        let mut final_moves = B::empty();
                        for mv in all {
                            final_moves |= &mv.1;
                        }
                        if final_moves.is_any() {
                            return false;
                        }
                        return true;
                    }
                }
                false
            }
            None => false,
        }
    }

    /// Check if player is in stalemate.
    fn is_stalemate(&self, color: &Color) -> Result<(), MoveError> {
        let moves = self.legal_moves(color);
        for m in moves {
            if m.1.count() > 0 {
                return Ok(());
            }
        }
        Err(MoveError::DrawByStalemate)
    }

    /// Returns Vector of all checks.
    fn check_moves(&self, color: &Color) -> Vec<B> {
        let mut all = vec![];
        let ksq = self.find_king(color);
        if ksq.is_none() {
            return vec![];
        }
        let ksq = ksq.unwrap();

        for s in [
            PieceType::Queen,
            PieceType::Rook,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Pawn,
            PieceType::Chancellor,
            PieceType::ArchBishop,
        ]
        .iter()
        {
            if !self.variant().can_buy(s) {
                continue;
            }
            let p = Piece {
                piece_type: *s,
                color: *color,
            };
            let move_candidates =
                self.move_candidates(&ksq, p, MoveType::Plinth);
            // this is enemy
            let bb = &(&self.type_bb(s) & &self.player_bb(color.flip()))
                & &move_candidates;
            for psq in bb {
                let fix = A::between(ksq, psq);
                all.push(&fix | &bb);
            }
        }
        all
    }

    /// Returns a `BitBoard` where the given piece at the given square can move.
    fn move_candidates(&self, sq: &S, p: Piece, move_type: MoveType<S>) -> B {
        let blockers = move_type.blockers(self, &p.color);

        let bb = match p.piece_type {
            PieceType::Rook => {
                A::get_sliding_attacks(PieceType::Rook, sq, blockers)
            }
            PieceType::Bishop => {
                A::get_sliding_attacks(PieceType::Bishop, sq, blockers)
            }
            PieceType::Queen => {
                A::get_sliding_attacks(PieceType::Queen, sq, blockers)
            }
            PieceType::Knight => {
                A::get_non_sliding_attacks(PieceType::Knight, sq, p.color)
            }
            PieceType::Pawn => {
                A::get_non_sliding_attacks(PieceType::Pawn, sq, p.color)
            }
            PieceType::King => {
                A::get_non_sliding_attacks(PieceType::King, sq, p.color)
            }
            PieceType::Chancellor => {
                &A::get_non_sliding_attacks(PieceType::Knight, sq, p.color)
                    | &A::get_sliding_attacks(PieceType::Rook, sq, blockers)
            }
            PieceType::ArchBishop => {
                &A::get_non_sliding_attacks(PieceType::Knight, sq, p.color)
                    | &A::get_sliding_attacks(PieceType::Bishop, sq, blockers)
            }
            PieceType::Giraffe => A::get_girrafe_attacks(sq),
            _ => B::empty(),
        };
        move_type.moves(self, &bb, p, *sq)
    }

    /// Make move from `Move`. It can be of three types.
    /// It's useful for all three stages of the game.
    fn make_move(&mut self, m: Move<S>) -> Result<Outcome, MoveError>;

    /// Returns BitBoard with rank. Panics if file is bigger than expected.
    fn file_bb(&self, rank: usize) -> B;
}

#[derive(Debug)]
pub enum MoveType<S: Square> {
    Empty,
    Plinth,
    NoKing { king: S },
}

impl<S> MoveType<S>
where
    S: Square + Hash,
{
    pub fn blockers<B: BitBoard<S>, A: Attacks<S, B>, P: Play<S, B, A>>(
        &self,
        position: &P,
        c: &Color,
    ) -> B
    where
        for<'b> &'b B: BitOr<&'b B, Output = B>,
        for<'b> &'b B: BitAnd<&'b B, Output = B>,
        for<'a> &'a B: Not<Output = B>,
        for<'a> &'a B: BitOr<&'a S, Output = B>,
        for<'a> &'a B: BitAnd<&'a S, Output = B>,
    {
        match self {
            MoveType::Empty => B::empty(),
            MoveType::Plinth => {
                &position.occupied_bb() | &position.player_bb(Color::NoColor)
            }
            MoveType::NoKing { king } => {
                let king = B::from_square(king);
                &(&(&position.occupied_bb()
                    | &position.player_bb(Color::NoColor))
                    & &!&king)
                    | &position.player_bb(*c)
            }
        }
    }

    pub fn moves<B: BitBoard<S>, A: Attacks<S, B>, P: Play<S, B, A>>(
        &self,
        position: &P,
        bb: &B,
        p: Piece,
        sq: S,
    ) -> B
    where
        for<'b> &'b B: BitOr<&'b B, Output = B>,
        for<'b> &'b B: BitAnd<&'b B, Output = B>,
        for<'a> &'a B: Not<Output = B>,
        for<'a> &'a B: BitOr<&'a S, Output = B>,
        for<'a> &'a B: BitAnd<&'a S, Output = B>,
    {
        let my_color = p.color;
        let without_main_color = bb & &!&position.player_bb(my_color);
        let knights = [
            PieceType::Knight,
            PieceType::ArchBishop,
            PieceType::Chancellor,
            PieceType::Giraffe,
        ];
        match self {
            MoveType::Empty => B::empty(),
            MoveType::Plinth => {
                if !knights.contains(&p.piece_type) {
                    let mut without_plinth = &(without_main_color)
                        & &!&position.player_bb(Color::NoColor);
                    if p.piece_type == PieceType::Pawn {
                        without_plinth &= &position.player_bb(p.color.flip());
                        let up_sq = &!&position.player_bb(p.color.flip())
                            & &self.pawn_move::<B, A, P>(sq, &p.color);
                        without_plinth |= &up_sq;
                        without_plinth &= &!&position.player_bb(Color::NoColor);
                        without_plinth
                    } else {
                        without_plinth
                    }
                } else {
                    without_main_color
                }
            }
            MoveType::NoKing { king } => {
                if !knights.contains(&p.piece_type) {
                    if p.piece_type == PieceType::Pawn {
                        let up_sq = self.pawn_move::<B, A, P>(sq, &p.color);
                        return bb & &!&up_sq;
                    }
                    &((bb) & &!&position.player_bb(Color::NoColor))
                        | &(bb & &B::from_square(&king.to_owned()))
                } else {
                    bb | &(bb & &B::from_square(&king.to_owned()))
                }
            }
        }
    }

    pub fn pawn_move<B: BitBoard<S>, A: Attacks<S, B>, P: Play<S, B, A>>(
        &self,
        sq: S,
        color: &Color,
    ) -> B
    where
        for<'b> &'b B: BitOr<&'b B, Output = B>,
        for<'b> &'b B: BitAnd<&'b B, Output = B>,
        for<'a> &'a B: Not<Output = B>,
        for<'a> &'a B: BitOr<&'a S, Output = B>,
        for<'a> &'a B: BitAnd<&'a S, Output = B>,
    {
        match color {
            &Color::White | &Color::Black => {
                A::get_pawn_moves(sq.index(), *color)
            }
            _ => B::empty(),
        }
    }
}
