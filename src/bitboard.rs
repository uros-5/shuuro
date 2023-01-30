use std::{
    fmt::Debug,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

use crate::Square;

pub trait BitBoard<S: Square>:
    Sized
    + Clone
    + Copy
    + Debug
    + Not
    + Default
    + for<'a> BitAnd<&'a Self, Output = Self>
    + for<'a> BitOr<Self, Output = Self>
    + for<'a> BitAndAssign<&'a Self>
    + for<'a> BitOrAssign<&'a Self>
    + for<'a> BitXor<&'a Self, Output = Self>
    + for<'a> BitXorAssign<&'a Self>
    + BitAndAssign<S>
    + BitOr<S>
    + BitOrAssign<S>
    + BitXor<S>
    + BitXorAssign<S>
{
    fn empty(&self) -> Self;
    fn is_any(&self) -> Self;
    fn is_empty(&self) -> Self;
    fn clear_at(&mut self);
    fn clear_all(&mut self);
    fn count(&self) -> u32;
    fn set_all(&mut self);
    fn pop<T: Square>(&mut self) -> Option<T>;
    fn pop_reverse<T: Square>(&mut self) -> Option<T>;
    fn merged(&self) -> u16;
    fn from_square<T: Square>(sq: T) -> Self;
    fn toggle(&mut self);
}
