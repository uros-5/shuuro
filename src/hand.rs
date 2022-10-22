use crate::{Color, Piece, PieceType};

/// Manages the number of each pieces in each player's hand.
///
/// # Examples
///
/// ```
/// use shuuro::{Color, Hand, Piece, PieceType};
///
/// let mut hand: Hand = Default::default();
/// let blue_pawn = Piece{piece_type: PieceType::Pawn, color: Color::Black};
/// let red_pawn = Piece{piece_type: PieceType::Pawn, color: Color::White};
///
/// hand.set(blue_pawn, 2);
/// hand.increment(blue_pawn);
/// assert_eq!(3, hand.get(blue_pawn));
/// assert_eq!(0, hand.get(red_pawn));
/// ```

#[derive(Debug, Clone, Default)]
pub struct Hand {
    inner: [u8; 16],
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

    /// Converts hand to sfen.
    pub fn to_sfen(&self, c: Color) -> String {
        let mut sum = String::from("");
        for pt in PieceType::iter() {
            if !pt.eq(&PieceType::Plinth) {
                // || !pt.eq(&PieceType::King) {
                let piece = Piece {
                    piece_type: pt,
                    color: c,
                };
                let counter = self.get(piece);
                for _i in 0..counter {
                    sum.push_str(&format!("{}", piece.to_string()));
                }
            }
        }
        sum
    }

    /// Set hand with all pieces from str.
    pub fn set_hand(&mut self, s: &str) {
        for i in s.chars() {
            match Piece::from_sfen(i) {
                Some(i) => self.increment(i),
                None => (),
            }
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
            PieceType::ArchRook => 6,
            PieceType::ArchBishop => 7,
            _ => return None,
        };
        let offset = if p.color == Color::Black { 0 } else { 8 };

        Some(base + offset)
    }
}
