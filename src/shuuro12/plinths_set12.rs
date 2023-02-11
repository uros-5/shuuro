use crate::{attacks::Attacks, bitboard::BitBoard, plinths_set::PlinthGen, Square};

use super::{attacks12::Attacks12, bitboard12::BB12, square12::Square12};
use std::{
    marker::PhantomData,
    ops::{BitAnd, BitOr, Not},
};

pub struct PlinthGen12<S, B>
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

impl PlinthGen<Square12, BB12<Square12>> for PlinthGen12<Square12, BB12<Square12>> {
    fn king_moves(&self, sq: Square12) -> BB12<Square12> {
        Attacks12::get_non_sliding_attacks(crate::PieceType::King, &sq, crate::Color::White)
    }

    fn plinths_count(&self, _count: usize) -> bool {
        false
        // matches!((8).cmp(&count), std::cmp::Ordering::Equal)
    }
}

impl PlinthGen12<Square12, BB12<Square12>> {
    pub fn start(&self) -> BB12<Square12> {
        let left_rank = &[1, 2, 3, 4, 5, 6];
        let right_rank = &[7, 8, 9, 10, 11, 12];
        let left_files = &['a', 'b', 'c', 'd', 'e', 'f'];
        let right_files = &['g', 'h', 'i', 'j', 'k', 'l'];
        self.generate_plinths(left_rank, right_rank, left_files, right_files)
    }
}

impl Default for PlinthGen12<Square12, BB12<Square12>> {
    fn default() -> Self {
        PlinthGen12 {
            _a: PhantomData,
            _s: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{attacks::Attacks, shuuro12::attacks12::Attacks12};

    use super::PlinthGen12;

    #[test]
    fn generate_all_plinths() {
        Attacks12::init();
        let pl = PlinthGen12::default();
        let b = pl.start();
        assert_eq!(b.count(), 8);
    }
}
