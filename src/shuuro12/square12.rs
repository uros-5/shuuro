use std::fmt::Display;

use crate::{Color, Square};

const ASCII_1: u8 = 1;
const _ASCII_12: u8 = 12;
const ASCII_LOWER_A: u8 = b'a';
const ASCII_LOWER_L: u8 = b'l';

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Square12 {
    inner: u8,
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
