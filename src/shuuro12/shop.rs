#[cfg(test)]
mod tests {

    use crate::{
        shuuro12::square12::Square12, Color, Move, Piece, PieceType, Selection,
    };

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
        let mut shop = Selection::<Square12>::default();
        for case in cases.iter() {
            let piece: Piece = Piece {
                piece_type: case.0,
                color: case.1,
            };
            for _i in 0..case.2 {
                shop.play(Move::Select { piece });
            }
            assert_eq!(shop.get(piece), case.2);
        }
        shop.confirm(Color::White);
        assert_eq!(shop.credit(Color::White), 800 - 260);
        assert_eq!(shop.credit(Color::Black), 800 - 690);
        assert!(!shop.is_confirmed(Color::Black));
        assert!(shop.is_confirmed(Color::White));
    }

    #[test]
    fn set_hand() {
        let cases = [
            ("RRPPnnnQQ", Color::White, 380, "KQQRRPP"),
            ("nQrrPnNQqqqqqbbr", Color::Black, 700, "kqqqrrrbbnn"),
        ];
        for case in cases {
            let mut shop = Selection::<Square12>::default();
            shop.set_hand(case.0);
            assert_eq!(shop.credit(case.1), 800 - case.2);
            assert_eq!(shop.to_sfen(case.1, true), case.3);
        }
    }
}
