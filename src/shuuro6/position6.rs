use crate::attacks::Attacks;
use crate::position::Board;
use crate::position::Placement;
use crate::position::Play;
use crate::position::Position;
use crate::position::Rules;
use crate::position::Sfen;
use crate::position::SfenHistory;
use crate::Move;
use crate::MoveData;
use crate::Piece;
use crate::PieceType;
use crate::SfenError;
use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;

use crate::{
    bitboard::BitBoard, position::Outcome, Color, Hand, Square, Variant,
};

use super::attacks6::Attacks6;
use super::board_defs::FILE_BB;
use super::board_defs::RANK_BB;
use super::plinths_set6::PlinthGen6;
use super::square6::consts::A1;
use super::square6::consts::A6;
use super::square6::consts::F1;
use super::square6::consts::F6;
use super::{bitboard6::BB6, square6::Square6};

#[derive(Clone, Debug)]
pub struct P6<S, B>
where
    S: Square,
    B: BitBoard<S>,
{
    board: PieceGrid,
    hand: Hand,
    ply: u16,
    side_to_move: Color,
    move_history: Vec<Move<Square6>>,
    history: SfenHistory<BB6<Square6>>,
    occupied_bb: BB6<Square6>,
    color_bb: [BB6<Square6>; 3],
    game_status: Outcome,
    variant: Variant,
    placement_moves: HashMap<usize, BB6<Square6>>,
    legal_moves: HashMap<Square6, BB6<Square6>>,

    pub type_bb: [BB6<Square6>; 10],
    _a: PhantomData<B>,
    _s: PhantomData<S>,
}

impl Default for P6<Square6, BB6<Square6>> {
    fn default() -> P6<Square6, BB6<Square6>> {
        P6 {
            side_to_move: Color::Black,
            board: PieceGrid([None; 36]),
            hand: Default::default(),
            ply: 0,
            move_history: Default::default(),
            history: SfenHistory::default(),
            occupied_bb: Default::default(),
            color_bb: Default::default(),
            type_bb: Default::default(),
            game_status: Outcome::MoveOk,
            variant: Variant::Standard,
            _a: PhantomData,
            _s: PhantomData,
            placement_moves: Default::default(),
            legal_moves: Default::default(),
        }
    }
}

impl Board<Square6, BB6<Square6>, Attacks6<Square6, BB6<Square6>>>
    for P6<Square6, BB6<Square6>>
{
    fn new() -> Self {
        Default::default()
    }

    fn set_piece(&mut self, sq: Square6, p: Option<Piece>) {
        self.board.set(sq, p)
    }

    fn piece_at(&self, sq: Square6) -> &Option<Piece> {
        self.board.get(sq)
    }

    fn player_bb(&self, c: Color) -> BB6<Square6> {
        self.color_bb[c.index()]
    }

    fn occupied_bb(&self) -> BB6<Square6> {
        self.occupied_bb
    }

    fn type_bb(&self, pt: &PieceType) -> BB6<Square6> {
        self.type_bb[pt.index()]
    }

    fn xor_player_bb(&mut self, color: Color, sq: Square6) {
        self.color_bb[color.index()] ^= &sq;
    }

    fn xor_type_bb(&mut self, piece_type: PieceType, sq: Square6) {
        self.type_bb[piece_type.index()] ^= &sq;
    }

    fn xor_occupied(&mut self, sq: Square6) {
        self.occupied_bb ^= &sq;
    }

    fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    fn empty_all_bb(&mut self) {
        self.occupied_bb = BB6::empty();
        self.color_bb = Default::default();
        self.type_bb = Default::default();
    }

    fn sfen_to_bb(&mut self, piece: Piece, sq: &Square6) {
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

    fn insert_move(&mut self, sfen: Move<Square6>) {
        self.move_history.push(sfen);
    }

    fn insert_sfen(&mut self, fen: String) {
        self.history.add_move((
            self.player_bb(Color::White),
            self.player_bb(Color::Black),
            fen,
        ));
    }

    fn clear_sfen_history(&mut self) {
        self.move_history.clear();
    }

    fn set_move_history(&mut self, history: Vec<Move<Square6>>) {
        self.move_history = history;
    }

    fn move_history(&self) -> &[Move<Square6>] {
        &self.move_history
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
        6
    }

    fn get_sfen_history(&self) -> &SfenHistory<BB6<Square6>> {
        &self.history
    }
}

impl Sfen<Square6, BB6<Square6>, Attacks6<Square6, BB6<Square6>>>
    for P6<Square6, BB6<Square6>>
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

    fn update_player(&mut self, piece: Piece, sq: &Square6) {
        self.set_piece(*sq, Some(piece));
        self.occupied_bb |= sq;
        self.color_bb[piece.color.index()] |= sq;
        self.type_bb[piece.piece_type.index()] |= sq;
    }
}

