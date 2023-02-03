use std::fmt;

use crate::shuuro_rules::Color;

#[macro_export]
macro_rules! temp_moves {
    ($f1: ident, $f2: ident, $self: ident) => {
        if let Some(sq) = $self.$f1() {
            if let Some(sq2) = sq.$f1() {
                if let Some(sq3) = sq2.$f2() {
                    return Some([sq, sq2, sq3]);
                } else {
                    return None;
                }
            } else {
                None
            }
        } else {
            None
        }
    };

    ($f1: ident, $f2: ident, $self: ident, $a: expr) => {
        if let Some(sq) = $self.$f1() {
            if let Some(sq2) = sq.$f1() {
                return Some([sq, sq2]);
            } else {
                return None;
            }
        } else {
            return None;
        }
    };
}

pub trait Square: Sized + Eq + fmt::Display + PartialEq + Clone + Copy {
    fn new(file: u8, rank: u8) -> Option<Self>;
    fn make_square(rank: Option<Self>, file: u8) -> Option<Self>;
    fn from_sfen(s: &str) -> Option<Self>;
    fn from_index(index: u8) -> Option<Self>;
    fn left(&self) -> Option<Self> {
        let file = self.file();
        if file == 0 {
            None
        } else {
            Self::new(self.rank(), file - 1)
        }
    }
    fn right(&self) -> Option<Self> {
        let file = self.file();
        if file == self.right_edge() {
            None
        } else {
            Self::new(self.rank(), file + 1)
        }
    }
    fn up(&self) -> Option<Self> {
        let rank = self.rank();
        if rank == self.up_edge() {
            None
        } else {
            Self::new(rank, self.file())
        }
    }
    fn down(&self) -> Option<Self> {
        let rank = self.rank();
        if rank == 0 {
            None
        } else {
            Self::new(rank - 1, self.file())
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
    fn left_up(&self) -> Option<[Self; 3]> {
        temp_moves!(left, up, self)
    }
    fn up_left(&self) -> Option<[Self; 3]> {
        temp_moves!(up, left, self)
    }
    fn up_right(&self) -> Option<[Self; 3]> {
        temp_moves!(up, right, self)
    }
    fn right_up(&self) -> Option<[Self; 3]> {
        temp_moves!(right, up, self)
    }
    fn left_down(&self) -> Option<[Self; 3]> {
        temp_moves!(left, down, self)
    }
    fn down_left(&self) -> Option<[Self; 3]> {
        temp_moves!(down, left, self)
    }
    fn down_right(&self) -> Option<[Self; 3]> {
        temp_moves!(down, right, self)
    }
    fn right_down(&self) -> Option<[Self; 3]> {
        temp_moves!(right, down, self)
    }
    fn nw(&self) -> Option<[Self; 2]> {
        temp_moves!(up, left, self, 1)
    }
    fn ne(&self) -> Option<[Self; 2]> {
        temp_moves!(up, right, self, 1)
    }
    fn sw(&self) -> Option<[Self; 2]> {
        temp_moves!(down, left, self, 1)
    }
    fn se(&self) -> Option<[Self; 2]> {
        temp_moves!(down, right, self, 1)
    }
    fn knight(&self) -> Vec<Self> {
        let mut all = vec![];
        let temp = [
            self.left_up(),
            self.up_left(),
            self.up_right(),
            self.right_up(),
            self.left_down(),
            self.down_left(),
            self.down_right(),
            self.right_down(),
        ];
        for i in temp {
            if let Some(squares) = i {
                for j in squares {
                    all.push(j);
                }
            }
        }
        all
    }
    fn right_edge(&self) -> u8;
    fn up_edge(&self) -> u8;
    fn to_int(&self) -> u8;
    fn in_promotion_zone(&self, c: Color) -> bool;
    fn rank(&self) -> u8;
    fn file(&self) -> u8;
    fn index(&self) -> usize;
}
