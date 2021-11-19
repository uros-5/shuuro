use std::fmt;
use std::ops;

use super::Square;

#[derive(Clone, Copy, Debug, Default)]
pub struct BitBoard(pub [u16; 9]);

impl BitBoard {
    pub fn empty() -> BitBoard {
        BitBoard([0, 0, 0, 0, 0, 0, 0, 0, 0])
    }

    pub fn is_any(&self) -> bool {
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

    pub fn is_empty(&self) -> bool {
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

    pub fn clear_at(&mut self, sq: Square) {
        *self &= &!&square_bb(sq)
    }

    pub fn clear_all(&mut self) {
        for i in 0..9 {
            self.0[i] = 0;
        }
    }

    pub fn count(&self) -> u32 {
        let mut counting = 0;
        for i in 0..9 {
            counting += self.0[i].count_ones();
        }
        counting
    }

    pub fn set_all(&mut self) {
        for i in 0..9 {
            self.0[i] = 1;
        }
    }

    pub fn pop(&mut self) -> Option<Square> {
        for i in 0..9 {
            if self.0[i] != 0 {
                let sq =
                    Square::from_index((15 * (1 + i) + i) as u8 - self.0[i].leading_zeros() as u8);
                self.clear_at(sq.unwrap());
                return sq;
            }
        }

        None
    }

    pub fn pop_reverse(&mut self) -> Option<Square> {
        for i in (0..9).rev() {
            if self.0[i] != 0 {
                let sq =
                    Square::from_index((15 * (1 + i) + i) as u8 - self.0[i].leading_zeros() as u8);
                self.clear_at(sq.unwrap());
                return sq;
            }
        }

        None
    }

    pub fn merge(&self) -> u16 {
        self.0[0]
            | self.0[1]
            | self.0[2]
            | self.0[3]
            | self.0[4]
            | self.0[5]
            | self.0[6]
            | self.0[7]
            | self.0[8]
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "+---+---+---+---+---+---+---+---+---+---+---+---+")?;

        for file in (0..12).rev() {
            write!(f, "|")?;
            for rank in 0..12 {
                let sq = Square::new(rank, file).unwrap();
                write!(f, " {} |", if (self & sq).is_empty() { " " } else { "X" })?;
            }
            //writeln!(f, " {}", (b'a' + rank) as char)?;
            writeln!(f, "\n+---+---+---+---+---+---+---+---+---+---+---+---+")?;
        }
        writeln!(f, "a   b   c   d   e   f   g   h   i   j   k   l")?;

        Ok(())
    }
}

impl<'a> ops::Not for &'a BitBoard {
    type Output = BitBoard;

    #[inline(always)]
    fn not(self) -> BitBoard {
        BitBoard([
            !self.0[0], !self.0[1], !self.0[2], !self.0[3], !self.0[4], !self.0[5], !self.0[6],
            !self.0[7], !self.0[8],
        ])
    }
}

impl<'a, 'b> ops::BitAnd<&'a BitBoard> for &'b BitBoard {
    type Output = BitBoard;

