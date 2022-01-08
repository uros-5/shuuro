use crate::{BitBoard, Color, Piece, PieceType, SfenError, Square};
use itertools::Itertools;

pub trait SfenStr {
    fn piece_at(&self, sq: Square) -> &Option<Piece>;
    fn player_bb(&self, c: Color) -> BitBoard;
    fn stm(&self) -> Color;
    fn hand_get(&self, p: Piece) -> u8;
    fn ply(&self) -> u16;

    fn generate_sfen(&self) -> String {
        fn add_num_space(num_spaces: i32, mut s: String) -> String {
            if num_spaces == 10 {
                s.push_str("55");
            } else if num_spaces == 11 {
                s.push_str("56");
            } else if num_spaces == 12 {
                s.push_str("57");
            } else {
                s.push_str(&num_spaces.to_string());
            }
            s
        }

        let board = (0..12)
            .map(|row| {
                let mut s = String::new();
                let mut num_spaces = 0;
                for file in 0..12 {
                    let sq = Square::new(file, row).unwrap();
                    match *self.piece_at(sq) {
                        Some(pc) => {
                            if num_spaces > 0 {
                                let mut _s = add_num_space(num_spaces, s);
                                s = _s;
                                num_spaces = 0;
                            }

                            if (&self.player_bb(Color::NoColor) & sq).is_any() {
                                if pc.piece_type == PieceType::Knight {
                                    s.push_str("L");
                                } else {
                                    ()
                                    //return Err(SfenError::IllegalPieceTypeOnPlynth);
                                }
                            }

                            s.push_str(&pc.to_string());
                        }
                        None => {
                            if (&self.player_bb(Color::NoColor) & sq).is_any() {
                                s.push_str("L0");
                            } else {
                                num_spaces += 1;
                            }
                        }
                    }
                }

                if num_spaces > 0 {
                    let _s = add_num_space(num_spaces, s);
                    s = _s;
                    //num_spaces = 0;
                }

                s
            })
            .join("/");

        let color = if self.stm() == Color::Blue { "b" } else { "r" };

        let mut hand = [Color::Blue, Color::Red]
            .iter()
            .map(|c| {
                PieceType::iter()
                    .filter(|pt| pt.is_hand_piece())
                    .map(|pt| {
                        let pc = Piece {
                            piece_type: pt,
                            color: *c,
                        };
                        let n = self.hand_get(pc);

                        if n == 0 {
                            "".to_string()
                        } else if n == 1 {
                            format!("{}", pc)
                        } else {
                            format!("{}{}", n, pc)
                        }
                    })
                    .join("")
            })
            .join("");

        if hand.is_empty() {
            hand = "-".to_string();
        }

        format!("{} {} {} {}", board, color, hand, self.ply())
    }

    fn empty_bb(&mut self);

    fn set_piece(&mut self, sq: Square, p: Option<Piece>);

    fn update_color_bb(&mut self, c: Color, sq: Square);

    fn update_bb(&mut self, p: Piece, sq: Square);

    fn parse_sfen_board(&mut self, s: &str) -> Result<(), SfenError> {
        let rows = s.split('/');
        self.empty_bb();

        for (i, row) in rows.enumerate() {
            if i >= 12 {
                return Err(SfenError::IllegalBoardState);
            }

            let mut j = 0;

            let mut is_promoted = false;
            for c in row.chars() {
                match c {
                    '+' => {
                        is_promoted = true;
                    }
                    '0' => {
                        let sq = Square::new(j, i as u8).unwrap();
                        self.set_piece(sq, None);
                        j += 1;
                    }
                    n if n.is_digit(11) => {
                        if let Some(n) = n.to_digit(11) {
                            for _ in 0..n {
                                if j >= 12 {
                                    return Err(SfenError::IllegalBoardState);
                                }

                                let sq = Square::new(j, i as u8).unwrap();

                                self.set_piece(sq, None);

                                j += 1;
                            }
                        }
                    }
                    s => match Piece::from_sfen(s) {
                        Some(mut piece) => {
                            if j >= 12 {
                                return Err(SfenError::IllegalBoardState);
                            }

                            if is_promoted {
                                if let Some(promoted) = piece.piece_type.promote() {
                                    piece.piece_type = promoted;
                                } else {
                                    return Err(SfenError::IllegalPieceType);
                                }
                            }

                            let sq = Square::new(j, i as u8).unwrap();
                            if piece.piece_type == PieceType::Plynth {
                                self.update_color_bb(piece.color, sq);
                                continue;
                            }
                            self.set_piece(sq, Some(piece));
                            self.update_bb(piece, sq);
                            j += 1;
                            is_promoted = false;
                        }
                        None => return Err(SfenError::IllegalPieceType),
                    },
                }
            }
        }

        Ok(())
    }
}
