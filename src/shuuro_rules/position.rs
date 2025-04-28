use std::{
    clone::Clone,
    collections::{HashMap, VecDeque},
    hash::Hash,
};

use crate::{
    attacks::Attacks, bitboard::BitBoard, Color, Hand, Move, MoveData,
    MoveError, Piece, PieceType, SfenError, Square, Variant,
};

use super::piece_type::PieceTypeIter;

/// Outcome stores information about outcome after move.
#[derive(Debug, Clone, PartialEq)]
pub enum Outcome {
    Check { color: Color },
    Checkmate { color: Color },
    DrawByAgreement,
    DrawByRepetition,
    DrawByMaterial,
    Resign { color: Color },
    LostOnTime { color: Color },
    FirstMoveError { color: Color },
    Stalemate,
    MoveNotOk,
    MoveOk,
}

impl ToString for Outcome {
    fn to_string(&self) -> String {
        match &self {
            Outcome::Check { color } => {
                format!("Check by {}", color.to_string())
            }
            Outcome::Checkmate { color } => {
                format!("Checkmate. {} won.", color.to_string())
            }
            Outcome::DrawByAgreement => "Draw by agreement".to_string(),
            Outcome::DrawByRepetition => "Draw by repetition".to_string(),
            Outcome::DrawByMaterial => "Draw by material".to_string(),
            Outcome::Stalemate => "Stalemate".to_string(),
            Outcome::MoveOk => "Live".to_string(),
            Outcome::MoveNotOk => "Illegal move".to_string(),
            Outcome::Resign { color } => {
                format!("Resignation by {}", color.to_string())
            }
            Outcome::LostOnTime { color } => {
                format!("{} lost on time", color.to_string())
            }
            Outcome::FirstMoveError { color } => {
                format!("{} lost on first move.", color.to_string())
            }
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<i32> for Outcome {
    fn into(self) -> i32 {
        match self {
            Outcome::MoveNotOk => -2,
            Outcome::MoveOk => -1,
            Outcome::Check { color: _ } => -1,
            Outcome::Checkmate { color: _ } => 1,
            Outcome::Stalemate => 3,
            Outcome::DrawByRepetition => 4,
            Outcome::DrawByAgreement => 5,
            Outcome::DrawByMaterial => 6,
            Outcome::Resign { color: _ } => 7,
            Outcome::LostOnTime { color: _ } => 8,
            Outcome::FirstMoveError { color: _ } => 9,
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
    fn insert_move(&mut self, sfen: Move<S>);
    /// Insert sfen move
    fn insert_sfen(&mut self, m: String);
    /// Clear sfen_history
    fn clear_sfen_history(&mut self);
    /// Set sfen history.
    fn set_sfen_history(&mut self, history: Vec<String>) {
        for i in history {
            let m: Result<Move<S>, ()> = Move::try_from(i);
            if let Ok(_) = m {
                // self.insert_sfen(m);
            }
        }
    }
    /// Set history of previous moves.
    fn set_move_history(&mut self, history: Vec<Move<S>>);
    /// Returns history of all moves in `Move2` format.
    fn move_history(&self) -> &[Move<S>];
    /// Returns history of all moves in `Vec` format.
    fn get_sfen_history(&self) -> &SfenHistory<B>;
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
    fn find_king(&self, c: Color) -> Option<S> {
        let mut bb = self.type_bb(&PieceType::King) & &self.player_bb(c);
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
            return format!("{} {}", sfen_history.first().2, ply);
        }
        format!(
            "{} {}",
            &sfen_history.first().2,
            ply - move_history.len() as u16
        )
    }

    /// Generate sfen.
    fn generate_sfen(&self) -> String {
        let dimension = self.dimensions();
        let mut fen = String::new();
        for rank in (0..dimension).rev() {
            let mut row_item = String::from("");
            let mut space = 0;
            for file in 0..dimension {
                let sq = S::new(file, rank).unwrap();
                match *self.piece_at(sq) {
                    Some(piece) => {
                        row_item = self.add_space(space, row_item);
                        space = 0;
                        if piece.piece_type == PieceType::Plinth {
                            row_item.push_str(&piece.to_string());
                            row_item.push('.');
                            continue;
                        }
                        if piece.piece_type.is_knight_piece() {
                            if (self.player_bb(Color::NoColor) & &sq).is_any() {
                                row_item.push('_');
                                space = 0;
                            }
                        } else {
                            space = 0;
                        }
                        row_item.push_str(&piece.to_string());
                    }
                    None => {
                        if (self.player_bb(Color::NoColor) & &sq).is_any() {
                            row_item = self.add_space(space, row_item);
                            space = 0;
                            row_item.push_str("_.");
                        } else {
                            space += 1;
                        }
                    }
                }
            }
            row_item = self.add_space(space, row_item);
            fen.push_str(&row_item);
            if rank < dimension && rank > 0 {
                fen.push('/');
            }
        }

        let black = self.get_hand(Color::Black, false);
        let white = self.get_hand(Color::White, false);
        let mut hand = String::new();
        hand.push_str(&black);
        hand.push_str(&white);
        if hand.is_empty() {
            hand = "-".to_string();
        }
        format!(
            "{} {} {} {}",
            fen,
            self.side_to_move().to_string(),
            hand,
            self.ply()
        )
    }

    fn add_space(&self, n: u8, mut s: String) -> String {
        let space = {
            if n == 0 {
                ""
            } else {
                &n.to_string()
            }
        };
        s.push_str(space);
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

    fn parse_space(
        &mut self,
        number: &mut String,
        mut current_file: u8,
        rank: u8,
    ) -> Result<u8, SfenError> {
        if number == "" {
            return Ok(current_file);
        }
        let n = number.parse::<u8>()?;
        if n != 0 {
            for _ in 0..n {
                if current_file >= self.dimensions() {
                    return Err(SfenError::UnknownFile);
                }
                let sq = S::new(current_file, rank as u8)
                    .ok_or(SfenError::IllegalBoardState)?;

                self.set_piece(sq, None);
                current_file += 1;
            }
        }
        number.clear();
        number.push('0');
        Ok(current_file)
    }

    fn parse_sfen_board(&mut self, fen: &str) -> Result<(), SfenError> {
        let ranks = fen.split('/');
        let dimension = self.dimensions();
        let mut rank = dimension;
        self.empty_all_bb();
        for file in ranks {
            rank -= 1;
            if rank >= dimension {
                return Err(SfenError::IllegalBoardState);
            }

            // '1rkb2/p5/4_.1/1n4/1KPP2/2B1R1 b - 15',

            let mut current_file = 0;
            let mut current_number = String::new();
            let mut is_plinth = false;
            for ch in file.chars() {
                match ch {
                    number if number.is_numeric() => {
                        current_number.push(number);
                        let n = current_number.parse::<u8>()?;

                        if n > dimension {
                            return Err(SfenError::UnknownFile);
                        }
                    }

                    _ if ch == '.' => {
                        if !is_plinth {
                            return Err(SfenError::IllegalBoardState);
                        }
                        is_plinth = false;
                        current_file += 1;
                    }

                    piece => {
                        let piece = Piece::from_sfen(piece)
                            .ok_or(SfenError::IllegalPieceType)?;
                        if current_file > dimension {
                            return Err(SfenError::UnknownFile);
                        }
                        current_file = self.parse_space(
                            &mut current_number,
                            current_file,
                            rank,
                        )?;
                        let sq = S::new(current_file, rank as u8)
                            .ok_or(SfenError::IllegalBoardState)?;
                        match piece.piece_type {
                            PieceType::Plinth => {
                                self.update_player(piece, &sq);
                                is_plinth = true;
                                self.set_piece(sq, None);
                            }
                            _ => {
                                self.update_player(piece, &sq);
                                current_file += 1;
                            }
                        }
                    }
                }
            }

            current_file =
                self.parse_space(&mut current_number, current_file, rank)?;
            if current_file > self.dimensions() {
                return Err(SfenError::UnknownFile);
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
    fn save_position(&mut self, move_data: Option<MoveData>) {
        let Some(move_data) = move_data else { return };

        let move_history = self.move_history();
        let Some(last) = move_history.last() else {
            return;
        };
        let Move::Normal { from, to, .. } = last else {
            return;
        };

        let mut sfen = self.generate_sfen();
        sfen.push(' ');
        sfen.push_str(&last.format(from, to, move_data));
        sfen.push(' ');
        sfen.push_str(&format!("{}_{}", from, to));
        self.insert_sfen(sfen);
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

pub enum PlacementError {
    NoKing,
    KingNotPlaced,
    FairyPieceError,
    EarlyPawnPlacement,
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

    /// Returns BitBoard with file. Panics if file is bigger than expected.
    fn rank_bb(&self, file: usize) -> B;

    /// Check if king is placed.
    fn is_king_placed(&self, c: Color) -> bool {
        let king = self.player_bb(c) & &self.type_bb(&PieceType::King);
        king.len() == 1
    }

    /// Returns BitBoard with king placement.
    fn king_files(&self, c: Color) -> B;

    /// Returns BitBoard with all empty squares.
    fn king_squares(&self, c: Color) -> B {
        let files = self.king_files(c);
        let plinths = self.player_bb(Color::NoColor);
        files & &!plinths
    }

    fn can_pawn_move(&self, p: Piece) -> bool {
        self.is_hand_empty(p.color, PieceType::Pawn)
    }

    fn new_placement_squares(&mut self, placement: HashMap<usize, B>);

    fn get_placement_squares(&self) -> &HashMap<usize, B>;

    /// All squares for current `Color`.
    fn placement_squares(&self, color: Color) -> HashMap<usize, B> {
        let mut placement = HashMap::new();
        let pieces = PieceTypeIter::default();

        for pt in pieces {
            let bb = self.empty_squares(Piece {
                piece_type: pt,
                color,
            });
            if let Ok(bb) = bb {
                placement.insert(pt as usize, bb);
            }
        }
        placement
    }

    fn empty_squares(&self, p: Piece) -> Result<B, PlacementError> {
        let color = p.color;
        if color == Color::NoColor {
            return Err(PlacementError::NoKing);
        }
        let delta = {
            if color == Color::White {
                1
            } else {
                -1
            }
        };
        let mut rank = {
            if color == Color::White {
                -1
            } else {
                self.dimensions() as i8
            }
        };
        let me = self.player_bb(color);
        let plinths = self.player_bb(Color::NoColor);

        let checks = self.placement_checks(p.color);
        if checks.is_any() {
            return Ok(checks);
        } else if !self.is_king_placed(p.color)
            && p.piece_type != PieceType::King
        {
            return Err(PlacementError::KingNotPlaced);
        }

        loop {
            rank += delta;
            let mut bb = self.rank_bb(rank as usize);
            bb &= &!me;
            if bb.is_empty() {
                continue;
            }

            match p.piece_type {
                PieceType::Knight
                | PieceType::Chancellor
                | PieceType::ArchBishop
                | PieceType::Giraffe => {
                    if !self.variant().can_select(&p.piece_type) {
                        return Err(PlacementError::FairyPieceError);
                    }

                    return Ok(bb);
                }
                PieceType::King => {
                    return Ok(self.king_squares(p.color));
                }
                PieceType::Pawn => {
                    bb &= &!plinths;
                    if bb.is_empty() {
                        continue;
                    } else if self.can_pawn_move(p) {
                        if rank == 0 || rank == self.dimensions() as i8 - 1 {
                            continue;
                        }
                        return Ok(bb);
                    } else {
                        return Err(PlacementError::EarlyPawnPlacement);
                    }
                }
                _ => {
                    bb &= &!plinths;
                    if bb.is_empty() {
                        continue;
                    }
                    return Ok(bb);
                }
            }
        }
    }

    fn placement_checks(&self, me: Color) -> B {
        let king = self.type_bb(&PieceType::King) & &self.player_bb(me);
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
                if !self.variant().can_select(&p) {
                    continue;
                }
                if pt == PieceType::Rook && !p.is_rook_type() {
                    continue;
                }
                if pt == PieceType::Bishop && !p.is_bishop_type() {
                    continue;
                }

                let mut them = self.type_bb(&p) & &self.player_bb(me.flip());

                if (them & &king_attacks).is_any() {
                    let mut between = A::between(king_sq, them.pop().unwrap());
                    match me {
                        Color::White => {
                            let sq = between.pop().unwrap();
                            return B::from_square(&sq);
                        }
                        Color::Black => {
                            let sq = between.pop_reverse().unwrap();
                            return B::from_square(&sq);
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
        if p.color != self.side_to_move()
            || self.hand(p) == 0
            || !(**&self
                .get_placement_squares()
                .get(&(p.piece_type as usize))
                .unwrap_or(&B::empty())
                & &sq)
                .is_any()
        {
            return None;
        }

        self.update_bb(p, sq);
        self.decrement_hand(p);
        let move_record = Move::Put { to: sq, piece: p };
        let sfen = self.generate_sfen().split(' ').next().unwrap().to_string();
        let hand = {
            let s = self.get_hand(Color::White, false)
                + &self.get_hand(Color::Black, false);
            if s.is_empty() {
                String::from("-")
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
            "{} {} {} {} {}",
            &sfen,
            self.side_to_move().to_string(),
            hand,
            ply,
            &move_record.to_string(),
        );
        self.new_placement_squares(self.placement_squares(self.side_to_move()));
        self.insert_sfen(record.to_string());
        return Some(record);
    }

    fn empty_placement_board() -> String;
}

pub trait Play<S, B, A>
where
    S: Square + Hash,
    B: BitBoard<S>,
    A: Attacks<S, B>,
    Self: Sized
        + Clone
        + Board<S, B, A>
        + Sfen<S, B, A>
        + Rules<S, B, A>
        + Placement<S, B, A>,
{
    // Play part.

    /// Create move from `&str`.
    fn play(&mut self, game_move: &str) -> Result<&Outcome, SfenError> {
        let game_move = Move::<S>::from_sfen(game_move)
            .ok_or(SfenError::IllegalPieceFound)?;
        let outcome = self.make_move(game_move);
        match outcome {
            Ok(i) => {
                self.update_outcome(i);
            }
            Err(error) => match error {
                MoveError::RepetitionDraw => {
                    self.update_outcome(Outcome::DrawByRepetition)
                }
                MoveError::Draw => {
                    self.update_outcome(Outcome::DrawByAgreement)
                }
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
        match sfen_history.repetition() {
            Some(_) => Err(MoveError::RepetitionDraw),
            None => Ok(()),
        }
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
                        }
                    }
                    continue;
                } else if c == Color::Black {
                    if let Some(sq) = file_with_plinths.pop() {
                        if sq.index() >= pawn.index() {
                            bb |= &pawn;
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

    fn check_moves(&self, me: Color) -> B {
        let king = self.player_bb(me) & &self.type_bb(&PieceType::King);
        if king.is_empty() {
            return B::empty();
        }

        let king_sq = &king.clone().pop().unwrap();

        let pieces = PieceTypeIter::default();
        let with_plinths = self.occupied_bb() | &self.player_bb(Color::NoColor);
        let mut checkers = B::empty();
        for pt in pieces {
            if !self.variant().can_select(&pt) {
                continue;
            }

            let moves = self.get_moves(
                king_sq,
                &Piece {
                    piece_type: pt,
                    color: me,
                },
                with_plinths,
            );
            let them = self.type_bb(&pt) & &self.player_bb(me.flip());
            let them = moves & &them;
            checkers |= &them;
        }
        checkers
    }

    fn new_legal_moves(&mut self, lm: HashMap<S, B>);
    fn get_legal_moves(&self) -> &HashMap<S, B>;

    /// Returns all legal moves where piece can be moved.
    fn legal_moves(&self, my_color: Color) -> HashMap<S, B> {
        let mut map = HashMap::new();
        let checkers = self.check_moves(my_color);
        let enemy_moves = self.enemy_moves(my_color);
        let king = self.find_king(my_color).expect("no king");
        if checkers.len() > 1 {
            let king_moves =
                self.non_legal_moves(&king).expect("piece not found");
            map.insert(king, king_moves & &!enemy_moves);
            return map;
        }
        let pinned_moves = self.pins(my_color);
        for sq in self.player_bb(my_color) {
            let my_moves = self.non_legal_moves(&sq).expect("piece not found");
            if king == sq {
                map.insert(king, my_moves & &!enemy_moves);
            } else {
                let _ = self
                    .unpin(&sq, &pinned_moves, my_moves, checkers)
                    .is_some_and(|b| {
                        map.insert(sq, b);
                        true
                    });
            }
        }
        map
    }

    /// Returns `BitBoard` of all moves by opponent.
    fn enemy_moves(&self, me: Color) -> B {
        if me == Color::NoColor {
            return B::empty();
        }
        let mut all = B::empty();
        let them = me.flip();
        let blockers = self.occupied_bb() | &self.player_bb(Color::NoColor);
        let king = self.find_king(me).unwrap();
        let blockers = blockers ^ &B::from_square(&king);
        for sq in self.player_bb(them).into_iter() {
            let piece = self.piece_at(sq);
            if let Some(piece) = piece {
                let moves = self.get_moves(&sq, piece, blockers);
                all |= &moves;
            }
        }
        all &= &!self.player_bb(Color::NoColor);
        all
    }
    /// Returns all non-legal moves.
    fn non_legal_moves(&self, square: &S) -> Option<B> {
        let Some(piece) = self.piece_at(*square) else {
            return None;
        };
        Some(self.move_candidates(square, *piece))
    }

    /// Returns HashMap of all pinned pieces.
    fn pinned_moves(&self, color: Color) -> HashMap<S, B> {
        let mut pins = HashMap::new();
        if color == Color::NoColor {
            return pins;
        }
        let ksq = self.find_king(color);
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
            if !self.variant().can_select(s) {
                continue;
            }
            let as_king_attacks = A::get_sliding_attacks(*s, &ksq, plinths);
            // this is enemy
            let enemy_bb = (self.type_bb(s) & &self.player_bb(color.flip()))
                & &as_king_attacks;
            for enemy in enemy_bb {
                // this piece is pinned
                let mut pinned = (A::between(ksq, enemy) & &self.occupied_bb())
                    & &!self.player_bb(Color::NoColor);
                // this is unpin
                let my_piece = pinned & &self.player_bb(color);
                if pinned.len() == 1 && my_piece.is_any() {
                    let unpin = (A::between(enemy, ksq) & &!pinned) | &enemy_bb;
                    let my_square = pinned.pop_reverse();
                    pins.insert(my_square.unwrap(), unpin);
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
        self.save_position(None);
        let king = self.find_king(self.side_to_move());
        if self.in_check(self.side_to_move().flip()) {
            let checkmate = Outcome::Checkmate {
                color: self.side_to_move(),
            };
            self.update_outcome(checkmate.clone());
            return Ok(checkmate);
        } else if self.is_hand_empty(self.side_to_move(), PieceType::Plinth)
            == false
        {
            let placement = self.placement_squares(self.side_to_move());
            self.new_placement_squares(placement);
            return Ok(Outcome::MoveOk);
        } else if king.is_none()
            && (self.occupied_bb() & &!self.player_bb(Color::NoColor)).len()
                == 0
        {
            return Ok(Outcome::MoveOk);
        } else if king.is_none() {
            return Err(SfenError::IllegalBoardState);
        }
        let lm = self.legal_moves(self.side_to_move());
        self.new_legal_moves(lm);
        Ok(Outcome::MoveOk)
    }

    fn in_check(&self, c: Color) -> bool {
        let king = &self.find_king(c);
        if let Some(k) = king {
            let check_moves = self.enemy_moves(c);
            return (check_moves & k).is_any();
        }
        false
    }

    /// Checks if given color is in checkmate.
    fn is_checkmate(&self, c: Color) -> bool {
        let king = self.find_king(c);
        match king {
            Some(k) => {
                if !self.in_check(c) {
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
    fn is_stalemate(&self, color: Color) -> Result<(), MoveError> {
        let moves = self.legal_moves(color);
        for m in moves {
            if m.1.len() > 0 {
                return Ok(());
            }
        }
        Err(MoveError::DrawByStalemate)
    }

    /// Returns a `BitBoard` where the given piece at the given square can move.
    fn move_candidates(&self, current_sq: &S, piece: Piece) -> B {
        let blockers = self.occupied_bb() | &self.player_bb(Color::NoColor);

        let attacks = self.get_moves(current_sq, &piece, blockers);

        let me = piece.color;
        let them = me.flip();
        let without_me = attacks & &!self.player_bb(me);

        let jumpers = [
            PieceType::Knight,
            PieceType::ArchBishop,
            PieceType::Chancellor,
            PieceType::Giraffe,
        ];
        if jumpers.contains(&piece.piece_type) {
            return without_me;
        }
        let without_plinth = without_me & &!self.player_bb(Color::NoColor);
        if piece.piece_type != PieceType::Pawn {
            return without_plinth;
        }
        self.pawn_pushes(current_sq, without_plinth, them, me, blockers)
    }

    fn pawn_pushes(
        &self,
        current_sq: &S,
        without_plinth: B,
        them: Color,
        me: Color,
        blockers: B,
    ) -> B {
        let pawn_captures = without_plinth & &self.player_bb(them);
        let pushes = A::get_pawn_moves(current_sq.index(), me);
        let mut double_pushes = pushes & &blockers;
        if double_pushes.len() == 2 {
            return pawn_captures;
        } else if double_pushes.len() == 0 {
            return pawn_captures | &pushes;
        }
        let to = double_pushes.pop().unwrap();
        let pushes = A::between(*current_sq, to);
        pushes | &pawn_captures
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

        let (from, to) =
            m.info().ok_or(MoveError::Inconsistent("No piece found"))?;
        let moved = self
            .piece_at(from)
            .ok_or(MoveError::Inconsistent("No piece found"))?;
        let captured = *self.piece_at(to);
        let outcome = Outcome::Checkmate { color: opponent };

        if moved.color != stm {
            return Err(MoveError::Inconsistent(
                "The piece is not from the side to move",
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

        let legal_moves = self.get_legal_moves().clone();

        let attacks = legal_moves.get(&from).ok_or(MoveError::Inconsistent(
            "The piece cannot move from there",
        ))?;

        if (*attacks & &to).is_empty() {
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
            if self.is_checkmate(stm) {
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
        let move_record = Move::Normal { from, to, placed };

        self.insert_move(move_record);

        self.save_position(Some(move_data));
        self.detect_repetition()?;
        self.detect_insufficient_material()?;

        if outcome == Outcome::MoveOk {
            self.is_stalemate(stm)?;
        }

        let lm = self.legal_moves(self.side_to_move());
        self.new_legal_moves(lm);

        Ok(outcome)
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

    fn pins(&self, color: Color) -> HashMap<S, B> {
        let mut pins = HashMap::new();
        if color == Color::NoColor {
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
            if !self.variant().can_select(&pt) {
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
                let my_piece = pinned & &self.player_bb(color);
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

    fn unpin(
        &self,
        sq: &S,
        pins: &HashMap<S, B>,
        my_moves: B,
        mut checks: B,
    ) -> Option<B> {
        let Some(piece) = self.piece_at(*sq) else {
            return None;
        };
        let king = self.find_king(piece.color).expect("no king");
        let mut moves = my_moves;
        let _ = pins.get(sq).is_some_and(|pin| {
            moves &= pin;
            true
        });

        if checks.len() == 1 {
            let attacker = checks.pop()?;
            let between = A::between(attacker, king);
            let between = between | &attacker;
            let moves = moves & &between;
            return Some(moves);
        }
        return Some(moves);
    }
}

#[derive(Default, Debug, Clone)]
pub struct SfenHistory<B: PartialEq + Clone> {
    moves: VecDeque<(B, B, String)>,
}

impl<B: PartialEq + Clone + std::fmt::Debug> SfenHistory<B> {
    pub fn add_move(&mut self, m: (B, B, String)) {
        self.moves.push_front(m);
        if self.moves.len() == 16 {
            self.moves.pop_back();
        }
    }

    pub fn repetition(&self) -> Option<()> {
        let mut count = 0;
        let last = self.moves.front()?;
        for i in &self.moves {
            if i.0 == last.0 && i.1 == last.1 {
                count += 1;
            }
            if count == 3 {
                return Some(());
            }
        }
        None
    }

    pub fn is_empty(&self) -> bool {
        self.moves.len() == 0
    }

    pub fn first(&self) -> (B, B, String) {
        return self.moves.front().unwrap().clone();
    }
}
