use std::{fmt, marker::PhantomData};

use crate::{
    attacks::Attacks,
    bitboard::BitBoard,
    position::{Board, Outcome, Placement, Play, Position, Rules, Sfen},
    Color, Hand, Move, MoveData, Piece, PieceType, SfenError, Square, Variant,
};

use super::{
    attacks12::Attacks12,
    bitboard12::BB12,
    board_defs::{FILE_BB, RANK_BB},
    plinths_set12::PlinthGen12,
    square12::{
        consts::{C1, C12, J1, J12},
        Square12,
    },
};

impl Position<Square12, BB12<Square12>, Attacks12<Square12, BB12<Square12>>>
    for P12<Square12, BB12<Square12>>
{
}

#[derive(Clone, Debug)]
pub struct P12<S, B>
where
    S: Square,
    B: BitBoard<S>,
{
    board: PieceGrid,
    hand: Hand,
    ply: u16,
    side_to_move: Color,
    move_history: Vec<Move<Square12>>,
    occupied_bb: BB12<Square12>,
    color_bb: [BB12<Square12>; 3],
    game_status: Outcome,
    variant: Variant,
    pub type_bb: [BB12<Square12>; 10],
    _a: PhantomData<B>,
    _s: PhantomData<S>,
}

impl<S, B> P12<S, B>
where
    S: Square,
    B: BitBoard<S>,
{
    //
}

impl Rules<Square12, BB12<Square12>, Attacks12<Square12, BB12<Square12>>>
    for P12<Square12, BB12<Square12>>
{
}

impl Board<Square12, BB12<Square12>, Attacks12<Square12, BB12<Square12>>>
    for P12<Square12, BB12<Square12>>
{
    fn new() -> Self {
        Default::default()
    }

    fn set_piece(&mut self, sq: Square12, p: Option<Piece>) {
        self.board.set(sq, p)
    }

    fn piece_at(&self, sq: Square12) -> &Option<Piece> {
        self.board.get(sq)
    }

    fn player_bb(&self, c: Color) -> BB12<Square12> {
        self.color_bb[c.index()]
    }

    fn occupied_bb(&self) -> BB12<Square12> {
        self.occupied_bb
    }

    fn type_bb(&self, pt: &PieceType) -> BB12<Square12> {
        self.type_bb[pt.index()]
    }

    fn xor_player_bb(&mut self, color: Color, sq: Square12) {
        self.color_bb[color.index()] ^= &sq;
    }

    fn xor_type_bb(&mut self, piece_type: PieceType, sq: Square12) {
        self.type_bb[piece_type.index()] ^= &sq;
    }

    fn xor_occupied(&mut self, sq: Square12) {
        self.occupied_bb ^= &sq;
    }

    fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    fn empty_all_bb(&mut self) {
        self.occupied_bb = BB12::empty();
        self.color_bb = Default::default();
        self.type_bb = Default::default();
    }

    fn sfen_to_bb(&mut self, piece: Piece, sq: &Square12) {
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

    fn insert_sfen(&mut self, sfen: Move<Square12>) {
        self.move_history.push(sfen);
    }

    fn insert_move(&mut self, move_record: Move<Square12>) {
        self.move_history.push(move_record)
    }

    fn clear_sfen_history(&mut self) {
        self.move_history.clear();
    }

    fn set_move_history(&mut self, history: Vec<Move<Square12>>) {
        self.move_history = history;
    }

    fn move_history(&self) -> &[Move<Square12>] {
        &self.move_history
    }

    fn update_last_move(&mut self, m: &str) {
        if let Some(last) = self.move_history.last_mut() {
            match last {
                Move::Put { ref mut fen, .. } => {
                    *fen = String::from(m);
                }
                Move::Normal { ref mut fen, .. } => {
                    *fen = String::from(m);
                }
                _ => (),
            }
        }
    }

    fn hand(&self, p: Piece) -> u8 {
        self.hand.get(p)
    }

    fn get_hand(&self, c: Color, long: bool) -> String {
        self.hand.to_sfen(c, long)
    }

    fn set_hand(&mut self, s: &str) {
        self.hand.set_hand(s);
    }

    fn decrement_hand(&mut self, p: Piece) {
        self.hand.decrement(p);
    }

    fn dimensions(&self) -> u8 {
        12
    }
}

impl Sfen<Square12, BB12<Square12>, Attacks12<Square12, BB12<Square12>>>
    for P12<Square12, BB12<Square12>>
{
    fn clear_hand(&mut self) {
        self.hand.clear();
    }

    fn new_hand(&mut self, hand: Hand) {
        self.hand = hand;
    }

    fn insert_in_hand(&mut self, p: Piece, num: u8) {
        self.hand.set(p, num);
    }

    fn parse_sfen_ply(&mut self, s: &str) -> Result<(), SfenError> {
        self.ply = s.parse()?;
        Ok(())
    }

    fn update_player(&mut self, piece: Piece, sq: &Square12) {
        self.set_piece(*sq, Some(piece));
        self.occupied_bb |= sq;
        self.color_bb[piece.color.index()] |= sq;
        self.type_bb[piece.piece_type.index()] |= sq;
    }
}

impl Placement<Square12, BB12<Square12>, Attacks12<Square12, BB12<Square12>>>
    for P12<Square12, BB12<Square12>>
{
    fn generate_plinths(&mut self) {
        let bb = PlinthGen12::default().start();
        self.color_bb[Color::NoColor.index()] = bb;
    }

    fn king_files(&self, c: &Color) -> BB12<Square12> {
        match c {
            Color::Black => Attacks12::between(C12, J12),
            Color::White => Attacks12::between(C1, J1),
            Color::NoColor => BB12::empty(),
        }
    }

    fn rank_bb(&self, file: usize) -> BB12<Square12> {
        RANK_BB[file]
    }

    fn update_bb(&mut self, p: Piece, sq: Square12) {
        self.set_piece(sq, Some(p));
        self.occupied_bb |= &sq;
        self.color_bb[p.color.index()] |= &sq;
        self.type_bb[p.piece_type.index()] |= &sq;
    }

    fn empty_placement_board() -> String {
        String::from("57/57/57/57/57/57/57/57/57/57/57/57 w")
    }
}

impl Play<Square12, BB12<Square12>, Attacks12<Square12, BB12<Square12>>>
    for P12<Square12, BB12<Square12>>
{
    fn file_bb(&self, file: usize) -> BB12<Square12> {
        FILE_BB[file]
    }

    fn game_status(&self) -> Outcome {
        self.game_status.clone()
    }

    fn update_after_move(
        &mut self,
        from: Square12,
        to: Square12,
        placed: Piece,
        moved: Piece,
        captured: Option<Piece>,
        opponent: Color,
        mut move_data: MoveData,
    ) -> MoveData {
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
            move_data = move_data.captured(captured);
            //self.hand.increment(pc);
        }

        self.side_to_move = opponent;
        self.ply += 1;
        move_data
    }
}

