use std::fmt;

use crate::shuuro_rules::Color;

#[macro_export]
macro_rules! knight {
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
}

pub trait Square: Sized + fmt::Display + PartialEq + Clone + Copy {
    fn new(file: u8, rank: u8) -> Option<Self>;
    fn make_square(rank: Option<Self>, file: u8) -> Option<Self>;
    fn from_sfen(s: &str) -> Option<Self>;
    fn from_index(index: u8) -> Option<Self>;
    fn left(&self) -> Option<Self>;
    fn right(&self) -> Option<Self>;
    fn up(&self) -> Option<Self>;
    fn down(&self) -> Option<Self>;
    fn upward(&self) -> Option<Self>;
    fn backward(&self) -> Option<Self>;
    fn left_up(&self) -> Option<[Self; 3]> {
        knight!(left, up, self)
    }
    fn up_left(&self) -> Option<[Self; 3]> {
        knight!(up, left, self)
    }
    fn up_right(&self) -> Option<[Self; 3]> {
        knight!(up, right, self)
    }
    fn right_up(&self) -> Option<[Self; 3]> {
        knight!(right, up, self)
    }
    fn left_down(&self) -> Option<[Self; 3]> {
        knight!(left, down, self)
    }
    fn down_left(&self) -> Option<[Self; 3]> {
        knight!(down, left, self)
    }
    fn down_right(&self) -> Option<[Self; 3]> {
        knight!(down, right, self)
    }
    fn right_down(&self) -> Option<[Self; 3]> {
        knight!(right, down, self)
    }
    fn diagonal_left_up(&self) -> Option<Self>;
    fn diagonal_right_up(&self) -> Option<Self>;
    fn diagonal_left_down(&self) -> Option<Self>;
    fn diagonal_right_down(&self) -> Option<Self>;
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
    fn to_int(&self) -> u8;
    fn in_promotion_zone(&self, c: Color) -> bool;
    fn rank(&self) -> u8;
    fn file(&self) -> u8;
    fn index(&self) -> usize;
}
