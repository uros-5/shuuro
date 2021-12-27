//! A library for implementing Shuuro application.
//!
//! `shuuro` provides various types and implementations for representing concepts and rules in Shuuro.
#![recursion_limit = "144"]
pub mod attacks;
pub mod bitboard;
pub mod board_defs;
pub mod color;
pub mod error;
pub mod hand;
pub mod moves;
pub mod piece;
pub mod piece_type;
pub mod position;
pub mod square;

pub use self::bitboard::{square_bb, BitBoard, SQUARE_BB};
pub use self::color::{Color, ColorIter};
pub use self::piece::Piece;
pub use self::piece_type::PieceType;
pub use self::square::{consts, Square};
pub use attacks::{between, get_non_sliding_attacks, get_sliding_attacks, init};
pub use board_defs::{EMPTY_BB, FILE_BB, RANK_BB};
pub use error::*;
pub use hand::Hand;
pub use moves::Move;
pub use position::{MoveRecord, Position};
