use crate::Square;
use std::fmt;

/// Represents a move which either is a normal move or a drop move.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Move {
    Normal {
        from: Square,
        to: Square,
        promote: bool,
    },
}

impl Move {
    /// Creates a new instance of `Move` from SFEN formatted string.
    pub fn from_sfen(s: &str) -> Option<Move> {
        if s.len() != 4 && (s.len() != 5 || s.chars().nth(4).unwrap() != '+') {
            return None;
        }

        let first = s.chars().next().unwrap();
        if first.is_digit(10) {
            if let Some(from) = Square::from_sfen(&s[0..2]) {
                if let Some(to) = Square::from_sfen(&s[2..4]) {
                    let promote = s.len() == 5;

                    return Some(Move::Normal { from, to, promote });
                }
            }

            return None;
        }

        None
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Move::Normal { from, to, promote } => {
                write!(f, "{}{}{}", from, to, if promote { "+" } else { "" })
            }
            Move::Drop { to, piece_type } => {
                write!(f, "{}*{}", piece_type.to_string().to_uppercase(), to)
            }
        }
    }
}
