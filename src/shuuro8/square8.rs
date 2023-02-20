use std::{cmp::Ordering, fmt::Display};

use crate::{Color, Square};

const ASCII_1: u8 = 1;
const ASCII_LOWER_A: u8 = b'a';
const ASCII_LOWER_H: u8 = b'h';

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Square8 {
    inner: u8,
}

pub struct SquareIter {
    current: Square8,
}

impl Square8 {
    pub fn iter() -> SquareIter {
        SquareIter {
            current: Square8 { inner: 0 },
        }
    }

    pub fn incr(&mut self) {
        self.inner += 1;
    }
}

impl Iterator for SquareIter {
    type Item = Square8;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current.incr();
        let index = current.index();
        match 144.cmp(&index) {
            Ordering::Greater => Some(Square8 { inner: index as u8 }),
            _ => None,
        }
    }
}

impl Display for Square8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", (self.file() + ASCII_LOWER_A) as char, {
            self.rank() + ASCII_1
        })
    }
}

impl Square for Square8 {
    fn new(file: u8, rank: u8) -> Option<Self> {
        if file > 7 || rank > 7 {
            return None;
        }

        Some(Square8 {
            inner: rank * 8 + file,
        })
    }

    fn from_sfen(s: &str) -> Option<Self> {
        let bytes: &[u8] = s.as_bytes();
        let mut chars = s.chars();

        if bytes.len() > 2
            || s.is_empty()
            || bytes.is_empty()
            || bytes[0] < ASCII_LOWER_A
            || bytes[0] > ASCII_LOWER_H
        {
            return None;
        }
        let file = bytes[0] - ASCII_LOWER_A;
        let _first = chars.next().unwrap();
        let rank: String = chars.take(1).collect();

        let rank = rank.parse::<u8>();
        match rank {
            Ok(i) => {
                if i > 8 {
                    return None;
                }
                debug_assert!(
                    file < 9 && i < 9,
                    "{} parsed as (file: {}, rank: {})",
                    s,
                    file,
                    i - ASCII_1
                );

                Some(Square8 {
                    inner: (i - ASCII_1) * 8 + file,
                })
            }
            Err(_i) => None,
        }
    }

    fn from_index(index: u8) -> Option<Self> {
        if index >= 64 {
            return None;
        }

        Some(Square8 { inner: index })
    }

    fn right_edge(&self) -> u8 {
        7
    }

    fn up_edge(&self) -> u8 {
        7
    }

    fn to_int(&self) -> u8 {
        self.inner
    }

    fn in_promotion_zone(&self, c: crate::Color) -> bool {
        match c {
            Color::White => self.rank() == 7,
            Color::Black => self.rank() == 0,
            Color::NoColor => false,
        }
    }

    fn rank(&self) -> u8 {
        self.inner / 8
    }

    fn file(&self) -> u8 {
        self.inner % 8
    }

    fn index(&self) -> usize {
        self.inner as usize
    }
}

pub mod consts {
    use super::Square8;

    macro_rules! make_square {
            {0, $t:ident $($ts:ident)+} => {
                /// Square generated from macro.
                pub const $t: Square8 = Square8 { inner: 0 };
                make_square!{1, $($ts)*}
            };
            {$n:expr, $t:ident $($ts:ident)+} => {
                /// Square8 generated from macro.
                pub const $t: Square8 = Square8 { inner: $n };
                make_square!{($n + 1), $($ts)*}
            };
            {$n:expr, $t:ident} => {
                /// Square8 generated from macro.
                pub const $t: Square8 = Square8 { inner: $n };
            };
        }
    make_square!(0, A1 B1 C1 D1 E1 F1 G1 H1
    A2 B2 C2 D2 E2 F2 G2 H2
    A3 B3 C3 D3 E3 F3 G3 H3
    A4 B4 C4 D4 E4 F4 G4 H4
    A5 B5 C5 D5 E5 F5 G5 H5
    A6 B6 C6 D6 E6 F6 G6 H6
    A7 B7 C7 D7 E7 F7 G7 H7
    A8 B8 C8 D8 E8 F8 G8 H8
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        for file in 0..8 {
            for rank in 0..8 {
                let sq = Square8::new(file, rank).unwrap();
                assert_eq!(file, sq.file());
                assert_eq!(rank, sq.rank());
            }
        }

        assert_eq!(None, Square8::new(8, 0));
        assert_eq!(None, Square8::new(0, 8));
        assert_eq!(None, Square8::new(13, 8));
    }

    #[test]
    fn from_sfen() {
        let ok_cases = [
            ("a8", 0, 7),
            ("a5", 0, 4),
            ("a1", 0, 0),
            ("e5", 4, 4),
            ("h8", 7, 7),
            ("h1", 7, 0),
        ];
        let ng_cases = ["", "s9", "_a", "a14", "9 ", " a", "9", "foo"];

        for case in ok_cases.iter() {
            let sq = Square8::from_sfen(case.0);
            assert!(sq.is_some());
            assert_eq!(case.1, sq.unwrap().file());
            assert_eq!(case.2, sq.unwrap().rank());
        }

        for case in ng_cases.iter() {
            assert!(
                Square8::from_sfen(case).is_none(),
                "{case} should cause an error"
            );
        }
    }

    #[test]
    fn from_index() {
        for i in 0..64 {
            assert!(Square8::from_index(i).is_some());
        }

        assert!(Square8::from_index(145).is_none());
    }

    #[test]
    fn to_sfen() {
        let cases = [
            ("a7", 0, 6),
            ("a1", 0, 0),
            ("e5", 4, 4),
            ("d4", 3, 3),
            ("f7", 5, 6),
        ];

        for case in cases.iter() {
            let sq = Square8::new(case.1, case.2).unwrap();
            assert_eq!(case.0, sq.to_string());
        }
    }
}
