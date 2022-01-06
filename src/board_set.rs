use std::fmt::{Display, Formatter, Result};

use crate::{
    get_sliding_attacks, BitBoard, Color, Hand, Piece, PieceGrid, PieceType, SfenStr, Square,
    EMPTY_BB, FILE_BB,
};

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
    pub fn set_hand(&mut self, s: &str) {
        self.hand.set_hand(&s);
    }

    pub fn get_hand(&mut self, c: Color) -> String {
        return self.hand.to_sfen(c);
    }

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

    pub fn empty_file(&self, p: Piece) -> BitBoard {
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

    pub fn place(&mut self, p: Piece, sq: Square) {
        if self.hand.get(p) > 0 {
            if (&self.empty_file(p) & sq).is_any() {
                self.update_bb(p, sq);
                self.hand.decrement(p);
            }
        }
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
        write!(f, "{}", self.occupied_bb);
        Ok(())
    }
}

mod tests {
    use crate::square::consts::*;
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
                println!("{}", &bb);
                assert!((&bb & sq).is_any());
            }
        }
    }

    #[test]
    fn empty_file() {
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
            let file = position_set.empty_file(Piece {
                piece_type: case.0,
                color: case.1,
            });
            println!("{}", file);
            assert_eq!(file.count(), case.2);
        }
        assert_eq!(position_set.get_hand(Color::Blue), "rrbn");
    }

    #[test]
    fn all_squares() {
        let mut position_set = PositionSet::default();
        position_set.set_hand("RRRKkrrr");
    }
}
