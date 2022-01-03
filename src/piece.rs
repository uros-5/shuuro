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
            if c == 'L' {
                Color::NoColor
            } else {
                Color::Red
            }
        } else {
            Color::Blue
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
    /// let pc2 = Piece{piece_type: PieceType::Queen, color: Color::Blue};
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
    /// let pc2 = Piece{piece_type: PieceType::Queen, color: Color::Blue};
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
        if self.color == Color::Red {
            write!(f, "{}", self.piece_type.to_string().to_uppercase())
        } else {
            write!(f, "{}", self.piece_type)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece_type::PieceTypeIter;

    #[test]
    fn from_sfen() {
        let ok_cases = [
            ('k', PieceType::King, Color::Blue),
            ('r', PieceType::Rook, Color::Blue),
            ('b', PieceType::Bishop, Color::Blue),
            ('n', PieceType::Knight, Color::Blue),
            ('p', PieceType::Pawn, Color::Blue),
            ('q', PieceType::Queen, Color::Blue),
            ('K', PieceType::King, Color::Red),
            ('R', PieceType::Rook, Color::Red),
            ('B', PieceType::Bishop, Color::Red),
            ('N', PieceType::Knight, Color::Red),
            ('P', PieceType::Pawn, Color::Red),
            ('Q', PieceType::Queen, Color::Red),
            ('L', PieceType::Plynth, Color::NoColor),
        ];
        let ng_cases = ['\0', ' ', '_', 'a', 'z', '+', 'A', 'Z'];

        for case in ok_cases.iter() {
            let pc = Piece::from_sfen(case.0);
            assert!(pc.is_some());
            assert_eq!(case.1, pc.unwrap().piece_type);
            assert_eq!(case.2, pc.unwrap().color);
        }

        for case in ng_cases.iter() {
            assert!(Piece::from_sfen(*case).is_none());
        }
    }

    #[test]
    fn to_sfen() {
        let ok_cases = [
            ("k", PieceType::King),
            ("r", PieceType::Rook),
            ("b", PieceType::Bishop),
            ("n", PieceType::Knight),
            ("p", PieceType::Pawn),
            ("q", PieceType::Queen),
        ];

        for case in ok_cases.iter() {
            let rpc = Piece {
                piece_type: case.1,
                color: Color::Red,
            };
            let bpc = Piece {
                piece_type: case.1,
                color: Color::Blue,
            };
            assert_eq!(case.0.to_uppercase(), rpc.to_string());
            assert_eq!(case.0, bpc.to_string());
        }
        assert_eq!(
            "L",
            Piece {
                piece_type: PieceType::Plynth,
                color: Color::NoColor
            }
            .to_string()
        );
    }

    #[test]
    fn promote() {
        let iterator = PieceTypeIter::new();
        for i in iterator {
            match i {
                PieceType::Pawn => {
                    let bpc = Piece {
                        piece_type: i,
                        color: Color::Blue,
                    }
                    .promote()
                    .unwrap();
                    assert_eq!(
                        Piece {
                            piece_type: PieceType::Queen,
                            color: Color::Blue
                        },
                        bpc
                    );

                    let rpc = Piece {
                        piece_type: i,
                        color: Color::Red,
                    }
                    .promote()
                    .unwrap();
                    assert_eq!(
                        Piece {
                            piece_type: PieceType::Queen,
                            color: Color::Red
                        },
                        rpc
                    );
                }
                _ => {
                    assert!(Piece {
                        piece_type: i,
                        color: Color::Red
                    }
                    .promote()
                    .is_none());
                    assert!(Piece {
                        piece_type: i,
                        color: Color::Blue
                    }
                    .promote()
                    .is_none());
                }
            }
        }
    }

    #[test]
    fn unpromote() {
        let iterator = PieceTypeIter::new();
        for i in iterator {
            match i {
                PieceType::Queen => {
                    assert_eq!(
                        Some(Piece {
                            piece_type: PieceType::Pawn,
                            color: Color::Blue
                        }),
                        Piece {
                            piece_type: i,
                            color: Color::Blue
                        }
                        .unpromote()
                    )
                }
                _ => {
                    assert!(Piece {
                        piece_type: i,
                        color: Color::Blue
                    }
                    .unpromote()
                    .is_none())
                }
            }
        }
    }

    #[test]
    fn flip() {
        let rpc = Piece {
            piece_type: PieceType::Pawn,
            color: Color::Red,
        };
        let bpc = Piece {
            piece_type: PieceType::Pawn,
            color: Color::Blue,
        };

        assert_eq!(Color::Blue, rpc.flip().color);
        assert_eq!(Color::Red, bpc.flip().color);
    }
}
