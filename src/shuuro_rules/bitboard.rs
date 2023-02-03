use std::{
    fmt::Debug,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

use crate::shuuro_rules::Square;

pub trait BitBoard<S: Square>:
    Sized
    + Clone
    + Copy
    + Debug
    + Not
    + Default
    + for<'b> BitOr<&'b Self, Output = Self>
    + for<'a> BitAnd<&'a Self, Output = Self>
    + for<'a> BitAndAssign<&'a Self>
    + for<'a> BitOrAssign<&'a Self>
    + for<'a> BitXor<&'a Self, Output = Self>
    + for<'a> BitXorAssign<&'a Self>
    + BitAndAssign<S>
    + BitOr<S>
    + BitOrAssign<S>
    + BitXor<S>
    + BitXorAssign<S>
    + Iterator<Item = S>
// where
//     for<'a> &'a Self: BitOr<&'a Self, Output = Self>,
//     for<'a> &'a Self: Not<Output = Self>,
{
    fn empty() -> Self;
    fn is_any(&self) -> bool;
    fn is_empty(&self) -> bool;
    fn clear_at(&mut self);
    fn clear_all(&mut self);
    fn count(&self) -> u32;
    fn set_all(&mut self);
    fn pop(&mut self) -> Option<S>;
    fn pop_reverse(&mut self) -> Option<S>;
    fn merged(&self) -> u16;
    fn from_square(sq: &S) -> Self;
    fn toggle(&mut self);
}
