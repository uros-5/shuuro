use std::{fmt, marker::PhantomData};

use crate::{
    attacks::Attacks,
    bitboard::BitBoard,
    position::{Board, Outcome, Placement, Play, Position, Rules, Sfen},
    Color, Hand, Move, MoveData, Piece, PieceType, SfenError, Square, Variant,
};

use super::{
    attacks8::Attacks8,
    bitboard8::BB8,
    board_defs::{FILE_BB, RANK_BB},
    plinths_set8::PlinthGen8,
    square8::{
        consts::{A1, A8, H1, H8},
        Square8,
    },
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
{
    board: PieceGrid,
    hand: Hand,
    ply: u16,
    side_to_move: Color,
    move_history: Vec<Move<Square8>>,
    occupied_bb: BB8<Square8>,
    color_bb: [BB8<Square8>; 3],
    game_status: Outcome,
    variant: Variant,
    pub type_bb: [BB8<Square8>; 10],
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

    fn insert_sfen(&mut self, sfen: Move<Square8>) {
        self.move_history.push(sfen);
    }

    fn insert_move(&mut self, move_record: Move<Square8>) {
        self.move_history.push(move_record)
    }

    fn clear_sfen_history(&mut self) {
        self.move_history.clear();
    }

    fn set_move_history(&mut self, history: Vec<Move<Square8>>) {
        self.move_history = history;
    }

    fn move_history(&self) -> &[Move<Square8>] {
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
        8
    }
}

impl Sfen<Square8, BB8<Square8>, Attacks8<Square8, BB8<Square8>>>
    for P8<Square8, BB8<Square8>>
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

    fn rank_bb(&self, file: usize) -> BB8<Square8> {
        RANK_BB[file]
    }

    fn update_bb(&mut self, p: Piece, sq: Square8) {
        self.set_piece(sq, Some(p));
        self.occupied_bb |= &sq;
        self.color_bb[p.color.index()] |= &sq;
        self.type_bb[p.piece_type.index()] |= &sq;
    }

    fn empty_placement_board() -> String {
        String::from("8/8/8/8/8/8/8/8 w")
    }

    fn king_files(&self, c: &Color) -> BB8<Square8> {
        match c {
            Color::Black => Attacks8::between(A8, H8),
            Color::White => Attacks8::between(A1, H1),
            Color::NoColor => BB8::empty(),
        }
    }
}
impl Rules<Square8, BB8<Square8>, Attacks8<Square8, BB8<Square8>>>
    for P8<Square8, BB8<Square8>>
{
}

impl Play<Square8, BB8<Square8>, Attacks8<Square8, BB8<Square8>>>
    for P8<Square8, BB8<Square8>>
{
    fn game_status(&self) -> Outcome {
        self.game_status.clone()
    }

    fn file_bb(&self, file: usize) -> BB8<Square8> {
        FILE_BB[file]
    }

    fn update_after_move(
        &mut self,
        from: Square8,
        to: Square8,
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
            // sfen_history: Default::default(),
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
                if let Some(sq) = Square8::new(file, rank) {
                    if let Some(ref piece) = *self.piece_at(sq) {
                        write!(f, "{piece}")?;
                        let plinth: BB8<Square8> =
                            self.player_bb(Color::NoColor) & &sq;
                        if plinth.is_any() {
                            write!(f, " L|")?;
                        } else {
                            write!(f, "  |")?;
                        }
                    } else {
                        let plinth: BB8<Square8> =
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
pub mod tests {
    use crate::{
        attacks::Attacks,
        bitboard::BitBoard,
        position::{Board, Play},
        shuuro8::{
            attacks8::Attacks8,
            square8::{
                consts::{
                    A6, A7, B4, B5, B6, B7, D6, E7, F4, F5, F6, F8, G2, G3,
                },
                Square8,
            },
        },
        Color, Variant,
    };

    use super::P8;

    fn setup() {
        Attacks8::init();
    }

    #[test]
    fn legal_moves_bishop() {
        setup();
        let cases = [(
            "n1rnkb2/3p1p1p/4pn_.1/ppp1_n3/3_.1P2/1_NP1PN2/PP1P2PP/1KQ1NN2 b - 1",
            F8,
            4,
        )];
        for case in cases {
            let mut pos = P8::default();
            pos.set_sfen(case.0).expect("failed to parse sfen string");
            let legal_moves = pos.legal_moves(&Color::Black);
            if let Some(b) = legal_moves.get(&case.1) {
                assert_eq!(b.len(), case.2);
            }
        }
    }

    #[test]
    fn pinned_bb() {
        setup();

        let cases: &[(&str, &[Square8], &[Square8])] = &[
            ("q4k1n/8/3R1q2/6Q1/2b5/5Q2/8/1Q2K3 w - 1", &[F6], &[]),
            ("q4k1n/5_.2/3R1q2/6Q1/2b5/5Q2/8/1Q2K3 w - 1", &[], &[]),
            ("5k1n/4q3/8/q1R3Q1/1Qb5/8/8/1Q2K3 b - 1", &[], &[B4]),
            ("5k1n/2R1r3/8/7Q/1Qb2q2/5Q2/7q/3K4 b - 1", &[E7, F4], &[]),
            ("5k1n/2R1r3/_.2_.4/7Q/1Qb2q2/5Q2/7q/3K4 b - 1", &[F4], &[]),
            ("5k1n/2RP4/3p4/7Q/1Qb2q2/5Q2/7q/3K4 b - 1", &[D6, F4], &[]),
        ];

        let mut pos = P8::new();
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
    fn pinned_and_in_check() {
        setup();

        let cases = [
            ("5k1n/2RP4/3p2N1/5q2/1Qb4Q/5Q2/7q/3K4 b - 1", F5, 0, false),
            ("5k1n/2RP4/3p2A1/5q2/1Qb4Q/5Q2/7q/3K4 b - 1", F5, 0, true),
            ("5k1n/2RP4/3p2C1/5q2/1Qb4Q/5Q2/7q/3K4 b - 1", F5, 0, true),
        ];
        let mut pos = P8::new();
        for case in cases {
            pos.set_sfen(case.0).expect("faled to parse SFEN string");
            if case.3 {
                pos.update_variant(Variant::StandardFairy);
            }
            let moves = pos.legal_moves(&Color::Black);
            if let Some(moves) = moves.get(&case.1) {
                assert_eq!(moves.len(), case.2);
            }
        }
    }

    #[test]
    fn pawn_moves2() {
        setup();
        let cases: &[(&str, Color, Square8, &[Square8])] = &[
            (
                "5k1n/ppRP2N1/B2p4/5q2/1Qb4Q/5Q2/7q/3K4 b - 1",
                Color::Black,
                B7,
                &[B6, B5, A6],
            ),
            (
                "5k1n/ppRP2N1/B2p4/5q2/1Qb4Q/5Q2/7q/3K4 b - 1",
                Color::Black,
                A7,
                &[],
            ),
            (
                "5k1n/ppRP2N1/B2p1p2/5q2/1Qb4Q/5Qp1/7q/3K4  b - 1",
                Color::Black,
                F6,
                &[],
            ),
            (
                "5k1n/ppRP2N1/B2p1p2/5q2/1Qb4Q/5Qp1/7q/3K4  b - 1",
                Color::Black,
                G3,
                &[G2],
            ),
        ];
        let mut pos = P8::new();
        for case in cases {
            pos.set_sfen(case.0).expect("failed to parse SFEN string");

            let moves = pos.legal_moves(&case.1);
            if let Some(moves) = moves.get(&case.2) {
                assert_eq!(moves.len(), case.3.len() as u32);
            }
        }
    }
}
