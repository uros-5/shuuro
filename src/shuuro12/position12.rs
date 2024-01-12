use std::{fmt, marker::PhantomData};

use crate::{
    bitboard::BitBoard,
    position::{Board, Outcome, Placement, Play, Position, Rules, Sfen},
    Color, Hand, Move, MoveData, Piece, PieceType, SfenError, Square, Variant,
};

use super::{
    attacks12::Attacks12,
    bitboard12::BB12,
    board_defs::{FILE_BB, RANK_BB},
    plinths_set12::PlinthGen12,
    square12::Square12,
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
        self.color_bb[Color::NoColor.index()] = PlinthGen12::default().start();
    }

    fn white_placement_attacked_ranks(&self) -> BB12<Square12> {
        RANK_BB[1] | &RANK_BB[2]
    }

    fn black_placement_attacked_ranks(&self) -> BB12<Square12> {
        RANK_BB[9] | &RANK_BB[10]
    }

    fn black_ranks(&self) -> [usize; 3] {
        [11, 10, 9]
    }

    fn king_files<const K: usize>(&self) -> [&str; K] {
        let temp: [&str; 6] = ["d", "e", "f", "g", "h", "i"];
        let mut files: [&str; K] = [""; K];
        for (i, v) in temp.iter().enumerate() {
            files[i] = v;
        }
        files
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
        Color, Move, Piece, Shop, Variant,
    };

    pub const START_POS: &str = "KR55/57/57/57/57/57/57/57/57/57/57/kr55 b - 1";

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
                "BBQ8K/57/2L03L05/4R7/3L08/57/57/5ppp4/nnq/L0L0L09/57/1L05k4 b - 1",
                &[A9, B9, C9, F8, G8, H8, H12],
                &[A1, B1, C1, L1, E4],
                &[G3, B12, C3, D5],
            ),
            (
                "9K2/57/6PPPP2/3Q6N1/57/57/57/5ppp4/7qk3/57/57/57 b P 1",
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
                "5NNQK3/8B3/57/57/57/8r3/57/57/pp55/1k55/57/57 w - 1",
                &[],
                &[I2],
            ),
            (
                "5NNQK3/8B3/8R3/57/57/8r3/57/57/pp55/1k55/57/57 w - 1",
                &[],
                &[],
            ),
            (
                "12/57/2K9/PPPPPP/57/57/2R2B6/57/2rn8/2k3q2R2/57/57 b - 1",
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
        let sfen = "6L03B1/2LN2K3P2/3pPL04L01/57/57/57/7L04/3L08/8L03/q56/L056/3kqbr5 b - 38";
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
        pos.set_sfen("57/9K2/8L0Q2/4P6L0/6P5/L03L07/55L01/1L055/2q9/57/L056/6kL04 w - 55")
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
            "57/9K2/8L03/56L0/5Q6/L03L07/55L01/1L055/5P6/4k7/L056/7L04 b - 72",
        )
        .expect("failed to parse SFEN string");
        let in_check = pos.in_check(Color::Black);
        assert!(in_check);
    }

    #[test]
    fn legal_moves_pawn() {
        setup();
        let cases = [(
            "4K1Q4LN/4L07/2L09/6P5/6q5/55L01/57/9L02/6L05/L01L09/5p6/6k5 w - 11",
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
        pos.set_sfen("5K6/55PL0/3N8/2L09/5L06/8nL02/1L03Q4L01/7L04/57/57/L03pr6/5k1Q4 b - 50")
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
                "4K6LN/4L07/2L09/57/57/6Q3L01/6P5/5k3L02/6L05/L01L09/5p6/57 b - 19",
                4,
            ),
            (
                "4K6LN/4L07/2L09/57/57/6Q3L01/6P5/R4k3L02/6L05/L01L09/5p6/57 b - 19",
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
        pos.set_sfen("6KL04/3L08/57/57/9L02/4L07/6L04L0/3L08/57/57/1L055/57 b kr12pQ9P 1")
            .expect("failed to parse sfen string");
        assert_eq!(pos.hand(Piece::from_sfen('p').unwrap()), 12);
        assert_eq!(pos.hand(Piece::from_sfen('P').unwrap()), 9);
    }

    #[test]
    fn move_candidates2() {
        setup();

        let mut pos = P12::new();
        pos.set_sfen("R3N7/4K7/57/57/57/57/57/bppp8/4k7/57/57/57 b - 1")
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
        pos.set_sfen("57/5R5K/57/57/3b1L04k1/5p4b1/4n7/57/55R1/57/57/57 w - 1")
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
        let sfen = "4K5B1/5P2L03/57/2L02Q6/4L04L02/57/56L0/1L055/57/5Ln5L0/3p8/1q3kn5 b - 11";
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
                "4K1Q4LN/4L07/2L09/57/6P5/55L01/57/9L02/6L05/L01L09/5p6/6k5 b - 12",
                "f11",
                "f10",
            ),
            (
                "2N1L0QKRL01N1/2P4P3P/55L01/57/57/L056/57/57/3L08/8L03/2pp2L05/4qLnk2r1b b - 15",
                "c11",
                "c10",
            ),
            (
                "2N1L0QKRL01N1/2P4P3P/55L01/57/57/L056/57/57/3L08/8L03/2pp2L05/4qLnk2r1b b - 15",
                "d11",
                "d10",
            ),
            (
                "4KN1Q4/4L0P1P1PP1/1P55/3L08/56L0/6L05/4L01L05/57/57/6L05/p7ppp1/L02q1k3r2 b - 16",
                "a11",
                "a10",
            ),
            ("2LNN2KNBB2/6L03P1/57/7L04/1L055/57/9L02/57/6L05/ppp2p2pQ2/pppL0p1ppp1pp/L02kq7 b - 29",
             "i11",
             "j10")
        ];
        let ng_cases = [(
            "4K1Q4LN/2P1L07/2L09/57/6P5/55L01/57/9L02/6L05/L01L09/5p6/6k5 w - 12",
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
            "4KN1Q4/4L0P1P1PP1/8r3/3L08/56L0/6L05/4L01L01q3/57/57/6L05/pP3k2ppp1/L01r9 w - 33";
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
                "2q1L0QKRL01N1/2P8P/7P2L01/57/57/L056/57/57/3L08/8L03/2pp2L05/5Lnk2r1b b - 20",
                C1,
                15,
            ),
            (
                "2B2KQ5/5R3P2/L06LN4/57/57/2L06L02/57/3L06L01/6L05/3p8/L04p1pp3/2n1rqk3b1 b - 17",
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
            "4K1Q4LN/4L07/2L09/57/57/55L01/6P5/9L02/5kL05/L01L09/5p6/57 w - 17";
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
                "KQR9/1PPP8/57/57/57/57/57/57/57/57/1ppp8/qkb9 w - 1",
                false,
                true,
            ),
            (
                "5QR5/1K55/57/57/57/57/57/57/57/57/57/5k6 b - 1",
                true,
                false,
            ),
            (
                "2RNBKQBNR2/57/2PPPPPPPP2/57/57/57/57/57/57/2pppppppp2/57/2rnbkqbnr2 b - 1",
                false,
                false,
            ),
            (
                "RR5K4/7L04/QP55/7L04/57/57/57/nbq9/7q4/57/57/56k w - 1",
                false,
                false,
            ),
            ("KQP8/2n8/57/57/57/57/57/k11/57/57/57/57 w - 1", false, true),
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
        let sfen = "57/2r9/K56/3q8/57/57/57/57/56k/57/57/57 b - 1";
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
                "1K8r1/9rr1/57/57/57/57/57/57/57/k11/57/57 b - 1",
                true,
                Color::White,
            ),
            (
                "5RNB4/5K4r1/6B5/57/57/57/57/57/ppppp7/57/57/9k2 w - 1",
                false,
                Color::Black,
            ),
            (
                "12/57/7k3Q/57/57/KRn9/57/57/57/57/57/57 b - 1",
                false,
                Color::White,
            ),
            (
                "57/9K2/L06L04/57/57/2L06L02/57/3L05BL01/6LN5/4Qk6/L056/6n5 b - 69",
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
        pos.set_sfen("57/57/PPPQP4K2/7RR3/57/57/57/4pp6/2kr8/57/57/57 b - 1")
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
            "57/3KRRB5/5PP5/57/57/57/57/qbbn8/57/6k5/57/57 w K2RB2P 1";
        let test_cases = [
            (
                D2,
                E1,
                false,
                true,
                "4K7/4RRB5/5PP5/57/57/57/57/qbbn8/57/6k5/57/57 b K2RB2P 2",
            ),
            (
                E2,
                E7,
                false,
                true,
                "57/3K1RB5/5PP5/57/57/57/4R7/qbbn8/57/6k5/57/57 b K2RB2P 2",
            ),
            (
                G2,
                I4,
                false,
                true,
                "57/3KRR6/5PP5/8B3/57/57/57/qbbn8/57/6k5/57/57 b K2RB2P 2",
            ),
            (
                F2,
                F1,
                false,
                true,
                "5R6/3KR1B5/5PP5/57/57/57/57/qbbn8/57/6k5/57/57 b K2RB2P 2",
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
        pos.set_sfen("57/1K8RR/57/57/57/r9k1/57/57/57/57/57/57 b kr 1")
            .expect("failed to parse SFEN string");
        let move_ = Move::new(A6, A1);
        assert!(pos.make_move(move_).is_err());

        pos.set_sfen("7K4/1RR9/57/57/57/r9k1/57/57/57/57/57/57 b kr 1")
            .expect("failed to parse SFEN string");
        let move_ = Move::new(K6, K5);
        assert!(pos.make_move(move_).is_ok());
    }

    #[test]
    fn pawn_promoted() {
        setup();
        let mut pos = P12::new();
        pos.set_sfen("7K4/1L01p1N6/57/5R2B3/1L05L04/9L02/57/5L06/57/7L04/5L04L01/2r2k1n4 b - 28")
            .expect("failed to parse SFEN string");
        let move_ = Move::from_sfen("d2_d1").unwrap();
        assert!(pos.make_move(move_).is_ok());
    }

    #[test]
    fn make_moves() {
        setup();
        let mut pos = P12::new();
        pos.set_sfen("6K5/57/57/6k5/57/PL055/57/p56/57/57/57/57 w - 1")
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

        pos.set_sfen("2RNBKQBNR2/57/2PPPPPPPP2/57/57/57/57/57/57/2pppppppp2/57/2rnbkqbnr2 b - 1")
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
            // ("57/7k4/57/5n6/57/57/57/1Q55/57/K56/57/57 b - 1", "f4", 0),
            // ("57/7k4/57/5n6/57/57/57/2Q9/57/K56/57/57 b - 1", "f4", 8),
            (
                "8K3/3PPPPP4/6p5/8R3/6pp4/57/57/57/8r3/57/8k3/57 w - 1",
                "i4",
                7,
            ),
            (
                "8K3/3PPPPP4/6p5/55R1/6pp4/57/57/57/8r3/57/8k3/57 w - 1",
                "k4",
                1,
            ),
            ("1K55/1qq9/57/57/57/57/57/1R55/57/2k9/57/57 w - 1", "b8", 0),
            ("1K55/q1q9/57/57/57/57/57/1R55/57/2k9/57/57 w - 1", "b8", 0),
            (
                "L056/2K3L03P1/1n3q3L02/57/4L07/57/57/L09L01/57/5L01k4/8L03/57 b - 48",
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
            let bb = position_set.king_squares::<6>(&case.0);
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
            .parse_sfen_board("6K5/57/57/57/57/57/57/57/57/57/57/7k4")
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
            .set_sfen("7L04/57/57/57/57/57/57/57/57/57/57/57 w K 1")
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
        position.set_sfen("6K5/57/57/L01L05L03/6L05/57/57/57/57/4L02L01L02/2L09/57 b krqpR2N3BQ 1").expect("sfen has wrong data");
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
                "5KRRR3/4PPPP4/57/5L06/57/57/57/57/57/57/57/L04kqqLn3",
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
            assert_eq!(file.len(), case.2);
        }
        assert_eq!(position_set.get_hand(Color::Black, true), "rrbn");
    }

    #[test]
    fn place_in_check() {
        setup();
        let black_fen = "5KQ2L02/9L02/57/57/3L08/5L06/2L09/2L09/8L03/57/9L02/6k5 b qrn2pN2P 3";
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
                "5K1Q4/L056/5L05L0/57/57/7L04/57/2L09/57/8L03/4L02L04/7k4 b q2rb4n3pQ2R3BN3P 3",
                11,
                Color::Black,
            ),
            (
                "4LN2K4/5L06/9L02/57/55L01/57/57/57/1L01L02L05/57/57/6kq3L0 w rnQNRBP 4",
                2,
                Color::White,
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
            assert_eq!(file.len(), case.1);
        }
    }

    #[test]
    fn is_check_fairy() {
        setup();
        let cases = [
            (
                "7K2Q1/2C1L01N5/6c5/1L05L04/57/8La3/57/4L07/56L0/57/1L09L0/3k8 w - 1",
                Color::White,
            ),
            (
                "55Q1/2C1L01NK4/6c5/1L05L01a2/57/8L03/57/4L07/56L0/57/1L09L0/3k8 w - 1",
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
            "4C1K3Q1/4L01N5/57/1L05L04/6c5/8L03/57/4L07/56L0/57/1L09L0/2a1k7 w - 9",
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
                [3, 3, 0],
                [2, 3, 2],
                [20, 60],
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
            let mut shop = Shop::<Square12>::default();
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
            "4C1K5/4L07/57/1L05L04/57/8L03/57/4L07/56L0/57/1L09L0/4k1c5 w aQN 4",
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
            "3KL0L0L0L0L0L0L0R/57/57/57/57/57/57/57/57/57/57/5k6 w 3C2P 4";
        let mut position = P12::default();
        position.update_variant(Variant::ShuuroFairy);
        position.set_sfen(fen).expect("error while parsing sfen");
        let moves = position.empty_squares(Piece::from_sfen('C').unwrap());
        assert_eq!(moves.len(), 10);
    }

    #[test]
    fn is_fairy_mate() {
        setup();
        let cases = ["2LN3KQ4/L09L01/57/8L03/57/57/5L03AL01/2A3L05/1L055/3n2a5/2p9/3cqkC5 b - 24"];
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
        let cases = ["2LN3KQ4/L09L01/57/8L03/57/57/5L03AL01/2A3L05/1L055/3n2aC4/2p9/3cqk6 w - 24"];
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
                "G55K/57/57/57/57/57/57/57/57/57/57/1r1k8 w - 1",
                &A1,
                vec![&B5, &E2],
            ),
            (
                "5L01L04/1K55/57/2L07L01/6g5/55L01/57/57/7L04/57/5k6/57 b - 1",
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
            "57/57/2K9/4L0N6/7L04/3L07L0/9L02/7L04/57/57/5L0p5/Ln4k6 b - 34",
            "6GKGGG1/57/57/57/1L09L0/3L03L04/57/1L055/8L03/2L09/57/4ggLggk3 - b 1"];
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
           "4B2K2L01/1L055/7L04/57/57/1L055/6L05/L056/3L02L05/57/57/4k7 b qrbQRB 3" 
        ];
        for case in cases {
            let mut position = P12::default();
            position.set_sfen(case).ok();
            let lm = position.empty_squares(Piece::from_sfen('q').unwrap());
            assert!(lm.len() == 11);
        }
    }
}
