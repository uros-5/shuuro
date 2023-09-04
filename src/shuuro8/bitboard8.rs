use crate::bitboard::BitBoard;
use crate::Square;

use super::square8::Square8;
use core::fmt;
use std::marker::PhantomData;
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct BB8<S: Square>(pub u64, PhantomData<S>);

impl BB8<Square8> {
    pub const fn new(b: u64) -> BB8<Square8> {
        BB8(b, PhantomData)
    }
}

impl BitAnd<&BB8<Square8>> for &BB8<Square8> {
    type Output = BB8<Square8>;

    #[inline(always)]
    fn bitand(self, rhs: &BB8<Square8>) -> BB8<Square8> {
        BB8::new(self.0 & rhs.0)
    }
}

impl BitXor<&BB8<Square8>> for BB8<Square8> {
    type Output = BB8<Square8>;

    #[inline(always)]
    fn bitxor(self, rhs: &BB8<Square8>) -> BB8<Square8> {
        BB8::new(self.0 ^ rhs.0)
    }
}
impl BitOr<&BB8<Square8>> for &BB8<Square8> {
    type Output = BB8<Square8>;

    #[inline(always)]
    fn bitor(self, rhs: &BB8<Square8>) -> BB8<Square8> {
        BB8::new(self.0 | rhs.0)
    }
}

impl Not for BB8<Square8> {
    type Output = BB8<Square8>;

    #[inline(always)]
    fn not(self) -> Self::Output {
        BB8::new(!self.0)
    }
}

impl Not for &BB8<Square8> {
    type Output = BB8<Square8>;

    #[inline(always)]
    fn not(self) -> Self::Output {
        BB8::new(!self.0)
    }
}

impl BitOr<&Square8> for &BB8<Square8> {
    type Output = BB8<Square8>;

    #[inline(always)]
    fn bitor(self, rhs: &Square8) -> BB8<Square8> {
        // let b = &square_bb::<Square8, B8<Square8>>(rhs);
        self | &square_bb(rhs)
    }
}

impl BitAndAssign<&BB8<Square8>> for BB8<Square8> {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: &BB8<Square8>) {
        self.0 &= rhs.0;
    }
}

impl BitOrAssign<&BB8<Square8>> for BB8<Square8> {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: &BB8<Square8>) {
        self.0 |= rhs.0;
    }
}

impl BitXorAssign<&BB8<Square8>> for BB8<Square8> {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: &BB8<Square8>) {
        self.0 ^= rhs.0;
    }
}

impl BitXorAssign<&Square8> for BB8<Square8> {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: &Square8) {
        let rhs = square_bb(rhs);
        self.0 ^= rhs.0;
    }
}

impl Iterator for BB8<Square8> {
    type Item = Square8;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_any() {
            self.pop_reverse()
        } else {
            None
        }
    }
}

impl BitOrAssign<&Square8> for BB8<Square8> {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: &Square8) {
        *self |= &square_bb(rhs);
    }
}

impl BitAnd<&Square8> for &BB8<Square8> {
    type Output = BB8<Square8>;

    #[inline(always)]
    fn bitand(self, rhs: &Square8) -> BB8<Square8> {
        self & &square_bb(rhs)
    }
}

impl fmt::Display for BB8<Square8> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "+---+---+---+---+---+---+---+---+")?;
        for rank in (0..8).rev() {
            write!(f, "|")?;
            for file in 0..8 {
                if let Some(sq) = Square8::new(file, rank) {
                    write!(
                        f,
                        " {} |",
                        if (self & &sq).is_empty() { " " } else { "X" }
                    )?;
                }
            }
            //writeln!(f, " {}", (b'a' + rank) as char)?;
            writeln!(f, "\n+---+---+---+---+---+---+---+---+")?;
        }
        writeln!(f, "a   b   c   d   e   f   g   h")?;
        Ok(())
    }
}

impl BitBoard<Square8> for BB8<Square8> {
    #[inline(always)]
    fn empty() -> Self {
        BB8::new(0)
    }

    #[inline(always)]
    fn full() -> Self {
        BB8::new(1)
    }

    #[inline(always)]
    fn is_any(&self) -> bool {
        self.0 != 0
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[inline(always)]
    fn clear_at(&mut self, sq: Square8) {
        *self &= &!&square_bb(&sq)
    }

    #[inline(always)]
    fn clear_all(&mut self) {
        self.0 = 0;
    }

    #[inline(always)]
    fn len(&self) -> u32 {
        self.0.count_ones()
    }

    #[inline(always)]
    fn set_all(&mut self) {
        self.0 = 1;
    }

    #[inline(always)]
    fn pop(&mut self) -> Option<Square8> {
        if self.is_empty() {
            None
        } else {
            let calc = self.0.trailing_zeros() as u64;
            let sq = Square8::from_index(calc as u8);
            self.0 = calc;
            sq
        }
    }

    #[inline(always)]
    fn pop_reverse(&mut self) -> Option<Square8> {
        if self.is_empty() {
            None
        } else {
            let calc = 63 - self.0.leading_zeros() as u64;
            let sq = Square8::from_index(calc as u8);
            if let Some(sq) = sq {
                self.clear_at(sq);
            }
            sq
        }
    }

    #[inline(always)]
    fn from_square(sq: &Square8) -> Self {
        square_bb(sq)
    }
}

pub fn square_bb(sq: &Square8) -> BB8<Square8> {
    SQUARE_BB[sq.index()]
}

pub const SQUARE_BB: [BB8<Square8>; 64] = {
    let mut squares = [BB8::new(0); 64];
    let mut i: u32 = 0;
    while i < 64 {
        squares[i as usize] = BB8::new(1 << i);
        i += 1;
    }
    squares
};
