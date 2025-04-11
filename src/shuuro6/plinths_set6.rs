use crate::{
    attacks::Attacks, bitboard::BitBoard, plinths_set::PlinthGen, Square,
};

use super::{attacks6::Attacks6, bitboard6::BB6, square6::Square6};
use std::marker::PhantomData;

pub struct PlinthGen6<S, B>
where
    S: Square,
    B: BitBoard<S>,
{
    _a: PhantomData<B>,
    _s: PhantomData<S>,
}

impl PlinthGen<Square6, BB6<Square6>> for PlinthGen6<Square6, BB6<Square6>> {
    fn king_moves(&self, sq: Square6) -> BB6<Square6> {
        Attacks6::get_non_sliding_attacks(
            crate::PieceType::King,
            &sq,
            crate::Color::White,
            BB6::empty(),
        )
    }

    fn y(&self) -> u8 {
        6
    }
}

impl PlinthGen6<Square6, BB6<Square6>> {
    pub fn start(&self) -> BB6<Square6> {
        let sections = [(0, 3, 0, 6, 1), (3, 7, 0, 6, 1)];
        self.generate_plinths(&sections)
    }
}

impl Default for PlinthGen6<Square6, BB6<Square6>> {
    fn default() -> Self {
        PlinthGen6 {
            _a: PhantomData,
            _s: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{attacks::Attacks, shuuro6::attacks6::Attacks6};

    use super::PlinthGen6;

    #[test]
    fn generate_all_plinths() {
        Attacks6::init();
        let pl = PlinthGen6::default();
        let b = pl.start();
        assert_eq!(b.count(), 2);
    }
}
