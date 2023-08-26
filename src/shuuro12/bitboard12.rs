use crate::bitboard::BitBoard;
use crate::Square;

use super::square12::Square12;
use core::fmt;
use std::marker::PhantomData;
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct BB12<S: Square>(pub (u128, u16), PhantomData<S>);

impl BB12<Square12> {
    pub const fn new(b: (u128, u16)) -> BB12<Square12> {
        BB12(b, PhantomData)
    }

    pub fn count2(&self) -> u32 {
        let mut counting = 0;
        counting += self.0 .0.count_ones();
        counting += self.0 .1.count_ones();
        counting
    }
}

impl BitAnd<&BB12<Square12>> for &BB12<Square12> {
    type Output = BB12<Square12>;

    #[inline(always)]
    fn bitand(self, rhs: &BB12<Square12>) -> BB12<Square12> {
        BB12::new((self.0 .0 & rhs.0 .0, self.0 .1 & rhs.0 .1))
    }
}

impl BitXor<&BB12<Square12>> for BB12<Square12> {
    type Output = BB12<Square12>;

    #[inline(always)]
    fn bitxor(self, rhs: &BB12<Square12>) -> BB12<Square12> {
        BB12::new((self.0 .0 ^ rhs.0 .0, self.0 .1 ^ rhs.0 .1))
    }
}

impl BitOr<&BB12<Square12>> for &BB12<Square12> {
    type Output = BB12<Square12>;

    #[inline(always)]
    fn bitor(self, rhs: &BB12<Square12>) -> BB12<Square12> {
        BB12::new((self.0 .0 | rhs.0 .0, self.0 .1 | rhs.0 .1))
    }
}

impl Not for BB12<Square12> {
    type Output = BB12<Square12>;

    #[inline(always)]
    fn not(self) -> Self::Output {
        BB12::new((!self.0 .0, !self.0 .1))
    }
}

impl Not for &BB12<Square12> {
    type Output = BB12<Square12>;

    #[inline(always)]
    fn not(self) -> Self::Output {
        BB12::new((!self.0 .0, !self.0 .1))
    }
}

impl BitOr<&Square12> for &BB12<Square12> {
    type Output = BB12<Square12>;

    #[inline(always)]
    fn bitor(self, rhs: &Square12) -> BB12<Square12> {
        // let b = &square_bb::<Square12, B12<Square12>>(rhs);
        self | &square_bb(rhs)
    }
}

impl BitAnd<&Square12> for &BB12<Square12> {
    type Output = BB12<Square12>;

    #[inline(always)]
    fn bitand(self, rhs: &Square12) -> BB12<Square12> {
        self & &square_bb(rhs)
    }
}

impl BitAndAssign<&BB12<Square12>> for BB12<Square12> {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: &BB12<Square12>) {
        self.0 .0 &= rhs.0 .0;
        self.0 .1 &= rhs.0 .1;
    }
}

impl BitOrAssign<&BB12<Square12>> for BB12<Square12> {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: &BB12<Square12>) {
        self.0 .0 |= rhs.0 .0;
        self.0 .1 |= rhs.0 .1;
    }
}

impl BitXorAssign<&BB12<Square12>> for BB12<Square12> {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: &BB12<Square12>) {
        self.0 .0 ^= rhs.0 .0;
        self.0 .1 ^= rhs.0 .1;
    }
}

impl BitXorAssign<&Square12> for BB12<Square12> {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: &Square12) {
        let rhs = square_bb(rhs);
        self.0 .0 ^= rhs.0 .0;
        self.0 .1 ^= rhs.0 .1;
    }
}

impl Iterator for BB12<Square12> {
    type Item = Square12;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_any() {
            self.pop_reverse()
        } else {
            None
        }
    }
}

impl BitOrAssign<&Square12> for BB12<Square12> {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: &Square12) {
        *self |= &square_bb(rhs);
    }
}

impl BitBoard<Square12> for BB12<Square12> {
    #[inline(always)]
    fn empty() -> Self {
        BB12::new((0, 0))
    }

    #[inline(always)]
    fn full() -> Self {
        BB12::new((1, 1))
    }

    #[inline(always)]
    fn is_any(&self) -> bool {
        (self.0 .0 | self.0 .1 as u128) != 0
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        (self.0 .0 | self.0 .1 as u128) == 0
    }

    #[inline(always)]
    fn clear_at(&mut self, sq: Square12) {
        *self &= &!&square_bb(&sq)
    }

    #[inline(always)]
    fn clear_all(&mut self) {
        self.0 .0 = 0;
        self.0 .1 = 0;
    }

    #[inline(always)]
    fn len(&self) -> u32 {
        let mut counting = 0;
        counting += self.0 .0.count_ones();
        counting += self.0 .1.count_ones();
        counting
    }

    #[inline(always)]
    fn set_all(&mut self) {
        self.0 .0 = 1;
        self.0 .1 = 1;
    }

    #[inline(always)]
    fn pop(&mut self) -> Option<Square12> {
        if self.0 .0 != 0 {
            let sq = Square12::from_index(self.0 .0.trailing_zeros() as u8);
            if sq.is_some() {
                self.0 .0 &= self.0 .0 - 1;
            }
            return sq;
        } else if self.0 .1 != 0 {
            let sq = Square12::from_index(self.0 .1.trailing_zeros() as u8);
            if sq.is_some() {
                self.0 .1 &= self.0 .1 - 1;
            }
            return sq;
        }
        None
    }

    #[inline(always)]
    fn pop_reverse(&mut self) -> Option<Square12> {
        if self.0 .1 != 0 {
            let sq = Square12::from_index(15 - self.0 .1.leading_zeros() as u8);
            if sq.is_some() {
                self.0 .1 &= self.0 .1 - 1;
            }
            return sq;
        } else if self.0 .0 != 0 {
            let sq =
                Square12::from_index(127 - self.0 .0.leading_zeros() as u8);
            if sq.is_some() {
                self.0 .0 &= self.0 .0 - 1;
            }
            return sq;
        }
        None
    }

    #[inline(always)]
    fn from_square(sq: &Square12) -> Self {
        square_bb(sq)
    }
}

impl fmt::Display for BB12<Square12> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "+---+---+---+---+---+---+---+---+---+---+---+---+")?;
        for rank in (0..12).rev() {
            write!(f, "|")?;
            for file in 0..12 {
                if let Some(sq) = Square12::new(file, rank) {
                    write!(
                        f,
                        " {} |",
                        if (self & &sq).is_empty() { " " } else { "X" }
                    )?;
                }
            }
            writeln!(f, "\n+---+---+---+---+---+---+---+---+---+---+---+---+")?;
        }
        writeln!(f, "a   b   c   d   e   f   g   h   i   j   k   l")?;
        Ok(())
    }
}

const fn all_squares() -> [BB12<Square12>; 144] {
    let mut all = [BB12::new((0, 0)); 144];
    let mut sq = 0;
    while sq < 128 {
        all[sq] = BB12::new((1 << sq, 0));
        sq += 1;
    }
    sq = 0;
    while sq < 16 {
        all[128 + sq] = BB12::new((0, 1 << sq));
        sq += 1;
    }
    all
}
pub const SQUARE_BB: [BB12<Square12>; 144] = all_squares();

pub fn square_bb(sq: &Square12) -> BB12<Square12> {
    SQUARE_BB[sq.index()]
}
