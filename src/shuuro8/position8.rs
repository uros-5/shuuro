use std::{
    fmt,
    marker::PhantomData,
    ops::{BitAnd, BitOr, BitXorAssign, Not},
};

use crate::{
    bitboard::BitBoard,
    position::{Board, Outcome, Placement, Play, Position, Rules, Sfen},
    Color, Hand, Move, MoveData, Piece, PieceType, SfenError, Square, Variant,
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

    fn white_placement_attacked_ranks(&self) -> BB8<Square8> {
        &RANK_BB[1] | &RANK_BB[2]
    }

    fn black_placement_attacked_ranks(&self) -> BB8<Square8> {
        &RANK_BB[5] | &RANK_BB[6]
    }

    fn black_ranks(&self) -> [usize; 3] {
        [7, 6, 5]
    }

    fn king_files<const K: usize>(&self) -> [&str; K] {
        let temp: [&str; 6] = ["b", "c", "d", "e", "f", "g"];
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

    fn empty_placement_board() -> String {
        String::from("8/8/8/8/8/8/8/8 w")
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
                            &self.player_bb(Color::NoColor) & &sq;
                        if plinth.is_any() {
                            write!(f, " L|")?;
                        } else {
                            write!(f, "  |")?;
                        }
                    } else {
                        let plinth: BB8<Square8> =
                            &self.player_bb(Color::NoColor) & &sq;
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