    #[inline(always)]
    fn bitand(self, rhs: &'a BitBoard) -> BitBoard {
        BitBoard([
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

impl<'a> ops::BitAndAssign<&'a BitBoard> for BitBoard {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: &'a BitBoard) {
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

impl<'a, 'b> ops::BitOr<&'a BitBoard> for &'b BitBoard {
    type Output = BitBoard;

    #[inline(always)]
    fn bitor(self, rhs: &'a BitBoard) -> BitBoard {
        BitBoard([
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

impl<'a> ops::BitOrAssign<&'a BitBoard> for BitBoard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: &'a BitBoard) {
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

impl<'a, 'b> ops::BitXor<&'a BitBoard> for &'b BitBoard {
    type Output = BitBoard;

    #[inline(always)]
    fn bitxor(self, rhs: &'a BitBoard) -> BitBoard {
        BitBoard([
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

impl<'a> ops::BitXorAssign<&'a BitBoard> for BitBoard {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: &'a BitBoard) {
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

impl<'a> ops::BitAnd<Square> for &'a BitBoard {
    type Output = BitBoard;

    #[inline(always)]
    fn bitand(self, rhs: Square) -> BitBoard {
        self & &square_bb(rhs)
    }
}

impl<'a> ops::BitAndAssign<Square> for BitBoard {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Square) {
        *self &= &square_bb(rhs)
    }
}

impl<'a> ops::BitOr<Square> for &'a BitBoard {
    type Output = BitBoard;

    #[inline(always)]
    fn bitor(self, rhs: Square) -> BitBoard {
        self | &square_bb(rhs)
    }
}

impl<'a> ops::BitOrAssign<Square> for BitBoard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Square) {
        *self |= &square_bb(rhs)
    }
}

impl<'a> ops::BitXor<Square> for &'a BitBoard {
    type Output = BitBoard;

    #[inline(always)]
    fn bitxor(self, rhs: Square) -> BitBoard {
        self ^ &square_bb(rhs)
    }
}

impl<'a> ops::BitXorAssign<Square> for BitBoard {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Square) {
        *self ^= &square_bb(rhs)
    }
}

impl Iterator for BitBoard {
    type Item = Square;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_any() {
            self.pop_reverse()
        } else {
            None
        }
    }
}

pub const SQUARE_BB: [BitBoard; 144] = [
    BitBoard([1, 0, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([1 << 1, 0, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([1 << 2, 0, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([1 << 3, 0, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([1 << 4, 0, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([1 << 5, 0, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([1 << 6, 0, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([1 << 7, 0, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([1 << 8, 0, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([1 << 9, 0, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([1 << 10, 0, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([1 << 11, 0, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([1 << 12, 0, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([1 << 13, 0, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([1 << 14, 0, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([1 << 15, 0, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 1, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 1 << 1, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 1 << 2, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 1 << 3, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 1 << 4, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 1 << 5, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 1 << 6, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 1 << 7, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 1 << 8, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 1 << 9, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 1 << 10, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 1 << 11, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 1 << 12, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 1 << 13, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 1 << 14, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 1 << 15, 0, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 1, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 1 << 1, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 1 << 2, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 1 << 3, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 1 << 4, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 1 << 5, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 1 << 6, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 1 << 7, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 1 << 8, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 1 << 9, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 1 << 10, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 1 << 11, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 1 << 12, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 1 << 13, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 1 << 14, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 1 << 15, 0, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 1, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 1 << 1, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 1 << 2, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 1 << 3, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 1 << 4, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 1 << 5, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 1 << 6, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 1 << 7, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 1 << 8, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 1 << 9, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 1 << 10, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 1 << 11, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 1 << 12, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 1 << 13, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 1 << 14, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 1 << 15, 0, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 1, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 1 << 1, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 1 << 2, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 1 << 3, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 1 << 4, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 1 << 5, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 1 << 6, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 1 << 7, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 1 << 8, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 1 << 9, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 1 << 10, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 1 << 11, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 1 << 12, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 1 << 13, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 1 << 14, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 1 << 15, 0, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 1, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 1 << 1, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 1 << 2, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 1 << 3, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 1 << 4, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 1 << 5, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 1 << 6, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 1 << 7, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 1 << 8, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 1 << 9, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 1 << 10, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 1 << 11, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 1 << 12, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 1 << 13, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 1 << 14, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 1 << 15, 0, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 1, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 1 << 1, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 1 << 2, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 1 << 3, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 1 << 4, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 1 << 5, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 1 << 6, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 1 << 7, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 1 << 8, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 1 << 9, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 1 << 10, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 1 << 11, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 1 << 12, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 1 << 13, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 1 << 14, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 1 << 15, 0, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 1, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 1 << 1, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 1 << 2, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 1 << 3, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 1 << 4, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 1 << 5, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 1 << 6, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 1 << 7, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 1 << 8, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 1 << 9, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 1 << 10, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 1 << 11, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 1 << 12, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 1 << 13, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 1 << 14, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 1 << 15, 0]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 0, 1]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 0, 1 << 1]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 0, 1 << 2]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 0, 1 << 3]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 0, 1 << 4]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 0, 1 << 5]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 0, 1 << 6]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 0, 1 << 7]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 0, 1 << 8]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 0, 1 << 9]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 0, 1 << 10]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 0, 1 << 11]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 0, 1 << 12]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 0, 1 << 13]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 0, 1 << 14]),
    BitBoard([0, 0, 0, 0, 0, 0, 0, 0, 1 << 15]),
];

pub fn square_bb(sq: Square) -> BitBoard {
    SQUARE_BB[sq.index()]
}
