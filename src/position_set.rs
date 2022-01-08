use std::fmt::{Display, Formatter, Result};

use crate::{
    generate_plynths, get_sliding_attacks, BitBoard, Color, Hand, Piece, PieceGrid, PieceType,
    SfenStr, Square, EMPTY_BB, FILE_BB,
};

/// Second phase of shuuro is done in this struct.
pub struct PositionSet {
    occupied_bb: BitBoard,
    color_bb: [BitBoard; 3],
    type_bb: [BitBoard; 7],
    side_to_move: Color,
    board: PieceGrid,
    hand: Hand,
    ply: u16,
}

impl PositionSet {
    /// Hand from shop is recommended to call this function.
    pub fn set_hand(&mut self, s: &str) {
        self.hand.set_hand(&s);
    }

    /// Get hand for specific color (in order K..P).
    pub fn get_hand(&mut self, c: Color) -> String {
        return self.hand.to_sfen(c);
    }

    /// Squares for king to be placed.
    fn king_squares(&self, c: &Color) -> BitBoard {
        let files = ['d', 'e', 'f', 'g', 'h', 'i'];
        let mut bb = EMPTY_BB;
        let mut all = |num: usize| -> BitBoard {
            for file in files {
                bb |= Square::from_sfen(&format!("{}{}", file, num)[..]).unwrap();
            }
            bb
        };
        match *c {
            Color::Blue => all(12),
            Color::Red => all(1),
            Color::NoColor => EMPTY_BB,
        }
    }

    /// Available squares for selected piece.
    pub fn empty_squares(&self, p: Piece) -> BitBoard {
        let test = |p: Piece, list: [usize; 3]| -> BitBoard {
            for file in list {
                let mut bb = FILE_BB[file];
                bb &= &!&self.color_bb[p.color.index()];
                let plynths = self.color_bb[Color::NoColor.index()];
                if bb.is_empty() {
                    continue;
                }
                match p.piece_type {
                    PieceType::Knight => {
                        return bb;
                    }
                    PieceType::King => {
                        self.king_squares(&p.color);
                    }
                    PieceType::Pawn => {
                        bb &= &!&plynths;
                        if bb.is_empty() {
                            continue;
                        } else if self.can_pawn_move(p) {
                            return bb;
                        } else {
                            return EMPTY_BB;
                        }
                    }
                    _ => {
                        bb &= &!&plynths;
                        if bb.is_empty() {
                            continue;
                        }
                        return bb;
                    }
                }
            }
            EMPTY_BB
        };
        let checks = self.checks(&p.color);
        if checks.is_any() {
            return checks;
        }
        match p.color {
            Color::Red => test(p, [0, 1, 2]),
            Color::Blue => test(p, [11, 10, 9]),
            Color::NoColor => EMPTY_BB,
        }
    }
    /// Returns BitBoard for all safe squares for selected side.
    fn checks(&self, attacked_color: &Color) -> BitBoard {
        let king = &self.type_bb[PieceType::King.index()] & &self.color_bb[attacked_color.index()];
        let mut all;
        for p in [PieceType::Queen, PieceType::Rook, PieceType::Bishop] {
            let bb = &self.type_bb[p.index()] & &self.color_bb[attacked_color.flip().index()];
            for i in bb {
                all = get_sliding_attacks(p, i, self.occupied_bb);
                if (&all & &king).is_any() {
                    match *attacked_color {
                        Color::Red => {
                            let files = &FILE_BB[1] | &FILE_BB[2];
                            return &(&files & &all) & &!&king;
                        }
                        Color::Blue => {
                            let files = &FILE_BB[9] | &FILE_BB[10];
                            return &(&files & &all) & &!&king;
                        }
                        Color::NoColor => {
                            return EMPTY_BB;
                        }
                    }
                }
            }
        }
        EMPTY_BB
    }

    /// Returns true if pawns can be placed on board.
    fn can_pawn_move(&self, p: Piece) -> bool {
        if self.is_hand_empty(&p.color, PieceType::Pawn) {
            true
        } else {
            false
        }
    }

    /// Returns true if hand with excluded piece is empty.
    pub fn is_hand_empty(&self, c: &Color, excluded: PieceType) -> bool {
        for pt in PieceType::iter() {
            if pt != excluded {
                let counter = self.hand.get(Piece {
                    piece_type: pt,
                    color: *c,
                });
                if counter != 0 {
                    return false;
                }
            }
        }
        true
    }

