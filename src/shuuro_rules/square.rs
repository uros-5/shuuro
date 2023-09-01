use std::fmt::{self, Debug};

use crate::shuuro_rules::Color;

#[macro_export]
macro_rules! ray {
    ($f1: ident, $f2: ident, $self: ident, $a: expr) => {
        if let Some(sq) = $self.$f1() {
            if let Some(sq2) = sq.$f2() {
                return Some(sq2);
            } else {
                return None;
            }
        } else {
            return None;
        }
    };
}

pub trait Square:
    Sized + Eq + fmt::Display + Default + PartialEq + Clone + Copy
where
    Self: Debug,
{
    fn new(file: u8, rank: u8) -> Option<Self>;
    fn from_sfen(s: &str) -> Option<Self>;
    fn from_index(index: u8) -> Option<Self>;
    fn left(&self) -> Option<Self> {
        let file = self.file();
        if file == 0 {
            None
        } else {
            Self::new(file - 1, self.rank())
        }
    }
    fn right(&self) -> Option<Self> {
        let file = self.file();
        if file == self.right_edge() {
            None
        } else {
            Self::new(file + 1, self.rank())
        }
    }
    fn up(&self) -> Option<Self> {
        let rank = self.rank();
        if rank == self.up_edge() {
            None
        } else {
            Self::new(self.file(), rank + 1)
        }
    }
    fn down(&self) -> Option<Self> {
        let rank = self.rank();
        if rank == 0 {
            None
        } else {
            Self::new(self.file(), rank - 1)
        }
    }
    fn nw(&self) -> Option<Self> {
        ray!(up, left, self, 1)
    }
    fn nea(&self) -> Option<Self> {
        ray!(up, right, self, 1)
    }
    fn sw(&self) -> Option<Self> {
        ray!(down, left, self, 1)
    }
    fn se(&self) -> Option<Self> {
        ray!(down, right, self, 1)
    }
    fn x(&self, c: &Color) -> [Option<Self>; 2] {
        match c {
            Color::White => [self.nw(), self.nea()],
            Color::Black => [self.sw(), self.se()],
            _ => [None, None],
        }
    }
    fn right_edge(&self) -> u8;
    fn up_edge(&self) -> u8;
    fn to_int(&self) -> u8;
    fn in_promotion_zone(&self, c: Color) -> bool;
    fn first_pawn_rank(&self, c: Color) -> bool {
        match c {
            Color::White => self.rank() == 1,
            Color::Black => self.rank() == self.up_edge() - 1,
            Color::NoColor => false,
        }
    }
    fn rank(&self) -> u8;
    fn file(&self) -> u8;
    fn index(&self) -> usize;
}
