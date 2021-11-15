#![recursion_limit = "144"]
pub mod attacks;
pub mod bitboard;
pub mod board;
pub mod board_defs;
pub mod color;
pub mod error;
pub mod hand;
pub mod moves;
pub mod piece;
pub mod piece_type;
pub mod position;
pub mod square;

use std::collections::HashMap;

pub use self::bitboard::{BitBoard, SQUARE_BB};
pub use self::color::{Color, ColorIter};
pub use self::piece::Piece;
pub use self::piece_type::PieceType;
pub use self::square::Square;
pub use attacks::{between, get_non_sliding_attacks, get_sliding_attacks, init};
pub use board::Board;
pub use board_defs::{EMPTY_BB, FILE_BB, RANK_BB};
pub use error::*;
pub use hand::Hand;
pub use moves::Move;
pub use position::{MoveRecord, Position};

fn main() {
    init();
}
