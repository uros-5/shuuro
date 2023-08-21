#[cfg(test)]
pub mod position_tests {

    use shuuro::{
        attacks::Attacks,
        bitboard::BitBoard,
        piece_type::PieceType,
        position::{Board, MoveType, Outcome, Placement, Play},
        shuuro8::{
            attacks8::Attacks8,
            position8::P8,
            square8::{consts::*, Square8},
        },
        square::Square,
        Color, Move, Piece, Variant,
    };

    pub const START_POS: &str = "KR6/8/8/8/8/8/8/kr6 b - 1";

    fn setup() {
        Attacks8::init();
    }

    #[test]
    fn piece_exist() {
        setup();
        let mut pos = P8::new();
        pos.set_sfen(START_POS).unwrap();
        let sq = Square8::from_index(56).unwrap();
        let piece = Piece {
            piece_type: PieceType::King,
            color: Color::Black,
        };
        assert_eq!(Some(piece), *pos.piece_at(sq));
    }

    #[test]
    fn player_bb() {
        //
        setup();
        let cases: &[(&str, &[Square8], &[Square8], &[Square8])] = &[(
            "RNBQKBNR/PPPPPPPP/3L03L0/8/5L02/2L05/pppppppp/rnbqkbnr w - 1",
            &[A1, B1, C1, D1, E1, F1, G1, H1],
            &[A8, B8, C8, D8, E8, F8, G8, H8],
            &[D3, H3, F5, C6],
        )];

        let mut pos = P8::new();
        for case in cases {
            pos.set_sfen(case.0).expect("faled to parse SFEN string");
            let black = pos.player_bb(Color::Black);
            let white = pos.player_bb(Color::White);

            assert_eq!(case.2.len(), black.count() - 8);
            for sq in case.2 {
                assert!((&black & sq).is_any());
            }

            assert_eq!(case.1.len(), white.count() - 8);
            for sq in case.1 {
                assert!((&white & sq).is_any());
            }

            let plinths = pos.player_bb(Color::NoColor);

            for sq in case.3 {
                assert!((&plinths & sq).is_any())
            }
        }
    }

    #[test]
    fn pinned_bb() {
        setup();

        let cases: &[(&str, &[Square8], &[Square8])] = &[
            ("4KR2/3B4/8/1b6/8/8/8/1k1r4 w - 1", &[D2], &[]),
            ("6K1/5QR1/4B3/8/8/1b6/8/1k1r4 w - 1", &[], &[]),
            (
                "6K1/1p3QR1/4B3/4Q3/7B/1b6/4bb2/R2rkr1Q b - 1",
                &[],
                &[D8, F8, E7, F7],
            ),
        ];

        let mut pos = P8::new();
        for case in cases {
            pos.set_sfen(case.0).expect("faled to parse SFEN string");
            let white = pos.pinned_bb(Color::White);
            let black = pos.pinned_bb(Color::Black);

            assert_eq!(case.2.len(), black.count());
            for sq in case.2 {
                assert!((&black & sq).is_any());
            }

            assert_eq!(case.1.len(), white.count());
            for sq in case.1 {
                assert!((&white & sq).is_any());
            }
        }
    }

    #[test]
    fn pawn_vs_knight() {
        setup();
        let sfen = "5K2/2N1LNR2/1B1p4/8/6Ln1/7q/2r5/2k1r3 b - 38";
        let mut pos = P8::new();
        pos.set_sfen(sfen).expect("failed to parse SFEN string");
        let lm = pos.legal_moves(&Color::Black);
        if let Some(b) = lm.get(&D3) {
            assert!(b.count() == 2);
        }
    }

    #[test]
    fn pawn_not_pinned() {
        setup();
        let mut pos = P8::new();
        pos.set_sfen("5K2/4PR2/1B1Q4/3N1N2/1B1p2n1/7q/2r5/2k1r3 w - 55")
            .expect("failed to parse SFEN string");
        let lm = pos.legal_moves(&Color::White);
        if let Some(b) = lm.get(&E2) {
            assert_eq!(b.count(), 2);
        }
    }

    #[test]
    fn pawn_check_king() {
        setup();
        let mut pos = P8::new();
        pos.set_sfen("6K1/1p1pP1Rp/1B6/5N2/1B2Q1n1/7q/2r2N2/2k1r3 w - 1")
            .expect("failed to parse SFEN string");
        let in_check = pos.in_check(Color::White);
        assert!(in_check);
    }

