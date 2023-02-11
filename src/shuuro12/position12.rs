use std::{
    fmt,
    marker::PhantomData,
    ops::{BitAnd, BitOr, BitXorAssign, Not},
};

use crate::{
    bitboard::BitBoard,
    position::{Outcome, Position},
    Color, Hand, Move, MoveError, MoveRecord, Piece, PieceType, SfenError, Square, Variant,
};

use super::{
    attacks12::Attacks12, bitboard12::BB12, board_defs::FILE_BB, plinths_set12::PlinthGen12,
    square12::Square12,
};

pub struct P12<S, B>
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
    move_history: Vec<MoveRecord<Square12>>,
    sfen_history: Vec<String>,
    occupied_bb: BB12<Square12>,
    color_bb: [BB12<Square12>; 3],
    game_status: Outcome,
    variant: Variant,
    pub type_bb: [BB12<Square12>; 7],
    _a: PhantomData<B>,
    _s: PhantomData<S>,
}

impl Position<Square12, BB12<Square12>, Attacks12<Square12, BB12<Square12>>>
    for P12<Square12, BB12<Square12>>
{
    const ID: usize = 3;
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

    fn outcome(&self) -> Outcome {
        self.game_status.clone()
    }

    fn update_outcome(&mut self, outcome: Outcome) {
        self.game_status = outcome;
    }

    fn variant(&self) -> Variant {
        self.variant.clone()
    }

    fn update_variant(&mut self, variant: Variant) {
        self.variant = variant;
    }

    fn make_move(&mut self, m: Move<Square12>) -> Result<Outcome, MoveError> {
        todo!()
    }

    fn insert_sfen(&mut self, sfen: &str) {
        self.sfen_history.push(sfen.to_string());
    }

    fn insert_move(&mut self, move_record: MoveRecord<Square12>) {
        self.move_history.push(move_record)
    }

    fn detect_repetition(&self) -> Result<(), MoveError> {
        if self.sfen_history.len() < 9 {
            return Ok(());
        }

        let cur = self.sfen_history.last().unwrap();
        let lm = cur;
        let lm_str = cur.split_whitespace().rev().last().unwrap();
        let mut cnt = 0;
        for (_i, entry) in self.sfen_history.iter().rev().enumerate() {
            let s = entry.split_whitespace().rev().last().unwrap();
            if lm == entry && s == lm_str {
                cnt += 1;
                if cnt == 3 {
                    return Err(MoveError::RepetitionDraw);
                }
            }
        }
        Ok(())
    }

    fn clear_sfen_history(&mut self) {
        self.sfen_history.clear();
    }

    fn set_sfen_history(&mut self, history: Vec<String>) {
        self.sfen_history = history;
    }

    fn set_move_history(&mut self, history: Vec<MoveRecord<Square12>>) {
        self.move_history = history;
    }

    fn move_history(&self) -> &[MoveRecord<Square12>] {
        &self.move_history
    }

    fn get_move_history(&self) -> &Vec<MoveRecord<Square12>> {
        &self.move_history
    }

    fn get_sfen_history(&self) -> &Vec<String> {
        &self.sfen_history
    }

    fn parse_sfen_ply(&mut self, s: &str) -> Result<(), SfenError> {
        self.ply = s.parse()?;
        Ok(())
    }

    fn generate_plinths(&mut self) {
        self.color_bb[Color::NoColor.index()] = PlinthGen12::default().start();
    }

    fn insert_in_hand(&mut self, p: Piece, num_pieces: u8) {
        self.hand
            .set(p, if num_pieces == 0 { 1 } else { num_pieces })
    }

    fn set_hand(&mut self, s: &str) {
        self.hand.set_hand(&s);
    }

    fn get_hand(&self, c: Color) -> String {
        self.hand.to_sfen(c)
    }

    fn get_hand_piece(&self, p: Piece) -> u8 {
        self.hand.get(p)
    }

    fn clear_hand(&mut self) {
        self.hand.clear();
    }

    fn hand(&self, p: Piece) -> u8 {
        self.hand.get(p)
    }

    fn king_files<const K: usize>(&self) -> [&str; K] {
        let temp: [&str; 6] = ["d", "e", "f", "g", "h", "i"];
        let mut files: [&str; K] = [""; K];
        for (i, v) in temp.iter().enumerate() {
            files[i] = v;
        }
        files
    }

    fn black_ranks(&self) -> [usize; 3] {
        [11, 10, 9]
    }

    fn file_bb(&self, file: usize) -> BB12<Square12> {
        FILE_BB[file]
    }

    fn white_files(&self) -> BB12<Square12> {
        todo!()
    }

    fn black_files(&self) -> BB12<Square12> {
        todo!()
    }

    fn is_hand_empty(&self, c: Color, excluded: PieceType) -> bool {
        todo!()
    }

    fn decrement_hand(&mut self, p: Piece) {
        todo!()
    }

    fn update_bb(&mut self, p: Piece, sq: Square12) {
        todo!()
    }

    fn halfmoves(&self) -> BB12<Square12> {
        todo!()
    }

    fn dimensions(&self) -> u8 {
        12
    }

    fn us(&self) -> BB12<Square12> {
        self.color_bb[self.side_to_move.index()]
    }

    fn them(&self) -> BB12<Square12> {
        self.color_bb[self.side_to_move.flip().index()]
    }

    fn play(&mut self, from: &str, to: &str) -> Result<&Outcome, SfenError> {
        todo!()
    }

    fn update_player(&mut self, piece: Piece, sq: &Square12) {
        self.set_piece(*sq, Some(piece));
        self.occupied_bb |= sq;
        self.color_bb[piece.color.index()] |= sq;
        self.type_bb[piece.piece_type.index()] |= sq;
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
            write!(fmt, "{:?} ", pc)?;
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
            sfen_history: Default::default(),
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