impl Placement<Square6, BB6<Square6>, Attacks6<Square6, BB6<Square6>>>
    for P6<Square6, BB6<Square6>>
{
    fn generate_plinths(&mut self) {
        self.color_bb[Color::NoColor.index()] = PlinthGen6::default().start();
    }

    fn rank_bb(&self, file: usize) -> BB6<Square6> {
        RANK_BB[file]
    }

    fn update_bb(&mut self, p: Piece, sq: Square6) {
        self.set_piece(sq, Some(p));
        self.occupied_bb |= &sq;
        self.color_bb[p.color.index()] |= &sq;
        self.type_bb[p.piece_type.index()] |= &sq;
    }

    fn empty_placement_board() -> String {
        String::from("6/6/6/6/6/6 w")
    }

    fn king_files(&self, c: Color) -> BB6<Square6> {
        match c {
            Color::Black => Attacks6::between(A6, F6),
            Color::White => Attacks6::between(A1, F1),
            Color::NoColor => BB6::empty(),
        }
    }

    fn new_placement_squares(
        &mut self,
        placement: std::collections::HashMap<usize, BB6<Square6>>,
    ) {
        self.placement_moves = placement;
    }

    fn get_placement_squares(&self) -> &HashMap<usize, BB6<Square6>> {
        &self.placement_moves
    }
}

impl Rules<Square6, BB6<Square6>, Attacks6<Square6, BB6<Square6>>>
    for P6<Square6, BB6<Square6>>
{
}

impl Play<Square6, BB6<Square6>, Attacks6<Square6, BB6<Square6>>>
    for P6<Square6, BB6<Square6>>
{
    fn game_status(&self) -> Outcome {
        self.game_status.clone()
    }

    fn file_bb(&self, file: usize) -> BB6<Square6> {
        FILE_BB[file]
    }

    fn update_after_move(
        &mut self,
        from: Square6,
        to: Square6,
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
            if cap.piece_type != PieceType::Plinth {
                self.occupied_bb ^= &to;
                self.type_bb[cap.piece_type.index()] ^= &to;
                self.color_bb[cap.color.index()] ^= &to;
                move_data = move_data.captured(captured);
            }
            //self.hand.increment(pc);
        }

        self.side_to_move = opponent;
        self.ply += 1;
        move_data
    }

    fn new_legal_moves(
        &mut self,
        lm: std::collections::HashMap<Square6, BB6<Square6>>,
    ) {
        self.legal_moves = lm;
    }

    fn get_legal_moves(&self) -> &HashMap<Square6, BB6<Square6>> {
        &self.legal_moves
    }
}

#[derive(Clone)]
pub struct PieceGrid([Option<Piece>; 36]);

impl PieceGrid {
    pub fn get(&self, sq: Square6) -> &Option<Piece> {
        &self.0[sq.index()]
    }

    pub fn set(&mut self, sq: Square6, pc: Option<Piece>) {
        self.0[sq.index()] = pc;
    }
}

impl Default for PieceGrid {
    fn default() -> Self {
        PieceGrid([None; 36])
    }
}

