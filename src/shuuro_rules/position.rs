use std::{
    clone::Clone, cmp::Ordering, collections::HashMap, hash::Hash,
    marker::PhantomData,
};

use itertools::Itertools;

use crate::{
    attacks::Attacks, bitboard::BitBoard, Color, Hand, Move, MoveData,
    MoveError, Piece, PieceType, SfenError, Square, Variant,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct Checks<S, B>
where
    S: Square + Hash,
    B: BitBoard<S>,
    Self: Sized,
{
    pub check: Option<B>,
    pub double_check: bool,
    pub enemy_moves: Option<B>,
    _0: PhantomData<S>,
}

impl<S, B> Checks<S, B>
where
    S: Square + Hash,
    B: BitBoard<S>,
    Self: Sized,
{
    fn add_enemy_moves(self, enemy_moves: B) -> Option<Self> {
        Some(Self {
            check: self.check,
            double_check: self.double_check,
            enemy_moves: Some(enemy_moves),
            _0: self._0,
        })
    }

    fn new(
        check: Option<B>,
        double_check: bool,
        enemy_moves: Option<B>,
    ) -> Self {
        Self {
            check,
            double_check,
            enemy_moves,
            _0: PhantomData,
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
        + Clone
        + Board<S, B, A>
        + Sfen<S, B, A>
        + Placement<S, B, A>
        + Play<S, B, A>
        + Rules<S, B, A>,
{
}

pub trait Board<S, B, A>
where
    S: Square + Hash,
    B: BitBoard<S>,
    Self: Sized + Clone,
    A: Attacks<S, B>,
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
    fn insert_sfen(&mut self, sfen: Move<S>);
    /// Insert new Move2 to move_history.
    fn insert_move(&mut self, m: Move<S>);
    /// Clear sfen_history
    fn clear_sfen_history(&mut self);
    /// Set sfen history.
    fn set_sfen_history(&mut self, history: Vec<String>) {
        for i in history {
            let m: Result<Move<S>, ()> = Move::try_from(i);
            if let Ok(m) = m {
                self.insert_sfen(m);
            }
        }
    }
    /// Set history of previous moves.
    fn set_move_history(&mut self, history: Vec<Move<S>>);
    /// Returns history of all moves in `Move2` format.
    fn move_history(&self) -> &[Move<S>];
    /// Update last move.
    fn update_last_move(&mut self, m: &str);
    /// Returns history of all moves in `Vec` format.
    fn get_sfen_history(&self) -> Vec<String> {
        let mut v = Vec::new();
        for i in self.move_history() {
            v.push(i.to_fen());
        }
        v
    }
    /// Get hand count for Piece.
    fn hand(&self, p: Piece) -> u8;
    /// Get hand in form of String
    fn get_hand(&self, c: Color, long: bool) -> String;
    /// Set hand from str.
    fn set_hand(&mut self, s: &str);
    /// Decrement player hand.
    fn decrement_hand(&mut self, p: Piece);
    /// Dimensions of board.
    fn dimensions(&self) -> u8;
    /// Returns `Square` if King is available.
    fn find_king(&self, c: &Color) -> Option<S> {
        let mut bb = self.type_bb(&PieceType::King) & &self.player_bb(*c);
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
    Self: Sized + Clone + Board<S, B, A>,
{
    // SFEN PART
    /// Convert current position to sfen.
    fn to_sfen(&self) -> String {
        let sfen_history = self.get_sfen_history();
        let move_history = self.move_history();
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
        let dimension = self.dimensions();
        let mut fen = String::new();
        let mut is_plinth = false;
        for rank in 0..dimension {
            dbg!(rank, dimension);
            let mut row_item = String::from("");
            let mut space = 0;
            for file in 0..dimension {
                let sq = S::new(file, rank).unwrap();
                match *self.piece_at(sq) {
                    Some(piece) => {
                        if space > 0 {
                            row_item = self.add_space(space, row_item);
                            space = 0;
                        }
                        if is_plinth {
                            if piece.piece_type.is_knight_piece() {
                                row_item.push_str(&piece.to_string());
                            }
                            is_plinth = false;
                        } else {
                            row_item.push_str(&piece.to_string());
                            space = 0;
                        }
                    }
                    None => {
                        if (self.player_bb(Color::NoColor) & &sq).is_any() {
                            row_item = self.add_space(space, row_item);
                            space = 0;
                            is_plinth = true;
                            row_item.push('L');
                        } else if is_plinth {
                            row_item.push('0');
                            space = 1;
                            is_plinth = false;
                        } else {
                            space += 1;
                        }
                    }
                }
            }
            if space > 0 {
                row_item = self.add_space(space, row_item);
            }
            fen.push_str(&row_item);
            if rank < dimension - 1 {
                if is_plinth {
                    fen.push('0');
                    is_plinth = false;
                }
                fen.push('/');
            }
        }
        let color = if self.side_to_move() == Color::Black {
            "b"
        } else {
            "w"
        };

        let black = self.get_hand(Color::Black, false);
        let white = self.get_hand(Color::White, false);
        let mut hand = String::new();
        hand.push_str(&black);
        hand.push_str(&white);
        if hand.is_empty() {
            hand = "-".to_string();
        }
        format!("{} {} {} {}", fen, color, hand, self.ply())
    }

    fn add_space(&self, n: u8, mut s: String) -> String {
        match n {
            10 => s.push_str("55"),
            11 => s.push_str("56"),
            12 => s.push_str("57"),
            _ => s.push_str(&n.to_string()),
        }
        s
    }

    fn clear_hand(&mut self);

    fn new_hand(&mut self, hand: Hand);

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

    fn parse_sfen_board(&mut self, fen: &str) -> Result<(), SfenError> {
        let ranks = fen.split('/');
        let dimension = self.dimensions();
        self.empty_all_bb();
        for (rank, file) in ranks.enumerate() {
            if rank >= dimension as usize {
                return Err(SfenError::IllegalBoardState);
            }
            let mut current_file = 0;
            let mut is_plinth = false;

            for ch in file.chars() {
                match ch {
                    n if n.is_numeric() => {
                        if let Some(n) = n.to_digit(10) {
                            if n != 0 {
                                for _ in 0..n {
                                    if current_file >= dimension {
                                        return Err(
                                            SfenError::IllegalBoardState,
                                        );
                                    }
                                    let sq = S::new(current_file, rank as u8)
                                        .unwrap();

                                    self.set_piece(sq, None);
                                    current_file += 1;
                                }
                            } else if n == 0 {
                                if is_plinth {
                                    is_plinth = false;
                                    let sq = S::new(current_file, rank as u8)
                                        .unwrap();
                                    self.set_piece(sq, None);
                                    current_file += 1;
                                } else {
                                    return Err(SfenError::IllegalBoardState);
                                }
                            }
                            //
                        }
                    }
                    s => {
                        if let Some(piece) = Piece::from_sfen(s) {
                            if current_file >= dimension {
                                return Err(SfenError::IllegalBoardState);
                            }
                            let sq = S::new(current_file, rank as u8).unwrap();
                            match piece.piece_type {
                                PieceType::Plinth => {
                                    self.update_player(piece, &sq);
                                    is_plinth = true;
                                    continue;
                                }
                                _ => {
                                    self.sfen_to_bb(piece, &sq);
                                    current_file += 1;
                                }
                            }
                        } else {
                            return Err(SfenError::IllegalPieceType);
                        }
                    }
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

        self.new_hand(Hand::from(s));

        Ok(())
    }

    /// Saves position in sfen_history.
    fn log_position(&mut self) {
        let mut sfen = self.generate_sfen().split(' ').take(3).join(" ");
        let move_history = self.move_history();
        if let Some(last) = move_history.last() {
            sfen.push_str(format!(" {} ", self.ply()).as_str());
            sfen.push_str(&last.to_string());
            sfen.push_str(&format!(" {}", &last.format()));
        }
        self.update_last_move(&sfen);
        // self.insert_sfen(&sfen);
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
    Self: Sized + Clone + Board<S, B, A> + Sfen<S, B, A>,
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
        let king = self.player_bb(c) & &self.type_bb(&PieceType::King);
        if king.len() == 1 {
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
            bb &= &!plinths;
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
        let calc = |p: Piece, ranks: [usize; 3]| -> B {
            for rank in ranks {
                let mut bb = self.rank_bb(rank);
                bb &= &!self.player_bb(p.color);
                let plinths = self.player_bb(Color::NoColor);
                if bb.is_empty() {
                    continue;
                }
                match p.piece_type {
                    PieceType::Knight
                    | PieceType::Chancellor
                    | PieceType::ArchBishop
                    | PieceType::Giraffe => {
                        return bb;
                    }
                    PieceType::King => {
                        return self.king_squares::<6>(&p.color);
                    }
                    PieceType::Pawn => {
                        bb &= &!plinths;
                        if bb.is_empty() {
                            continue;
                        } else if self.can_pawn_move(p) {
                            if rank == 0 || rank == self.black_ranks()[0] {
                                continue;
                            }
                            return bb;
                        } else {
                            return B::empty();
                        }
                    }
                    _ => {
                        bb &= &!plinths;
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
            self.type_bb(&PieceType::King) & &self.player_bb(*attacked_color);
        if king.is_empty() {
            return B::empty();
        }
        let occupied_bb = self.occupied_bb() | &self.player_bb(Color::NoColor);
        let king_sq = (king | &B::empty()).pop().unwrap();

        for pt in [PieceType::Rook, PieceType::Bishop] {
            let king_attacks =
                A::get_sliding_attacks(pt, &king_sq, occupied_bb);

            for p in [
                PieceType::Queen,
                PieceType::Rook,
                PieceType::Chancellor,
                PieceType::Bishop,
                PieceType::ArchBishop,
            ] {
                if !self.variant().can_buy(&p) {
                    continue;
                }
                if pt == PieceType::Rook && !p.is_rook_type() {
                    continue;
                }
                if pt == PieceType::Bishop && !p.is_bishop_type() {
                    continue;
                }

                let them =
                    self.type_bb(&p) & &self.player_bb(attacked_color.flip());

                if (them & &king_attacks).is_any() {
                    match *attacked_color {
                        Color::White => {
                            let ranks = self.white_placement_attacked_ranks();
                            let attacks = (ranks & &king_attacks) & &!king;
                            return attacks;
                        }
                        Color::Black => {
                            let ranks = self.black_placement_attacked_ranks();
                            let attacks = (ranks & &king_attacks) & &!king;
                            return attacks;
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
        } else if self.hand(p) > 0 && (self.empty_squares(p) & &sq).is_any() {
            self.update_bb(p, sq);
            self.decrement_hand(p);
            let move_record = Move::Put {
                to: sq,
                piece: p,
                fen: String::new(),
            };
            let sfen =
                self.generate_sfen().split(' ').next().unwrap().to_string();
            let hand = {
                let s = self.get_hand(Color::White, false)
                    + &self.get_hand(Color::Black, false);
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
                &move_record.to_string(),
                &sfen,
                hand,
                self.side_to_move().to_string(),
                ply
            );
            self.update_last_move(&record);
            // self.insert_sfen(&record);
            return Some(record);
        }
        None
    }

    fn empty_placement_board() -> String;
}

pub trait Play<S, B, A>
where
    S: Square + Hash,
    B: BitBoard<S>,
    A: Attacks<S, B>,
    Self: Sized + Clone + Board<S, B, A> + Sfen<S, B, A> + Rules<S, B, A>,
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
        let m = Move::new(from, to);
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
        let sfen_history = self.move_history();
        let mut h = Vec::new();
        for i in sfen_history {
            if let Move::Normal { fen, .. } = i {
                h.push(fen);
                if h.len() > 10 {
                    break;
                }
            }
        }
        let sfen_history: Vec<&&String> = h.iter().rev().take(15).collect();
        let cur = sfen_history.last().unwrap();
        let last_sfen = cur.split_whitespace().rev().last().unwrap();
        let mut cnt = 0;
        for entry in sfen_history.iter().rev() {
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
        let minor = [PieceType::Knight, PieceType::Bishop, PieceType::Giraffe];
        if self.occupied_bb().len() == 2 {
            return Err(MoveError::DrawByInsufficientMaterial);
        }
        for c in Color::iter() {
            if c == Color::NoColor {
                continue;
            }
            let mut bb = B::empty();
            for i in major {
                bb |= &(self.player_bb(c) & &self.type_bb(&i));
            }
            if bb.is_any() {
                return Ok(());
            }
            for i in minor {
                bb |= &(self.player_bb(c) & &self.type_bb(&i));
            }
            let minor_count = bb.len();
            if minor_count >= 3 {
                return Ok(());
            }
            for pawn in self.player_bb(c) & &self.type_bb(&PieceType::Pawn) {
                let file = pawn.file();
                let file = self.file_bb(file as usize);
                let mut file_with_plinths =
                    file & &self.player_bb(Color::NoColor);
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
                        if sq.index() >= pawn.index() {
                            bb |= &pawn;
                            // return Ok(());
                        }
                    }
                    continue;
                }
            }
            if bb.len() == 0 || minor_count == bb.len() {
                continue;
            }

            return Ok(());
        }
        Err(MoveError::DrawByInsufficientMaterial)
    }

    /// Returns all legal moves where piece can be moved.
    fn legal_moves(&self, color: &Color) -> HashMap<S, B> {
        let mut map = HashMap::new();
        let pinned_moves = self.pins(color);
        let check_moves = self.check_moves(*color);
        let enemy_moves = self.enemy_moves(color);
        let move_task = check_moves.add_enemy_moves(enemy_moves).unwrap();
        let king = self.find_king(color).unwrap();
        for sq in self.player_bb(*color) {
            let my_moves = self.non_legal_moves(&sq);
            if check_moves.check.is_some() {
                if king == sq {
                    map.insert(king, my_moves & &!enemy_moves);
                } else {
                    let moves =
                        self.fix_pin(&sq, &pinned_moves, move_task, my_moves);
                    map.insert(sq, moves);
                }
            } else {
                let moves =
                    self.fix_pin(&sq, &pinned_moves, move_task, my_moves);
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

    // /// Returns  `BitBoard` of all moves after fixing pin.
    // fn fix_pin(
    //     &self,
    //     sq: &S,
    //     pins: &HashMap<S, B>,
    //     checks: &Vec<B>,
    //     my_moves: B,
    // ) -> B {
    //     let piece = self.piece_at(*sq).unwrap();
    //     if let Some(pin) = pins.get(sq) {
    //         match (1).cmp(&checks.len()) {
    //             Ordering::Equal => {
    //                 let checks = checks.get(0).unwrap();
    //                 &(checks & pin) & &my_moves
    //             }
    //             Ordering::Greater => pin & &my_moves,
    //             Ordering::Less => B::empty(),
    //         }
    //     } else {
    //         let mut my_moves = my_moves;
    //         let enemy_moves = self.enemy_moves(&piece.color);
    //         if piece.piece_type == PieceType::King {
    //             my_moves = my_moves & &!enemy_moves;
    //             return my_moves;
    //         } else if checks.len() > 1 {
    //             return B::empty();
    //         }
    //         for bb in checks.iter() {
    //             my_moves &= bb;
    //         }
    //         my_moves
    //     }
    // }

    /// Returns `BitBoard` of all moves by opponent.
    fn enemy_moves(&self, color: &Color) -> B {
        match color {
            Color::Black | Color::White => {
                let mut all = B::empty();
                let enemy = color.flip();
                let blockers =
                    self.occupied_bb() | &self.player_bb(Color::NoColor);
                let king = self.find_king(color).unwrap();
                let blockers = blockers ^ &B::from_square(&king);
                for sq in self.player_bb(enemy).into_iter() {
                    let piece = self.piece_at(sq);
                    if let Some(piece) = piece {
                        if piece.piece_type == PieceType::Pawn {
                            let moves =
                                self.get_moves(&sq, piece, blockers | &king);
                            all |= &moves;
                        }
                        let moves = self.get_moves(&sq, piece, blockers);
                        all |= &moves;
                    }
                }
                all &= &!self.player_bb(Color::NoColor);
                all
            }
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
            let enemy_bb = (self.type_bb(s) & &self.player_bb(color.flip()))
                & &piece_attacks;
            for psq in enemy_bb {
                // this piece is pinned
                let mut pinned = (A::between(ksq, psq) & &self.occupied_bb())
                    & &!self.player_bb(Color::NoColor);
                // this is unpin
                let my_piece = pinned & &self.player_bb(color);
                if pinned.len() == 1 && my_piece.is_any() {
                    let fix = (A::between(psq, ksq) & &!pinned) | &enemy_bb;
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
        if let Some(k) = king {
            let check_moves = self.enemy_moves(&c);
            return (check_moves & k).is_any();
            // if let MoveTask::Checks { check, .. } = check_moves {
            //     if let Some(check) = check {
            //         return !check.is_empty();
            //     }
            // }
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

    fn gen_move_data(
        &self,
        legal_moves: &HashMap<S, B>,
        (from, to): (S, S),
        piece: Piece,
        move_data: MoveData,
    ) -> MoveData {
        let color = self.player_bb(piece.color);
        let pieces = self.type_bb(&piece.piece_type);
        let all = color & &pieces;
        let (mut same_file, mut same_rank) = (false, false);
        for p in all {
            if let Some(targets) = legal_moves.get(&p) {
                if (*targets & &to).is_any() {
                    if from.rank() == p.rank() {
                        same_rank = true;
                    } else if from.file() == p.file() {
                        same_file = true;
                    }
                }
            }
        }
        move_data.precise(same_file, same_rank)
    }

    /// Check if player is in stalemate.
    fn is_stalemate(&self, color: &Color) -> Result<(), MoveError> {
        let moves = self.legal_moves(color);
        for m in moves {
            if m.1.len() > 0 {
                return Ok(());
            }
        }
        Err(MoveError::DrawByStalemate)
    }

    /// Returns a `BitBoard` where the given piece at the given square can move.
    fn move_candidates(
        &self,
        current_sq: &S,
        piece: Piece,
        move_type: MoveType<S>,
    ) -> B {
        let blockers = move_type.blockers(self, &piece.color);

        let attacks = self.get_moves(current_sq, &piece, blockers);

        move_type.moves(self, &attacks, piece, *current_sq)
    }

    /// Returns BitBoard with rank. Panics if file is bigger than expected.
    fn file_bb(&self, rank: usize) -> B;

    #[allow(clippy::too_many_arguments)]
    fn update_after_move(
        &mut self,
        from: S,
        to: S,
        placed: Piece,
        moved: Piece,
        captured: Option<Piece>,
        opponent: Color,
        move_data: MoveData,
    ) -> MoveData;

    fn game_status(&self) -> Outcome;

    /// Make move from `Move`. It can be of three types.
    /// It's useful for all three stages of the game.
    fn make_move(&mut self, m: Move<S>) -> Result<Outcome, MoveError> {
        let mut promoted = false;
        let stm = self.side_to_move();
        let opponent = stm.flip();
        if let Some((from, to)) = m.info() {
            //
            let moved = self
                .piece_at(from)
                .ok_or(MoveError::Inconsistent("No piece found"))?;
            let captured = *self.piece_at(to);
            let outcome = Outcome::Checkmate { color: opponent };
            let legal_moves = self.legal_moves(&stm);

            if moved.color != stm {
                return Err(MoveError::Inconsistent(
                    "The piece is not for the side to move",
                ));
            } else if self.game_status() == outcome {
                return Err(MoveError::Inconsistent("Match is over."));
            }

            match captured {
                Some(_i) => {
                    if moved.piece_type == PieceType::Pawn
                        && to.in_promotion_zone(moved.color)
                    {
                        promoted = true;
                    }
                }
                None => {
                    if moved.piece_type == PieceType::Pawn
                        && to.in_promotion_zone(moved.color)
                    {
                        promoted = true;
                    }
                }
            }

            if let Some(attacks) = legal_moves.get(&from) {
                if (*attacks & &to).is_empty() {
                    return Err(MoveError::Inconsistent(
                        "The piece cannot move to there",
                    ));
                }
            } else {
                return Err(MoveError::Inconsistent(
                    "The piece cannot move to there",
                ));
            }
            let mut move_data = MoveData::default();

            let placed = if promoted {
                match moved.promote() {
                    Some(promoted) => promoted,
                    None => {
                        return Err(MoveError::Inconsistent(
                            "This type of piece cannot promote",
                        ));
                    }
                }
            } else {
                moved
            };

            move_data = move_data.promoted(promoted);
            move_data = move_data.piece(Some(moved));

            move_data = self.update_after_move(
                from, to, placed, moved, captured, opponent, move_data,
            );

            let stm = self.side_to_move();

            let outcome = {
                if self.is_checkmate(&stm) {
                    move_data = move_data.checks(false, true);
                    Outcome::Checkmate { color: stm.flip() }
                } else if self.in_check(stm) {
                    move_data = move_data.checks(true, false);
                    Outcome::Check { color: stm }
                } else if (self.player_bb(stm.flip())
                    & &self.type_bb(&PieceType::King))
                    .len()
                    == 0
                {
                    move_data = move_data.checks(false, true);
                    Outcome::Checkmate { color: stm.flip() }
                } else {
                    Outcome::MoveOk
                }
            };

            move_data =
                self.gen_move_data(&legal_moves, (from, to), moved, move_data);
            let move_record = Move::Normal {
                from,
                to,
                placed,
                move_data,
                fen: String::new(),
            };

            self.insert_move(move_record);

            self.log_position();
            self.detect_repetition()?;
            self.detect_insufficient_material()?;

            if outcome == Outcome::MoveOk {
                self.is_stalemate(&stm)?;
            }
            Ok(outcome)
        } else {
            Err(MoveError::Inconsistent("No piece found"))
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
    pub fn blockers<B: BitBoard<S>, A: Attacks<S, B>, P: Play<S, B, A>>(
        &self,
        position: &P,
        c: &Color,
    ) -> B {
        match self {
            MoveType::Empty => B::empty(),
            MoveType::Plinth => {
                position.occupied_bb() | &position.player_bb(Color::NoColor)
            }
            MoveType::NoKing { king } => {
                let king = B::from_square(king);
                ((position.occupied_bb() | &position.player_bb(Color::NoColor))
                    & &!king)
                    | &position.player_bb(*c)
            }
        }
    }

    pub fn moves<B: BitBoard<S>, A: Attacks<S, B>, P: Play<S, B, A>>(
        &self,
        position: &P,
        attacks: &B,
        piece: Piece,
        current_sq: S,
    ) -> B {
        let my_color = piece.color;
        let without_main_color = *attacks & &!position.player_bb(my_color);
        let knights = [
            PieceType::Knight,
            PieceType::ArchBishop,
            PieceType::Chancellor,
            PieceType::Giraffe,
        ];
        match self {
            MoveType::Empty => B::empty(),
            MoveType::Plinth => {
                if !knights.contains(&piece.piece_type) {
                    let mut without_plinth = (without_main_color)
                        & &!position.player_bb(Color::NoColor);
                    if piece.piece_type == PieceType::Pawn {
                        without_plinth &=
                            &position.player_bb(piece.color.flip());
                        let up_sq = !position.player_bb(piece.color.flip())
                            & &self.pawn_move::<B, A, P>(
                                current_sq,
                                &piece.color,
                                position,
                            );

                        without_plinth |=
                            &(up_sq & &!position.player_bb(my_color));
                        without_plinth &= &!position.player_bb(Color::NoColor);

                        without_plinth
                    } else {
                        without_plinth
                    }
                } else {
                    without_main_color
                }
            }
            MoveType::NoKing { king } => {
                if !knights.contains(&piece.piece_type) {
                    if piece.piece_type == PieceType::Pawn {
                        let up_sq = self.pawn_move::<B, A, P>(
                            current_sq,
                            &piece.color,
                            position,
                        );
                        return *attacks & &!up_sq;
                    }
                    ((*attacks) & &!position.player_bb(Color::NoColor))
                        | &(*attacks & &B::from_square(&king.to_owned()))
                } else {
                    *attacks | &(*attacks & &B::from_square(&king.to_owned()))
                }
            }
        }
    }

    pub fn pawn_move<B: BitBoard<S>, A: Attacks<S, B>, P: Play<S, B, A>>(
        &self,
        sq: S,
        color: &Color,
        position: &P,
    ) -> B {
        let pop_sq = |sq: S, mut blocked: B, color: &Color| -> B {
            let old = blocked;
            let deleted = {
                if color == &Color::White {
                    blocked.pop()
                } else {
                    blocked.pop_reverse()
                }
            };
            if let Some(deleted) = deleted {
                A::between(sq, deleted)
            } else {
                old
            }
        };

        let occupied =
            position.occupied_bb() | &position.player_bb(Color::NoColor);
        match color {
            &Color::White | &Color::Black => {
                let moves = A::get_pawn_moves(sq.index(), *color);
                let blocked = moves & &occupied;
                if blocked.len() == 2 {
                    B::empty()
                } else if blocked.len() == 0 && moves.len() == 2 {
                    moves
                } else if sq.first_pawn_rank(*color) {
                    pop_sq(sq, blocked, color)
                } else {
                    moves
                }
            }
            _ => B::empty(),
        }
    }
}

pub trait Rules<S, B, A>
where
    S: Square + Hash,
    B: BitBoard<S>,
    A: Attacks<S, B>,
    Self: Sized + Clone + Board<S, B, A>,
{
    fn get_moves(&self, current_sq: &S, piece: &Piece, blockers: B) -> B {
        match piece.piece_type {
            PieceType::Rook => {
                A::get_sliding_attacks(PieceType::Rook, current_sq, blockers)
            }
            PieceType::Bishop => {
                A::get_sliding_attacks(PieceType::Bishop, current_sq, blockers)
            }
            PieceType::Queen => {
                A::get_sliding_attacks(PieceType::Queen, current_sq, blockers)
            }
            PieceType::Knight => A::get_non_sliding_attacks(
                PieceType::Knight,
                current_sq,
                piece.color,
                blockers,
            ),
            PieceType::Pawn => A::get_non_sliding_attacks(
                PieceType::Pawn,
                current_sq,
                piece.color,
                blockers,
            ),
            PieceType::King => A::get_non_sliding_attacks(
                PieceType::King,
                current_sq,
                piece.color,
                blockers,
            ),
            PieceType::Chancellor => {
                A::get_non_sliding_attacks(
                    PieceType::Knight,
                    current_sq,
                    piece.color,
                    blockers,
                ) | &A::get_sliding_attacks(
                    PieceType::Rook,
                    current_sq,
                    blockers,
                )
            }
            PieceType::ArchBishop => {
                A::get_non_sliding_attacks(
                    PieceType::Knight,
                    current_sq,
                    piece.color,
                    B::empty(),
                ) | &A::get_sliding_attacks(
                    PieceType::Bishop,
                    current_sq,
                    blockers,
                )
            }
            PieceType::Giraffe => A::get_giraffe_attacks(current_sq),
            _ => B::empty(),
        }
    }

    fn check_moves(&self, attacked_color: Color) -> Checks<S, B> {
        let mut king =
            self.type_bb(&PieceType::King) & &self.player_bb(attacked_color);
        if king.is_empty() {
            return Checks::new(None, false, None);
        }
        let occupied_bb = self.occupied_bb() | &self.player_bb(Color::NoColor);
        let king = king.pop().unwrap();
        let mut double_check = false;
        let mut check = None;
        let pt_iter = PieceType::iter();
        for pt in pt_iter {
            //
            match pt {
                PieceType::King => continue,
                _ => {
                    if !self.variant().can_buy(&pt) {
                        continue;
                    }
                    let moves = self.get_moves(
                        &king,
                        &Piece {
                            piece_type: pt,
                            color: attacked_color,
                        },
                        occupied_bb,
                    );
                    let them = self.type_bb(&pt)
                        & &self.player_bb(attacked_color.flip());
                    let mut attackers = (them & &moves) & &them;
                    let len = attackers.len();
                    match &len.cmp(&1) {
                        Ordering::Equal => {
                            if check.is_some() {
                                double_check = true;
                                return Checks::new(None, double_check, None);
                            } else if pt.is_non_sliding_piece() {
                                check = Some(moves);
                                double_check = false;
                            } else if let Some(attacker) = attackers.pop() {
                                let between = A::between(attacker, king);
                                let between = between | &attacker;
                                if between.len() == 0 {
                                    check = Some(moves);
                                } else {
                                    check = Some(between | &attacker);
                                }
                            }
                        }
                        Ordering::Greater => {
                            double_check = true;
                            return Checks::new(None, double_check, None);
                        }
                        _ => {}
                    }
                }
            }
        }
        Checks::new(check, double_check, None)
    }

    fn pins(&self, color: &Color) -> HashMap<S, B> {
        let mut pins = HashMap::new();
        if color == &Color::NoColor {
            return pins;
        }
        let ksq = self.find_king(color);
        if ksq.is_none() {
            return pins;
        }
        let ksq = ksq.unwrap();
        let plinths = self.player_bb(Color::NoColor);
        for pt in [
            PieceType::Rook,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::Chancellor,
            PieceType::ArchBishop,
        ] {
            if !self.variant().can_buy(&pt) {
                continue;
            }
            let attacks = A::get_sliding_attacks(pt, &ksq, plinths);
            // enemies
            let enemy_bb =
                (self.type_bb(&pt) & &self.player_bb(color.flip())) & &attacks;
            for enemy_sq in enemy_bb {
                // this piece is pinned
                let mut pinned = (A::between(ksq, enemy_sq)
                    & &self.occupied_bb())
                    & &!self.player_bb(Color::NoColor);
                let my_piece = pinned & &self.player_bb(*color);
                if pinned.len() == 1 && my_piece.is_any() {
                    let unpin =
                        (A::between(enemy_sq, ksq) & &!pinned) | &enemy_bb;
                    let my_square = pinned.pop_reverse();
                    pins.insert(my_square.unwrap(), unpin);
                }
            }
        }
        pins
    }

    fn my_moves(&self, square: &S, blockers: B) -> B {
        let piece = self.piece_at(*square);
        match piece {
            Some(i) => self.get_moves(square, i, blockers),
            None => B::empty(),
        }
    }

    fn fix_pin(
        &self,
        sq: &S,
        pins: &HashMap<S, B>,
        checks: Checks<S, B>,
        my_moves: B,
    ) -> B {
        match self.piece_at(*sq) {
            Some(piece) => {
                if let Some(pin) = pins.get(sq) {
                    if let Some(check) = checks.check {
                        (check & pin) & &my_moves
                    } else {
                        *pin & &my_moves
                    }
                } else {
                    let mut my_moves = my_moves;
                    if piece.piece_type == PieceType::King {
                        if let Some(enemy_moves) = checks.enemy_moves {
                            my_moves &= &!enemy_moves;
                            return my_moves;
                        }
                    } else if checks.double_check {
                        return B::empty();
                    }
                    if let Some(check) = checks.check {
                        my_moves &= &check;
                    }
                    my_moves
                }
            }
            None => B::empty(),
        }
    }
}
