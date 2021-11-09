use crate::{Color, Piece, PieceType};

#[derive(Debug, Default)]
pub struct Hand {
    inner: [u8; 14],
}

impl Hand {
    /// Returns a number of the given piece.
    pub fn get(&self, p: Piece) -> u8 {
        Hand::index(p).map(|i| self.inner[i]).unwrap_or(0)
    }

    /// Sets a number of the given piece.
    pub fn set(&mut self, p: Piece, num: u8) {
        if let Some(i) = Hand::index(p) {
            self.inner[i] = num;
        }
    }

    /// Increments a number of the given piece.
    pub fn increment(&mut self, p: Piece) {
        if let Some(i) = Hand::index(p) {
            self.inner[i] += 1
        }
    }

    /// Decrements a number of the given piece.
    pub fn decrement(&mut self, p: Piece) {
        if let Some(i) = Hand::index(p) {
            self.inner[i] -= 1
        }
    }

    /// Clears all pieces.
    pub fn clear(&mut self) {
        for i in 0..self.inner.len() {
            self.inner[i] = 0;
        }
    }

    fn index(p: Piece) -> Option<usize> {
        let base = match p.piece_type {
            PieceType::King => 0,
            PieceType::Queen => 1,
            PieceType::Rook => 2,
            PieceType::Bishop => 3,
            PieceType::Knight => 4,
            PieceType::Pawn => 5,
            _ => return None,
        };
        let offset = if p.color == Color::Blue { 0 } else { 10 };

        Some(base + offset)
    }
}
