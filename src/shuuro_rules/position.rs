use std::{
    collections::HashMap,
    ops::{BitAnd, BitOr, Not},
};

use crate::{bitboard::BitBoard, Color, MoveRecord, Piece, PieceType, Square, Variant};

pub trait Position<S, B>
where
    S: Square,
    B: BitBoard<S>,
    Self: Sized,
    for<'b> &'b B: BitOr<&'b B, Output = B>,
    for<'b> &'b B: BitAnd<&'b B, Output = B>,
    for<'a> &'a B: Not<Output = B>,
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
    fn move_candidates(&self, sq: &S, move_type: MoveType<S>);
    fn legal_moves(&self, square: &S) -> HashMap<S, B>;
    fn non_legal_moves(&self, square: &S) -> B;
    fn pinned_moves(&self, square: &S);
    fn halfmoves(&self) -> B;
    fn us(&self) -> B;
    fn them(&self) -> B;
}

pub struct Pin<S, B>
where
    S: Square,
    B: BitBoard<S>,
    for<'b> &'b B: BitOr<&'b B, Output = B>,
    for<'a> &'a B: Not<Output = B>,
{
    pub square: Option<S>,
    pub bb: B,
}

impl<S, B> Pin<S, B>
where
    S: Square,
    B: BitBoard<S>,
    for<'b> &'b B: BitOr<&'b B, Output = B>,
    for<'a> &'a B: Not<Output = B>,
{
    pub fn new(square: S, bb: B) -> Self {
        Pin {
            square: Some(square),
            bb,
        }
    }

    pub fn square(&self) -> Option<S> {
        self.square
    }
    pub fn fix(&self) -> B {
        self.bb
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
    S: Square,
{
    pub fn blockers<B: BitBoard<S>, P: Position<S, B>>(&self, position: &P, c: &Color) -> B
    where
        for<'b> &'b B: BitOr<&'b B, Output = B>,
        for<'b> &'b B: BitAnd<&'b B, Output = B>,
        for<'a> &'a B: Not<Output = B>,
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

    pub fn moves<B: BitBoard<S>, P: Position<S, B>>(
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
                    let without_plinth =
                        &(without_main_color) & &!&position.player_bb(Color::NoColor);
                    if p.piece_type == PieceType::Pawn {
                        let up_sq = self.get_pawn_square(sq, &p.color);
                        let without_up = &without_main_color & &!&B::from_square(&up_sq);
                        let up_sq = &B::from_square(&up_sq) & &!&position.player_bb(p.color.flip());
                        let primary_bb = &without_up & &position.player_bb(p.color.flip());
                        let moves = &(&primary_bb
                            | &(&up_sq & &!&position.player_bb(Color::NoColor)))
                            & &!&position.player_bb(p.color);
                        let other_without_plinth = &position.player_bb(p.color.flip())
                            & &position.player_bb(Color::NoColor);
                        &moves & &!&other_without_plinth
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
