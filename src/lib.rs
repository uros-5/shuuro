////! A library for implementing Shuuro application.
////!
////! `shuuro` provides various types and implementations for representing concepts and rules in Shuuro.
////!
////! # Examples
////!
////! Shuuro shop - buying pieces:
////! ```
////! use shuuro::{Shop, PieceType, Piece, Color, Move};
////!
////! let mut shop = Shop::default();
////! for i in 0..5 {
////!     let piece = Piece{piece_type: PieceType::Queen, color: Color::Black};
////!     shop.play(Move::Buy { piece });
////! }
////! assert_ne!(shop.to_sfen(Color::Black), "kqqqq");
////! assert_eq!(shop.credit(Color::Black), 800 - 110 * 3);
////! ```
////!
////! Shuuro deploy - placing pieces on board:
////!
////! ```
////! use shuuro::{Position, PieceType, Color, consts::{D1, F12}, Piece, init};
////! init();
////! let mut pos = Position::default();
////! pos.set_hand("KQQNNBkrrrqnnPPP");
////!
////! let white_king = (Piece{ piece_type: PieceType::King, color: Color::White }, D1);
////! let black_king = (Piece{ piece_type: PieceType::King, color: Color::Black }, F12);
////! pos.place(white_king.0, white_king.1 );
////! pos.place(black_king.0, black_king.1);
////! assert_eq!(pos.generate_sfen(), "3K8/57/57/57/57/57/57/57/57/57/57/5k6 w q3r2n2QB2N3P 2");
////! ```
////!
////! Shuuro fight - play like normal chess:
////! ```
////! use shuuro::*;
////! use shuuro::consts::*;
////! init();
////! let mut pos = Position::default();
////! pos.set_sfen("1K2RR6/PPP9/57/57/57/57/57/57/L05L05/pppppp6/1k64/57 w - 0");
////! let move_ = Move::Normal {from: B1, to: A1, promote: false};
////! pos.make_move(move_);
////! // Move can be made also with: pos.play("b1", "a1");
////! assert_eq!(pos.generate_sfen(), "K3RR6/PPP9/57/57/57/57/57/57/L05L05/pppppp6/1k55/57 b - 1");
////!
////! ```

#![recursion_limit = "144"]
pub mod shuuro12;
pub mod shuuro_rules;
pub use shuuro_rules::*;
