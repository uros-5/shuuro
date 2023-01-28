use crate::Square;

pub trait BitBoard {
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
}
