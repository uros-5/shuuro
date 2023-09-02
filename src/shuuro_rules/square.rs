use std::fmt::{self, Debug};

use crate::shuuro_rules::Color;

pub trait Square:
    Sized + Eq + fmt::Display + Default + PartialEq + Clone + Copy
where
    Self: Debug,
{
    fn new(file: u8, rank: u8) -> Option<Self>;
    fn from_sfen(s: &str) -> Option<Self>;
    fn from_index(index: u8) -> Option<Self>;
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
