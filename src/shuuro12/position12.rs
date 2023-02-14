use std::{
    fmt,
    marker::PhantomData,
    ops::{BitAnd, BitOr, BitXorAssign, Not},
};

use crate::{
    bitboard::BitBoard,
    position::{Board, Outcome, Placement, Play, Position, Sfen},
    Color, Hand, Move, MoveError, MoveRecord, Piece, PieceType, SfenError, Square, Variant,
};

use super::{
    attacks12::Attacks12,
    bitboard12::BB12,
    board_defs::{FILE_BB, RANK_BB},
    plinths_set12::PlinthGen12,
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
        12
    }
}

impl Sfen<Square12, BB12<Square12>, Attacks12<Square12, BB12<Square12>>>
    for P12<Square12, BB12<Square12>>
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

    fn file_bb(&self, file: usize) -> BB12<Square12> {
        FILE_BB[file]
    }

    fn update_bb(&mut self, p: Piece, sq: Square12) {
        self.set_piece(sq, Some(p));
        self.occupied_bb |= &sq;
        self.color_bb[p.color.index()] |= &sq;
        self.type_bb[p.piece_type.index()] |= &sq;
    }
}

impl Play<Square12, BB12<Square12>, Attacks12<Square12, BB12<Square12>>>
    for P12<Square12, BB12<Square12>>
{
    fn make_move(&mut self, m: Move<Square12>) -> Result<Outcome, MoveError> {
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
                if moved.piece_type == PieceType::Pawn && to.in_promotion_zone(moved.color) {
                    promoted = true;
                }
            }
            None => {
                if moved.piece_type == PieceType::Pawn && to.in_promotion_zone(moved.color) {
                    promoted = true;
                }
            }
        }

        if let Some(attacks) = legal_moves.get(&from) {
            if (attacks & &to).is_empty() {
                return Err(MoveError::Inconsistent("The piece cannot move to there"));
            }
        } else {
            return Err(MoveError::Inconsistent("The piece cannot move to there"));
        }

        let placed = if promoted {
            match moved.promote() {
                Some(promoted) => promoted,
                None => {
                    return Err(MoveError::Inconsistent("This type of piece cannot promote"));
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

// impl Position<Square12, BB12<Square12>, Attacks12<Square12, BB12<Square12>>>
//     for P12<Square12, BB12<Square12>>
// {
//     const ID: usize = 3;

//     fn make_move(&mut self, m: Move<Square12>) -> Result<Outcome, MoveError> {
//         let mut promoted = false;
//         let stm = self.side_to_move();
//         let opponent = stm.flip();
//         let (from, to) = m.info();
//         let moved = self
//             .piece_at(from)
//             .ok_or(MoveError::Inconsistent("No piece found"))?;
//         let captured = *self.piece_at(to);
//         let outcome = Outcome::Checkmate { color: opponent };
//         let legal_moves = self.legal_moves(&stm);

//         if moved.color != stm {
//             return Err(MoveError::Inconsistent(
//                 "The piece is not for the side to move",
//             ));
//         } else if self.game_status == outcome {
//             return Err(MoveError::Inconsistent("Match is over."));
//         }

//         match captured {
//             Some(_i) => {
//                 if moved.piece_type == PieceType::Pawn && to.in_promotion_zone(moved.color) {
//                     promoted = true;
//                 }
//             }
//             None => {
//                 if moved.piece_type == PieceType::Pawn && to.in_promotion_zone(moved.color) {
//                     promoted = true;
//                 }
//             }
//         }

//         if let Some(attacks) = legal_moves.get(&from) {
//             if (attacks & &to).is_empty() {
//                 return Err(MoveError::Inconsistent("The piece cannot move to there"));
//             }
//         } else {
//             return Err(MoveError::Inconsistent("The piece cannot move to there"));
//         }

//         let placed = if promoted {
//             match moved.promote() {
//                 Some(promoted) => promoted,
//                 None => {
//                     return Err(MoveError::Inconsistent("This type of piece cannot promote"));
//                 }
//             }
//         } else {
//             moved
//         };

//         self.set_piece(from, None);
//         self.set_piece(to, Some(placed));
//         self.occupied_bb ^= &from;
//         self.occupied_bb ^= &to;
//         self.type_bb[moved.piece_type.index()] ^= &from;
//         self.type_bb[placed.piece_type.index()] ^= &to;
//         self.color_bb[moved.color.index()] ^= &from;
//         self.color_bb[placed.color.index()] ^= &to;

//         if let Some(ref cap) = captured {
//             self.occupied_bb ^= &to;
//             self.type_bb[cap.piece_type.index()] ^= &to;
//             self.color_bb[cap.color.index()] ^= &to;
//             //self.hand.increment(pc);
//         }

//         self.side_to_move = opponent;
//         self.ply += 1;

//         let move_record = MoveRecord::Normal {
//             from,
//             to,
//             placed,
//             captured,
//             promoted,
//         };

//         self.move_history.push(move_record);

//         self.log_position();
//         self.detect_repetition()?;
//         self.detect_insufficient_material()?;

//         if self.is_checkmate(&self.side_to_move) {
//             return Ok(Outcome::Checkmate {
//                 color: self.side_to_move.flip(),
//             });
//         } else if self.in_check(self.side_to_move) {
//             return Ok(Outcome::Check {
//                 color: self.side_to_move,
//             });
//         } else if (&self.color_bb[self.side_to_move.flip().index()]
//             & &self.type_bb[PieceType::King.index()])
//             .count()
//             == 0
//         {
//             return Ok(Outcome::Checkmate {
//                 color: self.side_to_move.flip(),
//             });
//         }
//         self.is_stalemate(&self.side_to_move)?;
//         Ok(Outcome::MoveOk)
//     }

//     fn insert_sfen(&mut self, sfen: &str) {
//         self.sfen_history.push(sfen.to_string());
//     }

//     fn insert_move(&mut self, move_record: MoveRecord<Square12>) {
//         self.move_history.push(move_record)
//     }

//     fn detect_repetition(&self) -> Result<(), MoveError> {
//         if self.sfen_history.len() < 9 {
//             return Ok(());
//         }

//         let cur = self.sfen_history.last().unwrap();
//         let lm = cur;
//         let lm_str = cur.split_whitespace().rev().last().unwrap();
//         let mut cnt = 0;
//         for (_i, entry) in self.sfen_history.iter().rev().enumerate() {
//             let s = entry.split_whitespace().rev().last().unwrap();
//             if lm == entry && s == lm_str {
//                 cnt += 1;
//                 if cnt == 3 {
//                     return Err(MoveError::RepetitionDraw);
//                 }
//             }
//         }
//         Ok(())
//     }

//     fn clear_sfen_history(&mut self) {
//         self.sfen_history.clear();
//     }

//     fn set_sfen_history(&mut self, history: Vec<String>) {
//         self.sfen_history = history;
//     }

//     fn set_move_history(&mut self, history: Vec<MoveRecord<Square12>>) {
//         self.move_history = history;
//     }

//     fn move_history(&self) -> &[MoveRecord<Square12>] {
//         &self.move_history
//     }

//     fn get_move_history(&self) -> &Vec<MoveRecord<Square12>> {
//         &self.move_history
//     }

//     fn get_sfen_history(&self) -> &Vec<String> {
//         &self.sfen_history
//     }

//     fn parse_sfen_ply(&mut self, s: &str) -> Result<(), SfenError> {
//         self.ply = s.parse()?;
//         Ok(())
//     }

//     fn generate_plinths(&mut self) {
//         self.color_bb[Color::NoColor.index()] = PlinthGen12::default().start();
//     }

//     fn insert_in_hand(&mut self, p: Piece, num_pieces: u8) {
//         self.hand
//             .set(p, if num_pieces == 0 { 1 } else { num_pieces })
//     }

//     fn set_hand(&mut self, s: &str) {
//         self.hand.set_hand(s);
//     }

//     fn get_hand(&self, c: Color) -> String {
//         self.hand.to_sfen(c)
//     }

//     fn get_hand_piece(&self, p: Piece) -> u8 {
//         self.hand.get(p)
//     }

//     fn clear_hand(&mut self) {
//         self.hand.clear();
//     }

//     fn hand(&self, p: Piece) -> u8 {
//         self.hand.get(p)
//     }

//     fn king_files<const K: usize>(&self) -> [&str; K] {
//         let temp: [&str; 6] = ["d", "e", "f", "g", "h", "i"];
//         let mut files: [&str; K] = [""; K];
//         for (i, v) in temp.iter().enumerate() {
//             files[i] = v;
//         }
//         files
//     }

//     fn black_ranks(&self) -> [usize; 3] {
//         [11, 10, 9]
//     }

//     fn file_bb(&self, file: usize) -> BB12<Square12> {
//         FILE_BB[file]
//     }

//     fn white_placement_ranks(&self) -> BB12<Square12> {
//         &RANK_BB[1] | &RANK_BB[2]
//     }
//     fn black_placement_ranks(&self) -> BB12<Square12> {
//         &RANK_BB[9] | &RANK_BB[10]
//     }

//     fn decrement_hand(&mut self, p: Piece) {
//         self.hand.decrement(p);
//     }

//     fn update_bb(&mut self, p: Piece, sq: Square12) {
//         self.set_piece(sq, Some(p));
//         self.occupied_bb |= &sq;
//         self.color_bb[p.color.index()] |= &sq;
//         self.type_bb[p.piece_type.index()] |= &sq;
//     }

//     fn dimensions(&self) -> u8 {
//         12
//     }

//     fn us(&self) -> BB12<Square12> {
//         self.color_bb[self.side_to_move.index()]
//     }

//     fn them(&self) -> BB12<Square12> {
//         self.color_bb[self.side_to_move.flip().index()]
//     }

//     fn play(&mut self, from: &str, to: &str) -> Result<&Outcome, SfenError> {
//         let from_: Square12;
//         let to_: Square12;
//         match Square12::from_sfen(from) {
//             Some(i) => from_ = i,
//             None => {
//                 return Err(SfenError::IllegalPieceFound);
//             }
//         };
//         match Square12::from_sfen(to) {
//             Some(i) => to_ = i,
//             None => {
//                 return Err(SfenError::IllegalPieceFound);
//             }
//         };
//         let m = Move::Normal {
//             from: from_,
//             to: to_,
//             promote: false,
//         };
//         let outcome = self.make_move(m);
//         match outcome {
//             Ok(i) => {
//                 self.game_status = i;
//             }
//             Err(error) => match error {
//                 MoveError::RepetitionDraw => self.game_status = Outcome::DrawByRepetition,
//                 MoveError::Draw => self.game_status = Outcome::Draw,
//                 MoveError::DrawByInsufficientMaterial => self.game_status = Outcome::DrawByMaterial,
//                 MoveError::DrawByStalemate => self.game_status = Outcome::Stalemate,
//                 _ => {
//                     return Err(SfenError::IllegalMove);
//                 }
//             },
//         }

//         return Ok(self.outcome());
//     }

//     fn update_player(&mut self, piece: Piece, sq: &Square12) {
//         self.set_piece(*sq, Some(piece));
//         self.occupied_bb |= sq;
//         self.color_bb[piece.color.index()] |= sq;
//         self.type_bb[piece.piece_type.index()] |= sq;
//     }
// }

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
