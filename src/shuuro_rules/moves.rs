use crate::shuuro_rules::{Piece, Square};
use std::fmt;

/// Represents a move which either is a normal move or a drop move.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Move<S: Square> {
    Buy { piece: Piece },
    Put { to: S, piece: Piece },
    Normal { from: S, to: S, promote: bool },
}

impl<S: Square> Move<S> {
    /// Creates a new instance of `Self` from SFEN formatted string.
    pub fn from_sfen(s: &str) -> Option<Self> {
        if s.len() > 7 {
            return None;
        }

        if s.contains('_') {
            Self::get_normal_move(s)
        } else {
            let buy_move = Self::get_buy_move(s);
            match buy_move {
                Some(m) => Some(m),
                None => Self::get_put_move(s),
            }
        }
    }

    /// Information about normal move.
    pub fn info(&self) -> (S, S) {
        match self {
            Self::Normal {
                from,
                to,
                promote: _,
            } => (*from, *to),
            _ => (
                Square::from_index(0).unwrap(),
                Square::from_index(0).unwrap(),
            ),
        }
    }

    /// Creating new normal with 'from' and 'to' Square.
    pub fn new(from: S, to: S, promote: bool) -> Self {
        Self::Normal { from, to, promote }
    }

    /// Getting buy move from str.
    pub fn get_buy_move(s: &str) -> Option<Self> {
        if s.len() == 2 && s.starts_with('+') {
            if let Some(piece) = Piece::from_sfen(s.chars().nth(1).unwrap()) {
                return Some(Self::Buy { piece });
            }
        }
        None
    }

    /// Getting put move from str.
    pub fn get_put_move(s: &str) -> Option<Self> {
        let mut fen_parts = s.split('@');
        if let Some(piece_str) = fen_parts.next() {
            if let Some(piece_char) = piece_str.chars().next() {
                if let Some(piece) = Piece::from_sfen(piece_char) {
                    if let Some(to) = fen_parts.next() {
                        if let Some(to) = Square::from_sfen(to) {
                            return Some(Self::Put { piece, to });
                        }
                    }
                }
            }
        }
        None
    }

    /// Getting normal move from str.
    pub fn get_normal_move(s: &str) -> Option<Self> {
        let mut fen_parts = s.split('_');
        if let Some(from) = fen_parts.next() {
            if let Some(from) = Square::from_sfen(from) {
                if let Some(to) = fen_parts.next() {
                    if let Some(to) = Square::from_sfen(to) {
                        return Some(Self::Normal {
                            from,
                            to,
                            promote: false,
                        });
                    }
                }
            }
        }

        None
    }
}

impl<S: Square> fmt::Display for Move<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Move::Buy { piece } => {
                write!(f, "+{piece}")
            }
            Move::Put { to, piece } => {
                write!(f, "{piece}@{to}")
            }
            Move::Normal { from, to, promote } => {
                write!(
                    f,
                    "{}_{}{}",
                    from,
                    to,
                    if promote { "fixthis" } else { "" }
                )
            }
        }
    }
}

/// MoveRecord stores information necessary to undo the move.
#[derive(Debug, Clone)]
pub enum MoveRecord<S: Square> {
    Buy {
        piece: Piece,
    },
    Put {
        to: S,
        piece: Piece,
    },
    Normal {
        from: S,
        to: S,
        placed: Piece,
        move_data: MoveData,
    },
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

impl<S: Square> MoveRecord<S> {
    /// Converts the move into SFEN formatted string.
    pub fn to_sfen(&self) -> String {
        match *self {
            MoveRecord::Buy { piece } => format!("+{piece}"),
            MoveRecord::Put { to, piece } => format!("{piece}@{to}"),
            MoveRecord::Normal {
                from,
                to,
                move_data,
                ..
            } => format!(
                "{}_{}{}",
                from,
                to,
                if move_data.promoted { "*" } else { "" }
            ),
        }
    }
}

impl<S: Square> PartialEq<Move<S>> for MoveRecord<S> {
    fn eq(&self, other: &Move<S>) -> bool {
        match (self, other) {
            (
                &MoveRecord::Normal {
                    from: f1,
                    to: t1,
                    move_data,
                    ..
                },
                &Move::Normal {
                    from: f2,
                    to: t2,
                    promote,
                },
            ) => f1 == f2 && t1 == t2 && promote == move_data.promoted,
            (
                &MoveRecord::Buy { piece: piece1 },
                &Move::Buy { piece: piece2 },
            ) => piece1 == piece2,
            (
                &MoveRecord::Put {
                    to: to1,
                    piece: piece1,
                },
                &Move::Put {
                    to: to2,
                    piece: piece2,
                },
            ) => to1 == to2 && piece1 == piece2,
            _ => false,
        }
    }
}

impl<S> MoveRecord<S>
where
    S: Square,
{
    pub fn format(&self) -> String {
        if let MoveRecord::Normal {
            from,
            to,
            move_data,
            ..
        } = &self
        {
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
                    let file = self.same_format(from, 0, false);
                    let rank = self.same_format(from, 1, true);
                    format!("{file}{rank}")
                } else if move_data.same_file {
                    self.same_format(from, 1, true)
                } else if move_data.same_rank {
                    from.to_string().chars().next().unwrap().to_string()
                } else {
                    "".to_string()
                }
            };

            let captures = {
                if move_data.captured.is_some() {
                    if piece.is_empty() {
                        format!("{}x", self.same_format(from, 0, false))
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
        " ".to_string()
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

// pub fn generate_normal_sfen(f) {

// }
