use std::{
    fmt,
    marker::PhantomData,
    ops::{BitAnd, BitOr, BitXorAssign, Not},
};

use crate::{
    bitboard::BitBoard,
    position::{Board, Outcome, Placement, Play, Position, Sfen},
    Color, Hand, Move, MoveError, MoveRecord, Piece, PieceType, SfenError,
    Square, Variant,
};

use super::{
    attacks8::Attacks8,
    bitboard8::BB8,
    board_defs::{FILE_BB, RANK_BB},
    plinths_set8::PlinthGen8,
    square8::Square8,
};

impl Position<Square8, BB8<Square8>, Attacks8<Square8, BB8<Square8>>>
    for P8<Square8, BB8<Square8>>
{
}

#[derive(Clone, Debug)]
pub struct P8<S, B>
where
    S: Square,
    B: BitBoard<S>,
    for<'b> &'b B: BitOr<&'b B, Output = B>,
    for<'a> &'a B: BitAnd<&'a B, Output = B>,
    for<'a> &'a B: Not<Output = B>,
    for<'a> &'a B: BitOr<&'a S, Output = B>,
    for<'a> &'a B: BitAnd<&'a S, Output = B>,
    for<'a> B: BitXorAssign<&'a S>,
{
    board: PieceGrid,
    hand: Hand,
    ply: u16,
    side_to_move: Color,
    move_history: Vec<MoveRecord<Square8>>,
    sfen_history: Vec<String>,
    occupied_bb: BB8<Square8>,
    color_bb: [BB8<Square8>; 3],
    game_status: Outcome,
    variant: Variant,
    pub type_bb: [BB8<Square8>; 17],
    _a: PhantomData<B>,
    _s: PhantomData<S>,
}

impl Board<Square8, BB8<Square8>, Attacks8<Square8, BB8<Square8>>>
    for P8<Square8, BB8<Square8>>
{
    fn new() -> Self {
        Default::default()
    }

    fn set_piece(&mut self, sq: Square8, p: Option<Piece>) {
        self.board.set(sq, p)
    }

    fn piece_at(&self, sq: Square8) -> &Option<Piece> {
        self.board.get(sq)
    }

    fn player_bb(&self, c: Color) -> BB8<Square8> {
        self.color_bb[c.index()]
    }

    fn occupied_bb(&self) -> BB8<Square8> {
        self.occupied_bb
    }

    fn type_bb(&self, pt: &PieceType) -> BB8<Square8> {
        self.type_bb[pt.index()]
    }

    fn xor_player_bb(&mut self, color: Color, sq: Square8) {
        self.color_bb[color.index()] ^= &sq;
    }

    fn xor_type_bb(&mut self, piece_type: PieceType, sq: Square8) {
        self.type_bb[piece_type.index()] ^= &sq;
    }

    fn xor_occupied(&mut self, sq: Square8) {
        self.occupied_bb ^= &sq;
    }

    fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    fn empty_all_bb(&mut self) {
        self.occupied_bb = BB8::empty();
        self.color_bb = Default::default();
        self.type_bb = Default::default();
    }

    fn sfen_to_bb(&mut self, piece: Piece, sq: &Square8) {
        self.set_piece(*sq, Some(piece));
        self.occupied_bb |= sq;
        self.color_bb[piece.color.index()] |= sq;
        self.type_bb[piece.piece_type.index()] |= sq;
    }

    fn ply(&self) -> u16 {
        self.ply
    }

    fn increment_ply(&mut self) {
        self.ply += 1;
    }

    fn flip_side_to_move(&mut self) {
        self.side_to_move = self.side_to_move.flip();
    }

    fn update_side_to_move(&mut self, c: Color) {
        self.side_to_move = c;
    }

    fn outcome(&self) -> &Outcome {
        &self.game_status
    }

    fn update_outcome(&mut self, outcome: Outcome) {
        self.game_status = outcome;
    }

    fn variant(&self) -> Variant {
        self.variant
    }

    fn update_variant(&mut self, variant: Variant) {
        self.variant = variant;
    }

    fn insert_sfen(&mut self, sfen: &str) {
        self.sfen_history.push(sfen.to_string());
    }

    fn insert_move(&mut self, move_record: MoveRecord<Square8>) {
        self.move_history.push(move_record)
    }

    fn clear_sfen_history(&mut self) {
        self.sfen_history.clear();
    }

    fn set_sfen_history(&mut self, history: Vec<String>) {
        self.sfen_history = history;
    }

    fn set_move_history(&mut self, history: Vec<MoveRecord<Square8>>) {
        self.move_history = history;
    }

    fn move_history(&self) -> &[MoveRecord<Square8>] {
        &self.move_history
    }

    fn get_move_history(&self) -> &Vec<MoveRecord<Square8>> {
        &self.move_history
    }

    fn get_sfen_history(&self) -> &Vec<String> {
        &self.sfen_history
    }

    fn hand(&self, p: Piece) -> u8 {
        self.hand.get(p)
    }

    fn get_hand(&self, c: Color) -> String {
        self.hand.to_sfen(c)
    }

    fn set_hand(&mut self, s: &str) {
        self.hand.set_hand(s);
    }

    fn decrement_hand(&mut self, p: Piece) {
        self.hand.decrement(p);
    }

    fn dimensions(&self) -> u8 {
        8
    }
}

