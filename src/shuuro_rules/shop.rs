use crate::shuuro_rules::Square;
use std::{
    sync::{Arc, Mutex},
    u8,
};

use crate::shuuro_rules::{variant::Variant, Color, Piece, PieceType};
use crate::shuuro_rules::{Hand, Move, MoveRecord};

fn get_pricing() -> [(i32, u8); 10] {
    let prices = [0, 110, 70, 40, 40, 10, 130, 130, 70, 0];
    let count = [1, 3, 6, 9, 9, 18, 3, 3, 4, 0];
    let mut pricing: [(i32, u8); 10] = [(0, 0); 10];
    let pt_iter = PieceType::iter();
    for pt in pt_iter {
        pricing[pt.index()] = (prices[pt.index()], count[pt.index()]);
    }
    pricing
}

/// Used for buying pieces.
#[derive(Clone, Debug)]
pub struct Shop<S: Square> {
    credit: [i32; 2],
    hand: Hand,
    confirmed: [bool; 2],
    pricing: [(i32, u8); 10],
    move_history: Arc<Mutex<Vec<MoveRecord<S>>>>,
    sfen_history: Arc<Mutex<Vec<(String, u8)>>>,
    variant: Variant,
}

impl<S: Square> Shop<S> {
    /// Change variant
    pub fn change_variant(&mut self, variant: &String) {
        self.variant = self.variant.change_variant(variant);
        let credit = self.variant.start_credit();
        self.credit = [credit, credit];
    }

    pub fn variant(&self) -> Variant {
        self.variant
    }

    /// Buying piece with specific color.
    pub fn play(&mut self, mv: Move<S>) -> Option<[bool; 2]> {
        if let Move::Buy { piece } = mv {
            if !self.variant.can_buy(&piece.piece_type)
                || piece.color == Color::NoColor
            {
                return None;
            } else if !self.is_confirmed(piece.color) {
                let (piece_price, piece_count) =
                    self.pricing[piece.piece_type.index()];
                if self.credit[piece.color.index()] >= piece_price {
                    if self.hand.get(piece) < piece_count {
                        self.hand.increment(piece);
                        self.credit[piece.color.index()] =
                            self.credit(piece.color) - piece_price;
                        let move_record = MoveRecord::Buy { piece };
                        self.sfen_history.lock().unwrap().push((
                            move_record.to_sfen(),
                            self.hand.get(piece),
                        ));
                        self.move_history.lock().unwrap().push(move_record);
                    }
                    if self.credit[piece.color.index()] == 0 {
                        self.confirm(piece.color);
                    }
                    return Some(self.confirmed);
                }
            }
        }
        None
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
            let piece = Piece::from_sfen(i).unwrap();
            self.play(Move::Buy { piece });
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
        let mut h = self.sfen_history.lock().unwrap();
        h.clear();
        h.extend(history);
    }

    pub fn set_move_history(&mut self, history: Vec<MoveRecord<S>>) {
        let mut h = self.move_history.lock().unwrap();
        h.clear();
        h.extend(history);
    }

    #[warn(unused_variables)]
    pub fn get_sfen_history(&self, _color: &Color) -> Vec<(String, u8)> {
        self.sfen_history.lock().unwrap().clone()
    }
}

impl<S: Square> Default for Shop<S> {
    fn default() -> Self {
        let mut shop = Shop {
            credit: [800; 2],
            hand: Hand::default(),
            confirmed: [false, false],
            pricing: get_pricing(),
            move_history: Default::default(),
            sfen_history: Default::default(),
            variant: Variant::Shuuro,
        };
        shop.set_kings();
        shop
    }
}
