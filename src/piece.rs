use crate::{Color, PieceType, Square};
use std::fmt;

/// Represents a piece on the game board.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn from_sfen(c: char) -> Option<Piece> {
        let color = if c.is_uppercase() {
            Color::Blue
        } else {
            Color::Red
        };

        PieceType::from_sfen(c).map(|piece_type| Piece { piece_type, color })
    }

    pub fn promote(self) -> Option<Piece> {
        self.piece_type.promote().map(|pt| Piece {
            piece_type: pt,
            color: self.color,
        })
    }

    pub fn unpromote(self) -> Option<Piece> {
        self.piece_type.unpromote().map(|pt| Piece {
            piece_type: pt,
            color: self.color,
        })
    }

    pub fn flip(self) -> Piece {
        Piece {
            piece_type: self.piece_type,
            color: self.color.flip(),
        }
    }

    pub fn is_placeable_at(self, sq: Square) -> bool {
        match self.piece_type {
            PieceType::Pawn => sq.relative_file(self.color) > 0,
            PieceType::Knight => sq.relative_file(self.color) > 1,
            _ => true,
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.color == Color::Blue {
            write!(f, "{}", self.piece_type.to_string().to_uppercase())
        } else {
            write!(f, "{}", self.piece_type)
        }
    }
}
