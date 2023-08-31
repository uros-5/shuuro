use std::ops::{BitAnd, BitOr, Not};

use rand::prelude::*;

use crate::{bitboard::BitBoard, Square};

type Section = (u8, u8, u8, u8, u8);

pub trait PlinthGen<S: Square, B: BitBoard<S>>
where
    for<'b> &'b B: BitOr<&'b B, Output = B>,
    for<'a> &'a B: BitAnd<&'a B, Output = B>,
    for<'a> &'a B: BitAnd<&'a S, Output = B>,
    for<'a> &'a B: Not<Output = B>,
    for<'a> &'a B: BitOr<&'a S, Output = B>,
{
    fn king_moves(&self, sq: S) -> B;

    fn y(&self) -> u8;

    fn generate_plinths(&self, sections: &[Section]) -> B {
        let mut plinths = B::empty();
        let mut rang = thread_rng();
        let rank = self.y();
        for section in sections {
            let mut bb = B::empty();
            let mut i = 0;
            while i < section.4 {
                let y = rang.gen_range(section.0..section.1);
                let x = rang.gen_range(section.2..section.3);
                let sq = (y * rank) + x;
                if let Some(sq) = S::from_index(sq) {
                    let king_moves = self.king_moves(sq);
                    if (&king_moves & &bb).is_empty() {
                        bb |= &sq;
                        i += 1;
                    }
                }
            }
            plinths |= &bb;
        }
        plinths
    }
}
