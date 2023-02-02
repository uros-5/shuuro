use std::ops::{BitAnd, BitOr};

use rand::prelude::*;

use crate::{bitboard::BitBoard, Square};

pub trait PlinthGen<S: Square> {
    fn gen_square<const R: usize, const F: usize>(
        &self,
        rang: &mut ThreadRng,
        ranks: &[char; R],
        files: &[char; F],
    ) -> S {
        let rank = rang.gen_range(0..R);
        let file = rang.gen_range(0..F);
        S::from_sfen(&format!("{}{}", ranks[rank], files[file])[..]).unwrap()
    }

    fn two_plinths<B, const C: usize>(&self, ranks: &[char; C], files: &[char; C]) -> B
    where
        B: BitBoard<S>,
        for<'a> &'a B: BitOr<&'a S, Output = B>,
        for<'a> &'a B: BitOr<S, Output = B>,
        for<'a> &'a B: BitAnd<S, Output = B>,
    {
        let mut rang = thread_rng();
        let sq1: S = self.gen_square(&mut rang, ranks, files);
        let bb = &B::empty() | sq1;
        let attacks: B = self.moves(sq1);
        loop {
            let sq2: S = self.gen_square(&mut rang, ranks, files);
            let check = &attacks & sq2;
            if check.is_empty() {
                return &bb | sq2;
            }
        }
    }
    fn moves<B: BitBoard<S>>(&self, sq: S) -> B;
    fn random_number() -> u8;
    fn generate_plinths<B, const R: usize, const F: usize>(
        &self,
        ranks: &[char; R],
        files: &[char; F],
    ) -> B;
}
