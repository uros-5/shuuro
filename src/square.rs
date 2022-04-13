use crate::color::Color;
use std::{fmt, iter};

const ASCII_1: u8 = 1;
const ASCII_12: u8 = 12;
const ASCII_LOWER_A: u8 = b'a';
const ASCII_LOWER_L: u8 = b'l';

/*

const ASCII_8: u8 = 8;
const ASCII_LOWER_H: u8 = b'h';

const ASCII_6: u8 = 6;
const ASCII_LOWER_F: u8 = b'f';
*/

/// Represents a position of each cell in the game board.
///
/// # Examples
///
/// ```
/// use shuuro::Square;
///
/// let sq = Square::new(4, 4).unwrap();
/// assert_eq!("e5", sq.to_string());
/// ```
///
/// `Square` can be created by parsing a SFEN formatted string as well.
///
/// ```
/// use shuuro::Square;
///
/// let sq = Square::from_sfen("e5").unwrap();
/// assert_eq!(4, sq.file());
/// assert_eq!(4, sq.rank());
/// ```
#[derive(Debug, Eq, Clone, Copy, Hash)]
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
        let mut chars = s.chars();

        if bytes.len() > 3
            || s.len() == 0
            || bytes.len() == 0
            || bytes[0] < ASCII_LOWER_A
            || bytes[0] > ASCII_LOWER_L
        //|| bytes[1] < ASCII_1
        //|| bytes[1] > ASCII_12
        {
            return None;
        }
        //println!("{} , {}", bytes[0], ASCII_LOWER_A);
        let file = bytes[0] - ASCII_LOWER_A;
        let _first = chars.nth(0).unwrap();
        let rank: String = chars.take(2).collect();

        let rank = rank.parse::<u8>();
        match rank {
            Ok(i) => {
                if i > 12 {
                    return None;
                }
                debug_assert!(
                    file < 13 && i < 13,
                    "{} parsed as (file: {}, rank: {})",
                    s,
                    file,
                    i - ASCII_1
                );

                Some(Square {
                    inner: (i - ASCII_1) * 12 + file,
                })
            }
            Err(_i) => None,
        }

        //let rank = bytes[1] - ASCII_1;
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
    /// let sq = B2;
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

    /// Returns a relative rank as if the specified color is Black.
    pub fn relative_file(self, c: Color) -> u8 {
        if c == Color::Black {
            self.file()
        } else {
            11 - self.file()
        }
    }

    /// Tests if the square is in a promotion zone.
    pub fn in_promotion_zone(self, c: Color) -> bool {
        match c {
            Color::White => {
                if self.rank() == 11 {
                    true
                } else {
                    false
                }
            }
            Color::Black => {
                if self.rank() == 0 {
                    true
                } else {
                    false
                }
            }
            Color::NoColor => false,
        }
    }

    /// Converts the instance into the unique number for array indexing purpose.
    pub fn index(self) -> usize {
        self.inner as usize
    }
}

impl PartialEq for Square {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
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
            (self.file() + ASCII_LOWER_A) as char,
            (self.rank() + ASCII_1) as u8
        )
    }
}

pub mod consts {
    use crate::Square;

    macro_rules! make_square {
            {0, $t:ident $($ts:ident)+} => {
                /// Square generated from macro.
                pub const $t: Square = Square { inner: 0 };
                make_square!{1, $($ts)*}
            };
            {$n:expr, $t:ident $($ts:ident)+} => {
                /// Square generated from macro.
                pub const $t: Square = Square { inner: $n };
                make_square!{($n + 1), $($ts)*}
            };
            {$n:expr, $t:ident} => {
                /// Square generated from macro.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        for file in 0..12 {
            for rank in 0..12 {
                let sq = Square::new(file, rank).unwrap();
                assert_eq!(file, sq.file());
                assert_eq!(rank, sq.rank());
            }
        }

        assert_eq!(None, Square::new(12, 0));
        assert_eq!(None, Square::new(0, 12));
        assert_eq!(None, Square::new(13, 12));
    }

    #[test]
    fn from_sfen() {
        let ok_cases = [
            ("a9", 0, 8),
            ("a11", 0, 10),
            ("a1", 0, 0),
            ("e5", 4, 4),
            ("i9", 8, 8),
            ("i1", 8, 0),
        ];
        let ng_cases = ["", "s9", "_a", "a14", "9 ", " a", "9", "foo"];

        for case in ok_cases.iter() {
            let sq = Square::from_sfen(case.0);
            assert!(sq.is_some());
            assert_eq!(case.1, sq.unwrap().file());
            assert_eq!(case.2, sq.unwrap().rank());
        }

        for case in ng_cases.iter() {
            assert!(
                Square::from_sfen(case).is_none(),
                "{} should cause an error",
                case
            );
        }
    }

    #[test]
    fn from_index() {
        for i in 0..144 {
            assert!(Square::from_index(i).is_some());
        }

        assert!(Square::from_index(145).is_none());
    }

    #[test]
    fn to_sfen() {
        let cases = [
            ("a9", 0, 8),
            ("a1", 0, 0),
            ("e5", 4, 4),
            ("i9", 8, 8),
            ("i1", 8, 0),
        ];

        for case in cases.iter() {
            let sq = Square::new(case.1, case.2).unwrap();
            assert_eq!(case.0, sq.to_string());
        }
    }
}
