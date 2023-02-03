use std::{
    collections::HashMap,
    hash::Hash,
    ops::{BitAnd, BitOr, Not},
};

use crate::{
    attacks::Attacks, bitboard::BitBoard, Color, MoveRecord, Piece, PieceType, Square, Variant,
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
    fn new() -> Self;
    fn piece_at(&self, sq: S) -> &Option<Piece>;
    fn player_bb(&self, c: Color) -> B;
    fn occupied_bb(&self) -> B;
    fn hand(&self, p: Piece) -> u8;
    fn side_to_move(&self) -> Color;
    fn ply(&self) -> u16;
    fn move_history(&self) -> &[MoveRecord<S>];
    fn outcome(&self);
    fn variant(&self) -> Variant;
    fn update_variant(&mut self);
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
    fn legal_moves(&self, square: &S) -> HashMap<S, B> {
        let mut map = HashMap::new();
        let my_moves = self.non_legal_moves(&square);
        let pinned_moves = self.pinned_moves(&square);
        let check_moves = self.check_moves(&square);
        if check_moves.len() > 0 {
            let piece = self.piece_at(*square).unwrap();
            let king = self.find_king(&piece.color).unwrap();
            if king == *square {
                let enemy_moves = self.enemy_moves(&square);
                map.insert(king, &my_moves & &!&enemy_moves);
                return map;
            } else {
                let moves = self.fix_pin(square, &pinned_moves, check_moves, my_moves);
                map.insert(*square, moves);
                return map;
            }
        }

        return self.fix_pin(square, &pinned_moves, check_moves, my_moves);
    }
    fn non_legal_moves(&self, square: &S) -> B {
        let piece = self.piece_at(*square);
        match piece {
            Some(i) => self.move_candidates(square, *i, MoveType::Plinth),
            None => B::empty(),
        }
    }
    fn pinned_moves(&self, square: &S) -> Pin<S, B> {
        let piece = self.piece_at(*square);
        match piece {
            Some(i) => {
                let ksq = self.find_king(&i.color);
                if ksq.is_none() {
                    return Pin::default();
                }
                let ksq = ksq.unwrap();
                let mut pin: Pin<S, B> = Pin::default();
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
                    if self.variant().can_buy(s) {
                        continue;
                    }
                    let piece_attacks = A::get_sliding_attacks(*s, &ksq, plinths);
                    // this is enemy
                    let enemy_bb =
                        &(&self.type_bb(s) & &self.player_bb(i.color.flip())) & &piece_attacks;
                    for psq in enemy_bb {
                        // this piece is pinned
                        let mut pinned = &(&A::between(ksq, psq) & &self.occupied_bb())
                            & &!&self.player_bb(Color::NoColor);
                        // this is unpin
                        let my_piece = &pinned & &self.player_bb(i.color);
                        if pinned.count() == 1 && my_piece.is_any() {
                            let fix = &(&A::between(psq, ksq) & &!&pinned) | &enemy_bb;
                            let my_square = pinned.pop_reverse();
                            if &my_square.unwrap() == square {
                                pin = Pin::new(my_square.unwrap(), fix);
                                return pin;
                            } else {
                                return Pin::default();
                            }
                        }
                    }
                }
                pin
            }
            None => Pin::default(),
        }
    }
    fn pinned_bb(&self, c: Color) -> B {
        let mut bb = B::empty();
        for sq in self.player_bb(c) {
            let pinned = self.pinned_moves(&sq);
            if let Some(p) = pinned.square {
                bb |= p;
            }
        }
        bb
    }

    fn check_moves(&self, square: &S) -> Vec<B> {
        let piece = self.piece_at(*square);
        match piece {
            Some(i) => {
                let ksq = self.find_king(&i.color);
                if ksq.is_none() {
                    return vec![];
                }
                let ksq = ksq.unwrap();
                let mut all = vec![];

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
                        color: i.color,
                    };
                    let move_candidates = self.move_candidates(&ksq, p, MoveType::Plinth);
                    // this is enemy
                    let bb =
                        &(&self.type_bb(s) & &self.player_bb(i.color.flip())) & &move_candidates;
                    for psq in bb {
                        let fix = A::between(ksq, psq);
                        all.push(fix | &bb);
                    }
                }
                all
            }
            None => vec![], //
        }
    }

    fn in_check(&self, c: Color) -> bool {
        let king = &self.find_king(&c);
        if let Some(k) = king {
            let check_moves = self.check_moves(k);
            return !check_moves.is_empty();
        }
        false
    }

    fn is_checkmate(&self, c: &Color) -> bool {
        let king = self.find_king(c);
        match king {
            Some(k) => {
                if !self.in_check(*c) {
                    return false;
                }
                let king_moves = self.legal_moves(&k);
                if !king_moves.is_empty() {
                    let mut final_moves = B::empty();
                    for sq in self.player_bb(*c) {
                        for m in &self.legal_moves(&sq) {
                            final_moves |= m.1;
                        }
                    }
                    if final_moves.is_any() {
                        return false;
                    }
                    return true;
                }
                false
            }
            None => false,
        }
    }

    fn fix_pin(&self, sq: &S, pin: &Pin<S, B>, checks: Vec<B>, my_moves: B) -> B {
        let piece = self.piece_at(*sq).unwrap();
        if let Some(_square) = pin.square {
            if checks.len() == 1 {
                let checks = checks.get(0).unwrap();
                return &(checks & &pin.fix()) & &my_moves;
            } else if checks.len() > 1 {
                return B::empty();
            }
            &pin.fix() & &my_moves
        } else {
            let mut my_moves = my_moves;
            let enemy_moves = self.enemy_moves(&self.find_king(&piece.color).unwrap());
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
    fn enemy_moves(&self, king: &S) -> B {
        let piece = self.piece_at(*king);
        match piece {
            Some(i) => self.color_moves(&i.color.flip()),
            None => B::empty(),
        }
    }
    fn type_bb(&self, pt: &PieceType) -> B;
    fn find_king(&self, c: &Color) -> Option<S> {
        let mut bb = &self.type_bb(&PieceType::King) & &self.player_bb(*c);
        if bb.is_any() {
            bb.pop_reverse()
        } else {
            None
        }
    }
    fn log_position(&mut self);
    fn generate_sfen(&self);
    fn make_move(&mut self);
    fn halfmoves(&self) -> B;
    fn us(&self) -> B;
    fn them(&self) -> B;
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
