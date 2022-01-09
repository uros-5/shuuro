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
//! Shuuro deploy - placing pieces on board.
//!
//! ```
//! use shuuro::{Position, PieceType, Color, consts::{D1, F12}, Piece, init};
//! init();
//! let mut pos = Position::default();
//! pos.set_hand("KQQNNBkrrrqnnPPP");
//! pos.place(Piece{ piece_type: PieceType::King, color: Color::Red }, D1);
//! pos.place(Piece{ piece_type: PieceType::King, color: Color::Blue }, F12);
//! assert_eq!(pos.generate_sfen(), "3K8/57/57/57/57/57/57/57/57/57/57/5k6 r q3r2n2QB2N3P 1");
//! ```
//!
//! Shuuro fight - play like normal chess.
//! ```
//! use shuuro::{Position, init};
//! init();
//! let mut pos = Position::default();
//! pos.set_sfen("1K2RR6/PPP9/57/57/57/57/57/57/L05L05/pppppp6/1k64/57 r - 1");
//! pos.play("b1", "a1");
//! assert_eq!(pos.generate_sfen(), "K3RR6/PPP9/57/57/57/57/57/57/L05L05/pppppp6/1k55/57 b - 2");
//!
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
pub mod plinths_set;
pub mod position;
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
pub use plinths_set::generate_plinths;
pub use position::{MoveRecord, PieceGrid, Position};
pub use shop::Shop;