impl Sfen<Square8, BB8<Square8>, Attacks8<Square8, BB8<Square8>>>
    for P8<Square8, BB8<Square8>>
{
    fn clear_hand(&mut self) {
        self.hand.clear();
    }

    fn insert_in_hand(&mut self, p: Piece, num: u8) {
        self.hand.set(p, num);
    }

    fn parse_sfen_ply(&mut self, s: &str) -> Result<(), SfenError> {
        self.ply = s.parse()?;
        Ok(())
    }

    fn update_player(&mut self, piece: Piece, sq: &Square8) {
        self.set_piece(*sq, Some(piece));
        self.occupied_bb |= sq;
        self.color_bb[piece.color.index()] |= sq;
        self.type_bb[piece.piece_type.index()] |= sq;
    }
}

impl Placement<Square8, BB8<Square8>, Attacks8<Square8, BB8<Square8>>>
    for P8<Square8, BB8<Square8>>
{
    fn generate_plinths(&mut self) {
        self.color_bb[Color::NoColor.index()] = PlinthGen8::default().start();
    }

    fn white_placement_attacked_ranks(&self) -> BB8<Square8> {
        &RANK_BB[1] | &RANK_BB[2]
    }

    fn black_placement_attacked_ranks(&self) -> BB8<Square8> {
        &RANK_BB[6] | &RANK_BB[7]
    }

    fn black_ranks(&self) -> [usize; 3] {
        [7, 6, 5]
    }

    fn king_files<const K: usize>(&self) -> [&str; K] {
        let temp: [&str; 6] = ["d", "e", "f", "g", "h", "i"];
        let mut files: [&str; K] = [""; K];
        for (i, v) in temp.iter().enumerate() {
            files[i] = v;
        }
        files
    }

    fn rank_bb(&self, file: usize) -> BB8<Square8> {
        RANK_BB[file]
    }

    fn update_bb(&mut self, p: Piece, sq: Square8) {
        self.set_piece(sq, Some(p));
        self.occupied_bb |= &sq;
        self.color_bb[p.color.index()] |= &sq;
        self.type_bb[p.piece_type.index()] |= &sq;
    }
}

