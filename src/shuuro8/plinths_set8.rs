use crate::{
    attacks::Attacks, bitboard::BitBoard, plinths_set::PlinthGen, Square,
};

use super::{attacks8::Attacks8, bitboard8::BB8, square8::Square8};
use std::{
    marker::PhantomData,
    ops::{BitAnd, BitOr, Not},
};

pub struct PlinthGen8<S, B>
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

impl PlinthGen<Square8, BB8<Square8>> for PlinthGen8<Square8, BB8<Square8>> {
    fn king_moves(&self, sq: Square8) -> BB8<Square8> {
        Attacks8::get_non_sliding_attacks(
            crate::PieceType::King,
            &sq,
            crate::Color::White,
            BB8::empty(),
        )
    }

    fn y(&self) -> u8 {
        8
    }
}

impl PlinthGen8<Square8, BB8<Square8>> {
    pub fn start(&self) -> BB8<Square8> {
        let sections = [(2, 4, 0, 8, 2), (5, 7, 0, 8, 2)];
        self.generate_plinths(&sections)
    }
}

impl Default for PlinthGen8<Square8, BB8<Square8>> {
    fn default() -> Self {
        PlinthGen8 {
            _a: PhantomData,
            _s: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{attacks::Attacks, shuuro8::attacks8::Attacks8};

    use super::PlinthGen8;

    #[test]
    fn generate_all_plinths() {
        Attacks8::init();
        let pl = PlinthGen8::default();
        let b = pl.start();
        assert_eq!(b.count(), 4);
    }
}