    #[test]
    fn legal_moves_pawn() {
        setup();
        let cases = [("3Q2K1/4PL02/4pB2/8/8/3pp3/8/3kq3 b - 11", E3, 0)];
        for case in cases {
            let mut pos = P8::default();
            pos.set_sfen(case.0).expect("failed to parse sfen string");
            let legal_moves = pos.legal_moves(&Color::White);
            if let Some(b) = legal_moves.get(&case.1) {
                assert_eq!(b.count(), case.2);
            }
        }
    }

    #[test]
    fn check_while_pinned() {
        setup();
        let mut pos = P8::default();
        pos.set_sfen("6K1/4P3/4pB2/8/3Q4/8/3r4/1Q1kq3 b - 50")
            .expect("failed to parse sfen string");
        let legal_moves = pos.legal_moves(&Color::Black);
        if let Some(b) = legal_moves.get(&D7) {
            assert_eq!(b.count(), 0);
        }
    }

    #[test]
    fn king_moves() {
        setup();
        let cases = [
            ("1K1R1R2/8/8/8/8/8/8/4k3 b - 1", Color::Black, E8, 1),
            ("1K1R4/8/8/8/8/8/8/4k2Q b - 1", Color::Black, E8, 2),
            ("1K1R4/8/8/2Q5/8/1r6/1R6/1r2k3 w - 1", Color::White, B1, 4),
            ("3R1K1r/6r1/8/2Q5/8/8/1R6/1r2k3 w - 1", Color::White, F1, 1),
        ];
        for case in cases {
            let mut pos = P8::default();
            pos.set_sfen(case.0).expect("failed to parse sfen string");
            let legal_moves = pos.legal_moves(&case.1);
            if let Some(b) = legal_moves.get(&case.2) {
                assert_eq!(b.count(), case.3);
            }
        }
    }

    #[test]
    fn parse_sfen_hand() {
        setup();
        let cases = [
            (
                "8/PPPPPPPP/8/8/8/8/pppppppp/8 b 2RAC2NQK2rac2nqk 1",
                8,
                Variant::StandardFairy,
            ),
            (
                "8/PPPPPPPP/8/8/8/8/pppppppp/8 b 2R2BGAQK2r2bgaqk 1",
                8,
                Variant::StandardFairy,
            ),
            // (
            //     "4K3/8/8/1L01L04/4L03/6L01/8/8 b RBNNNPPPPPPPPPPPPkqrbbnnp 1",
            //     8,
            //     Variant::Standard,
            // ),
        ];
        for case in cases {
            let mut pos = P8::new();
            pos.update_variant(case.2);
            pos.set_sfen(case.0).expect("failed to parse sfen string");
            assert_eq!(pos.get_hand(Color::Black, true).len(), case.1);
        }
    }

    #[test]
    fn move_candidates() {
        setup();

        let cases = [("RNBQKBNR/PPPPPPPP/8/8/8/8/pppppppp/rnbqkbnr b - 1", 20)];
        for case in cases {
            let mut pos = P8::new();
            pos.set_sfen(case.0).expect("failed to parse SFEN string");

            let mut sum = 0;
            for sq in Square8::iter() {
                let pc = pos.piece_at(sq);

                if let Some(pc) = *pc {
                    if pc.color == pos.side_to_move() {
                        sum += pos
                            .move_candidates(&sq, pc, MoveType::Plinth)
                            .count();
                    }
                }
            }

            assert_eq!(sum, case.1);
        }
    }

    #[test]
    fn check_while_knight_on_plinth() {
        setup();
        let sfen = "RNBQKB1R/PPPPPLNPP/5P2/7b/8/8/pppppppp/rnbqk1nr b - 11";
        let mut pos = P8::new();
        pos.set_sfen(sfen).expect("failed to parse SFEN string");
        let legal_moves = pos.legal_moves(&Color::Black);
        if let Some(b) = legal_moves.get(&F2) {
            assert_eq!(b.count(), 4);
        }
    }

    #[test]
    fn pawn_captures_last_rank() {
        setup();
        let cases = [
            ("8/1K6/8/8/8/8/4P3/1k3n2 w - 1", Color::White, E7, F8),
            ("7R/1K4p1/8/8/8/8/8/1k6 b - 1", Color::Black, G2, H1),
        ];
        for case in cases {
            let mut position = P8::new();
            position
                .set_sfen(case.0)
                .expect("failed to parse sfen string");

            let pawn_moves = position.legal_moves(&case.1);
            if let Some(b) = pawn_moves.get(&case.2) {
                assert_eq!(b.count(), 2);
            }
            let m = Move::Normal {
                from: case.2,
                to: case.3,
                promote: false,
            };
            let result = position.make_move(m);
            assert!(result.is_ok());
            assert_eq!(
                position.piece_at(case.3).unwrap().piece_type,
                PieceType::Queen
            );
        }
    }

