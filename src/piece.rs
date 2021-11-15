use crate::{Color, PieceType, Square};
use std::fmt;

/// Represents a piece on the game board.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    /// Creates a new instance of `Piece` from SFEN formatted string.
    pub fn from_sfen(c: char) -> Option<Piece> {
        let color = if c.is_uppercase() {
            Color::Blue
        } else {
            Color::Red
        };

        PieceType::from_sfen(c).map(|piece_type| Piece { piece_type, color })
    }
    /// Returns an instance of `Piece` after promotion.
    ///
    /// # Examples
    ///
    /// ```
    /// use shuuro::{Color, PieceType, Piece};
    ///
    /// let pc1 = Piece{piece_type: PieceType::Pawn, color: Color::Blue};
    /// let pc2 = Piece{piece_type: PieceType::ProPawn, color: Color::Blue};
    ///
    /// assert_eq!(Some(pc2), pc1.promote());
    /// assert_eq!(None, pc2.promote());
    ///
    pub fn promote(self) -> Option<Piece> {
        self.piece_type.promote().map(|pt| Piece {
            piece_type: pt,
            color: self.color,
        })
    }

    /// Returns an instance of `Piece` before promotion.
    ///
    /// # Examples
    ///
    /// ```
    /// use shuuro::{Color, PieceType, Piece};
    ///
    /// let pc1 = Piece{piece_type: PieceType::Pawn, color: Color::Blue};
    /// let pc2 = Piece{piece_type: PieceType::ProPawn, color: Color::Blue};
    ///
    /// assert_eq!(Some(pc1), pc2.unpromote());
    /// assert_eq!(None, pc1.unpromote());
    /// ```
    pub fn unpromote(self) -> Option<Piece> {
        self.piece_type.unpromote().map(|pt| Piece {
            piece_type: pt,
            color: self.color,
        })
    }
    /// Returns an instance of `Piece` with the reversed color.
    ///
    /// # Examples
    ///
    /// ```
    /// use shuuro::{Color, PieceType, Piece};
    ///
    /// let pc1 = Piece{piece_type: PieceType::Pawn, color: Color::Blue};
    /// let pc2 = Piece{piece_type: PieceType::Pawn, color: Color::Red};
    ///
    /// assert_eq!(pc2, pc1.flip());
    /// assert_eq!(pc1, pc2.flip());
    /// ```
    pub fn flip(self) -> Piece {
        Piece {
            piece_type: self.piece_type,
            color: self.color.flip(),
        }
    }
    /// Tests if it is legal to place this piece at the given square.
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
