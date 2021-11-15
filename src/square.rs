use crate::color::Color;
use std::{fmt, iter};

const ASCII_1: u8 = b'1';
const ASCII_12: u8 = 12;
const ASCII_LOWER_A: u8 = b'a';
const ASCII_LOWER_L: u8 = b'l';

/// Represents a position of each cell in the game board.
///
/// # Examples
///
/// ```
/// use shuuro::Square;
///
/// let sq = Square::new(4, 4).unwrap();
/// assert_eq!("5e", sq.to_string());
/// ```
///
/// `Square` can be created by parsing a SFEN formatted string as well.
///
/// ```
/// use shuuro::Square;
///
/// let sq = Square::from_sfen("5e").unwrap();
/// assert_eq!(4, sq.file());
/// assert_eq!(4, sq.rank());
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Square {
    inner: u8,
}

impl Square {
    /// Creates a new instance of `Square`.
    ///
    /// `file` can take a value from 0('1') to 11('12'), while `rank` is from 0('a') to 12('l').
    pub fn new(file: u8, rank: u8) -> Option<Square> {
        if file > 11 || rank > 11 {
            return None;
        }

        Some(Square {
            inner: rank * 12 + file,
        })
    }
    /// Creates a new instance of `Square` from SFEN formatted string.
    pub fn from_sfen(s: &str) -> Option<Square> {
        let bytes: &[u8] = s.as_bytes();

        if bytes.len() != 2
            || bytes[0] < ASCII_LOWER_A
            || bytes[0] > ASCII_LOWER_L
            || bytes[1] < ASCII_1
            || bytes[1] > ASCII_12
        {
            return None;
        }

        let file = bytes[0] - ASCII_LOWER_A;
        let rank = bytes[1] - ASCII_1;

        debug_assert!(
            file < 11 && rank < 11,
            "{} parsed as (file: {}, rank: {})",
            s,
            file,
            rank
        );

        Some(Square {
            inner: rank * 12 + file,
        })
    }
    /// Creates a new instance of `Square` with the given index value.
    pub fn from_index(index: u8) -> Option<Square> {
        if index >= 144 {
            return None;
        }

        Some(Square { inner: index })
    }



    /// Returns an iterator of all variants.
    pub fn iter() -> SquareIter {
        SquareIter { current: 0 }
    }

    /// Returns a rank of the square.
    pub fn rank(self) -> u8 {
        self.inner / 12
    }

    /// Returns a file of the square.
    pub fn file(self) -> u8 {
        self.inner % 12
    }

    /// Returns a new `Square` instance by moving the file and the rank values.
    ///
    /// # Examples
    ///
    /// ```
    /// use shuuro::square::consts::*;
    ///
    /// let sq = SQ_2B;
    /// let shifted = sq.shift(2, 3).unwrap();
    ///
    /// assert_eq!(3, shifted.file());
    /// assert_eq!(4, shifted.rank());
    /// ```
    pub fn shift(self, df: i16, dr: i16) -> Option<Square> {
        let f = self.file() as i16 + df;
        let r = self.rank() as i16 + dr;

        if !(0..12).contains(&f) || !(0..12).contains(&r) {
            return None;
        }

        Some(Square {
            inner: (r * 12 + f) as u8,
        })
    }

    /// Returns a relative rank as if the specified color is Blue.
    ///
    /// # Examples
    ///
    /// ```
    /// use shuuro::Color;
    /// use shuuro::square::consts::*;
    ///
    /// let sq = SQ_1G;
    ///
    /// assert_eq!(6, sq.relative_file(Color::Blue));
    /// assert_eq!(2, sq.relative_file(Color::Red));
    /// ```
    pub fn relative_file(self, c: Color) -> u8 {
        if c == Color::Blue {
            self.file()
        } else {
            11 - self.file()
        }
    }

    /// Converts the instance into the unique number for array indexing purpose.
    pub fn index(self) -> usize {
        self.inner as usize
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        debug_assert!(
            self.file() < 12 && self.rank() < 12,
            "trying to stringify an invalid square: {:?}",
            self
        );
        write!(
            f,
            "{}{}",
            (self.file() + ASCII_1) as char,
            (self.rank() + ASCII_LOWER_A) as char
        )
    }
}

pub mod consts {
    /*
        macro_rules! make_square {
            {0, $t:ident $($ts:ident)+} => {
                pub const $t: Square = Square { inner: 0 };
                make_square!{1, $($ts)*}
            };
            {$n:expr, $t:ident $($ts:ident)+} => {
                pub const $t: Square = Square { inner: $n };
                make_square!{($n + 1), $($ts)*}
            };
            {$n:expr, $t:ident} => {
                pub const $t: Square = Square { inner: $n };
            };
        }
        make_square!(0, A1 B1 C1 D1 E1 F1 G1 H1 I1 J1 K1 L1
        A2 B2 C2 D2 E2 F2 G2 H2 I2 J2 K2 L2
        A3 B3 C3 D3 E3 F3 G3 H3 I3 J3 K3 L3
        A4 B4 C4 D4 E4 F4 G4 H4 I4 J4 K4 L4
        A5 B5 C5 D5 E5 F5 G5 H5 I5 J5 K5 L5
        A6 B6 C6 D6 E6 F6 G6 H6 I6 J6 K6 L6
        A7 B7 C7 D7 E7 F7 G7 H7 I7 J7 K7 L7
        A8 B8 C8 D8 E8 F8 G8 H8 I8 J8 K8 L8
        A9 B9 C9 D9 E9 F9 G9 H9 I9 J9 K9 L9
        A10 B10 C10 D10 E10 F10 G10 H10 I10 J10 K10 L10
        A11 B11 C11 D11 E11 F11 G11 H11 I11 J11 K11 L11
        A12 B12 C12 D12 E12 F12 G12 H12 I12 J12 K12 L12
        );
    */
}

pub struct SquareIter {
    current: u8,
}

impl iter::Iterator for SquareIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.current;

        if cur >= 144 {
            return None;
        }

        self.current += 1;

        Some(Square { inner: cur })
    }
}