    #[test]
    fn knight_jumps_move() {
        setup();
        let cases = [
            ("1K6/3N4/8/1L06/2L05/n7/8/3k1r2 b - 17", "a6", "c5"),
            ("1K6/8/3N4/1Ln6/8/8/8/k4r2 w - 17", "d3", "b4"),
        ];
        for case in cases {
            let mut position = P8::new();
            position
                .set_sfen(case.0)
                .expect("failed to parse sfen string");

            let result = position.play(case.1, case.2);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn in_check() {
        setup();

        let test_cases = [
            ("8/5rK1/3n4/8/8/8/8/k7 w - 1", false, true),
            ("8/5r2/3n4/2K5/8/8/8/k1Q5 b - 3", true, false),
            (
                "R1BQ1RK1/P3PPPP/2N2N2/B7/8/3p4/pp1L0pppp/rn1qkbnr w - 1",
                false,
                false,
            ),
            ("8/1Q4K1/5L0N1/8/8/2L05/1b5r/1k2q3 w - 4", false, false),
        ];

        let mut pos = P8::new();
        for case in test_cases.iter() {
            pos.set_sfen(case.0).expect("failed to parse sfen string");
            assert_eq!(case.1, pos.in_check(Color::Black));
            assert_eq!(case.2, pos.in_check(Color::White));
        }
    }

    #[test]
    fn is_stalemate() {
        setup();

        let cases = [
            ("8/8/8/8/8/1K6/8/k1Q5 b - 1", Color::Black),
            ("8/8/8/4K3/8/4NN2/8/7k b - 1", Color::Black),
            ("6K1/8/6k1/2b1b3/8/8/8/8 w - 1", Color::White),
        ];

        for case in cases {
            let mut pos = P8::new();
            pos.set_sfen(case.0).expect("failed to parse sfen string");
            if let Err(res) = pos.is_stalemate(&case.1) {
                assert_eq!(res.to_string(), "stalemate detected");
            }
        }

        let stalemate_moves = [(
            "8/q6P/3Ln1KP1/PLnP2P2/pB1pL0p2/BQ2p2L0/2n4p/k2b4 w - 97",
            "b6",
            "b7",
            Color::Black,
        )];

        for case in stalemate_moves {
            let mut pos = P8::new();
            pos.set_sfen(case.0).expect("failed to parse sfen string");
            let valid = pos.play(case.1, case.2).is_ok();
            assert!(valid);
        }
    }

    #[test]
    fn detect_insufficient_material() {
        setup();
        let cases = [
            ("1L03L02/1p6/4K3/5P2/2k5/1L06/5L02/8 b - 1", true),
            ("8/8/1p2K3/5L02/2k5/5P2/1L06/8 b - 1", false),
        ];
        for case in cases {
            let mut pos = P8::new();
            pos.set_sfen(case.0).expect("failed to parse sfen string");
            assert_eq!(pos.detect_insufficient_material().is_err(), case.1);
        }
    }

    #[test]
    fn king_place_moves() {
        setup();
        let mut position = P8::new();
        position.update_variant(Variant::Standard);
        position
            .set_sfen("8/8/3L04/1L06/8/L01L05/8/8 w kqnKQR 0")
            .ok();
        if position
            .place(
                Piece {
                    piece_type: PieceType::King,
                    color: Color::White,
                },
                C1,
            )
            .is_some()
        {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn black_pawns_placement() {
        setup();
        let mut position = P8::default();
        position
            .set_sfen("3QKB2/8/2L05/L07/8/4L02L0/8/2q1k3 b PPPpppppp 1")
            .ok();
        assert!(position.place(Piece::from_sfen('p').unwrap(), C7).is_some());
    }

    #[test]
    fn check_in_placement() {
        setup();
        let mut position = P8::default();
        position
            .set_sfen("2KQ4/8/1L06/3L04/7L0/5L02/8/2q2k2 w 6p3P 4")
            .ok();
        assert!(position.place(Piece::from_sfen('P').unwrap(), C2).is_some());
        // assert!(false);
    }

    #[test]
    fn black_pawn_moves() {
        setup();
        let mut position = P8::default();
        position
            .set_sfen("NNNKN3/PPPPPP1P/PL0PL0N1P1/5P2/8/ppPbpL0pL0/ppppbp1p/1bb1k1b1 b - 48")
            .ok();
        let lm = position.legal_moves(&Color::Black);
        assert_eq!(1, lm.get(&B7).unwrap().count());
    }

    #[test]
    fn move_notation() {
        setup();
        let cases = [
            (
                "1BK1R2B/8/2N3N1/1R2p2R/8/8/4R3/2k5 w - 19",
                vec![
                    (B4, E4, false, "Rbxe4"),
                    (C3, E4, false, "Ncxe4"),
                    (E1, E4, false, "R1xe4"),
                    (E7, E4, false, "R7xe4"),
                    (H1, E4, false, "Bhxe4"),
                ],
            ),
            (
                "3Q4/1K6/8/8/8/8/4P3/1k3n2 w - 1",
                vec![
                    (E7, E8, true, "e8=Q+"),
                    (E7, F8, true, "exf8=Q+"),
                    (D1, D6, true, "Qd6+"),
                ],
            ),
            ("8/3p2K1/8/8/Q7/8/8/2k5 b - 1", vec![(D2, D1, true, "d1=Q")]),
            (
                "7K/5p2/8/8/4P1q1/8/8/2k5 b - 1",
                vec![(G5, G1, false, "Qg1#"), (F2, F1, false, "f1=Q+")],
            ),
            (
                "5K2/8/1Q6/8/8/8/2P5/kq6 w - 1",
                vec![
                    (B3, B7, false, "Qb7+"),
                    (C7, C8, false, "c8=Q"),
                    (C7, B8, false, "cxb8=Q#"),
                ],
            ),
            (
                "5K2/8/1Q3Q2/8/1Q3Q2/8/2P5/1q5k w - 1",
                vec![
                    (F3, D5, false, "Qf3d5"),
                    (F5, D5, false, "Qf5d5"),
                    (B3, D5, false, "Qb3d5"),
                    (B5, D7, false, "Qbd7"),
                    (B5, E5, false, "Qbe5#"),
                    (F5, F8, false, "Qf8+"),
                ],
            ),
            (
                "1K6/8/8/8/8/4Q3/3P1Pn1/3bqkn1 w - 1",
                vec![(D7, E8, true, "dxe8=Q+"), (F7, E8, true, "fxe8=Q+")],
            ),
        ];

        for case in cases {
            for mn in case.1 {
                let mut position = P8::default();
                position.set_sfen(case.0).ok();
                let m = Move::Normal {
                    from: mn.0,
                    to: mn.1,
                    promote: mn.2,
                };
                let _ = position.make_move(m).is_ok();
                let last = position.get_move_history().last().unwrap();
                let notation = last.format();
                assert_eq!(&notation, mn.3);
            }
        }
    }

    #[test]
    fn example_with_moves() {
        setup();
        let moves = [
            "g8_f6", "g2_g3", "d7_d6", "f2_f4", "f6_g4", "f1_g2", "c7_c6",
            "b2_b3", "g4_e5", "c1_b2", "f8_e6", "c2_c4", "e6_c7", "b2_d4",
            "c6_c5", "d4_f2", "e5_d3", "g2_b7", "g7_g6", "h1_c6", "e8_f8",
            "a2_a4", "a7_a5", "d1_f1", "e7_e6", "e2_e4", "f7_f5", "e4_f5",
            "g6_f5", "f2_e3", "h8_g6", "f1_g2", "g6_e5", "c6_b5", "d3_e1",
            "g2_h1", "f8_g7", "b7_a6", "d6_d5", "e3_c5", "g7_g4", "g1_d4",
            "g4_d1", "b1_a2", "d1_d2", "d4_b2", "e1_d3", "c5_d4", "d3_b4",
            "a2_b1", "d2_c2", "b1_a1", "c2_b3", "a1_b1", "b3_a2", "b1_c1",
            "e5_d3", "c1_d2", "d3_b2", "d4_a7", "b8_a8", "d2_e3", "a8_a7",
            "e3_f3", "b2_d3", "h1_g1", "a7_b8", "g1_b6", "b8_a8", "b6_b7",
        ];

        let fen =
            "1KBQ1BBB/PPPPPPPP/3L04/1L06/4L03/7L0/pppppppp/1k1bqnnn b - 1";

        let mut position = P8::default();
        let _ = position.set_sfen(fen).is_ok();

        for m in moves {
            let mut parts = m.split('_');
            let from = parts.next().unwrap();
            let to = parts.next().unwrap();
            let _ = position.play(from, to).is_ok();
        }

        assert_eq!(
            position.outcome(),
            &Outcome::Checkmate {
                color: Color::White
            }
        );
    }
}
