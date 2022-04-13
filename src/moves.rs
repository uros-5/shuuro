use crate::{Piece, Square};
use std::fmt;

/// Represents a move which either is a normal move or a drop move.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Move {
    Buy {
        piece: Piece,
    },
    Put {
        to: Square,
        piece: Piece,
    },
    Normal {
        from: Square,
        to: Square,
        promote: bool,
    },
}

impl Move {
    /// Creates a new instance of `Move` from SFEN formatted string.
    pub fn from_sfen(s: &str) -> Option<Move> {
        if s.len() > 7 {
            return None;
        }

        if s.contains("_") {
            let mut fen_parts = s.split("_");
            if let Some(from) = fen_parts.next() {
                if let Some(from) = Square::from_sfen(from) {
                    if let Some(to) = fen_parts.next() {
                        if let Some(to) = Square::from_sfen(to) {
                            return Some(Move::Normal {
                                from,
                                to,
                                promote: false,
                            });
                        }
                    }
                }
            }

            return None;
        } else {
            let buy_move = Move::get_buy_move(&s);
            match buy_move {
                Some(m) => {
                    return Some(m);
                }
                None => {
                    return Move::get_put_move(&s);
                }
            }
        }
    }

    /// Information about normal move.
    pub fn info(&self) -> (Square, Square) {
        match self {
            Move::Normal {
                from,
                to,
                promote: _,
            } => (*from, *to),
            _ => (
                Square::from_index(0).unwrap(),
                Square::from_index(0).unwrap(),
            ),
        }
    }

    /// Creating new normal with 'from' and 'to' Square.
    pub fn new(from: Square, to: Square, promote: bool) -> Move {
        Move::Normal { from, to, promote }
    }

    /// Getting buy move from str.
    pub fn get_buy_move(s: &str) -> Option<Move> {
        if s.len() == 2 {
            if s.chars().nth(0).unwrap() == '+' {
                if let Some(piece) = Piece::from_sfen(s.chars().nth(1).unwrap()) {
                    return Some(Move::Buy { piece });
                }
            }
        }
        None
    }

    /// Getting put move from str.
    pub fn get_put_move(s: &str) -> Option<Move> {
        let mut fen_parts = s.split("@");
        if let Some(piece_str) = fen_parts.next() {
            if let Some(piece_char) = piece_str.chars().next() {
                if let Some(piece) = Piece::from_sfen(piece_char) {
                    if let Some(to) = fen_parts.next() {
                        if let Some(to) = Square::from_sfen(to) {
                            return Some(Move::Put { piece, to });
                        }
                    }
                }
            }
        }
        return None;
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Move::Buy { piece } => {
                write!(f, "+{}", piece)
            }
            Move::Put { to, piece } => {
                write!(f, "{}@{}", piece, to)
            }
            Move::Normal { from, to, promote } => {
                write!(f, "{}_{}{}", from, to, if promote { "" } else { "" })
            }
        }
    }
}

/// MoveRecord stores information necessary to undo the move.
#[derive(Debug, Clone)]
pub enum MoveRecord {
    Buy {
        piece: Piece,
    },
    Put {
        to: Square,
        piece: Piece,
    },
    Normal {
        from: Square,
        to: Square,
        placed: Piece,
        captured: Option<Piece>,
        promoted: bool,
    },
}

impl MoveRecord {
    /// Converts the move into SFEN formatted string.
    pub fn to_sfen(&self) -> String {
        match *self {
            MoveRecord::Buy { piece } => format!("+{}", piece),
            MoveRecord::Put { to, piece } => format!("{}@{}", piece, to),
            MoveRecord::Normal {
                from, to, promoted, ..
            } => format!("{}_{}{}", from, to, if promoted { "*" } else { "" }),
        }
    }
}

impl PartialEq<Move> for MoveRecord {
    fn eq(&self, other: &Move) -> bool {
        match (self, other) {
            (
                &MoveRecord::Normal {
                    from: f1,
                    to: t1,
                    promoted,
                    ..
                },
                &Move::Normal {
                    from: f2,
                    to: t2,
                    promote,
                },
            ) => f1 == f2 && t1 == t2 && promote == promoted,
            (&MoveRecord::Buy { piece: piece1 }, &Move::Buy { piece: piece2 }) => piece1 == piece2,
            (
                &MoveRecord::Put {
                    to: to1,
                    piece: piece1,
                },
                &Move::Put {
                    to: to2,
                    piece: piece2,
                },
            ) => to1 == to2 && piece1 == piece2,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{square::consts::*, Color, PieceType};

    #[test]
    fn from_sfen() {
        let ok_cases = [
            (
                "a10_b10",
                Move::Normal {
                    from: A10,
                    to: B10,
                    promote: false,
                },
            ),
            (
                "a9_a1",
                Move::Normal {
                    from: A9,
                    to: A1,
                    promote: false,
                },
            ),
            (
                "b4_j12",
                Move::Normal {
                    from: B4,
                    to: J12,
                    promote: false,
                },
            ),
        ];

        for (i, case) in ok_cases.iter().enumerate() {
            let m = Move::from_sfen(case.0);
            assert!(m.is_some(), "failed at #{}", i);
            assert_eq!(case.1, m.unwrap(), "failed at #{}", i);
        }
    }

    #[test]
    fn from_sfen_buy() {
        let ok_cases = [
            (
                "+n",
                Piece {
                    piece_type: PieceType::Knight,
                    color: Color::Black,
                },
            ),
            (
                "+P",
                Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::White,
                },
            ),
            (
                "+r",
                Piece {
                    piece_type: PieceType::Rook,
                    color: Color::Black,
                },
            ),
        ];
        for (i, case) in ok_cases.iter().enumerate() {
            let m = Move::from_sfen(case.0);
            assert!(m.is_some(), "failed at #{}", i);
            assert_eq!(Move::Buy { piece: case.1 }, m.unwrap(), "failed at #{}", i);
        }

        let ng_cases = ["++", "+1", "+@", "+NN", "+LL", "+*"];

        for (i, case) in ng_cases.iter().enumerate() {
            let m = Move::from_sfen(case);
            assert!(m.is_none(), "failed at #{}", i);
        }
    }

    #[test]
    fn from_sfen_place() {
        let ok_cases = [
            ("R@b1", PieceType::Rook, Color::White, B1, false),
            ("n@d12", PieceType::Knight, Color::Black, D12, false),
            ("P@g3", PieceType::Pawn, Color::White, G3, false),
        ];

        for (i, case) in ok_cases.iter().enumerate() {
            let m = Move::from_sfen(case.0);
            assert!(!m.is_none(), "failed at #{}", i);
            assert_eq!(
                Move::Put {
                    to: case.3,
                    piece: Piece {
                        piece_type: case.1,
                        color: case.2
                    }
                },
                m.unwrap(),
                "failed at #{}",
                i
            );
        }

        let ng_cases = [("S@b4", B4), ("V@b4", B4), ("s@b55", J12)];

        for (i, case) in ng_cases.iter().enumerate() {
            let m = Move::from_sfen(case.0);
            assert!(!m.is_some(), "failed at #{}", i);
        }
    }

    #[test]
    fn to_sfen() {
        let cases = [
            (
                "c7_e9",
                Move::Normal {
                    from: C7,
                    to: E9,
                    promote: false,
                },
            ),
            (
                "f9_j5",
                Move::Normal {
                    from: F9,
                    to: J5,
                    promote: false,
                },
            ),
        ];

        for (i, case) in cases.iter().enumerate() {
            assert_eq!(case.1.to_string(), case.0, "failed at #{}", i);
        }
    }
}
