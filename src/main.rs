#![recursion_limit = "144"]
pub mod bitboard;
pub mod board_defs;
pub mod color;
pub mod piece_type;
pub mod square;
pub mod attacks;

pub use self::bitboard::SQUARE_BB;
pub use self::color::Color;
pub use self::piece_type::PieceType;
pub use self::square::Square;
pub use board_defs::{EMPTY_BB, FILE_BB, RANK_BB};
pub use attacks::init;

fn main() {
    init();
}
