use std::{
    marker::PhantomData,
    ops::{BitAnd, BitOr, Not},
};

pub use crate::attacks::Attacks;
use crate::{bitboard::BitBoard, Color, Square};

use super::{bitboard12::B12, square12::Square12};

static mut NON_SLIDING_ATTACKS: [[[B12<Square12>; 144]; 6]; 2] =
    [[[B12::new([0, 0, 0, 0, 0, 0, 0, 0, 0]); 144]; 6]; 2];

static mut PAWN_MOVES: [[B12<Square12>; 144]; 2] =
    [[B12::new([0, 0, 0, 0, 0, 0, 0, 0, 0]); 144]; 2];

static mut RAYS: [[B12<Square12>; 144]; 8] = [[B12::new([0, 0, 0, 0, 0, 0, 0, 0, 0]); 144]; 8];

#[derive(Clone, Copy, Debug, Default)]
pub struct Attacks12<S, B>
where
    S: Square,
    B: BitBoard<S>,
    for<'b> &'b B: BitOr<&'b B, Output = B>,
    for<'a> &'a B: BitAnd<&'a B, Output = B>,
    for<'a> &'a B: Not<Output = B>,
    for<'a> &'a B: BitOr<&'a S, Output = B>,
    for<'a> &'a B: BitAnd<&'a S, Output = B>,
{
    _a: PhantomData<B>,
    _s: PhantomData<S>,
}

impl Attacks<Square12, B12<Square12>> for Attacks12<Square12, B12<Square12>> {
    fn init_pawn_moves() {
        for color in Color::iter() {
            // for src in Square12::
        }
    }

    fn init_pawn_attacks() {
        todo!()
    }

    fn init_knight_attacks() {
        todo!()
    }

    fn init_king_attacks() {
        todo!()
    }

    fn init_north_ray() {
        todo!()
    }

    fn init_south_ray() {
        todo!()
    }

    fn init_east_ray() {
        todo!()
    }

    fn init_west_ray() {
        todo!()
    }

    fn init_north_east_ray() {
        todo!()
    }

    fn init_north_west_ray() {
        todo!()
    }

    fn init_south_east_ray() {
        todo!()
    }

    fn init_south_west_ray() {
        todo!()
    }

    fn init_between() {
        todo!()
    }

    fn get_non_sliding_attacks(
        piece_type: crate::PieceType,
        square: &Square12,
        color: crate::Color,
    ) -> B12<Square12> {
        todo!()
    }

    fn get_sliding_attacks(
        piece_type: crate::PieceType,
        square: &Square12,
        blockers: B12<Square12>,
    ) -> B12<Square12> {
        todo!()
    }

    fn get_positive_ray_attacks(
        dir: crate::attacks::Ray,
        square: usize,
        blockers: B12<Square12>,
    ) -> B12<Square12> {
        todo!()
    }

    fn get_negative_ray_attacks(
        dir: crate::attacks::Ray,
        square: usize,
        blockers: B12<Square12>,
    ) -> B12<Square12> {
        todo!()
    }

    fn between(sq1: Square12, sq2: Square12) -> B12<Square12> {
        todo!()
    }
}