impl Play<Square8, BB8<Square8>, Attacks8<Square8, BB8<Square8>>>
    for P8<Square8, BB8<Square8>>
{
    fn make_move(&mut self, m: Move<Square8>) -> Result<Outcome, MoveError> {
        let mut promoted = false;
        let stm = self.side_to_move();
        let opponent = stm.flip();
        let (from, to) = m.info();
        let moved = self
            .piece_at(from)
            .ok_or(MoveError::Inconsistent("No piece found"))?;
        let captured = *self.piece_at(to);
        let outcome = Outcome::Checkmate { color: opponent };
        let legal_moves = self.legal_moves(&stm);

        if moved.color != stm {
            return Err(MoveError::Inconsistent(
                "The piece is not for the side to move",
            ));
        } else if self.game_status == outcome {
            return Err(MoveError::Inconsistent("Match is over."));
        }

        match captured {
            Some(_i) => {
                if moved.piece_type == PieceType::Pawn
                    && to.in_promotion_zone(moved.color)
                {
                    promoted = true;
                }
            }
            None => {
                if moved.piece_type == PieceType::Pawn
                    && to.in_promotion_zone(moved.color)
                {
                    promoted = true;
                }
            }
        }

        if let Some(attacks) = legal_moves.get(&from) {
            if (attacks & &to).is_empty() {
                return Err(MoveError::Inconsistent(
                    "The piece cannot move to there",
                ));
            }
        } else {
            return Err(MoveError::Inconsistent(
                "The piece cannot move to there",
            ));
        }

        let placed = if promoted {
            match moved.promote() {
                Some(promoted) => promoted,
                None => {
                    return Err(MoveError::Inconsistent(
                        "This type of piece cannot promote",
                    ));
                }
            }
        } else {
            moved
        };

        self.set_piece(from, None);
        self.set_piece(to, Some(placed));
        self.occupied_bb ^= &from;
        self.occupied_bb ^= &to;
        self.type_bb[moved.piece_type.index()] ^= &from;
        self.type_bb[placed.piece_type.index()] ^= &to;
        self.color_bb[moved.color.index()] ^= &from;
        self.color_bb[placed.color.index()] ^= &to;

        if let Some(ref cap) = captured {
            self.occupied_bb ^= &to;
            self.type_bb[cap.piece_type.index()] ^= &to;
            self.color_bb[cap.color.index()] ^= &to;
            //self.hand.increment(pc);
        }

        self.side_to_move = opponent;
        self.ply += 1;

        let move_record = MoveRecord::Normal {
            from,
            to,
            placed,
            captured,
            promoted,
        };

        self.move_history.push(move_record);

        self.log_position();
        self.detect_repetition()?;
        self.detect_insufficient_material()?;

        if self.is_checkmate(&self.side_to_move) {
            return Ok(Outcome::Checkmate {
                color: self.side_to_move.flip(),
            });
        } else if self.in_check(self.side_to_move) {
            return Ok(Outcome::Check {
                color: self.side_to_move,
            });
        } else if (&self.color_bb[self.side_to_move.flip().index()]
            & &self.type_bb[PieceType::King.index()])
            .count()
            == 0
        {
            return Ok(Outcome::Checkmate {
                color: self.side_to_move.flip(),
            });
        }
        self.is_stalemate(&self.side_to_move)?;
        Ok(Outcome::MoveOk)
    }

    fn file_bb(&self, file: usize) -> BB8<Square8> {
        FILE_BB[file]
    }
}

#[derive(Clone)]
pub struct PieceGrid([Option<Piece>; 64]);

impl PieceGrid {
    pub fn get(&self, sq: Square8) -> &Option<Piece> {
        &self.0[sq.index()]
    }

    pub fn set(&mut self, sq: Square8, pc: Option<Piece>) {
        self.0[sq.index()] = pc;
    }
}

impl Default for PieceGrid {
    fn default() -> Self {
        PieceGrid([None; 64])
    }
}

impl fmt::Debug for PieceGrid {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "PieceGrid {{ ")?;

        for pc in self.0.iter() {
            write!(fmt, "{pc:?} ")?;
        }
        write!(fmt, "}}")
    }
}

