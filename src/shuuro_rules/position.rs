use std::{
    collections::HashMap,
    hash::Hash,
    ops::{BitAnd, BitOr, Not},
};

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
    for<'b> &'b B: BitOr<&'b B, Output = B>,
    for<'b> &'b B: BitAnd<&'b B, Output = B>,
    for<'a> &'a B: Not<Output = B>,
    for<'a> &'a B: BitOr<&'a S, Output = B>,
    B: Not,
{
    /// Creates a new instance of `Position` with an empty board.
    fn new() -> Self;
    /// Returns a piece at the given square.
    fn piece_at(&self, sq: S) -> &Option<Piece>;
    /// Returns a bitboard containing pieces of the given player.
    fn player_bb(&self, c: Color) -> B;
    /// Returns occupied bitboard, all pieces except plinths.
    fn occupied_bb(&self) -> B;
    /// Returns `BitBoard` of all `PieceType`.
    fn type_bb(&self, pt: &PieceType) -> B;
    /// Returns the side to make a move next.
    fn side_to_move(&self) -> Color;
    /// Returns a history of all moves made since the beginning of the game.
    fn ply(&self) -> u16;
    /// Returns current status of the game.
    fn outcome(&self) -> Outcome;
    /// Returns current variant.
    fn variant(&self) -> Variant;
    /// Changing to other variant.
    fn update_variant(&mut self, variant: Variant);
    /// Make move from `Move`. It can be of three types.
    /// It's useful for all three stages of the game.
    fn make_move(&mut self, m: Move<S>) -> Result<Outcome, MoveError>;
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
    fn log_position(&mut self);
    /// Set `Position` from `&str`.
    fn set_sfen(&mut self, sfe_str: &str) -> Result<Outcome, SfenError>;
    /// Set sfen history.
    fn set_sfen_history(&mut self, history: Vec<(String, u16)>);
    /// Set history of previous moves.
    fn set_move_history(&mut self, history: Vec<MoveRecord<S>>);
    ///  Returns history of all moves in `MoveRecord` format.
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
    fn to_sfen(&self) -> String;
    fn parse_sfen_hand(&mut self, s: &str) -> Result<(), SfenError>;
    fn parse_sfen_ply(&mut self, s: &str) -> Result<(), SfenError>;
    fn parse_sfen_board(&mut self, s: &str) -> Result<(), SfenError>;
    fn generate_sfen(&self);
    // PLACEMENT PART
    fn generate_plinths(&mut self);
    fn set_hand(&mut self, s: &str);
    fn get_hand(&self, c: Color) -> String;
    /// Returns the number of the given piece in hand.
    fn hand(&self, p: Piece) -> u8;
    fn king_squares(&self, c: &Color) -> B;
    fn empty_squares(&self, p: Piece) -> B;
    fn is_king_placed(&self, c: Color) -> bool;
    fn checks(&self, attacked_color: &Color) -> B;
    fn can_pawn_move(&self, p: Piece) -> bool;
    fn is_hand_empty(&self, c: Color, excluded: PieceType) -> bool;
    fn place(&mut self, p: Piece, sq: S) -> Option<String>;
    fn update_bb(&mut self, p: Piece, sq: S);
    fn halfmoves(&self) -> B;
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
    fn pinned_moves(&self, color: Color) -> HashMap<S, Pin<S, B>> {
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
                    let pin = Pin::new(my_square.unwrap(), fix);
                    pins.insert(my_square.unwrap(), pin);
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
                bb |= sq;
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
            if self.variant().can_buy(s) {
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
                all.push(fix | &bb);
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
    fn fix_pin(&self, sq: &S, pins: &HashMap<S, Pin<S, B>>, checks: &Vec<B>, my_moves: B) -> B {
        let piece = self.piece_at(*sq).unwrap();
        if let Some(pin) = pins.get(sq) {
            if checks.len() == 1 {
                let checks = checks.get(0).unwrap();
                return &(checks & &pin.fix()) & &my_moves;
            } else if checks.len() > 1 {
                return B::empty();
            }
            &pin.fix() & &my_moves
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

pub struct Pin<S, B>
where
    S: Square,
    B: BitBoard<S>,
{
    pub square: Option<S>,
    pub bb: B,
}

impl<S, B> Pin<S, B>
where
    S: Square,
    B: BitBoard<S>,
{
    pub fn new(square: S, bb: B) -> Self {
        Pin {
            square: Some(square),
            bb,
        }
    }

    pub fn new_square(&mut self, square: Option<S>) {
        self.square = square;
    }

    pub fn new_fix(&mut self, bb: B) {
        self.bb = bb;
    }

    pub fn square(&self) -> Option<S> {
        self.square
    }
    pub fn fix(&self) -> B {
        self.bb
    }
}

impl<S: Square, B: BitBoard<S>> Default for Pin<S, B> {
    fn default() -> Self {
        Pin {
            square: None,
            bb: B::empty(),
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
                        without_plinth |= self.get_pawn_square(sq, &p.color);
                        without_plinth &= &!&position.player_bb(p.color.flip());
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
