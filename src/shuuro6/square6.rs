use std::{cmp::Ordering, fmt::Display};

use crate::{Color, Square};

const ASCII_1: u8 = 1;
const ASCII_LOWER_A: u8 = b'a';
const ASCII_LOWER_F: u8 = b'f';

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Square6 {
    inner: u8,
}

pub struct SquareIter {
    current: Square6,
}

impl Square6 {
    pub fn iter() -> SquareIter {
        SquareIter {
            current: Square6 { inner: 0 },
        }
    }

    pub fn incr(&mut self) {
        self.inner += 1;
    }

    pub const fn create(inner: u32) -> Option<Square6> {
        Some(Square6 { inner: inner as u8 })
    }
}

impl Iterator for SquareIter {
    type Item = Square6;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current.incr();
        let index = current.index();
        match 36.cmp(&index) {
            Ordering::Greater => Some(Square6 { inner: index as u8 }),
            _ => None,
        }
    }
}

impl Display for Square6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", (self.file() + ASCII_LOWER_A) as char, {
            self.rank() + ASCII_1
        })
    }
}

impl Square for Square6 {
    fn new(file: u8, rank: u8) -> Option<Self> {
        if file > 5 || rank > 5 {
            return None;
        }

        Some(Square6 {
            inner: rank * 6 + file,
        })
    }

    fn from_sfen(s: &str) -> Option<Self> {
        let bytes: &[u8] = s.as_bytes();
        let mut chars = s.chars();

        if bytes.len() > 2
            || s.is_empty()
            || bytes.is_empty()
            || bytes[0] < ASCII_LOWER_A
            || bytes[0] > ASCII_LOWER_F
        {
            return None;
        }
        let file = bytes[0] - ASCII_LOWER_A;
        let _first = chars.next().unwrap();
        let rank: String = chars.take(1).collect();

        let rank = rank.parse::<u8>();
        match rank {
            Ok(i) => {
                if i > 6 {
                    return None;
                }
                debug_assert!(
                    file < 7 && i < 7,
                    "{} parsed as (file: {}, rank: {})",
                    s,
                    file,
                    i - ASCII_1
                );

                Some(Square6 {
                    inner: (i - ASCII_1) * 6 + file,
                })
            }
            Err(_i) => None,
        }
    }

    fn from_index(index: u8) -> Option<Self> {
        if index >= 36 {
            return None;
        }

        Some(Square6 { inner: index })
    }

    fn right_edge(&self) -> u8 {
        6
    }

    fn up_edge(&self) -> u8 {
        6
    }

    fn to_int(&self) -> u8 {
        self.inner
    }

    fn in_promotion_zone(&self, c: Color) -> bool {
        match c {
            Color::White => self.rank() == 5,
            Color::Black => self.rank() == 0,
            Color::NoColor => false,
        }
    }

    fn rank(&self) -> u8 {
        self.inner / 6
    }

    fn file(&self) -> u8 {
        self.inner % 6
    }

    fn index(&self) -> usize {
        self.inner as usize
    }
}

pub mod consts {
    use super::Square6;

    macro_rules! make_square {
            {0, $t:ident $($ts:ident)+} => {
                /// Square generated from macro.
                pub const $t: Square6 = Square6 { inner: 0 };
                make_square!{1, $($ts)*}
            };
            {$n:expr, $t:ident $($ts:ident)+} => {
                /// Square6 generated from macro.
                pub const $t: Square6 = Square6 { inner: $n };
                make_square!{($n + 1), $($ts)*}
            };
            {$n:expr, $t:ident} => {
                /// Square6 generated from macro.
                pub const $t: Square6 = Square6 { inner: $n };
            };
        }
    make_square!(0, A1 B1 C1 D1 E1 F1
    A2 B2 C2 D2 E2 F2
    A3 B3 C3 D3 E3 F3
    A4 B4 C4 D4 E4 F4
    A5 B5 C5 D5 E5 F5
    A6 B6 C6 D6 E6 F6
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        for file in 0..6 {
            for rank in 0..6 {
                if let Some(sq) = Square6::new(file, rank) {
                    assert_eq!(file, sq.file());
                    assert_eq!(rank, sq.rank());
                }
            }
        }

        assert_eq!(None, Square6::new(6, 0));
        assert_eq!(None, Square6::new(0, 6));
        assert_eq!(None, Square6::new(13, 6));
    }

    #[test]
    fn from_sfen() {
        let ok_cases = [
            ("a6", 0, 5),
            ("a5", 0, 4),
            ("a1", 0, 0),
            ("e5", 4, 4),
            ("f6", 5, 5),
            ("f1", 5, 0),
        ];
        let ng_cases = ["", "s9", "_a", "a14", "9 ", " a", "9", "foo"];

        for case in ok_cases.iter() {
            if let Some(sq) = Square6::from_sfen(case.0) {
                assert_eq!(case.1, sq.file());
                assert_eq!(case.2, sq.rank());
            }
        }

        for case in ng_cases.iter() {
            assert!(
                Square6::from_sfen(case).is_none(),
                "{case} should cause an error"
            );
        }
    }

    #[test]
    fn from_index() {
        for i in 0..36 {
            assert!(Square6::from_index(i).is_some());
        }

        assert!(Square6::from_index(145).is_none());
    }

    #[test]
    fn to_sfen() {
        let cases = [
            ("a6", 0, 5),
            ("a1", 0, 0),
            ("e5", 4, 4),
            ("d4", 3, 3),
            ("f5", 5, 4),
        ];

        for case in cases.iter() {
            if let Some(sq) = Square6::new(case.1, case.2) {
                assert_eq!(case.0, sq.to_string());
            }
        }
    }
}
