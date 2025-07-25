use crate::{
    color::Color,
    shuuro_rules::{Piece, Square},
};
use std::fmt;

/// Represents a move which either is a select move, drop move or normal move.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Move<S: Square> {
    Select { piece: Piece },
    Put { to: S, piece: Piece },
    Normal { from: S, to: S, placed: Piece },
}

impl<S: Square> Move<S> {
    /// Creating new normal with 'from' and 'to' Square.
    pub fn new(from: S, to: S) -> Self {
        Self::Normal {
            from,
            to,
            placed: Piece {
                piece_type: crate::PieceType::Rook,
                color: Color::Black,
            },
        }
    }

    /// Creates a new instance of `Self` from SFEN formatted string.
    pub fn from_sfen(s: &str) -> Option<Self> {
        if s.len() > 7 {
            return None;
        }
        let select_move = Self::parse_select_move(s);
        if !select_move.is_none() {
            return select_move;
        }

        let place_move = Self::parse_place_move(s);
        if !place_move.is_none() {
            return place_move;
        }

        let normal_move = Self::get_normal_move(s);
        if !normal_move.is_none() {
            return normal_move;
        }

        None
    }

    /// Information about normal move.
    pub fn info(&self) -> Option<(S, S)> {
        match self {
            Self::Normal { from, to, .. } => Some((*from, *to)),
            _ => None,
        }
    }

    /// Getting select move from str.
    pub fn parse_select_move(s: &str) -> Option<Self> {
        if s.len() == 2 && s.starts_with('+') {
            if let Some(piece) = Piece::from_sfen(s.chars().nth(1).unwrap()) {
                return Some(Self::Select { piece });
            }
        }
        None
    }

    /// Getting put move from str.
    pub fn parse_place_move(s: &str) -> Option<Self> {
        let mut place_move = s.split("@");
        let piece = place_move.next()?.chars().next()?;
        let piece = Piece::from_sfen(piece)?;
        let to = place_move.next()?;
        let to = S::from_sfen(to)?;
        Some(Self::Put { piece, to })
    }

    /// Getting normal move from str.
    pub fn get_normal_move(s: &str) -> Option<Self> {
        let mut fen_parts = s.split('_');
        let from = fen_parts.next()?;
        let from = Square::from_sfen(from)?;
        let to = fen_parts.next()?;
        let to = Square::from_sfen(to)?;
        Some(Self::new(from, to))
    }

    pub fn to_fen(&self) -> String {
        match &self {
            Move::Put { piece, to } => format!("{}@{}", piece, to),
            Move::Normal { from, to, .. } => {
                format!("{}_{}", from, to)
            }
            Move::Select { piece } => format!("+{}", piece),
        }
    }

    pub fn format(&self, from: &S, to: &S, move_data: MoveData) -> String {
        let piece = move_data.piece.unwrap().to_string().to_uppercase();
        let move_to = to.to_string();

        let action = {
            if move_data.checkmate {
                "#"
            } else if move_data.check {
                "+"
            } else {
                ""
            }
        };
        let piece = {
            if piece == "P" {
                String::from("")
            } else {
                piece
            }
        };
        let promote = {
            if move_data.promoted && piece.is_empty() {
                "=Q"
            } else {
                ""
            }
        };

        let same = {
            if piece.is_empty() {
                piece.to_string()
            } else if move_data.same_rank && move_data.same_file {
                let file = self.same_format(&from, 0, false);
                let rank = self.same_format(&from, 1, true);
                format!("{file}{rank}")
            } else if move_data.same_file {
                self.same_format(&from, 1, true)
            } else if move_data.same_rank {
                from.to_string().chars().next().unwrap().to_string()
            } else {
                "".to_string()
            }
        };

        let captures = {
            if move_data.captured.is_some() {
                if piece.is_empty() {
                    format!("{}x", self.same_format(&from, 0, false))
                } else {
                    "x".to_string()
                }
            } else {
                "".to_string()
            }
        };

        return format!(
            "{}{}{}{}{}{}",
            piece, same, captures, move_to, promote, action
        );
    }

    fn same_format(&self, from: &S, skip: usize, is_numeric: bool) -> String {
        let c = |x: &char| -> bool {
            if is_numeric {
                x.is_numeric()
            } else {
                !x.is_numeric()
            }
        };
        let rank = from.to_string();
        let rank = rank.chars().skip(skip).take_while(c);
        rank.collect()
    }
}

impl<S: Square> TryFrom<String> for Move<S> {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.contains('@') {
            if let Some(m) = value.split('_').next() {
                if let Some(m) = Self::parse_place_move(m) {
                    return Ok(m);
                }
            }
        } else if value.contains('-') {
            let mut parts = value.split(' ');
            for _i in 0..3 {
                parts.next();
            }
            if let Some(m) = parts.next() {
                if let Some(m) = Self::get_normal_move(m) {
                    return Ok(m);
                }
            }
        } else if let Some(m) = Self::parse_select_move(&value) {
            return Ok(m);
        }
        Err(())
    }
}

impl<S: Square> fmt::Display for Move<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Move::Select { piece } => {
                write!(f, "+{piece}")
            }
            Move::Put { to, piece, .. } => {
                write!(f, "{piece}@{to}")
            }
            Move::Normal { from, to, .. } => {
                write!(f, "{}_{}", from, to,)
            }
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct MoveData {
    check: bool,
    checkmate: bool,
    same_file: bool,
    same_rank: bool,
    captured: Option<Piece>,
    piece: Option<Piece>,
    promoted: bool,
}

impl MoveData {
    pub fn checks(mut self, check: bool, checkmate: bool) -> Self {
        self.check = check;
        self.checkmate = checkmate;
        self
    }

    pub fn precise(mut self, same_file: bool, same_rank: bool) -> Self {
        self.same_file = same_file;
        self.same_rank = same_rank;
        self
    }

    pub fn captured(mut self, captured: Option<Piece>) -> Self {
        self.captured = captured;
        self
    }

    pub fn promoted(mut self, promoted: bool) -> Self {
        self.promoted = promoted;
        self
    }

    pub fn piece(mut self, piece: Option<Piece>) -> Self {
        self.piece = piece;
        self
    }
}
