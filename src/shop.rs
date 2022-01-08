use crate::{Color, Hand, Piece, PieceType};

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
pub struct Shop {
    credit: [i32; 2],
    hand: Hand,
    confirmed: [bool; 2],
    pricing: [(i32, u8); 7],
}

impl Shop {
    /// Buying piece with specific color.
    pub fn buy(&mut self, p: Piece) {
        if !self.is_confirmed(p.color) {
            let (piece_price, piece_count) = self.pricing[p.piece_type.index()];
            if self.credit[p.color.index()] >= piece_price as i32 {
                if self.hand.get(p) < piece_count {
                    self.hand.increment(p);
                    self.credit[p.color.index()] = self.credit(p.color) - piece_price;
                }
                if self.credit[p.color.index()] == 0 {
                    self.confirm(p.color);
                }
            }
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
            self.buy(Piece::from_sfen(i).unwrap());
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
                self.buy(Piece {
                    piece_type: PieceType::King,
                    color: c,
                });
            }
        }
    }
}

impl Default for Shop {
    fn default() -> Self {
        let mut shop = Shop {
            credit: [800; 2],
            hand: Hand::default(),
            confirmed: [false, false],
            pricing: get_pricing(),
        };
        shop.set_kings();
        shop
    }
}

#[cfg(test)]
mod tests {

    use crate::{Color, Piece, PieceType};

    use super::Shop;

    #[test]
    fn buy() {
        let cases = [
            (PieceType::Pawn, Color::Red, 4),
            (PieceType::Queen, Color::Red, 2),
            (PieceType::Bishop, Color::Blue, 3),
            (PieceType::Rook, Color::Blue, 3),
            (PieceType::Queen, Color::Blue, 3),
            (PieceType::Pawn, Color::Blue, 3),
        ];
        let mut shop = Shop::default();
        for case in cases.iter() {
            let p: Piece = Piece {
                piece_type: case.0,
                color: case.1,
            };
            for _i in 0..case.2 {
                shop.buy(p);
            }
            assert_eq!(shop.get(p), case.2);
        }
        shop.confirm(Color::Red);
        assert_eq!(shop.credit(Color::Red), 800 - 260);
        assert_eq!(shop.credit(Color::Blue), 800 - 690);
        assert_ne!(shop.is_confirmed(Color::Blue), true);
        assert_eq!(shop.is_confirmed(Color::Red), true);
    }

    #[test]
    fn set_hand() {
        let cases = [
            ("RRPPnnnQQ", Color::Red, 380, "KQQRRPP"),
            ("nQrrPnNQqqqqqbbr", Color::Blue, 700, "kqqqrrrbbnn"),
        ];
        for case in cases {
            let mut shop = Shop::default();
            shop.set_hand(case.0);
            assert_eq!(shop.credit(case.1), 800 - case.2);
            assert_eq!(shop.to_sfen(case.1), case.3);
        }
    }
}
