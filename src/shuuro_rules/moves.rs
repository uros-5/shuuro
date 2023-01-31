use crate::shuuro_rules::{Piece, Square};
use std::fmt;

/// Represents a move which either is a normal move or a drop move.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Move<S: Square> {
    Buy { piece: Piece },
    Put { to: S, piece: Piece },
    Normal { from: S, to: S, promote: bool },
}

impl<S: Square> Move<S> {
    /// Creates a new instance of `Self` from SFEN formatted string.
    pub fn from_sfen(s: &str) -> Option<Self> {
        if s.len() > 7 {
            return None;
        }

        if s.contains("_") {
            Self::get_normal_move(&s)
        } else {
            let buy_move = Self::get_buy_move(&s);
            match buy_move {
                Some(m) => Some(m),
                None => Self::get_put_move(&s),
            }
        }
    }

    /// Information about normal move.
    pub fn info(&self) -> (S, S) {
        match self {
            Self::Normal {
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
    pub fn new(from: S, to: S, promote: bool) -> Self {
        Self::Normal { from, to, promote }
    }

    /// Getting buy move from str.
    pub fn get_buy_move(s: &str) -> Option<Self> {
        if s.len() == 2 && s.chars().nth(0).unwrap() == '+' {
            if let Some(piece) = Piece::from_sfen(s.chars().nth(1).unwrap()) {
                return Some(Self::Buy { piece });
            }
        }
        None
    }

    /// Getting put move from str.
    pub fn get_put_move(s: &str) -> Option<Self> {
        let mut fen_parts = s.split("@");
        if let Some(piece_str) = fen_parts.next() {
            if let Some(piece_char) = piece_str.chars().next() {
                if let Some(piece) = Piece::from_sfen(piece_char) {
                    if let Some(to) = fen_parts.next() {
                        if let Some(to) = Square::from_sfen(to) {
                            return Some(Self::Put { piece, to });
                        }
                    }
                }
            }
        }
        return None;
    }

    /// Getting normal move from str.
    pub fn get_normal_move(s: &str) -> Option<Self> {
        let mut fen_parts = s.split("_");
        if let Some(from) = fen_parts.next() {
            if let Some(from) = Square::from_sfen(from) {
                if let Some(to) = fen_parts.next() {
                    if let Some(to) = Square::from_sfen(to) {
                        return Some(Self::Normal {
                            from,
                            to,
                            promote: false,
                        });
                    }
                }
            }
        }

        return None;
    }
}

impl<S: Square> fmt::Display for Move<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Move::Buy { piece } => {
                write!(f, "+{}", piece)
            }
            Move::Put { to, piece } => {
                write!(f, "{}@{}", piece, to)
            }
            Move::Normal { from, to, promote } => {
                write!(f, "{}_{}{}", from, to, if promote { "fixthis" } else { "" })
            }
        }
    }
}

/// MoveRecord stores information necessary to undo the move.
#[derive(Debug, Clone)]
pub enum MoveRecord<S: Square> {
    Buy {
        piece: Piece,
    },
    Put {
        to: S,
        piece: Piece,
    },
    Normal {
        from: S,
        to: S,
        placed: Piece,
        captured: Option<Piece>,
        promoted: bool,
    },
}

impl<S: Square> MoveRecord<S> {
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

impl<S: Square> PartialEq<Move<S>> for MoveRecord<S> {
    fn eq(&self, other: &Move<S>) -> bool {
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
