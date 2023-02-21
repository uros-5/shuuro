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
    attacks8::Attacks8, bitboard8::BB8, board_defs::RANK_BB,
    plinths_set8::PlinthGen8, square8::Square8,
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
