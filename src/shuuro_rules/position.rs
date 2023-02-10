use std::{
    cmp::Ordering,
    collections::HashMap,
    hash::Hash,
    ops::{BitAnd, BitOr, BitOrAssign, Not},
};

use itertools::Itertools;

use crate::{
    attacks::Attacks, bitboard::BitBoard, Color, Move, MoveError, MoveRecord, Piece, PieceType,
    SfenError, Square, Variant,
};

pub trait Position<S, B, A>
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
    fn sfen_to_bb(&mut self, piece: Piece, j: u8, i: usize);
    /// Returns a history of all moves made since the beginning of the game.
    fn ply(&self) -> u16;
    /// Increment ply
    fn increment_ply(&mut self);
    /// Change side to move.
    fn flip_side_to_move(&mut self);
    /// Set new stm
    fn update_side_to_move(&mut self, c: Color);
    /// Returns current status of the game.
    fn outcome(&self) -> Outcome;
    /// Set new outcome
    fn update_outcome(&mut self, outcome: Outcome);
    /// Returns current variant.
    fn variant(&self) -> Variant;
    /// Changing to other variant.
    fn update_variant(&mut self, variant: Variant);
    /// Make move from `Move`. It can be of three types.
    /// It's useful for all three stages of the game.
    fn make_move(&mut self, m: Move<S>) -> Result<Outcome, MoveError>;
    /// Insert new sfen to sfen history.
    fn insert_sfen(&mut self, sfen: &str);
    /// Insert new MoveRecord to move_history.
    fn insert_move(&mut self, m: MoveRecord<S>);
    /// Detecting ininsufficient material for both sides.
    fn detect_insufficient_material(&self) -> Result<(), MoveError> {
        let major = [PieceType::Rook, PieceType::Queen];
        let minor = [PieceType::Knight, PieceType::Bishop];
        if self.occupied_bb().count() == 2 {
            return Err(MoveError::DrawByInsufficientMaterial);
        }
        for c in Color::iter() {
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
            if bb.count() == 1 && bb.count() == 0 {
                continue;
            }

            return Ok(());
        }
        Err(MoveError::DrawByInsufficientMaterial)
    }
    /// If last position has appeared three times then it's draw.
    fn detect_repetition(&self) -> Result<(), MoveError>;
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
    /// Set `Position` from `&str`.
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

    /// Clear sfen_history
    fn clear_sfen_history(&mut self);
    /// Set sfen history.
    fn set_sfen_history(&mut self, history: Vec<(String, u16)>);
    /// Set history of previous moves.
    fn set_move_history(&mut self, history: Vec<MoveRecord<S>>);
    /// Returns history of all moves in `MoveRecord` format.
    fn move_history(&self) -> &[MoveRecord<S>];
    fn get_move_history(&self) -> &Vec<MoveRecord<S>>;
    /// Returns history of all moves in `Vec` format.
    fn get_sfen_history(&self) -> &Vec<(String, u16)>;
    /// Check if last move leads to stalemate.
    fn is_stalemate(&self, color: &Color) -> Result<(), MoveError> {
        let moves = self.legal_moves(color);
        for m in moves {
            if m.1.count() > 0 {
                return Ok(());
            }
        }
        Err(MoveError::DrawByStalemate)
    }
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
            format!("{} {}", sfen_history.first().unwrap().0, ply);
        }
        format!(
            "{} {}",
            &sfen_history.first().unwrap().0,
            ply - move_history.len() as u16
        )
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
                            let num2 = format!("{}{}", num_pieces, n as u8).parse::<u8>().unwrap();
                            num_pieces = num2;
                            continue;
                        }
                        num_pieces = n as u8;
                    }
                }
                s => {
                    match Piece::from_sfen(s) {
                        Some(p) => {
                            self.insert_in_hand(p, if num_pieces == 0 { 1 } else { num_pieces })
                        }
                        None => return Err(SfenError::IllegalPieceType),
                    };
                    num_pieces = 0;
                }
            }
        }

        Ok(())
    }

    fn parse_sfen_ply(&mut self, s: &str) -> Result<(), SfenError>;
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
        let rows = s.split('/');
        let dimension = self.dimensions();
        self.empty_all_bb();
        for (i, row) in rows.enumerate() {
            if i >= dimension as usize {
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
                    n if n.is_numeric() => {
                        if let Some(n) = n.to_digit(11) {
                            for _ in 0..n {
                                if j >= dimension {
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
                            if j >= dimension {
                                return Err(SfenError::IllegalBoardState);
                            }

                            if is_promoted {
                                if let Some(promoted) = piece.piece_type.promote() {
                                    piece.piece_type = promoted;
                                } else {
                                    return Err(SfenError::IllegalPieceType);
                                }
                            }

                            self.sfen_to_bb(piece, j, i);
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

                            if (&self.player_bb(Color::NoColor) & &sq).is_any() {
                                if knights.contains(&pc.piece_type) {
                                    s.push('L');
                                } else {

                                    //return Err(SfenError::IllegalPieceTypeOnPlinth);
                                }
                            }

                            s.push_str(&pc.to_string());
                        }
                        None => {
                            if (&self.player_bb(Color::NoColor) & &sq).is_any() {
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
                        let n = self.get_hand_piece(pc);

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
    // PLACEMENT PART
    fn generate_plinths(&mut self);
    fn insert_in_hand(&mut self, p: Piece, num: u8);
    fn set_hand(&mut self, s: &str);
    fn get_hand(&self, c: Color) -> String;
    fn get_hand_piece(&self, p: Piece) -> u8;
    fn clear_hand(&mut self);
    /// Returns the number of the given piece in hand.
    fn hand(&self, p: Piece) -> u8;
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
    fn king_files<const K: usize>(&self) -> [&str; K];
    fn empty_squares(&self, p: Piece) -> B;
    fn is_king_placed(&self, c: Color) -> bool {
        let king = &self.player_bb(c) & &self.type_bb(&PieceType::King);
        if king.count() == 1 {
            return true;
        }
        false
    }
    fn checks(&self, attacked_color: &Color) -> B {
        let king = &self.type_bb(&PieceType::King) & &self.player_bb(*attacked_color);
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
            let bb = &self.type_bb(&p) & &self.player_bb(attacked_color.flip());
            for i in bb {
                all = A::get_sliding_attacks(p, &i, occupied_bb);
                if (&all & &king).is_any() {
                    match *attacked_color {
                        Color::White => {
                            let files = self.white_files();
                            return &(&files & &all) & &!&king;
                        }
                        Color::Black => {
                            let files = self.black_files();
                            return &(&files & &all) & &!&king;
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
    fn white_files(&self) -> B;
    fn black_files(&self) -> B;
    fn can_pawn_move(&self, p: Piece) -> bool {
        self.is_hand_empty(p.color, PieceType::Pawn)
    }
    fn is_hand_empty(&self, c: Color, excluded: PieceType) -> bool;
    fn decrement_hand(&mut self, p: Piece);
    fn place(&mut self, p: Piece, sq: S) -> Option<String> {
        if self.get_hand_piece(p) > 0 && (&self.empty_squares(p) & &sq).is_any() {
            self.update_bb(p, sq);
            self.decrement_hand(p);
            let move_record = MoveRecord::Put { to: sq, piece: p };
            let sfen = self.generate_sfen().split(' ').next().unwrap().to_string();
            let hand = {
                let s = self.get_hand(Color::White) + &self.get_hand(Color::Black)[..];
                if s.is_empty() {
                    String::from(" ")
                } else {
                    s
                }
            };
            self.increment_ply();
            let ply = self.ply();

            self.insert_move(move_record.clone());
            if !self.is_hand_empty(p.color, PieceType::Plinth) {
                self.flip_side_to_move();
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
    fn update_bb(&mut self, p: Piece, sq: S);
    fn halfmoves(&self) -> B;
    fn dimensions(&self) -> u8;
    fn us(&self) -> B;
    fn them(&self) -> B;
    /// Create move from `&str`.
    fn play(&mut self, from: &str, to: &str) -> Result<&Outcome, SfenError>;
    /// Returns a `BitBoard` where the given piece at the given square can move.
    fn move_candidates(&self, sq: &S, p: Piece, move_type: MoveType<S>) -> B {
        let blockers = move_type.blockers(self, &p.color);

        let bb = match p.piece_type {
            PieceType::Rook => A::get_sliding_attacks(PieceType::Rook, sq, blockers),
            PieceType::Bishop => A::get_sliding_attacks(PieceType::Bishop, sq, blockers),
            PieceType::Queen => A::get_sliding_attacks(PieceType::Queen, sq, blockers),
            PieceType::Knight => A::get_non_sliding_attacks(PieceType::Knight, sq, p.color),
            PieceType::Pawn => A::get_non_sliding_attacks(PieceType::Pawn, sq, p.color),
            PieceType::King => A::get_non_sliding_attacks(PieceType::King, sq, p.color),
            PieceType::Chancellor => {
                &A::get_non_sliding_attacks(PieceType::Knight, sq, p.color)
                    | &A::get_sliding_attacks(PieceType::Rook, sq, blockers)
            }
            PieceType::ArchBishop => {
                &A::get_non_sliding_attacks(PieceType::Knight, sq, p.color)
                    | &A::get_sliding_attacks(PieceType::Bishop, sq, blockers)
            }
            _ => B::empty(),
        };
        move_type.moves(self, &bb, p, *sq)
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
                    let moves = self.fix_pin(&sq, &pinned_moves, &check_moves, my_moves);
                    map.insert(sq, moves);
                }
            } else {
                let moves = self.fix_pin(&sq, &pinned_moves, &check_moves, my_moves);
                map.insert(sq, moves);
            }
        }
        map
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
            let enemy_bb = &(&self.type_bb(s) & &self.player_bb(color.flip())) & &piece_attacks;
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
            let move_candidates = self.move_candidates(&ksq, p, MoveType::Plinth);
            // this is enemy
            let bb = &(&self.type_bb(s) & &self.player_bb(color.flip())) & &move_candidates;
            for psq in bb {
                let fix = A::between(ksq, psq);
                all.push(&fix | &bb);
            }
        }
        all
    }
    /// Checks if the king with the given color is in check.
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
    /// Returns  `BitBoard` of all moves after fixing pin.
    fn fix_pin(&self, sq: &S, pins: &HashMap<S, B>, checks: &Vec<B>, my_moves: B) -> B {
        let piece = self.piece_at(*sq).unwrap();
        if let Some(pin) = pins.get(sq) {
            match (1).cmp(&checks.len()) {
                Ordering::Equal => {
                    let checks = checks.get(0).unwrap();
                    &(checks & pin) & &my_moves
                }
                Ordering::Greater => B::empty(),
                Ordering::Less => pin & &my_moves,
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
    /// Returns `BitBoard` of all moves by opponent.
    fn enemy_moves(&self, color: &Color) -> B {
        match color {
            Color::Black | Color::White => self.color_moves(&color.flip()),
            Color::NoColor => B::empty(),
        }
    }
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
            Outcome::Checkmate { color } => format!("Checkmate_{}", color.to_string()),
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
    pub fn blockers<B: BitBoard<S>, A: Attacks<S, B>, P: Position<S, B, A>>(
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
            MoveType::Plinth => &position.occupied_bb() | &position.player_bb(Color::NoColor),
            MoveType::NoKing { king } => {
                let king = B::from_square(king);
                &(&(&position.occupied_bb() | &position.player_bb(Color::NoColor)) & &!&king)
                    | &position.player_bb(*c)
            }
        }
    }

    pub fn moves<B: BitBoard<S>, A: Attacks<S, B>, P: Position<S, B, A>>(
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
        ];
        match self {
            MoveType::Empty => B::empty(),
            MoveType::Plinth => {
                if !knights.contains(&p.piece_type) {
                    let mut without_plinth =
                        &(without_main_color) & &!&position.player_bb(Color::NoColor);
                    if p.piece_type == PieceType::Pawn {
                        let up_sq = &!&position.player_bb(p.color.flip())
                            & &self.get_pawn_square(sq, &p.color);
                        without_plinth |= &up_sq;
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
                        let up_sq = self.get_pawn_square(sq, &p.color);
                        let up_sq = B::from_square(&up_sq);
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

    pub fn get_pawn_square(&self, sq: S, color: &Color) -> S {
        match color {
            &Color::White | &Color::Black => S::from_index(sq.index() as u8).unwrap(),
            _ => sq,
        }
    }
}