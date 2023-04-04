use std::fmt::{self, Debug};

use crate::shuuro_rules::Color;

#[macro_export]
macro_rules! l_moves {
    ($f1: ident, $f2: ident, $self: ident, $c: expr) => {
        if let Some(sq) = $self.$f1() {
            if let Some(sq2) = sq.$f1() {
                if $c == true {
                    if let Some(sq3) = sq2.$f2() {
                        return Some(sq3);
                    } else {
                        return None;
                    }
                } else {
                    if let Some(sq3) = sq2.$f1() {
                        if let Some(sq4) = sq3.$f1() {
                            if let Some(sq5) = sq4.$f2() {
                                return Some(sq5);
                            } else {
                                return None;
                            }
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    };
}

#[macro_export]
macro_rules! x {
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

#[macro_export]
macro_rules! giraffe {
    ($f1: ident, $f2: ident, $self: ident) => {
        if let Some(sq) = $self.$f1() {
            if let Some(sq2) = sq.$f1() {
                if let Some(sq3) = sq2.$f1() {
                    if let Some(sq4) = sq3.$f1() {
                        if let Some(sq5) = sq4.$f2() {
                            return Some(sq5);
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
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
    fn upward(&self, c: &Color) -> Option<Self> {
        match c {
            Color::White => self.up(),
            Color::Black => self.down(),
            Color::NoColor => None,
        }
    }
    fn backward(&self, c: &Color) -> Option<Self> {
        match c {
            Color::White => self.down(),
            Color::Black => self.up(),
            Color::NoColor => None,
        }
    }
    fn left_up(&self, c: bool) -> Option<Self> {
        l_moves!(left, up, self, c)
    }
    fn up_left(&self, c: bool) -> Option<Self> {
        l_moves!(up, left, self, c)
    }
    fn up_right(&self, c: bool) -> Option<Self> {
        l_moves!(up, right, self, c)
    }
    fn right_up(&self, c: bool) -> Option<Self> {
        l_moves!(right, up, self, c)
    }
    fn left_down(&self, c: bool) -> Option<Self> {
        l_moves!(left, down, self, c)
    }
    fn down_left(&self, c: bool) -> Option<Self> {
        l_moves!(down, left, self, c)
    }
    fn down_right(&self, c: bool) -> Option<Self> {
        l_moves!(down, right, self, c)
    }
    fn right_down(&self, c: bool) -> Option<Self> {
        l_moves!(right, down, self, c)
    }
    fn nw(&self) -> Option<Self> {
        x!(up, left, self, 1)
    }
    fn nea(&self) -> Option<Self> {
        x!(up, right, self, 1)
    }
    fn sw(&self) -> Option<Self> {
        x!(down, left, self, 1)
    }
    fn se(&self) -> Option<Self> {
        x!(down, right, self, 1)
    }
    fn l_moves(&self, c: bool) -> [Option<Self>; 8] {
        let mut all: [Option<Self>; 8] = [None; 8];
        let temp = [
            self.left_up(c),
            self.up_left(c),
            self.up_right(c),
            self.right_up(c),
            self.left_down(c),
            self.down_left(c),
            self.down_right(c),
            self.right_down(c),
        ];
        for (i, square) in temp.into_iter().flatten().enumerate() {
            all[i] = Some(square);
        }
        all
    }
    fn knight(&self) -> [Option<Self>; 8] {
        self.l_moves(true)
    }
    fn giraffe(&self) -> [Option<Self>; 8] {
        self.l_moves(false)
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