    /// Placing piece on square.
    pub fn place(&mut self, p: Piece, sq: Square) {
        if self.hand.get(p) > 0 {
            if (&self.empty_squares(p) & sq).is_any() {
                self.update_bb(p, sq);
                self.hand.decrement(p);
                if !self.is_hand_empty(&p.color.flip(), PieceType::Plynth) {
                    self.side_to_move = p.color.flip();
                }
            }
        }
    }

    /// Generating random plynths.
    pub fn generate_plynths(&mut self) {
        self.color_bb[Color::NoColor.index()] = generate_plynths();
    }
}

impl SfenStr for PositionSet {
    fn piece_at(&self, sq: Square) -> &Option<Piece> {
        self.board.get(sq)
    }

    fn player_bb(&self, c: Color) -> BitBoard {
        self.color_bb[c.index()]
    }

    fn stm(&self) -> Color {
        self.side_to_move
    }

    fn hand_get(&self, p: Piece) -> u8 {
        self.hand.get(p)
    }

    fn ply(&self) -> u16 {
        self.ply
    }

    fn set_piece(&mut self, sq: Square, p: Option<Piece>) {
        self.board.set(sq, p);
    }

    fn empty_bb(&mut self) {
        self.occupied_bb = BitBoard::empty();
        self.color_bb = Default::default();
        self.type_bb = Default::default();
    }

    fn update_color_bb(&mut self, c: Color, sq: Square) {
        self.color_bb[c.index()] |= sq;
    }

    fn update_bb(&mut self, p: Piece, sq: Square) {
        self.set_piece(sq, Some(p));
        self.occupied_bb |= sq;
        self.color_bb[p.color.index()] |= sq;
        self.type_bb[p.piece_type.index()] |= sq;
    }
}

impl Default for PositionSet {
    fn default() -> Self {
        PositionSet {
            occupied_bb: BitBoard::empty(),
            color_bb: Default::default(),
            type_bb: Default::default(),
            hand: Default::default(),
            board: PieceGrid::default(),
            side_to_move: Color::Red,
            ply: 1,
        }
    }
}

impl Display for PositionSet {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.occupied_bb)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::consts::*;
    use crate::{init, Color, Piece, PieceType, PositionSet, SfenStr};

    fn setup() {
        init();
    }

    #[test]
    fn king_squares() {
        let position_set = PositionSet::default();
        let cases = [
            (Color::Blue, 6, [D12, E12, F12, G12, H12, I12]),
            (Color::Red, 6, [D1, E1, F1, G1, H1, I1]),
        ];
        for case in cases {
            let bb = position_set.king_squares(&case.0);
            assert_eq!(bb.count(), case.1);
            for sq in case.2 {
                assert!((&bb & sq).is_any());
            }
        }
    }

    #[test]
    fn is_hand_empty() {
        setup();
        let mut position_set = PositionSet::default();
        position_set
            .parse_sfen_board("6K5/57/57/57/57/57/57/57/57/57/57/7k4")
            .expect("error while parsing sfen");
        position_set.set_hand("rrRqNNqq");
        let cases = [
            (PieceType::Knight, Color::Red, A1),
            (PieceType::Queen, Color::Blue, D12),
            (PieceType::Rook, Color::Red, C1),
            (PieceType::Queen, Color::Blue, I12),
            (PieceType::Knight, Color::Red, H1),
            (PieceType::Rook, Color::Blue, B12),
            (PieceType::Rook, Color::Blue, G12),
            (PieceType::Queen, Color::Blue, F12),
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
        assert!(position_set.is_hand_empty(&Color::Blue, PieceType::Plynth));
        assert!(position_set.is_hand_empty(&Color::Red, PieceType::Plynth));
    }

    #[test]
    fn generate_plynths() {
        setup();
        let mut position_set = PositionSet::default();
        position_set.generate_plynths();
        assert_eq!(position_set.color_bb[Color::NoColor.index()].count(), 8);
    }

    #[test]
    fn empty_squares() {
        setup();
        let mut position_set = PositionSet::default();
        position_set
            .parse_sfen_board("5KRRR3/4PPPP4/57/5L06/57/57/57/57/57/57/57/L04kqqLn3")
            .expect("error while parsing sfen");
        position_set.set_hand("NrNNbrn");
        let cases = [
            (PieceType::Knight, Color::Blue, 8),
            (PieceType::Bishop, Color::Blue, 7),
        ];
        for case in cases {
            let file = position_set.empty_squares(Piece {
                piece_type: case.0,
                color: case.1,
            });
            println!("{}", file);
            assert_eq!(file.count(), case.2);
        }
        assert_eq!(position_set.get_hand(Color::Blue), "rrbn");
    }
}