impl fmt::Display for P6<Square6, BB6<Square6>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "+---+---+---+---+---+---+")?;

        for rank in (0..6).rev() {
            write!(f, "|")?;
            for file in 0..6 {
                if let Some(sq) = Square6::new(file, rank) {
                    if let Some(ref piece) = *self.piece_at(sq) {
                        write!(f, "{piece}")?;
                        let plinth: BB6<Square6> =
                            self.player_bb(Color::NoColor) & &sq;
                        if plinth.is_any() {
                            write!(f, " L|")?;
                        } else {
                            write!(f, "  |")?;
                        }
                    } else {
                        let plinth: BB6<Square6> =
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
            writeln!(f, "\n+---+---+---+---+---+---+")?;
        }
        writeln!(f, "a   b   c   d   e   f")?;
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

impl fmt::Debug for PieceGrid {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "PieceGrid {{ ")?;

        for pc in self.0.iter() {
            write!(fmt, "{pc:?} ")?;
        }
        write!(fmt, "}}")
    }
}

impl Position<Square6, BB6<Square6>, Attacks6<Square6, BB6<Square6>>>
    for P6<Square6, BB6<Square6>>
{
}

#[cfg(test)]
pub mod tests {
    use crate::{
        attacks::Attacks,
        bitboard::BitBoard,
        position::{Board, Placement, Play, Sfen},
        shuuro6::{
            attacks6::Attacks6,
            square6::{
                consts::{B3, C4, C5, C6, D2, D3, D4, E1, E4, E5, E6, F6},
                Square6,
            },
        },
        Color, Move, Piece, PieceType, Variant,
    };

    use super::P6;

    fn setup() {
        Attacks6::init();
    }

    #[test]
    fn legal_moves_bishop() {
        setup();
        let cases = [("b2k2/2p1_.1/6/3B2/PP_.3/2K1N1 w - 1", D3, 7)];
        for case in cases {
            let mut pos = P6::default();
            pos.set_sfen(case.0).expect("failed to parse sfen string");
            let legal_moves = pos.legal_moves(Color::White);
            if let Some(b) = legal_moves.get(&case.1) {
                assert_eq!(b.len(), case.2);
            }
        }
    }

    #[test]
    fn pinned_bb() {
        setup();

        let cases: &[(&str, &[Square6], &[Square6])] = &[
            ("4k1/ppb3/2p1q1/1BP2P/4Q1/1NK1R1 b - 1", &[C4, E4], &[]),
            ("5k/1pp1b1/p3_.b/1BP2P/3Q2/1_.K1Rr w - 1", &[], &[D2, E1]),
            ("5k/1pp1b1/p3_.b/1BP1QP/3_.2/1_.K1Rr w - 1", &[], &[E1]),
        ];

        let mut pos = P6::new();
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

        let cases = [("5k/Pb1C2/1Rqb2/2Q2P/_.4_A/2K3 b - 1", D4, 0, true)];
        let mut pos = P6::new();
        for case in cases {
            pos.set_sfen(case.0).expect("faled to parse SFEN string");
            if case.3 {
                pos.update_variant(Variant::ShuuroMiniFairy);
            }
            let moves = pos.legal_moves(Color::Black);
            if let Some(moves) = moves.get(&case.1) {
                assert_eq!(moves.len(), case.2);
            }
        }
    }

    #[test]
    fn placement2() {
        setup();
        let cases = [
            (
                "6/6/4_.1/5_./6/6 w kr2bnKRBNP 0",
                PieceType::King,
                Color::White,
                E1,
                "6/6/4_.1/5_./6/4K1 b RBNPkr2bn 1 K@e1",
            ),
            (
                "1b1k2/3_.2/6/6/2_.3/N1NK2 b n 6",
                PieceType::Knight,
                Color::Black,
                F6,
                "1b1k1n/3_.2/6/6/2_.3/N1NK2 b - 7 n@f6",
            ),
        ];
        for case in cases {
            let mut pos = P6::new();
            pos.set_sfen(case.0).expect("failed to parse SFEN string");
            let m = pos.place(
                Piece {
                    piece_type: case.1,
                    color: case.2,
                },
                case.3,
            );
            assert!(!m.is_none());
            assert_eq!(case.4, m.unwrap());
        }
    }

    #[test]
    fn normal_move() {
        setup();
        let cases = [
            (
                "3k_n1/2pn2/6/6/5_/2RRK1 b - 0",
                PieceType::Knight,
                Color::Black,
                E6,
                D4,
            ),
            (
                "2kr1b/6/_.5/6/3_.2/2NKN1 b - 7",
                PieceType::Bishop,
                Color::Black,
                F6,
                E5,
            ),
        ];
        for case in cases {
            let mut pos = P6::new();
            pos.set_sfen(case.0).expect("failed to parse");
            let _ = Move::Normal {
                from: case.3,
                to: case.4,
                placed: Piece {
                    color: case.2,
                    piece_type: case.1,
                },
            };
            let outcome = pos.play(&format!(
                "{}_{}",
                case.3.to_string(),
                case.4.to_string()
            ));
            assert!(outcome.is_ok());
        }
    }

    #[test]
    fn pawn_promotion() {
        setup();
        let cases = [(
            "k3b1/1_.P3/6/P4_./1PNb2/1K4 w - 17",
            Piece {
                piece_type: PieceType::Pawn,
                color: Color::White,
            },
            C5,
            C6,
        )];
        for case in cases {
            let mut position = P6::default();
            position.set_sfen(case.0).ok();
            position.play(&format!("{}_{}", case.2, case.3)).ok();
            let sfen = position.generate_sfen();
            assert!(sfen.contains("Q"));
        }
    }

    #[test]
    fn plinth_count() {
        setup();
        let cases = [(
            "1rkb2/p1n3/4_.1/1_.4/1KPP2/2BR2 b - 13",
            Piece {
                piece_type: PieceType::Knight,
                color: Color::Black,
            },
            C5,
            B3,
        )];
        for case in cases {
            let mut position = P6::default();
            position.set_sfen(case.0).ok();
            position.play(&format!("{}_{}", case.2, case.3)).ok();
            assert_eq!(position.player_bb(Color::NoColor).len(), 2)
        }
    }
}
