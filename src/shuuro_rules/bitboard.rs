use std::{
    fmt::{Debug, Display},
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
    },
};

use crate::shuuro_rules::Square;

pub trait BitBoard<S: Square>:
    Sized
    + Clone
    + Copy
    + Debug
    + Not
    + Default
    + Display
    + for<'a> BitAndAssign<&'a Self>
    + for<'a> BitOrAssign<&'a Self>
    + for<'a> BitXor<&'a Self, Output = Self>
    + for<'a> BitXorAssign<&'a Self>
    + for<'a> BitOr<&'a Self, Output = Self>
    + for<'a> BitAnd<&'a Self, Output = Self>
    + for<'a> Not<Output = Self>
    + for<'a> BitOr<&'a S, Output = Self>
    + for<'a> BitAnd<&'a S, Output = Self>
    + for<'a> BitOrAssign<&'a S>
    + for<'a> PartialEq<Self>
    + Iterator<Item = S>
{
    fn empty() -> Self;
    fn full() -> Self;
    fn is_any(&self) -> bool;
    fn is_empty(&self) -> bool;
    fn clear_at(&mut self, sq: S);
    fn clear_all(&mut self);
    fn len(&self) -> u32;
    fn set_all(&mut self);
    fn pop(&mut self) -> Option<S>;
    fn pop_reverse(&mut self) -> Option<S>;
    fn from_square(sq: &S) -> Self;
}
