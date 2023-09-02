use crate::shuuro_rules::{Color, Piece, PieceType};

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
    inner: [u8; 18],
}

impl Hand {
    /// Returns a number of the given piece.
    pub fn get(&self, p: Piece) -> u8 {
        Hand::index(p).map(|i| self.inner[i]).unwrap_or(0)
    }

    /// Sets a number of the given piece.
    pub fn set(&mut self, p: Piece, num: u8) {
        if let Some(index) = Hand::index(p) {
            for _ in 0..num {
                self.inner[index] += 1;
            }
        }
    }

    /// Increments a number of the given piece.
    pub fn increment(&mut self, p: Piece) {
        if let Some(i) = Hand::index(p) {
            self.inner[i] += 1;
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
    pub fn to_sfen(&self, c: Color, long: bool) -> String {
        let mut sum = String::from("");
        for pt in PieceType::iter() {
            if !pt.eq(&PieceType::Plinth) {
                let piece = Piece {
                    piece_type: pt,
                    color: c,
                };
                let counter = self.get(piece);
                if long {
                    for _i in 0..counter {
                        sum.push_str(&piece.to_string());
                    }
                } else if counter > 0 {
                    sum.push_str(&format!("{}{}", counter, &piece.to_string()));
                }
            }
        }
        sum
    }

    /// Set hand with all pieces from str.
    pub fn set_hand(&mut self, s: &str) {
        let hand = Hand::from(s);
        self.inner = hand.inner;
    }

    fn index(p: Piece) -> Option<usize> {
        let base = match p.piece_type {
            PieceType::Plinth => return None,
            _ => p.piece_type.index(),
        };
        let offset = if p.color == Color::Black { 0 } else { 9 };

        Some(base + offset)
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Hand {
        let mut hand = Hand::default();
        let mut count = String::new();
        for ch in value.chars() {
            match ch {
                n if n.is_numeric() => {
                    count.push(n);
                }
                p => {
                    match Piece::from_sfen(p) {
                        Some(p) => {
                            let n = count.parse::<u8>();
                            if let Ok(n) = n {
                                hand.set(p, if n == 0 { 1 } else { n });
                            } else {
                                hand.set(p, 1);
                            }
                        }
                        None => return hand,
                    };
                    count.clear();
                }
            }
        }
        hand
    }
}

// impl ToString for Hand {
//     fn to_string(&self) -> String {
//         todo!()
//     }
// }
