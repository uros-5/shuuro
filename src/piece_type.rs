use std::{fmt, iter};

/// Represents a kind of pieces.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PieceType {
    King = 0,
    Queen = 1,
    Rook = 2,
    Bishop = 3,
    Knight = 4,
    Pawn = 5,
    Plinth = 6,
}

impl PieceType {
    /// Returns an iterator over all variants.
    pub fn iter() -> PieceTypeIter {
        PieceTypeIter::new()
    }

    /// Creates a new instance of `PieceType` from SFEN formatted string.
    pub fn from_sfen(c: char) -> Option<PieceType> {
        Some(match c {
            'k' | 'K' => PieceType::King,
            'q' | 'Q' => PieceType::Queen,
            'r' | 'R' => PieceType::Rook,
            'b' | 'B' => PieceType::Bishop,
            'n' | 'N' => PieceType::Knight,
            'p' | 'P' => PieceType::Pawn,
            'L' => PieceType::Plinth,
            _ => return None,
        })
    }

    /// Returns an instance of `PieceType` after promotion.
    ///
    /// # Examples
    ///
    /// ```
    /// use shuuro::PieceType;
    ///
    /// assert_eq!(Some(PieceType::Queen), PieceType::Pawn.promote());
    /// ```
    pub fn promote(self) -> Option<PieceType> {
        use self::PieceType::*;

        match self {
            Pawn => return Some(PieceType::Queen),
            _ => return None,
        }
    }

    /// Returns an instance of `PieceType` before promotion.
    ///
    /// # Examples
    ///
    /// ```
    /// use shuuro::PieceType;
    ///
    /// assert_eq!(Some(PieceType::Pawn), PieceType::Queen.unpromote());
    /// assert_eq!(None, PieceType::Pawn.unpromote());
    /// ```
    pub fn unpromote(self) -> Option<PieceType> {
        use self::PieceType::*;

        match self {
            Queen => return Some(Pawn),
            _ => return None,
        }
    }

    /// Checks if this piece type can be a part of hand pieces.
    pub fn is_hand_piece(self) -> bool {
        matches!(
            self,
            PieceType::Rook
                | PieceType::Bishop
                | PieceType::Queen
                | PieceType::King
                | PieceType::Knight
                | PieceType::Pawn
        )
    }

    /// Converts the instance into the unique number for array indexing purpose.
    pub fn index(self) -> usize {
        self as usize
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match *self {
                PieceType::Bishop => "b",
                PieceType::King => "k",
                PieceType::Knight => "n",
                PieceType::Pawn => "p",
                PieceType::Rook => "r",
                PieceType::Queen => "q",
                PieceType::Plinth => "L",
            }
        )
    }
}

pub struct PieceTypeIter {
    current: Option<PieceType>,
}

impl PieceTypeIter {
    pub fn new() -> PieceTypeIter {
        PieceTypeIter {
            current: Some(PieceType::King),
        }
    }
}

impl iter::Iterator for PieceTypeIter {
    type Item = PieceType;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        if let Some(current) = self.current {
            self.current = match current {
                PieceType::King => Some(PieceType::Queen),
                PieceType::Queen => Some(PieceType::Rook),
                PieceType::Rook => Some(PieceType::Bishop),
                PieceType::Bishop => Some(PieceType::Knight),
                PieceType::Knight => Some(PieceType::Pawn),
                PieceType::Pawn => Some(PieceType::Plinth),
                PieceType::Plinth => None,
            };
        }

        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_sfen() {
        let ok_cases = [
            ('k', PieceType::King),
            ('q', PieceType::Queen),
            ('r', PieceType::Rook),
            ('n', PieceType::Knight),
            ('b', PieceType::Bishop),
            ('p', PieceType::Pawn),
        ];
        let ng_cases = ['\0', ' ', '_', 'a', 'z', '+'];
        for case in ok_cases.iter() {
            assert_eq!(Some(case.1), PieceType::from_sfen(case.0));
            assert_eq!(
                Some(case.1),
                PieceType::from_sfen(case.0.to_uppercase().next().unwrap())
            );
        }

        for case in ng_cases.iter() {
            assert!(PieceType::from_sfen(*case).is_none());
        }
    }

    #[test]
    fn to_sfen() {
        let ok_cases = [
            ("k", PieceType::King),
            ("r", PieceType::Rook),
            ("b", PieceType::Bishop),
            ("n", PieceType::Knight),
            ("q", PieceType::Queen),
            ("p", PieceType::Pawn),
        ];

        for case in ok_cases.iter() {
            assert_eq!(case.0, case.1.to_string());
        }
    }

    #[test]
    fn promote() {
        let iterator = PieceTypeIter::new();
        for i in iterator {
            match i {
                PieceType::Pawn => assert_eq!(Some(PieceType::Queen), i.promote()),
                _ => assert!(i.promote().is_none()),
            }
        }
    }

    #[test]
    fn unpromote() {
        let iterator = PieceTypeIter::new();
        for i in iterator {
            match i {
                PieceType::Queen => assert_eq!(Some(PieceType::Pawn), i.unpromote()),
                _ => {
                    assert!(i.unpromote().is_none())
                }
            }
        }
    }
}
