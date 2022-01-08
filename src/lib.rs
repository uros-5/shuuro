//! A library for implementing Shuuro application.
//!
//! `shuuro` provides various types and implementations for representing concepts and rules in Shuuro.
//!
//! # Examples
//!
//! Shuuro shop - buying pieces.
//! ```
//! use shuuro::{Shop, PieceType, Piece, Color};
//!
//! let mut shop = Shop::default();
//! for i in 0..5 {
//!     shop.buy(Piece{piece_type: PieceType::Queen, color: Color::Blue});
//! }
//! assert_ne!(shop.to_sfen(Color::Blue), "kqqqq");
//! assert_eq!(shop.credit(Color::Blue), 800 - 110 * 3);
//! ```
//!
//! Shuuro set - placing pieces on board.
//!
//! ```
//! use shuuro::{PositionSet};
//!
//! let mut p = PositionSet::default();
//! ```
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
pub mod plynths_set;
pub mod position;
pub mod position_set;
pub mod sfen_str;
pub mod shop;
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
pub use plynths_set::generate_plynths;
pub use position::{MoveRecord, PieceGrid, Position};
pub use position_set::PositionSet;
pub use sfen_str::SfenStr;
pub use shop::Shop;
