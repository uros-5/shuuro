use std::u8;

use crate::{Color, Hand, Move, MoveRecord, Piece, PieceType};

fn get_pricing() -> [(i32, u8); 7] {
    let prices = [0, 110, 70, 40, 40, 10, 0];
    let count = [1, 3, 6, 9, 9, 18, 0];
    let mut pricing: [(i32, u8); 7] = [(0, 0); 7];
    let pt_iter = PieceType::iter();
    for pt in pt_iter {
        pricing[pt.index()] = (prices[pt.index()], count[pt.index()]);
    }
    return pricing;
}

/// Used for buying pieces.
#[derive(Debug, Clone)]
pub struct Shop {
    credit: [i32; 2],
    hand: Hand,
    confirmed: [bool; 2],
    pricing: [(i32, u8); 7],
    move_history: Vec<MoveRecord>,
    sfen_history: Vec<(String, u8)>,
    side_to_move: Color,
}

impl Shop {
    /// Buying piece with specific color.
    pub fn play(&mut self, mv: Move) {
        match mv {
            Move::Buy { piece } => {
                if !self.is_confirmed(piece.color) {
                    let (piece_price, piece_count) = self.pricing[piece.piece_type.index()];
                    if self.credit[piece.color.index()] >= piece_price as i32 {
                        if self.hand.get(piece) < piece_count {
                            self.hand.increment(piece);
                            self.credit[piece.color.index()] =
                                self.credit(piece.color) - piece_price;
                            let move_record = MoveRecord::Buy { piece };
                            self.sfen_history
                                .push((move_record.to_sfen().clone(), self.hand.get(piece)));
                            self.move_history.push(move_record);
                        }
                        if self.credit[piece.color.index()] == 0 {
                            self.confirm(piece.color);
                        }
                    }
                }
            }
            _ => (),
        }
    }

    /// Confirm your choice of pieces.
    pub fn confirm(&mut self, c: Color) {
        if self.credit(c) < 700 {
            self.confirmed[c.index()] = true;
        }
    }

    /// Set hand from string. Panics if wrong piece is found.
    pub fn set_hand(&mut self, s: &str) {
        for i in s.chars() {
            self.play(Move::Buy {
                piece: Piece::from_sfen(i).unwrap(),
            });
        }
    }
    /// Converts entire hand by color to string.
    pub fn to_sfen(&self, c: Color) -> String {
        self.hand.to_sfen(c)
    }

    /// Get how much pieces are left in hand.
    pub fn get(&self, p: Piece) -> u8 {
        self.hand.get(p)
    }

    /// Get how much credit one hand has.
    pub fn credit(&self, c: Color) -> i32 {
        self.credit[c.index()]
    }

    /// Checks if color is confirmed it's choice.
    pub fn is_confirmed(&self, c: Color) -> bool {
        self.confirmed[c.index()]
    }

    /// Set kings.
    fn set_kings(&mut self) {
        for c in Color::iter() {
            if c != Color::NoColor {
                self.play(Move::Buy {
                    piece: Piece {
                        piece_type: PieceType::King,
                        color: c,
                    },
                });
            }
        }
    }

    pub fn set_sfen_history(&mut self, history: Vec<(String, u8)>) {
        self.sfen_history = history;
    }

    pub fn set_move_history(&mut self, history: Vec<MoveRecord>) {
        self.move_history = history;
    }
    
    #[warn(unused_variables)]
    pub fn get_sfen_history(&self, _color: &Color) -> &Vec<(String, u8)> {
        &self.sfen_history
    }
}

impl Default for Shop {
    fn default() -> Self {
        let mut shop = Shop {
            credit: [800; 2],
            hand: Hand::default(),
            confirmed: [false, false],
            pricing: get_pricing(),
            move_history: Default::default(),
            sfen_history: Default::default(),
            side_to_move: Color::White,
        };
        shop.set_kings();
        shop
    }
}

#[cfg(test)]
mod tests {

    use crate::{Color, Move, Piece, PieceType};

    use super::Shop;

    #[test]
    fn play() {
        let cases = [
            (PieceType::Pawn, Color::White, 4),
            (PieceType::Queen, Color::White, 2),
            (PieceType::Bishop, Color::Black, 3),
            (PieceType::Rook, Color::Black, 3),
            (PieceType::Queen, Color::Black, 3),
            (PieceType::Pawn, Color::Black, 3),
        ];
        let mut shop = Shop::default();
        for case in cases.iter() {
            let piece: Piece = Piece {
                piece_type: case.0,
                color: case.1,
            };
            for _i in 0..case.2 {
                shop.play(Move::Buy { piece });
            }
            assert_eq!(shop.get(piece), case.2);
        }
        shop.confirm(Color::White);
        assert_eq!(shop.credit(Color::White), 800 - 260);
        assert_eq!(shop.credit(Color::Black), 800 - 690);
        assert_ne!(shop.is_confirmed(Color::Black), true);
        assert_eq!(shop.is_confirmed(Color::White), true);
    }

    #[test]
    fn set_hand() {
        let cases = [
            ("RRPPnnnQQ", Color::White, 380, "KQQRRPP"),
            ("nQrrPnNQqqqqqbbr", Color::Black, 700, "kqqqrrrbbnn"),
        ];
        for case in cases {
            let mut shop = Shop::default();
            shop.set_hand(case.0);
            assert_eq!(shop.credit(case.1), 800 - case.2);
            assert_eq!(shop.to_sfen(case.1), case.3);
        }
    }
}
