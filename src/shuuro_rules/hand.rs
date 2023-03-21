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
                // || !pt.eq(&PieceType::King) {
                let piece = Piece {
                    piece_type: pt,
                    color: c,
                };
                let counter = self.get(piece);
                if long {
                    for _i in 0..counter {
                        sum.push_str(&piece.to_string());
                    }
                } else {
                    sum.push_str(&format!("{}{}", counter, &piece.to_string()));
                }
            }
        }
        sum
    }

    /// Set hand with all pieces from str.
    pub fn set_hand(&mut self, s: &str) {
        for i in s.chars() {
            if let Some(i) = Piece::from_sfen(i) {
                self.increment(i)
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
            PieceType::Chancellor => 6,
            PieceType::ArchBishop => 7,
            PieceType::Giraffe => 8,
            _ => return None,
        };
        let offset = if p.color == Color::Black { 0 } else { 9 };

        Some(base + offset)
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut hand = Hand::default();

        let mut num_pieces: u8 = 0;

        for c in value.chars() {
            match c {
                n if n.is_numeric() => {
                    if let Some(n) = n.to_digit(19) {
                        if n == 9 {
                            num_pieces = n as u8;
                            continue;
                        } else if num_pieces != 0 {
                            let num2 = format!("{}{}", num_pieces, n as u8)
                                .parse::<u8>()
                                .unwrap();
                            num_pieces = num2;
                            continue;
                        }
                        num_pieces = n as u8;
                    }
                }
                s => {
                    match Piece::from_sfen(s) {
                        Some(p) => hand.set(
                            p,
                            if num_pieces == 0 { 1 } else { num_pieces },
                        ),
                        None => return hand,
                    };
                    num_pieces = 0;
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
