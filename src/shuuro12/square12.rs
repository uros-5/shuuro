use std::{cmp::Ordering, fmt::Display};

use crate::{Color, Square};

const ASCII_1: u8 = 1;
const _ASCII_12: u8 = 12;
const ASCII_LOWER_A: u8 = b'a';
const ASCII_LOWER_L: u8 = b'l';

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Square12 {
    inner: u8,
}

pub struct SquareIter {
    current: Square12,
}

impl Square12 {
    pub fn iter() -> SquareIter {
        SquareIter {
            current: Square12 { inner: 0 },
        }
    }

    pub fn incr(&mut self) {
        self.inner += 1;
    }
}

impl Iterator for SquareIter {
    type Item = Square12;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current.incr();
        let index = current.index();
        match 144.cmp(&index) {
            Ordering::Greater => Some(Square12 { inner: index as u8 }),
            _ => None,
        }
    }
}

impl Display for Square12 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", (self.file() + ASCII_LOWER_A) as char, {
            self.rank() + ASCII_1
        })
    }
}

impl Square for Square12 {
    fn new(file: u8, rank: u8) -> Option<Self> {
        if file > 11 || rank > 11 {
            return None;
        }

        Some(Square12 {
            inner: rank * 12 + file,
        })
    }

    fn from_sfen(s: &str) -> Option<Self> {
        let bytes: &[u8] = s.as_bytes();
        let mut chars = s.chars();

        if bytes.len() > 3
            || s.is_empty()
            || bytes.is_empty()
            || bytes[0] < ASCII_LOWER_A
            || bytes[0] > ASCII_LOWER_L
        //|| bytes[1] < ASCII_1
        //|| bytes[1] > ASCII_12
        {
            return None;
        }
        let file = bytes[0] - ASCII_LOWER_A;
        let _first = chars.next().unwrap();
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

                Some(Square12 {
                    inner: (i - ASCII_1) * 12 + file,
                })
            }
            Err(_i) => None,
        }
    }

    fn from_index(index: u8) -> Option<Self> {
        if index >= 144 {
            return None;
        }

        Some(Square12 { inner: index })
    }

    fn right_edge(&self) -> u8 {
        11
    }

    fn up_edge(&self) -> u8 {
        11
    }

    fn to_int(&self) -> u8 {
        self.inner
    }

    fn in_promotion_zone(&self, c: crate::Color) -> bool {
        match c {
            Color::White => self.rank() == 11,
            Color::Black => self.rank() == 0,
            Color::NoColor => false,
        }
    }

    fn rank(&self) -> u8 {
        self.inner / 12
    }

    fn file(&self) -> u8 {
        self.inner % 12
    }

    fn index(&self) -> usize {
        self.inner as usize
    }
}

pub mod consts {
    use super::Square12;

    macro_rules! make_square {
            {0, $t:ident $($ts:ident)+} => {
                /// Square generated from macro.
                pub const $t: Square12 = Square12 { inner: 0 };
                make_square!{1, $($ts)*}
            };
            {$n:expr, $t:ident $($ts:ident)+} => {
                /// Square12 generated from macro.
                pub const $t: Square12 = Square12 { inner: $n };
                make_square!{($n + 1), $($ts)*}
            };
            {$n:expr, $t:ident} => {
                /// Square12 generated from macro.
                pub const $t: Square12 = Square12 { inner: $n };
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
