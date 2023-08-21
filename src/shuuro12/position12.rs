use std::{
    fmt,
    marker::PhantomData,
    ops::{BitAnd, BitOr, BitXorAssign, Not},
};

use crate::{
    bitboard::BitBoard,
    position::{Board, Outcome, Placement, Play, Position, Sfen},
    Color, Hand, MoveData, MoveRecord, Piece, PieceType, SfenError, Square,
    Variant,
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
    pub type_bb: [BB12<Square12>; 17],
    _a: PhantomData<B>,
    _s: PhantomData<S>,
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

    fn insert_sfen(&mut self, sfen: &str) {
        self.sfen_history.push(sfen.to_string());
    }

    fn insert_move(&mut self, move_record: MoveRecord<Square12>) {
        self.move_history.push(move_record)
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
        &RANK_BB[1] | &RANK_BB[2]
    }

    fn black_placement_attacked_ranks(&self) -> BB12<Square12> {
        &RANK_BB[9] | &RANK_BB[10]
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

impl fmt::Display for P12<Square12, BB12<Square12>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "+---+---+---+---+---+---+---+---+---+---+---+---+")?;

        for rank in (0..12).rev() {
            write!(f, "|")?;
            for file in 0..12 {
                if let Some(ref piece) =
                    *self.piece_at(Square12::new(file, rank).unwrap())
                {
                    write!(f, "{piece}")?;
                    let plinth: BB12<Square12> = &self
                        .player_bb(Color::NoColor)
                        & &Square12::new(file, rank).unwrap();
                    if plinth.is_any() {
                        write!(f, " L|")?;
                    } else {
                        write!(f, "  |")?;
                    }
                } else {
                    let plinth: BB12<Square12> = &self
                        .player_bb(Color::NoColor)
                        & &Square12::new(file, rank).unwrap();
                    if plinth.is_any() {
                        write!(f, "{:>3}|", "L")?;
                    } else {
                        write!(f, "   |")?;
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
