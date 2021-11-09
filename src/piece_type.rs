use std::{fmt, iter};

use crate::piece::Piece;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PieceType {
    King = 0,
    Queen = 1,
    Rook = 2,
    Bishop = 3,
    Knight = 4,
    Pawn = 5,
    Plynth = 6,
}

impl PieceType {
    pub fn iter() -> PieceTypeIter {
        PieceTypeIter::new()
    }

    pub fn from_sfen(c: char) -> Option<PieceType> {
        Some(match c {
            'k' | 'K' => PieceType::King,
            'q' | 'Q' => PieceType::Queen,
            'r' | 'R' => PieceType::Rook,
            'b' | 'B' => PieceType::Bishop,
            'n' | 'N' => PieceType::Knight,
            'p' | 'P' => PieceType::Pawn,
            'L' => PieceType::Plynth,
            _ => return None,
        })
    }

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn promote(self) -> Option<PieceType> {
        use self::PieceType::Plynth;

        match self {
            Plynth => return None,
            _ => return Some(PieceType::Queen),
        }
    }

    pub fn unpromote(self) -> Option<PieceType> {
        use self::PieceType::Plynth;

        match self {
            Plynth => return None,
            _ => return Some(self),
        }
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
                PieceType::Plynth => "l",
            }
        )
    }
}

pub struct PieceTypeIter {
    current: Option<PieceType>,
}

impl PieceTypeIter {
    fn new() -> PieceTypeIter {
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
                PieceType::Pawn => Some(PieceType::Plynth),
                PieceType::Plynth => None,
            };
        }

        current
    }
}
