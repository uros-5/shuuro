use crate::shuuro_rules::Square;
use std::u8;

use crate::shuuro_rules::{variant::Variant, Color, Piece, PieceType};
use crate::shuuro_rules::{Hand, Move};

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

/// Used for selecting pieces.
#[derive(Clone, Debug)]
pub struct Selection<S: Square> {
    credit: [i32; 2],
    hand: Hand,
    confirmed: [bool; 2],
    pricing: [(i32, u8); 10],
    move_history: Vec<Move<S>>,
    sfen_history: Vec<(String, u8)>,
    variant: Variant,
}

impl<S: Square> Selection<S> {
    /// Change variant
    pub fn update_variant(&mut self, variant: Variant) {
        self.variant = variant;
        let credit = self.variant.start_credit();
        self.credit = [credit, credit];
        self.update_pricing();
    }

    /// Update pricing
    fn update_pricing(&mut self) {
        if self.variant == Variant::Standard
            || self.variant == Variant::StandardFairy
        {
            self.pricing[PieceType::Pawn.index()] = (10, 12);
        } else if self.variant == Variant::ShuuroMini
            || self.variant == Variant::ShuuroMiniFairy
        {
            self.pricing[PieceType::Pawn.index()] = (10, 8);
        }
    }

    pub fn variant(&self) -> Variant {
        self.variant
    }

    /// Selecting piece with specific color.
    pub fn play(&mut self, mv: Move<S>) -> Option<[bool; 2]> {
        if let Move::Select { piece } = mv {
            if !self.variant.can_select(&piece.piece_type)
                || piece.color == Color::NoColor
            {
                return None;
            } else if !self.is_confirmed(piece.color) {
                let (piece_price, piece_count) =
                    self.pricing[piece.piece_type.index()];
                if self.credit[piece.color.index()] >= piece_price {
                    let current_hand = self.hand.get(piece);
                    match current_hand.cmp(&piece_count) {
                        std::cmp::Ordering::Less => {
                            self.hand.increment(piece);
                            self.credit[piece.color.index()] =
                                self.credit(piece.color) - piece_price;
                            let move_record = Move::Select { piece };
                            self.sfen_history.push((
                                move_record.to_string(),
                                self.hand.get(piece),
                            ));
                            self.move_history.push(move_record);
                            if self.credit[piece.color.index()] == 0 {
                                self.confirm(piece.color);
                            }
                            return Some(self.confirmed);
                        }
                        std::cmp::Ordering::Equal => {
                            return None;
                        }
                        std::cmp::Ordering::Greater => return None,
                    }
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
        let mut selected = vec![];
        self.hand.set_hand(s);
        for color in Color::iter() {
            if color == Color::NoColor {
                continue;
            }
            for piece_type in PieceType::iter() {
                if piece_type == PieceType::Plinth {
                    continue;
                }
                let piece = Piece { piece_type, color };
                let mut count = self.hand.get(piece);
                let allowed_count = self.pricing[piece_type as usize];
                if count > allowed_count.1 {
                    count = allowed_count.1;
                    self.hand.just_set(piece, count);
                } else if piece_type == PieceType::King {
                    count = 1;
                }
                for _ in 0..count {
                    selected.push(Move::<S>::Select { piece });
                }
            }
        }
        self.hand = Hand::default();
        for m in selected {
            self.play(m);
        }
    }
    /// Converts entire hand by color to string.
    pub fn to_sfen(&self, c: Color, long: bool) -> String {
        self.hand.to_sfen(c, long)
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
                self.play(Move::Select {
                    piece: Piece {
                        piece_type: PieceType::King,
                        color: c,
                    },
                });
            }
        }
    }

    pub fn set_sfen_history(&mut self, history: Vec<(String, u8)>) {
        self.sfen_history.clear();
        self.sfen_history.extend(history);
    }

    pub fn set_move_history(&mut self, history: Vec<Move<S>>) {
        self.move_history.clear();
        self.move_history.extend(history);
    }

    #[warn(unused_variables)]
    pub fn get_sfen_history(&self, _color: &Color) -> Vec<(String, u8)> {
        self.sfen_history.clone()
    }
}

impl<S: Square> Default for Selection<S> {
    fn default() -> Self {
        let mut shop = Selection {
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
