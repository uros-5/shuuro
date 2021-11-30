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
        if s.len() > 7 {
            return None;
        }
        let mut fen_parts = s.split("_");

        if let Some(from) = Square::from_sfen(fen_parts.next().unwrap()) {
            if let Some(to) = Square::from_sfen(fen_parts.next().unwrap()) {
                return Some(Move::Normal {
                    from,
                    to,
                    promote: false,
                });
            }
        }
        None
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Move::Normal { from, to, promote } => {
                write!(f, "{}{}{}", from, to, if promote { "" } else { "" })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::square::consts::*;

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
    fn to_sfen() {
        let cases = [
            (
                "c7e9",
                Move::Normal {
                    from: C7,
                    to: E9,
                    promote: false,
                },
            ),
            (
                "f9j5",
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
