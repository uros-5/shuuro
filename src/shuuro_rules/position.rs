use std::collections::HashMap;

use crate::{bitboard::BitBoard, Color, MoveRecord, Piece, Square, Variant};

pub trait Position<S, B>
where
    S: Square,
    B: BitBoard<S>,
    Self: Sized,
{
    fn new() -> Self;
    fn piece_at(&self, sq: S) -> &Option<Piece>;
    fn player_bb(&self, c: Color) -> B;
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
    pub fn blockers<B: BitBoard<S>, P: Position<S, B>>(&self, position: P, c: &Color) -> B {}
}
