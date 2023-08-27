use crate::shuuro_rules::{Color, PieceType};
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
                Color::White
            }
        } else if c == 'l' {
            Color::NoColor
        } else {
            Color::Black
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
    /// let pc1 = Piece{piece_type: PieceType::Pawn, color: Color::Black};
    /// let pc2 = Piece{piece_type: PieceType::Queen, color: Color::Black};
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
    /// let pc1 = Piece{piece_type: PieceType::Pawn, color: Color::Black};
    /// let pc2 = Piece{piece_type: PieceType::Queen, color: Color::Black};
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
    /// let pc1 = Piece{piece_type: PieceType::Pawn, color: Color::Black};
    /// let pc2 = Piece{piece_type: PieceType::Pawn, color: Color::White};
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
    // Tests if it is legal to place this piece at the given square.
    // pub fn is_placeable_at(self, sq: Square) -> bool {
    //     match self.piece_type {
    //         PieceType::Pawn => sq.relative_file(self.color) > 0,
    //         PieceType::Knight => sq.relative_file(self.color) > 1,
    //         _ => true,
    //     }
    // }

    fn index(&self) -> usize {
        self.piece_type.index() + 10
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.color == Color::White {
            write!(f, "{}", self.piece_type.to_string().to_uppercase())
        } else {
            write!(f, "{}", self.piece_type)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::shuuro_rules::piece_type::PieceTypeIter;

    #[test]
    fn from_sfen() {
        let ok_cases = [
            ('k', PieceType::King, Color::Black),
            ('r', PieceType::Rook, Color::Black),
            ('b', PieceType::Bishop, Color::Black),
            ('n', PieceType::Knight, Color::Black),
            ('p', PieceType::Pawn, Color::Black),
            ('q', PieceType::Queen, Color::Black),
            ('K', PieceType::King, Color::White),
            ('R', PieceType::Rook, Color::White),
            ('B', PieceType::Bishop, Color::White),
            ('N', PieceType::Knight, Color::White),
            ('P', PieceType::Pawn, Color::White),
            ('Q', PieceType::Queen, Color::White),
            ('L', PieceType::Plinth, Color::NoColor),
        ];
        let ng_cases = ['\0', ' ', '_', 'j', 'z', '+', 'J', 'Z'];

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
                color: Color::White,
            };
            let bpc = Piece {
                piece_type: case.1,
                color: Color::Black,
            };
            assert_eq!(case.0.to_uppercase(), rpc.to_string());
            assert_eq!(case.0, bpc.to_string());
        }
        assert_eq!(
            "L",
            Piece {
                piece_type: PieceType::Plinth,
                color: Color::NoColor
            }
            .to_string()
        );
    }

    #[test]
    fn promote() {
        let iterator = PieceTypeIter::default();
        for i in iterator {
            match i {
                PieceType::Pawn => {
                    let bpc = Piece {
                        piece_type: i,
                        color: Color::Black,
                    }
                    .promote()
                    .unwrap();
                    assert_eq!(
                        Piece {
                            piece_type: PieceType::Queen,
                            color: Color::Black
                        },
                        bpc
                    );

                    let rpc = Piece {
                        piece_type: i,
                        color: Color::White,
                    }
                    .promote()
                    .unwrap();
                    assert_eq!(
                        Piece {
                            piece_type: PieceType::Queen,
                            color: Color::White
                        },
                        rpc
                    );
                }
                _ => {
                    assert!(Piece {
                        piece_type: i,
                        color: Color::White
                    }
                    .promote()
                    .is_none());
                    assert!(Piece {
                        piece_type: i,
                        color: Color::Black
                    }
                    .promote()
                    .is_none());
                }
            }
        }
    }

    #[test]
    fn unpromote() {
        let iterator = PieceTypeIter::default();
        for i in iterator {
            match i {
                PieceType::Queen => {
                    assert_eq!(
                        Some(Piece {
                            piece_type: PieceType::Pawn,
                            color: Color::Black
                        }),
                        Piece {
                            piece_type: i,
                            color: Color::Black
                        }
                        .unpromote()
                    )
                }
                _ => {
                    assert!(Piece {
                        piece_type: i,
                        color: Color::Black
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
            color: Color::White,
        };
        let bpc = Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
        };

        assert_eq!(Color::Black, rpc.flip().color);
        assert_eq!(Color::White, bpc.flip().color);
    }
}