impl Default for P8<Square8, BB8<Square8>> {
    fn default() -> P8<Square8, BB8<Square8>> {
        P8 {
            side_to_move: Color::Black,
            board: PieceGrid([None; 64]),
            hand: Default::default(),
            ply: 0,
            move_history: Default::default(),
            sfen_history: Default::default(),
            occupied_bb: Default::default(),
            color_bb: Default::default(),
            type_bb: Default::default(),
            game_status: Outcome::MoveOk,
            variant: Variant::Standard,
            _a: PhantomData,
            _s: PhantomData,
        }
    }
}

impl fmt::Display for P8<Square8, BB8<Square8>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "+---+---+---+---+---+---+---+---+")?;

        for rank in (0..8).rev() {
            write!(f, "|")?;
            for file in 0..8 {
                if let Some(ref piece) =
                    *self.piece_at(Square8::new(file, rank).unwrap())
                {
                    write!(f, "{piece}")?;
                    let plinth: BB8<Square8> = &self.player_bb(Color::NoColor)
                        & &Square8::new(file, rank).unwrap();
                    if plinth.is_any() {
                        write!(f, " L|")?;
                    } else {
                        write!(f, "  |")?;
                    }
                } else {
                    let plinth: BB8<Square8> = &self.player_bb(Color::NoColor)
                        & &Square8::new(file, rank).unwrap();
                    if plinth.is_any() {
                        write!(f, "{:>3}|", "L")?;
                    } else {
                        write!(f, "   |")?;
                    }
                }
            }

            //writeln!(f, " {}", (('a' as usize + row as usize) as u8) as char)?;
            writeln!(f, "\n+---+---+---+---+---+---+---+---+")?;
        }
        writeln!(f, "a   b   c   d   e   f   g   h")?;
        writeln!(
            f,
            "Side to move: {}",
            if self.side_to_move == Color::Black {
                "Black"
            } else {
                "White"
            }
        )?;

        let fmt_hand = |color: Color, f: &mut fmt::Formatter| -> fmt::Result {
            for pt in PieceType::iter().filter(|pt| pt.is_hand_piece()) {
                let pc = Piece {
                    piece_type: pt,
                    color,
                };
                let n = self.hand.get(pc);

                if n > 0 {
                    write!(f, "{pc}{n} ")?;
                }
            }
            Ok(())
        };
        write!(f, "Hand (Black): ")?;
        fmt_hand(Color::Black, f)?;
        writeln!(f)?;

        write!(f, "Hand (White): ")?;
        fmt_hand(Color::White, f)?;
        writeln!(f)?;

        write!(f, "Ply: {}", self.ply)?;

        Ok(())
    }
}

#[cfg(test)]
pub mod position_tests {

    use crate::{
        attacks::Attacks,
        bitboard::BitBoard,
        piece_type::PieceType,
        position::{Board, MoveType, Outcome, Placement, Play, Sfen},
        shuuro8::{
            attacks8::Attacks8,
            position8::P8,
            square8::{consts::*, Square8},
        },
        square::Square,
        Color, Move, Piece, Shop, Variant,
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
            assert_eq!(b.count(), 1);
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
            ("8/PPPPPPPP/8/8/8/8/pppppppp/8 b 2RAC2NQK2rac2nqk 1", 8),
            ("8/PPPPPPPP/8/8/8/8/pppppppp/8 b 2R2BGAQK2r2bgaqk 1", 8),
        ];
        for case in cases {
            let mut pos = P8::new();
            pos.set_sfen(case.0).expect("failed to parse sfen string");
            pos.update_variant(Variant::StandardFairy);
            assert_eq!(pos.get_hand(Color::Black).len(), case.1);
            assert_eq!(pos.get_hand(Color::Black).len(), case.1);
        }
    }

    #[test]
    fn move_candidates() {
        setup();

        let cases = [("RNBQKBNR/PPPPPPPP/8/8/8/8/pppppppp/rnbqkbnr b - 1", 12)];
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

            assert_eq!(12, case.1);
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
}
