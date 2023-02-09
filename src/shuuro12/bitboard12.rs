use crate::bitboard::BitBoard;
use crate::Square;

use super::square12::Square12;
use core::fmt;
use std::marker::PhantomData;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Clone, Copy, Debug, Default)]
pub struct B12<S: Square>(pub [u16; 9], PhantomData<S>);

impl B12<Square12> {
    pub const fn new(b: [u16; 9]) -> B12<Square12> {
        B12(b, PhantomData)
    }
}

impl BitAnd<&B12<Square12>> for &B12<Square12> {
    type Output = B12<Square12>;

    #[inline(always)]
    fn bitand(self, rhs: &B12<Square12>) -> B12<Square12> {
        B12::new([
            self.0[0] & rhs.0[0],
            self.0[1] & rhs.0[1],
            self.0[2] & rhs.0[2],
            self.0[3] & rhs.0[3],
            self.0[4] & rhs.0[4],
            self.0[5] & rhs.0[5],
            self.0[6] & rhs.0[6],
            self.0[7] & rhs.0[7],
            self.0[8] & rhs.0[8],
        ])
    }
}

impl BitXor<&B12<Square12>> for B12<Square12> {
    type Output = B12<Square12>;

    #[inline(always)]
    fn bitxor(self, rhs: &B12<Square12>) -> B12<Square12> {
        B12::new([
            self.0[0] ^ rhs.0[0],
            self.0[1] ^ rhs.0[1],
            self.0[2] ^ rhs.0[2],
            self.0[3] ^ rhs.0[3],
            self.0[4] ^ rhs.0[4],
            self.0[5] ^ rhs.0[5],
            self.0[6] ^ rhs.0[6],
            self.0[7] ^ rhs.0[7],
            self.0[8] ^ rhs.0[8],
        ])
    }
}

impl BitOr<&B12<Square12>> for &B12<Square12> {
    type Output = B12<Square12>;

    #[inline(always)]
    fn bitor(self, rhs: &B12<Square12>) -> B12<Square12> {
        B12::new([
            self.0[0] | rhs.0[0],
            self.0[1] | rhs.0[1],
            self.0[2] | rhs.0[2],
            self.0[3] | rhs.0[3],
            self.0[4] | rhs.0[4],
            self.0[5] | rhs.0[5],
            self.0[6] | rhs.0[6],
            self.0[7] | rhs.0[7],
            self.0[8] | rhs.0[8],
        ])
    }
}

impl Not for B12<Square12> {
    type Output = B12<Square12>;

    fn not(self) -> Self::Output {
        B12::new([
            !self.0[0], !self.0[1], !self.0[2], !self.0[3], !self.0[4], !self.0[5], !self.0[6],
            !self.0[7], !self.0[8],
        ])
    }
}

impl Not for &B12<Square12> {
    type Output = B12<Square12>;

    fn not(self) -> Self::Output {
        B12::new([
            !self.0[0], !self.0[1], !self.0[2], !self.0[3], !self.0[4], !self.0[5], !self.0[6],
            !self.0[7], !self.0[8],
        ])
    }
}

impl BitOr<&Square12> for &B12<Square12> {
    type Output = B12<Square12>;

    #[inline(always)]
    fn bitor(self, rhs: &Square12) -> B12<Square12> {
        // let b = &square_bb::<Square12, B12<Square12>>(rhs);
        self | &square_bb(rhs)
    }
}

impl BitAnd<&Square12> for &B12<Square12> {
    type Output = B12<Square12>;

    #[inline(always)]
    fn bitand(self, rhs: &Square12) -> B12<Square12> {
        self & &square_bb(rhs)
    }
}

impl BitAndAssign<&B12<Square12>> for B12<Square12> {
    #[inline(always)]

    fn bitand_assign(&mut self, rhs: &B12<Square12>) {
        self.0[0] &= rhs.0[0];
        self.0[1] &= rhs.0[1];
        self.0[2] &= rhs.0[2];
        self.0[3] &= rhs.0[3];
        self.0[4] &= rhs.0[4];
        self.0[5] &= rhs.0[5];
        self.0[6] &= rhs.0[6];
        self.0[7] &= rhs.0[7];
        self.0[8] &= rhs.0[8];
    }
}

impl BitOrAssign<&B12<Square12>> for B12<Square12> {
    #[inline(always)]

    fn bitor_assign(&mut self, rhs: &B12<Square12>) {
        self.0[0] |= rhs.0[0];
        self.0[1] |= rhs.0[1];
        self.0[2] |= rhs.0[2];
        self.0[3] |= rhs.0[3];
        self.0[4] |= rhs.0[4];
        self.0[5] |= rhs.0[5];
        self.0[6] |= rhs.0[6];
        self.0[7] |= rhs.0[7];
        self.0[8] |= rhs.0[8];
    }
}

impl BitXorAssign<&B12<Square12>> for B12<Square12> {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: &B12<Square12>) {
        self.0[0] ^= rhs.0[0];
        self.0[1] ^= rhs.0[1];
        self.0[2] ^= rhs.0[2];
        self.0[3] ^= rhs.0[3];
        self.0[4] ^= rhs.0[4];
        self.0[5] ^= rhs.0[5];
        self.0[6] ^= rhs.0[6];
        self.0[7] ^= rhs.0[7];
        self.0[8] ^= rhs.0[8];
    }
}

impl Iterator for B12<Square12> {
    type Item = Square12;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl BitOrAssign<&Square12> for B12<Square12> {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: &Square12) {
        *self |= &square_bb(rhs);
    }
}

impl BitBoard<Square12> for B12<Square12> {
    fn empty() -> Self {
        B12::new([0, 0, 0, 0, 0, 0, 0, 0, 0])
    }

