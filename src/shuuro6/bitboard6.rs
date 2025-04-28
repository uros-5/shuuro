use crate::bitboard::BitBoard;
use crate::Square;

use super::square6::Square6;
use core::fmt;
use std::marker::PhantomData;
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BB6<S: Square>(pub u64, PhantomData<S>);

impl BB6<Square6> {
    pub const fn new(b: u64) -> BB6<Square6> {
        BB6(b, PhantomData)
    }
}

impl BitAnd<&BB6<Square6>> for BB6<Square6> {
    type Output = BB6<Square6>;

    #[inline(always)]
    fn bitand(self, rhs: &BB6<Square6>) -> BB6<Square6> {
        BB6::new(self.0 & rhs.0)
    }
}

impl BitXor<&BB6<Square6>> for BB6<Square6> {
    type Output = BB6<Square6>;

    #[inline(always)]
    fn bitxor(self, rhs: &BB6<Square6>) -> BB6<Square6> {
        BB6::new(self.0 ^ rhs.0)
    }
}
impl BitOr<&BB6<Square6>> for BB6<Square6> {
    type Output = BB6<Square6>;

    #[inline(always)]
    fn bitor(self, rhs: &BB6<Square6>) -> BB6<Square6> {
        BB6::new(self.0 | rhs.0)
    }
}

impl Not for BB6<Square6> {
    type Output = BB6<Square6>;

    #[inline(always)]
    fn not(self) -> Self::Output {
        BB6::new(!self.0)
    }
}

impl Not for &BB6<Square6> {
    type Output = BB6<Square6>;

    #[inline(always)]
    fn not(self) -> Self::Output {
        BB6::new(!self.0)
    }
}

impl BitOr<&Square6> for BB6<Square6> {
    type Output = BB6<Square6>;

    #[inline(always)]
    fn bitor(self, rhs: &Square6) -> BB6<Square6> {
        // let b = &square_bb::<Square6, B8<Square6>>(rhs);
        self | &square_bb(rhs)
    }
}

impl BitAndAssign<&BB6<Square6>> for BB6<Square6> {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: &BB6<Square6>) {
        self.0 &= rhs.0;
    }
}

impl BitOrAssign<&BB6<Square6>> for BB6<Square6> {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: &BB6<Square6>) {
        self.0 |= rhs.0;
    }
}

impl BitXorAssign<&BB6<Square6>> for BB6<Square6> {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: &BB6<Square6>) {
        self.0 ^= rhs.0;
    }
}

impl BitXorAssign<&Square6> for BB6<Square6> {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: &Square6) {
        let rhs = square_bb(rhs);
        self.0 ^= rhs.0;
    }
}

impl Iterator for BB6<Square6> {
    type Item = Square6;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_any() {
            self.pop_reverse()
        } else {
            None
        }
    }
}

impl BitOrAssign<&Square6> for BB6<Square6> {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: &Square6) {
        *self |= &square_bb(rhs);
    }
}

impl BitAnd<&Square6> for BB6<Square6> {
    type Output = BB6<Square6>;

    #[inline(always)]
    fn bitand(self, rhs: &Square6) -> BB6<Square6> {
        self & &square_bb(rhs)
    }
}

impl fmt::Display for BB6<Square6> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "+---+---+---+---+---+---+")?;
        for rank in (0..6).rev() {
            write!(f, "|")?;
            for file in 0..6 {
                if let Some(sq) = Square6::new(file, rank) {
                    write!(
                        f,
                        " {} |",
                        if (*self & &sq).is_empty() { " " } else { "X" }
                    )?;
                }
            }
            //writeln!(f, " {}", (b'a' + rank) as char)?;
            writeln!(f, "\n+---+---+---+---+---+---+")?;
        }
        writeln!(f, "a   b   c   d   e   f")?;
        Ok(())
    }
}

impl BitBoard<Square6> for BB6<Square6> {
    #[inline(always)]
    fn empty() -> Self {
        BB6::new(0)
    }

    #[inline(always)]
    fn full() -> Self {
        BB6::new(1)
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
    fn clear_at(&mut self, sq: Square6) {
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
    fn pop(&mut self) -> Option<Square6> {
        if self.is_empty() {
            None
        } else {
            let calc = self.0.trailing_zeros() as u64;
            let sq = Square6::from_index(calc as u8);
            self.0 = calc;
            sq
        }
    }

    #[inline(always)]
    fn pop_reverse(&mut self) -> Option<Square6> {
        if self.is_empty() {
            None
        } else {
            let calc = 63 - self.0.leading_zeros() as u64;
            let sq = Square6::from_index(calc as u8);
            if let Some(sq) = sq {
                self.clear_at(sq);
            }
            sq
        }
    }

    #[inline(always)]
    fn from_square(sq: &Square6) -> Self {
        square_bb(sq)
    }
}

pub fn square_bb(sq: &Square6) -> BB6<Square6> {
    SQUARE_BB[sq.index()]
}

pub const SQUARE_BB: [BB6<Square6>; 36] = {
    let mut squares = [BB6::new(0); 36];
    let mut i: u32 = 0;
    while i < 36 {
        squares[i as usize] = BB6::new(1 << i);
        i += 1;
    }
    squares
};