#[derive(Clone)]
pub struct PieceGrid([Option<Piece>; 144]);

impl PieceGrid {
    pub fn get(&self, sq: Square12) -> &Option<Piece> {
        &self.0[sq.index()]
    }

    pub fn set(&mut self, sq: Square12, pc: Option<Piece>) {
        self.0[sq.index()] = pc;
    }
}

impl Default for PieceGrid {
    fn default() -> Self {
        PieceGrid([None; 144])
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

impl Default for P12<Square12, BB12<Square12>> {
    fn default() -> P12<Square12, BB12<Square12>> {
        P12 {
            side_to_move: Color::Black,
            board: PieceGrid([None; 144]),
            hand: Default::default(),
            ply: 0,
            move_history: Default::default(),
            // sfen_history: Default::default(),
            occupied_bb: Default::default(),
            color_bb: Default::default(),
            type_bb: Default::default(),
            game_status: Outcome::MoveOk,
            variant: Variant::Shuuro,
            _a: PhantomData,
            _s: PhantomData,
        }
    }
}

impl fmt::Display for P12<Square12, BB12<Square12>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "+---+---+---+---+---+---+---+---+---+---+---+---+")?;

        for rank in (0..12).rev() {
            write!(f, "|")?;
            for file in 0..12 {
                if let Some(sq) = Square12::new(file, rank) {
                    if let Some(ref piece) = *self.piece_at(sq) {
                        write!(f, "{piece}")?;
                        let plinth: BB12<Square12> =
                            self.player_bb(Color::NoColor) & &sq;
                        if plinth.is_any() {
                            write!(f, " L|")?;
                        } else {
                            write!(f, "  |")?;
                        }
                    } else {
                        let plinth: BB12<Square12> =
                            self.player_bb(Color::NoColor) & &sq;
                        if plinth.is_any() {
                            write!(f, "{:>3}|", "L")?;
                        } else {
                            write!(f, "   |")?;
                        }
                    }
                }
            }

            //writeln!(f, " {}", (('a' as usize + row as usize) as u8) as char)?;
            writeln!(f, "\n+---+---+---+---+---+---+---+---+---+---+---+---+")?;
        }
        writeln!(f, "a   b   c   d   e   f   g   h   i   j   k   l")?;
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
        shuuro12::{
            attacks12::Attacks12,
            position12::P12,
            square12::{consts::*, Square12},
        },
        square::Square,
        Color, Move, Piece, Selection, Variant,
    };

    pub const START_POS: &str = "kr10/12/12/12/12/12/12/12/12/12/12/KR10 b - 1";

    fn setup() {
        Attacks12::init();
    }

    #[test]
    fn piece_exist() {
        setup();
        let mut pos = P12::new();
        pos.set_sfen(START_POS)
            .expect("failed to parse SFEN string");
        if let Some(sq) = Square12::from_index(132) {
            let piece = Piece {
                piece_type: PieceType::King,
                color: Color::Black,
            };
            assert_eq!(Some(piece), *pos.piece_at(sq));
        }
    }

    #[test]
    fn player_bb() {
        setup();
        type CasePlayerBB<'a> =
            [(&'a str, &'a [Square12], &'a [Square12], &'a [Square12])];
        let cases: &CasePlayerBB = &[
            (
                "1_.5k4/12/_._._.9/nnq9/5ppp4/12/12/3_.8/4R7/2_.3_.5/12/BBQ8K b - 1",
                &[A9, B9, C9, F8, G8, H8, H12],
                &[A1, B1, C1, L1, E4],
                &[G3, B12, C3, D5],
            ),
            (
                "12/12/12/7qk3/5ppp4/12/12/12/3Q6N1/6PPPP2/12/9K2 b P 1",
                &[H9, I9, F8, G8, H8],
                &[G3, H3, I3, J3, D4, K4, J1],
                &[],
            ),
        ];

        let mut pos = P12::new();
        for case in cases {
            pos.set_sfen(case.0).expect("faled to parse SFEN string");
            let blue = pos.player_bb(Color::Black);
            let red = pos.player_bb(Color::White);

            assert_eq!(case.1.len(), { blue.len() as usize });
            for sq in case.1 {
                assert!((blue & sq).is_any());
            }

            assert_eq!(case.2.len(), { red.len() as usize });
            for sq in case.2 {
                assert!((red & sq).is_any());
            }

            for sq in case.3 {
                assert!((pos.player_bb(Color::NoColor) & sq).is_any())
            }
        }
    }

    #[test]
    fn pinned_bb() {
        setup();

        let cases: &[(&str, &[Square12], &[Square12])] = &[
            (
                "12/12/1k10/pp10/12/12/8r3/12/12/12/8B3/5NNQK3 w - 1",
                &[],
                // &[],
                &[I2],
            ),
            (
                "12/12/1k10/pp10/12/12/8r3/12/12/8R3/8B3/5NNQK3 w - 1",
                &[],
                &[],
            ),
            (
                "12/12/2k3q2R2/2rn8/12/2R2B6/12/12/PPPPPP/2K9/12/12 b - 1",
                &[C9, D9, G10],
                &[],
            ),
        ];

        let mut pos = P12::new();
        for case in cases {
            pos.set_sfen(case.0).expect("faled to parse SFEN string");
            let black = pos.pinned_bb(Color::Black);
            let white = pos.pinned_bb(Color::White);

            assert_eq!(case.1.len(), black.len() as usize);
            for sq in case.1 {
                assert!((black & sq).is_any());
            }

            assert_eq!(case.2.len(), white.len() as usize);
            for sq in case.2 {
                assert!((white & sq).is_any());
            }
        }
    }

    #[test]
    fn pawn_vs_knight() {
        setup();
        let sfen = "3kqbr5/_.11/q11/8_.3/3_.8/7_.4/12/12/12/3pP_.4_.1/2_N2K3P2/6_.3B1 b - 38";
        let mut pos = P12::new();
        pos.set_sfen(sfen).expect("failed to parse SFEN string");
        let lm = pos.legal_moves(&Color::Black);
        if let Some(b) = lm.get(&D3) {
            assert!(b.len() == 1);
        }
    }

    #[test]
    fn pawn_not_pinned() {
        setup();
        let mut pos = P12::new();
        pos.set_sfen("6k_.4/_.11/12/2q9/1_.10/10_.1/_.3_.7/6P5/4P6_./8_.Q2/9K2/12 w - 55")
            .expect("failed to parse SFEN string");
        let lm = pos.legal_moves(&Color::White);
        if let Some(b) = lm.get(&G5) {
            assert_eq!(b.len(), 1);
        }
    }

    #[test]
    fn pawn_check_king() {
        setup();
        let mut pos = P12::new();
        pos.set_sfen(
            "7_.4/_.11/4k7/5P6/1_.10/10_.1/_.3_.7/5Q6/11_./8_.3/9K2/12 b - 72",
        )
        .expect("failed to parse SFEN string");
        let in_check = pos.in_check(Color::Black);
        assert!(in_check);
    }

    #[test]
    fn legal_moves_pawn() {
        setup();
        let cases = [(
            "6k5/5p6/_.1_.9/6_.5/9_.2/12/10_.1/6q5/6P5/2_.9/4_.7/4K1Q4_N w - 11",
            G4,
            0,
        )];
        for case in cases {
            let mut pos = P12::default();
            pos.set_sfen(case.0).expect("failed to parse sfen string");
            let legal_moves = pos.legal_moves(&Color::White);
            if let Some(b) = legal_moves.get(&case.1) {
                assert_eq!(b.len(), case.2);
            }
        }
    }

    #[test]
    fn check_while_pinned() {
        setup();
        let mut pos = P12::default();
        pos.set_sfen("5k1Q4/_.3pr6/7B4/12/7_.4/1_.3Q4_.1/8n_.2/5_.6/2_.9/3N8/10P_./5K6 b - 50")
            .expect("failed to parse sfen string");
        let legal_moves = pos.legal_moves(&Color::Black);
        if let Some(b) = legal_moves.get(&F11) {
            assert_eq!(b.len(), 0);
        }
    }

    #[test]
    fn king_moves() {
        setup();
        let cases = [
            (
                "12/5p6/_.1_.9/6_.5/5k3_.2/6P5/6Q3_.1/12/12/2_.9/4_.7/4K6_N b - 19",
                4,
            ),
            (
                "12/5p6/_.1_.9/6_.5/R4k3_.2/6P5/6Q3_.1/12/12/2_.9/4_.7/4K6_N b - 19",
                3,
            ),
        ];
        for i in cases {
            let mut pos = P12::default();
            pos.set_sfen(i.0).expect("failed to parse sfen string");
            let legal_moves = pos.legal_moves(&Color::Black);
            if let Some(b) = legal_moves.get(&F8) {
                assert_eq!(b.len(), i.1);
            }
        }
    }

    #[test]
    fn parse_sfen_hand() {
        setup();
        let mut pos = P12::new();
        pos.set_sfen("12/1_.10/12/12/3_.8/6_.4_./4_.7/9_.2/12/12/3_.8/6K_.4 b kr12pQ9P 1")
            .expect("failed to parse sfen string");
        assert_eq!(pos.hand(Piece::from_sfen('p').unwrap()), 12);
        assert_eq!(pos.hand(Piece::from_sfen('P').unwrap()), 9);
    }

    #[test]
    fn move_candidates2() {
        setup();

        let mut pos = P12::new();
        pos.set_sfen("12/12/12/4k7/bppp8/12/12/12/12/12/4K7/R3N7 b - 1")
            .expect("failed to parse SFEN string");

        let mut sum = 0;
        for sq in Square12::iter() {
            let pc = pos.piece_at(sq);

            if let Some(pc) = *pc {
                if pc.color == pos.side_to_move() {
                    sum += pos.move_candidates(&sq, pc, MoveType::Plinth).len();
                }
            }
        }
        assert_eq!(21, sum);
    }

    #[test]
    fn move_candidates_plinth() {
        setup();
        let cases = [
            ("f2", PieceType::Rook, Color::White, 13, "f3", "Live"),
            ("e7", PieceType::Knight, Color::Black, 7, "g8", "Live"),
            ("f6", PieceType::Pawn, Color::Black, 0, "f7", ""),
        ];
        let mut pos = P12::new();
        pos.set_sfen("12/12/12/10R1/12/4n7/5p4b1/3b1_.4k1/12/12/5R5K/12 w - 1")
            .expect("failed to parse SFEN string");
        for case in cases {
            let bb = pos.move_candidates(
                &Square12::from_sfen(case.0).unwrap(),
                Piece {
                    piece_type: case.1,
                    color: case.2,
                },
                MoveType::Plinth,
            );
            assert_eq!(case.3, bb.len());
            let result = pos.play(case.0, case.4);
            if let Ok(result) = result {
                assert_eq!(result.to_string(), case.5);
            } else {
                assert!(result.is_err());
            }
        }
    }

    #[test]
    fn check_while_knight_on_plinth() {
        setup();
        let sfen = "1q3kn5/3p8/5_n5_./12/1_.10/11_./12/4_.4_.2/2_.2Q6/12/5P2_.3/4K5B1 b - 11";
        let mut pos = P12::new();
        pos.set_sfen(sfen).expect("failed to parse SFEN string");
        let legal_moves = pos.legal_moves(&Color::Black);
        if let Some(b) = legal_moves.get(&F10) {
            assert_eq!(b.len(), 6);
        }
    }

    #[test]
    fn pawn_moves() {
        setup();
        let cases = [
            (
                "6k5/5p6/_.1_.9/6_.5/9_.2/12/10_.1/6P5/12/2_.9/4_.7/4K1Q4_N b - 12",
                "f11",
                "f10",
            ),
            (
                "4q_nk2r1b/2pp2_.5/8_.3/3_.8/12/12/_.11/12/12/10_.1/2P4P3P/2N1_.QKR_.1N1 b - 15",
                "c11",
                "c10",
            ),
            (
                "4q_nk2r1b/2pp2_.5/8_.3/3_.8/12/12/_.11/12/12/10_.1/2P4P3P/2N1_.QKR_.1N1 b - 15",
                "d11",
                "d10",
            ),
            (
                "_.2q1k3r2/p7ppp1/6_.5/12/12/4_.1_.5/6_.5/11_./3_.8/1P10/4_.P1P1PP1/4KN1Q4 b - 16",
                "a11",
                "a10",
            ),
            ("_.2kq7/ppp_.p1ppp1pp/ppp2p2pQ2/6_.5/12/9_.2/12/1_.10/7_.4/12/6_.3P1/2_NN2KNBB2 b - 29",
             "i11",
             "j10")
        ];
        let ng_cases = [(
            "6k5/5p6/_.1_.9/6_.5/9_.2/12/10_.1/6P5/12/2_.9/2P1_.7/4K1Q4_N w - 12",
            "c2",
            "c3",
        )];

        for case in cases {
            let mut position = P12::new();
            position
                .set_sfen(case.0)
                .expect("failed to parse sfen string");

            let played = position.play(case.1, case.2);
            assert!(played.is_ok());
        }

        for case in ng_cases {
            let mut position = P12::new();
            position
                .set_sfen(case.0)
                .expect("failed to parse sfen string");

            let played = position.play(case.1, case.2);
            assert!(played.is_err());
        }
    }

    #[test]
    fn pawn_captures_last_rank() {
        setup();
        let sfen =
            "_.1r9/pP3k2ppp1/6_.5/12/12/4_.1_.1q3/6_.5/11_./3_.8/8r3/4_.P1P1PP1/4KN1Q4 w - 33";
        let mut position = P12::new();
        position
            .set_sfen(sfen)
            .expect("failed to parse sfen string");
        let pawn_moves = position.legal_moves(&Color::White);
        if let Some(b) = pawn_moves.get(&B11) {
            assert_eq!(b.len(), 2);
        }
        let result = position.play("b11", "c12");
        assert!(result.is_ok());
        assert_eq!(
            position.piece_at(C12).unwrap().piece_type,
            PieceType::Queen
        );
    }

    #[test]
    fn queen_moves_through() {
        setup();
        let cases = [
            (
                "5_nk2r1b/2pp2_.5/8_.3/3_.8/12/12/_.11/12/12/7P2_.1/2P8P/2q1_.QKR_.1N1 b - 20",
                C1,
                15,
            ),
            (
                "2n1rqk3b1/_.4p1pp3/3p8/6_.5/3_.6_.1/12/2_.6_.2/12/12/_.6_N4/5R3P2/2B2KQ5 b - 17",
                F12,
                7,
            ),
        ];
        for case in cases {
            let mut position = P12::new();
            position
                .set_sfen(case.0)
                .expect("failed to parse sfen string");
            let queen_moves = position.legal_moves(&Color::White);
            if let Some(b) = queen_moves.get(&case.1) {
                assert_eq!(b.len(), case.2);
            }
        }
    }

    #[test]
    fn knight_jumps_move() {
        setup();
        let sfen =
            "12/5p6/_.1_.9/5k_.5/9_.2/6P5/10_.1/12/12/2_.9/4_.7/4K1Q4_N w - 17";
        let mut position = P12::new();
        position
            .set_sfen(sfen)
            .expect("failed to parse sfen string");

        let result = position.play("l1", "j2");
        assert!(result.is_ok());
    }

    #[test]
    fn in_check() {
        setup();

        let test_cases = [
            (
                "qkb9/1ppp8/12/12/12/12/12/12/12/12/1PPP8/KQR9 w - 1",
                false,
                true,
            ),
            (
                "5k6/12/12/12/12/12/12/12/12/12/1K10/5QR5 b - 1",
                true,
                false,
            ),
            (
                "2rnbkqbnr2/12/2pppppppp2/12/12/12/12/12/12/2PPPPPPPP2/12/2RNBKQBNR2 b - 1",
                false,
                false,
            ),
            (
                "11k/12/12/7q4/nbq9/12/12/12/7_.4/QP10/7_.4/RR5K4 w - 1",
                false,
                false,
            ),
            ("12/12/12/12/k11/12/12/12/12/12/2n8/KQP8 w - 1", false, true),
        ];

        let mut pos = P12::new();
        for case in test_cases.iter() {
            pos.set_sfen(case.0).expect("failed to parse sfen string");
            assert_eq!(case.1, pos.in_check(Color::Black));
            assert_eq!(case.2, pos.in_check(Color::White));
        }
    }

    #[test]
    fn is_stalemate() {
        setup();
        let sfen = "12/12/12/11k/12/12/12/12/3q8/K11/2r9/12 b - 1";
        let mut pos = P12::new();
        pos.set_sfen(sfen).expect("failed to parse sfen string");
        let res = pos.play("d4", "c4");
        if let Ok(res) = res {
            assert_eq!(res.to_string(), "Stalemate");
        }
    }

    #[test]
    fn is_checkmate() {
        setup();
        let cases = [
            (
                "12/12/k11/12/12/12/12/12/12/12/9rr1/1K8r1 b - 1",
                true,
                Color::White,
            ),
            (
                "9k2/12/12/ppppp7/12/12/12/12/12/6B5/5K4r1/5RNB4 w - 1",
                false,
                Color::Black,
            ),
            (
                "12/12/12/12/12/12/KRn9/12/12/7k3Q/12/12 b - 1",
                false,
                Color::White,
            ),
            (
                "6n5/_.11/4Qk6/6_N5/3_.5B_.1/12/2_.6_.2/12/12/_.6_.4/9K2/12 b - 69",
                true,
                Color::Black,
            ),
        ];
        for case in cases.iter() {
            let mut pos = P12::new();
            pos.set_sfen(case.0).expect("failed to parse SFEN string");
            assert_eq!(case.1, pos.is_checkmate(&case.2));
        }
    }

    #[test]
    fn repetition() {
        setup();

        let mut pos = P12::new();
        pos.set_sfen("12/12/12/2kr8/4pp6/12/12/12/7RR3/PPPQP4K2/12/12 b - 1")
            .expect("failed to parse SFEN string");
        for i in 0..5 {
            assert!(pos.make_move(Move::new(D9, I9)).is_ok());
            assert!(pos.make_move(Move::new(H4, A4)).is_ok());
            assert!(pos.make_move(Move::new(I9, D9)).is_ok());
            assert!(pos.make_move(Move::new(A4, H4)).is_ok());
            if i == 1 {
                assert!(pos.make_move(Move::new(A4, H4)).is_err());
                break;
            }
        }
    }

    #[test]
    fn make_move() {
        setup();

        let base_sfen =
            "12/12/6k5/12/qbbn8/12/12/12/12/5PP5/3KRRB5/12 w K2RB2P 1";
        let test_cases = [
            (
                D2,
                E1,
                false,
                true,
                "12/12/6k5/12/qbbn8/12/12/12/12/5PP5/4RRB5/4K7 b K2RB2P 2",
            ),
            (
                E2,
                E7,
                false,
                true,
                "12/12/6k5/12/qbbn8/4R7/12/12/12/5PP5/3K1RB5/12 b K2RB2P 2",
            ),
            (
                G2,
                I4,
                false,
                true,
                "12/12/6k5/12/qbbn8/12/12/12/8B3/5PP5/3KRR6/12 b K2RB2P 2",
            ),
            (
                F2,
                F1,
                false,
                true,
                "12/12/6k5/12/qbbn8/12/12/12/12/5PP5/3KR1B5/5R6 b K2RB2P 2",
            ),
            (G3, H3, false, false, base_sfen),
        ];

        for case in test_cases.iter() {
            let mut pos = P12::new();
            pos.set_sfen(base_sfen)
                .expect("failed to parse SFEN string");
            let move_ = Move::new(case.0, case.1);
            assert_eq!(case.3, pos.make_move(move_).is_ok());
            assert_eq!(case.4, pos.generate_sfen());
        }

        let mut pos = P12::new();
        // Leaving the checked king is illegal.
        pos.set_sfen("12/12/12/12/12/12/r9k1/12/12/12/1K8RR/12 b kr 1")
            .expect("failed to parse SFEN string");
        let move_ = Move::new(A6, A1);
        assert!(pos.make_move(move_).is_err());

        pos.set_sfen("12/12/12/12/12/12/r9k1/12/12/12/1RR9/7K4 b kr 1")
            .expect("failed to parse SFEN string");
        let move_ = Move::new(K6, K5);
        assert!(pos.make_move(move_).is_ok());
    }

    #[test]
    fn pawn_promoted() {
        setup();
        let mut pos = P12::new();
        pos.set_sfen("2r2k1n4/5_.4_.1/7_.4/12/5_.6/12/9_.2/1_.5_.4/5R2B3/12/1_.1p1N6/7K4 b - 28")
            .expect("failed to parse SFEN string");
        let move_ = Move::from_sfen("d2_d1").unwrap();
        assert!(pos.make_move(move_).is_ok());
    }

    #[test]
    fn make_moves() {
        setup();
        let mut pos = P12::new();
        pos.set_sfen("12/12/12/12/p11/12/P_.10/12/6k5/12/12/6K5 w - 1")
            .expect("err");
        let m = Move::new(G1, G2);
        let m2 = Move::new(G4, G5);
        let m3 = Move::new(G5, G4);
        assert!(pos.make_move(m).is_ok());
        assert!(pos.make_move(m2).is_ok());
        assert!(pos.make_move(m3).is_err());
    }

    #[test]
    fn set_sfen_normal() {
        setup();

        let mut pos = P12::new();

        pos.set_sfen("2rnbkqbnr2/12/2pppppppp2/12/12/12/12/12/12/2PPPPPPPP2/12/2RNBKQBNR2 b - 1")
            .expect("failed to parse SFEN string");
        let filled_squares = [
            (0, 2, PieceType::Rook, Color::White),
            (0, 3, PieceType::Knight, Color::White),
            (0, 4, PieceType::Bishop, Color::White),
            (0, 5, PieceType::King, Color::White),
            (0, 6, PieceType::Queen, Color::White),
            (0, 7, PieceType::Bishop, Color::White),
            (0, 8, PieceType::Knight, Color::White),
            (0, 9, PieceType::Rook, Color::White),
            (2, 2, PieceType::Pawn, Color::White),
            (2, 3, PieceType::Pawn, Color::White),
            (2, 4, PieceType::Pawn, Color::White),
            (2, 5, PieceType::Pawn, Color::White),
            (2, 6, PieceType::Pawn, Color::White),
            (2, 7, PieceType::Pawn, Color::White),
            (2, 8, PieceType::Pawn, Color::White),
            (2, 9, PieceType::Pawn, Color::White),
            (9, 2, PieceType::Pawn, Color::Black),
            (9, 3, PieceType::Pawn, Color::Black),
            (9, 4, PieceType::Pawn, Color::Black),
            (9, 5, PieceType::Pawn, Color::Black),
            (9, 6, PieceType::Pawn, Color::Black),
            (9, 7, PieceType::Pawn, Color::Black),
            (9, 8, PieceType::Pawn, Color::Black),
            (9, 9, PieceType::Pawn, Color::Black),
            (11, 2, PieceType::Rook, Color::Black),
            (11, 3, PieceType::Knight, Color::Black),
            (11, 4, PieceType::Bishop, Color::Black),
            (11, 5, PieceType::King, Color::Black),
            (11, 6, PieceType::Queen, Color::Black),
            (11, 7, PieceType::Bishop, Color::Black),
            (11, 8, PieceType::Knight, Color::Black),
            (11, 9, PieceType::Rook, Color::Black),
        ];

        let empty_squares = [(0, 0, 12), (0, 11, 12), (5, 5, 4)];

        let hand_pieces = [
            (PieceType::Pawn, 0),
            (PieceType::Queen, 0),
            (PieceType::Knight, 0),
            (PieceType::Rook, 0),
            (PieceType::Bishop, 0),
        ];

        for case in filled_squares.iter() {
            let (row, file, pt, c) = *case;
            assert_eq!(
                Some(Piece {
                    piece_type: pt,
                    color: c,
                }),
                *pos.piece_at(Square12::new(file, row).unwrap())
            );
        }

        for case in empty_squares.iter() {
            let (row, file, len) = *case;
            for i in row..(row + len) {
                assert_eq!(
                    None,
                    *pos.piece_at(Square12::new(file, i).unwrap())
                );
            }
        }

        for case in hand_pieces.iter() {
            let (pt, n) = *case;
            assert_eq!(
                n,
                pos.hand(Piece {
                    piece_type: pt,
                    color: Color::Black,
                })
            );
            assert_eq!(
                n,
                pos.hand(Piece {
                    piece_type: pt,
                    color: Color::White,
                })
            );
        }

        assert_eq!(Color::Black, pos.side_to_move());
        assert_eq!(1, pos.ply());
    }

    #[test]
    fn all_legal_moves() {
        setup();
        let cases = [
            ("12/7k4/12/5n6/12/12/12/1Q10/12/K11/12/12 b - 1", "f4", 0),
            ("12/7k4/12/5n6/12/12/12/2Q9/12/K11/12/12 b - 1", "f4", 8),
            (
                "12/8k3/12/8r3/12/12/12/6pp4/8R3/6p5/3PPPPP4/8K3 w - 1",
                "i4",
                7,
            ),
            (
                "12/8k3/12/8r3/12/12/12/6pp4/10R1/6p5/3PPPPP4/8K3 w - 1",
                "k4",
                1,
            ),
            ("12/12/2k9/12/1R10/12/12/12/12/12/1qq9/1K10 w - 1", "b8", 0),
            ("12/12/2k9/12/1R10/12/12/12/12/12/q1q9/1K10 w - 1", "b8", 0),
            (
                "12/8_.3/5_.1k4/12/_.9_.1/12/12/4_.7/12/1n3q3_.2/2K3_.3P1/_.11 b - 48",
                "b3",
                6,
            ),
        ];
        for case in cases {
            let mut pos = P12::new();
            pos.set_sfen(case.0).expect("error while parsing sfen");
            let color = pos.side_to_move();
            let moves = pos.legal_moves(&color);
            if let Some(b) = moves.get(&Square12::from_sfen(case.1).unwrap()) {
                assert_eq!(b.len(), case.2);
            }
        }
    }

    #[test]
    fn king_squares() {
        let position_set = P12::default();
        let cases = [
            (Color::Black, 6, [D12, E12, F12, G12, H12, I12]),
            (Color::White, 6, [D1, E1, F1, G1, H1, I1]),
        ];
        for case in cases {
            let bb = position_set.king_squares(&case.0);
            assert_eq!(bb.len(), case.1);
            for sq in case.2 {
                assert!((bb & &sq).is_any());
            }
        }
    }

    #[test]
    fn is_hand_empty() {
        setup();
        let mut position_set = P12::default();
        position_set
            .parse_sfen_board("7k4/12/12/12/12/12/12/12/12/12/12/6K5")
            .expect("error while parsing sfen");
        position_set.set_hand("rrRqNqq");
        let cases = [
            (PieceType::Queen, Color::Black, D12),
            (PieceType::Rook, Color::White, C1),
            (PieceType::Queen, Color::Black, I12),
            (PieceType::Knight, Color::White, H1),
            (PieceType::Rook, Color::Black, B12),
            (PieceType::Rook, Color::Black, G12),
            (PieceType::Queen, Color::Black, F12),
        ];
        for case in cases {
            position_set.place(
                Piece {
                    piece_type: case.0,
                    color: case.1,
                },
                case.2,
            );
        }
        assert!(position_set.is_hand_empty(Color::Black, PieceType::Plinth));
        assert!(position_set.is_hand_empty(Color::White, PieceType::Plinth));
    }

    #[test]
    fn place_king() {
        setup();
        let mut position_set = P12::default();
        position_set
            .set_sfen("12/12/12/12/12/12/12/12/12/12/12/7_.4 w K 1")
            .expect("error");
        let cases = [(A1, 0), (B3, 0), (H1, 0), (G1, 1)];
        for case in cases {
            let piece = Piece {
                piece_type: PieceType::King,
                color: Color::White,
            };
            position_set.place(piece, case.0);
            assert_eq!(position_set.player_bb(Color::White).len(), case.1);
        }
    }

    #[test]
    fn generate_plinths() {
        setup();
        let mut position_set = P12::default();
        position_set.generate_plinths();
        assert_eq!(position_set.player_bb(Color::NoColor).len(), 8);
    }

    #[test]
    fn flip_empty_side() {
        setup();
        let mut position = P12::default();
        position.set_sfen("12/2_.9/4_.2_.1_.2/12/12/12/12/6_.5/_.1_.5_.3/12/12/6K5 b krqpR2N3BQ 1").expect("sfen has wrong data");
        let moves = [
            (Color::Black, PieceType::King, H12),
            (Color::White, PieceType::Queen, A1),
            (Color::Black, PieceType::Queen, E12),
            (Color::White, PieceType::Rook, D1),
            (Color::Black, PieceType::Rook, C12),
            (Color::White, PieceType::Bishop, B2),
            (Color::Black, PieceType::Pawn, G11),
        ];
        for m in moves {
            position.place(
                Piece {
                    piece_type: m.1,
                    color: m.0,
                },
                m.2,
            );
        }
        assert_eq!(position.side_to_move(), Color::White);
    }

    #[test]
    fn empty_squares() {
        setup();
        let mut position_set = P12::default();
        position_set
            .parse_sfen_board(
                "_.4kqq_n3/12/12/12/12/12/12/12/5_.6/12/4PPPP4/5KRRR3",
            )
            .expect("error while parsing sfen");
        position_set.set_hand("NrNNbrn");
        let cases = [
            (PieceType::Knight, Color::Black, 8),
            (PieceType::Bishop, Color::Black, 7),
        ];
        for case in cases {
            let file = position_set.empty_squares(Piece {
                piece_type: case.0,
                color: case.1,
            });
            assert_eq!(file.unwrap_or_default().len(), case.2);
        }
        assert_eq!(position_set.get_hand(Color::Black, true), "rrbn");
    }

    #[test]
    fn place_in_check() {
        setup();
        let black_fen = "6k5/9_.2/12/8_.3/2_.9/2_.9/5_.6/3_.8/12/12/9_.2/5KQ2_.2 b qrn2pN2P 3";
        let cases = [PieceType::Queen, PieceType::Pawn];
        for case in cases {
            let mut position_set = P12::default();
            position_set
                .set_sfen(black_fen)
                .expect("failed to parse sfen string");

            position_set.place(
                Piece {
                    piece_type: case,
                    color: Color::Black,
                },
                G11,
            );
            assert_eq!(position_set.ply(), 4);
        }
    }

    #[test]
    fn empty_squares_with_plinths() {
        setup();
        let cases = [
            (
                "7k4/4_.2_.4/8_.3/12/2_.9/12/7_.4/12/12/5_.5_./_.11/5K1Q4 b q2rb4n3pQ2R3BN3P 3",
                11,
                Color::Black,
            ),
            (
                "6kq3_./12/12/1_.1_.2_.5/12/12/12/10_.1/12/9_.2/5_.6/4_N2K4 w rnQNRBP 4",
                1,
                Color::White,
            ),
            (
                "6qk3_./12/12/1_.1_.2_.5/12/12/12/10_.1/12/9_.2/5_.6/4_N2RK3 b rnQNRBP 4",
                1,
                Color::Black,
            ),
        ];
        for case in cases {
            let mut position_set = P12::default();
            position_set
                .set_sfen(case.0)
                .expect("error while parsing sfen");
            let file = position_set.empty_squares(Piece {
                piece_type: PieceType::Knight,
                color: case.2,
            });
            assert_eq!(file.unwrap_or_default().len(), case.1);
        }
    }

    #[test]
    fn is_check_fairy() {
        setup();
        let cases = [
            (
                "3k8/1_.9_./12/11_./4_.7/12/8_a3/12/1_.5_.4/6c5/2C1_.1N5/7K2Q1 w - 1",
                Color::White,
            ),
            (
                "3k8/1_.9_./12/11_./4_.7/12/8_.3/12/1_.5_.1a2/6c5/2C1_.1NK4/10Q1 w - 1",
                Color::White,
            ),
        ];
        for case in cases {
            let mut position = P12::default();
            position.update_variant(Variant::ShuuroFairy);
            position.set_sfen(case.0).expect("error while parsing sfen");
            let check = position.in_check(case.1);
            assert!(check);
        }
    }

    #[test]
    fn is_piece_pinned_by_fairy2() {
        setup();
        let cases = [(
            "2a1k7/1_.9_./12/11_./4_.7/12/8_.3/6c5/1_.5_.4/12/4_.1N5/4C1K3Q1 w - 9",
            0,
        )];
        for case in cases {
            let mut position = P12::default();
            position.update_variant(Variant::ShuuroFairy);
            position.set_sfen(case.0).expect("error while parsing sfen");
            let moves = position.legal_moves(&Color::White);
            if let Some(b) = moves.get(&G2) {
                assert_eq!(b.len(), case.1);
            }
        }
    }

    #[test]
    fn fairy_shop() {
        let cases = [
            ("knnaaacqGKCCCQQPPP", [3, 0, 1], [1, 3, 0], [160, 160]),
            (
                "KCAAAAACCCRRRQQkccppaaaaagg",
                [3, 0, 0],
                [2, 3, 2],
                [50, 60],
            ),
            (
                "KRRRRRNNNNBBBCAkcrrrrbnggggg",
                [1, 0, 0],
                [1, 0, 4],
                [110, 100],
            ),
            ("k", [0, 0, 0], [0, 0, 0], [870, 870]),
        ];
        for case in cases {
            let mut shop = Selection::<Square12>::default();
            shop.update_variant(Variant::ShuuroFairy);
            shop.set_hand(case.0);

            assert_eq!(
                shop.get(Piece {
                    piece_type: PieceType::Chancellor,
                    color: Color::White
                }),
                case.1[0]
            );
            assert_eq!(
                shop.get(Piece {
                    piece_type: PieceType::ArchBishop,
                    color: Color::White
                }),
                case.1[1]
            );
            assert_eq!(
                shop.get(Piece {
                    piece_type: PieceType::Giraffe,
                    color: Color::White
                }),
                case.1[2]
            );
            assert_eq!(
                shop.get(Piece {
                    piece_type: PieceType::Chancellor,
                    color: Color::Black
                }),
                case.2[0]
            );
            assert_eq!(
                shop.get(Piece {
                    piece_type: PieceType::ArchBishop,
                    color: Color::Black
                }),
                case.2[1]
            );
            assert_eq!(
                shop.get(Piece {
                    piece_type: PieceType::Giraffe,
                    color: Color::Black
                }),
                case.2[2]
            );

            assert_eq!(shop.credit(Color::White), case.3[0]);
            assert_eq!(shop.credit(Color::Black), case.3[1]);
        }
    }

    #[test]
    fn check_in_fairy_deploy() {
        setup();
        let cases = [(
            "4k1c5/1_.9_./12/11_./4_.7/12/8_.3/12/1_.5_.4/12/4_.7/4C1K5 w aQN 4",
            Color::White,
            G1,
            true,
        )];

        for case in cases {
            let mut position = P12::default();
            position.update_variant(Variant::ShuuroFairy);
            position.set_sfen(case.0).expect("error while parsing sfen");
            let check = position.in_check(case.1);
            assert_eq!(check, case.3);
        }
    }

    #[test]
    fn deploy_fairy_on_plinth() {
        setup();
        let fen =
            "5k6/12/12/12/12/12/12/12/12/12/12/3K_._._._._._._.R w 3C2P 4";
        let mut position = P12::default();
        position.update_variant(Variant::ShuuroFairy);
        position.set_sfen(fen).expect("error while parsing sfen");
        let moves = position.empty_squares(Piece::from_sfen('C').unwrap());
        assert_eq!(moves.unwrap_or_default().len(), 10);
    }

    #[test]
    fn is_fairy_mate() {
        setup();
        let cases = ["3cqkC5/2p9/3n2a5/1_.10/2A3_.5/5_.3A_.1/12/12/8_.3/12/_.9_.1/2_N3KQ4 b - 24"];
        for case in cases {
            let mut position = P12::default();
            position.update_variant(Variant::ShuuroFairy);
            position.set_sfen(case).expect("error while parsing sfen");
            assert!(position.is_checkmate(&Color::Black));
        }
    }

    #[test]
    fn make_fairy_mate() {
        setup();
        let cases = ["3cqk6/2p9/3n2aC4/1_.10/2A3_.5/5_.3A_.1/12/12/8_.3/12/_.9_.1/2_N3KQ4 w - 24"];
        for case in cases {
            let mut position = P12::default();
            position.update_variant(Variant::ShuuroFairy);
            position.set_sfen(case).expect("error while parsing sfen");
            let mate = position.make_move(Move::from_sfen("h10_g12").unwrap());
            if let Ok(mate) = mate {
                assert_eq!(
                    mate,
                    Outcome::Checkmate {
                        color: Color::White
                    }
                );
            }
        }
    }

    #[test]
    fn giraffe_moves() {
        setup();
        let cases = [
            (
                "1r1k8/12/12/12/12/12/12/12/12/12/12/G10K w - 1",
                &A1,
                vec![&B5, &E2],
            ),
            (
                "12/5k6/12/7_.4/12/12/10_.1/6g5/2_.7_.1/12/1K10/5_.1_.4 b - 1",
                &G5,
                vec![&F9, &H9, &C6, &K6, &C4, &K4, &F1, &H1],
            ),
        ];
        for case in cases {
            let mut position = P12::default();
            position.update_variant(Variant::ShuuroFairy);
            position.set_sfen(case.0).expect("error while parsing sfen");
            let legal_moves = position.legal_moves(&position.side_to_move());
            if let Some(b) = legal_moves.get(case.1) {
                let len = case.2.len();
                for sq in case.2 {
                    assert!((*b & sq).is_any());
                }
                assert_eq!(b.len(), len as u32);
            }
        }
    }

    #[test]
    fn detect_insufficient_material() {
        setup();
        let cases = [
            "_n4k6/5_.p5/12/12/7_.4/9_.2/3_.7_./7_.4/4_.N6/2K9/12/12 b - 34",
            "4gg_ggk3/12/2_.9/8_.3/1_.10/12/3_.3_.4/1_.9_./12/12/12/6GKGGG1 - b 1"];
        for case in cases {
            let mut position = P12::default();
            position.set_sfen(case).ok();
            assert!(position.detect_insufficient_material().is_ok());
        }
    }

    #[test]
    fn bishop_placement_check() {
        setup();
        let cases = [
           "4k7/12/12/3_.2_.5/_.11/6_.5/1_.10/12/12/7_.4/1_.10/4B2K2_.1 b qrbQRB 3" 
        ];
        for case in cases {
            let mut position = P12::default();
            position.set_sfen(case).ok();
            let lm = position.empty_squares(Piece::from_sfen('q').unwrap());
            assert!(lm.unwrap_or_default().len() == 11);
        }
    }

    #[test]
    fn generate_sfen() {
        setup();
        for _ in 0..10 {
            let mut position = P12::default();
            position.generate_plinths();
            let fen = position.generate_sfen();
            let mut position = P12::default();
            let correct_position = position.set_sfen(&fen);
            assert!(correct_position.is_ok());
        }
    }
}