    fn is_any(&self) -> bool {
        (self.0[0]
            | self.0[1]
            | self.0[2]
            | self.0[3]
            | self.0[4]
            | self.0[5]
            | self.0[6]
            | self.0[7]
            | self.0[8])
            != 0
    }

    fn is_empty(&self) -> bool {
        (self.0[0]
            | self.0[1]
            | self.0[2]
            | self.0[3]
            | self.0[4]
            | self.0[5]
            | self.0[6]
            | self.0[7]
            | self.0[8])
            == 0
    }

    fn clear_at(&mut self, sq: Square12) {
        *self &= &!&square_bb(&sq)
    }

    fn clear_all(&mut self) {
        for i in 0..9 {
            self.0[i] = 0;
        }
    }

    fn count(&self) -> u32 {
        let mut counting = 0;
        for i in 0..9 {
            counting += self.0[i].count_ones();
        }
        counting
    }

    fn set_all(&mut self) {
        for i in 0..9 {
            self.0[i] = 1;
        }
    }

    fn pop(&mut self) -> Option<Square12> {
        for i in 0..9 {
            if self.0[i] != 0 {
                let sq = Square::from_index(self.0[i].trailing_zeros() as u8 + (i as u8 * 16));
                self.0[i] &= self.0[i] - 1;
                return sq;
            }
        }
        None
    }

    fn pop_reverse(&mut self) -> Option<Square12> {
        for i in 0..9 {
            if self.0[i] != 0 {
                let sq = Square::from_index(self.0[i].trailing_zeros() as u8 + (i as u8 * 16));
                self.0[i] &= self.0[i] - 1;
                return sq;
            }
        }
        None
    }

    fn from_square(sq: &Square12) -> Self {
        square_bb(sq)
    }
}

impl fmt::Display for B12<Square12> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "+---+---+---+---+---+---+---+---+---+---+---+---+")?;
        for rank in (0..12).rev() {
            write!(f, "|")?;
            for file in 0..12 {
                let sq = Square12::new(file, rank).unwrap();
                write!(f, " {} |", if (self & &sq).is_empty() { " " } else { "X" })?;
            }
            //writeln!(f, " {}", (b'a' + rank) as char)?;
            writeln!(f, "\n+---+---+---+---+---+---+---+---+---+---+---+---+")?;
        }
        writeln!(f, "a   b   c   d   e   f   g   h   i   j   k   l")?;
        Ok(())
    }
}

pub const SQUARE_BB: [B12<Square12>; 144] = [
    B12::new([1, 0, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([1 << 1, 0, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([1 << 2, 0, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([1 << 3, 0, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([1 << 4, 0, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([1 << 5, 0, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([1 << 6, 0, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([1 << 7, 0, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([1 << 8, 0, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([1 << 9, 0, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([1 << 10, 0, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([1 << 11, 0, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([1 << 12, 0, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([1 << 13, 0, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([1 << 14, 0, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([1 << 15, 0, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 1, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 1 << 1, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 1 << 2, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 1 << 3, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 1 << 4, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 1 << 5, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 1 << 6, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 1 << 7, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 1 << 8, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 1 << 9, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 1 << 10, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 1 << 11, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 1 << 12, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 1 << 13, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 1 << 14, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 1 << 15, 0, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 1, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 1 << 1, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 1 << 2, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 1 << 3, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 1 << 4, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 1 << 5, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 1 << 6, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 1 << 7, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 1 << 8, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 1 << 9, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 1 << 10, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 1 << 11, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 1 << 12, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 1 << 13, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 1 << 14, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 1 << 15, 0, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 1, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 1 << 1, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 1 << 2, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 1 << 3, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 1 << 4, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 1 << 5, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 1 << 6, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 1 << 7, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 1 << 8, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 1 << 9, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 1 << 10, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 1 << 11, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 1 << 12, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 1 << 13, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 1 << 14, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 1 << 15, 0, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 1, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 1 << 1, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 1 << 2, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 1 << 3, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 1 << 4, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 1 << 5, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 1 << 6, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 1 << 7, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 1 << 8, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 1 << 9, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 1 << 10, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 1 << 11, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 1 << 12, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 1 << 13, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 1 << 14, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 1 << 15, 0, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 1, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 1 << 1, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 1 << 2, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 1 << 3, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 1 << 4, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 1 << 5, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 1 << 6, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 1 << 7, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 1 << 8, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 1 << 9, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 1 << 10, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 1 << 11, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 1 << 12, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 1 << 13, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 1 << 14, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 1 << 15, 0, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 1, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 1 << 1, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 1 << 2, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 1 << 3, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 1 << 4, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 1 << 5, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 1 << 6, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 1 << 7, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 1 << 8, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 1 << 9, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 1 << 10, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 1 << 11, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 1 << 12, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 1 << 13, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 1 << 14, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 1 << 15, 0, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 1, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 1 << 1, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 1 << 2, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 1 << 3, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 1 << 4, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 1 << 5, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 1 << 6, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 1 << 7, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 1 << 8, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 1 << 9, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 1 << 10, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 1 << 11, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 1 << 12, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 1 << 13, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 1 << 14, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 1 << 15, 0]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 0, 1]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 0, 1 << 1]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 0, 1 << 2]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 0, 1 << 3]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 0, 1 << 4]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 0, 1 << 5]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 0, 1 << 6]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 0, 1 << 7]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 0, 1 << 8]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 0, 1 << 9]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 0, 1 << 10]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 0, 1 << 11]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 0, 1 << 12]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 0, 1 << 13]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 0, 1 << 14]),
    B12::new([0, 0, 0, 0, 0, 0, 0, 0, 1 << 15]),
];

pub fn square_bb(sq: &Square12) -> B12<Square12> {
    SQUARE_BB[sq.index()]
}
