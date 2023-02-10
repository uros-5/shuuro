use std::ops::{BitAnd, BitOr, Not};

use rand::prelude::*;

use crate::{bitboard::BitBoard, Square};

pub trait PlinthGen<S: Square, B: BitBoard<S>>
where
    for<'b> &'b B: BitOr<&'b B, Output = B>,
    for<'a> &'a B: BitAnd<&'a B, Output = B>,
    for<'a> &'a B: BitAnd<&'a S, Output = B>,
    for<'a> &'a B: Not<Output = B>,
    for<'a> &'a B: BitOr<&'a S, Output = B>,
    for<'a> &'a B: BitOr<S, Output = B>,
    for<'a> &'a B: BitAnd<S, Output = B>,
{
    fn gen_square<const R: usize, const F: usize>(
        &self,
        rang: &mut ThreadRng,
        ranks: &[u8; R],
        files: &[char; F],
    ) -> S {
        let rank = rang.gen_range(0..R);
        let file = rang.gen_range(0..F);
        S::from_sfen(&format!("{}{}", files[file], ranks[rank])[..]).unwrap()
    }

    fn two_plinths<const R: usize, const F: usize>(&self, ranks: &[u8; R], files: &[char; F]) -> B {
        let mut rang = thread_rng();
        let sq1: S = self.gen_square(&mut rang, ranks, files);
        #[allow(clippy::op_ref)]
        let bb = &B::empty() | sq1;
        let attacks: B = self.king_moves(sq1);
        loop {
            let sq2: S = self.gen_square(&mut rang, ranks, files);
            let check = &attacks & sq2;
            if check.is_empty() {
                #[allow(clippy::op_ref)]
                return &bb | sq2;
            }
        }
    }
    fn king_moves(&self, sq: S) -> B;
    fn random_number() -> u8;
    fn plinths_count(&self, count: usize) -> bool;
    fn generate_plinths<const R: usize, const F: usize>(
        &self,
        left_ranks: &[u8; R],
        right_ranks: &[u8; R],
        left_files: &[char; F],
        right_files: &[char; F],
    ) -> B {
        let mut bb = B::empty();
        for i in [left_files, right_files] {
            for j in [left_ranks, right_ranks] {
                let new_bb = self.two_plinths(j, i);
                if new_bb.count() == 2 {
                    bb |= &new_bb;
                }
            }
            if self.plinths_count(bb.count()) {
                break;
            }
        }
        bb
    }
}