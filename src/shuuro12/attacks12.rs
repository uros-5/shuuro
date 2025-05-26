// https://github.com/niklasf/shakmaty/blob/master/src/bootstrap.rs

use std::marker::PhantomData;

pub use crate::attacks::Attacks;
use crate::{attacks::Ray, bitboard::BitBoard, Color, PieceType, Square};

use super::{
    bitboard12::{BB12, SQUARE_BB},
    board_defs::{FILE_BB, RANK_BB},
    square12::Square12,
};

const KING_DELTAS: [i32; 8] = [13, 12, 11, 1, -13, -12, -11, -1];
const KNIGHT_DELTAS: [i32; 8] = [25, 23, 14, 10, -25, -23, -14, -10];
const GIRAFFE_DELTAS: [i32; 8] = [49, 47, 16, 8, -49, -47, -16, -8];
const WHITE_PAWN_DELTAS: [i32; 2] = [11, 13];
const BLACK_PAWN_DELTAS: [i32; 2] = [-11, -13];

pub static KNIGHT_ATTACKS: [BB12<Square12>; 144] =
    init_stepping_attacks(&KNIGHT_DELTAS);
pub static GIRAFFE_ATTACKS: [BB12<Square12>; 144] =
    init_stepping_attacks(&GIRAFFE_DELTAS);
pub static WHITE_PAWN_ATTACKS: [BB12<Square12>; 144] =
    init_stepping_attacks(&WHITE_PAWN_DELTAS);
pub static BLACK_PAWN_ATTACKS: [BB12<Square12>; 144] =
    init_stepping_attacks(&BLACK_PAWN_DELTAS);
pub static KING_MOVES: [BB12<Square12>; 144] =
    init_stepping_attacks(&KING_DELTAS);

static mut PAWN_MOVES: [[BB12<Square12>; 144]; 2] =
    [init_stepping_attacks(&[12]), init_stepping_attacks(&[-12])];

pub static mut RAYS: [[BB12<Square12>; 144]; 8] = [[BB12::new(0, 0); 144]; 8];

static mut BETWEEN_BB: [[BB12<Square12>; 144]; 144] =
    [[BB12::new(0, 0); 144]; 144];

#[derive(Clone, Copy, Debug, Default)]
pub struct Attacks12<S, B>
where
    S: Square,
    B: BitBoard<S>,
{
    _a: PhantomData<B>,
    _s: PhantomData<S>,
}

impl Attacks12<Square12, BB12<Square12>> {
    pub fn new() -> Self {
        Self {
            _a: PhantomData,
            _s: PhantomData,
        }
    }
}

impl Attacks<Square12, BB12<Square12>> for Attacks12<Square12, BB12<Square12>> {
    fn init_pawn_moves() {
        for color in [(Color::White, 0, 12), (Color::Black, 132, -12_isize)] {
            let index = color.0.index();
            match color.0 {
                Color::NoColor => (),
                _ => {
                    for i in color.1..color.1 + 12 {
                        unsafe {
                            PAWN_MOVES[index][i as usize] = BB12::empty();
                        }
                    }
                    let start = color.1 + color.2;
                    let end = start + 12;
                    for i in start..end {
                        let first = i + color.2;
                        let second = i + (color.2 * 2);
                        unsafe {
                            let sq = SQUARE_BB[first as usize]
                                | &SQUARE_BB[second as usize];
                            PAWN_MOVES[index][i as usize] |= &sq;
                        }
                    }
                }
            }
        }
    }

    fn init_quick() {}

    fn init_north_ray() {
        let empty = BB12::empty();
        for sq in 0..144 {
            let file = FILE_BB[sq % 12];
            let rank = sq / 12;
            let mut bb = file | &empty;
            (0..rank).for_each(|j| {
                bb &= &!&RANK_BB[j];
            });
            bb &= &!&SQUARE_BB[sq];
            unsafe {
                RAYS[Ray::North as usize][sq] = bb;
            }
        }
    }

    fn init_south_ray() {
        let empty = BB12::empty();
        for sq in 0..144 {
            let file = FILE_BB[sq % 12];
            let rank = sq / 12;
            let mut bb = file | &empty;
            (rank..12).for_each(|j| {
                bb &= &!&RANK_BB[j];
            });
            bb &= &!&SQUARE_BB[sq];
            unsafe {
                RAYS[Ray::South as usize][sq] = bb;
            }
        }
    }

    fn init_east_ray() {
        let empty = BB12::empty();
        for sq in 0..144 {
            let rank = RANK_BB[sq / 12];
            let mut bb = rank | &empty;
            (0..sq % 12).for_each(|j| {
                bb &= &!&FILE_BB[j];
            });
            bb &= &!&SQUARE_BB[sq];
            unsafe {
                RAYS[Ray::East as usize][sq] = bb;
            }
        }
    }

    fn init_west_ray() {
        let empty = BB12::empty();
        for sq in 0..144 {
            let rank = RANK_BB[sq / 12];
            let mut bb = rank | &empty;
            (sq % 12..12).for_each(|j| {
                bb &= &!&FILE_BB[j];
            });
            bb &= &!&SQUARE_BB[sq];
            unsafe {
                RAYS[Ray::West as usize][sq] = bb;
            }
        }
    }

    fn init_north_east_ray() {
        let delta = &[13];
        for sq in Square12::iter() {
            let bb = diagonal_ray(sq.index() as i32, delta, 13);
            unsafe {
                RAYS[Ray::NorthEast as usize][sq.index()] = bb;
            }
        }
    }

    fn init_north_west_ray() {
        let delta = &[11];
        for sq in Square12::iter() {
            let bb = diagonal_ray(sq.index() as i32, delta, 11);
            unsafe {
                RAYS[Ray::NorthWest as usize][sq.index()] = bb;
            }
        }
    }

    fn init_south_east_ray() {
        let delta = &[-11];
        for sq in Square12::iter() {
            let bb = diagonal_ray(sq.index() as i32, delta, -11);
            unsafe {
                RAYS[Ray::SouthEast as usize][sq.index()] = bb;
            }
        }
    }

    fn init_south_west_ray() {
        let delta = &[-13];
        for sq in Square12::iter() {
            let bb = diagonal_ray(sq.index() as i32, delta, -13);
            unsafe {
                RAYS[Ray::SouthWest as usize][sq.index()] = bb;
            }
        }
    }

    fn init_between() {
        for from in Square12::iter() {
            for to in Square12::iter() {
                if from == to {
                    continue;
                }
                let df = from.file() as i8 - to.file() as i8;
                let dr = from.rank() as i8 - to.rank() as i8;
                unsafe {
                    if df == 0 || dr == 0 {
                        BETWEEN_BB[from.index()][to.index()] =
                            Attacks12::get_sliding_attacks(
                                PieceType::Rook,
                                &from,
                                SQUARE_BB[to.index()],
                            ) & &Attacks12::get_sliding_attacks(
                                PieceType::Rook,
                                &to,
                                SQUARE_BB[from.index()],
                            );
                    } else if df.abs() == dr.abs() {
                        BETWEEN_BB[from.index()][to.index()] =
                            Attacks12::get_sliding_attacks(
                                PieceType::Bishop,
                                &from,
                                SQUARE_BB[to.index()],
                            ) & &Attacks12::get_sliding_attacks(
                                PieceType::Bishop,
                                &to,
                                SQUARE_BB[from.index()],
                            );
                    }
                }
            }
        }
    }

    fn get_non_sliding_attacks(
        piece_type: PieceType,
        square: &Square12,
        color: Color,
        _blockers: BB12<Square12>,
    ) -> BB12<Square12> {
        match piece_type {
            PieceType::King => KING_MOVES[square.index()],
            PieceType::Knight => KNIGHT_ATTACKS[square.index()],
            PieceType::Giraffe => GIRAFFE_ATTACKS[square.index()],
            PieceType::Pawn => match color {
                Color::Black => BLACK_PAWN_ATTACKS[square.index()], // & &blockers,
                Color::White => WHITE_PAWN_ATTACKS[square.index()], // & &blockers,
                Color::NoColor => BB12::empty(),
            },
            _ => BB12::empty(),
        }
    }

    fn get_giraffe_attacks(square: &Square12) -> BB12<Square12> {
        GIRAFFE_ATTACKS[square.index()]
    }

    fn get_sliding_attacks(
        piece_type: PieceType,
        square: &Square12,
        blockers: BB12<Square12>,
    ) -> BB12<Square12> {
        match piece_type {
            PieceType::Bishop | PieceType::ArchBishop => {
                Attacks12::get_bishop_attacks(square.index(), blockers)
            }
            PieceType::Rook | PieceType::Chancellor => {
                Attacks12::get_rook_attacks(square.index(), blockers)
            }
            PieceType::Queen => {
                Attacks12::get_bishop_attacks(square.index(), blockers)
                    | &Attacks12::get_rook_attacks(square.index(), blockers)
            }
            _ => BB12::empty(),
        }
    }

    fn get_positive_ray_attacks(
        dir: Ray,
        square: usize,
        blockers: BB12<Square12>,
    ) -> BB12<Square12> {
        unsafe {
            let attacks = RAYS[dir as usize][square];
            let mut blocked = attacks & &blockers;
            let block_square = blocked.pop();
            match block_square {
                Some(i) => attacks & &!&RAYS[dir as usize][i.index()],
                None => attacks,
            }
        }
    }

    fn get_negative_ray_attacks(
        dir: Ray,
        square: usize,
        blockers: BB12<Square12>,
    ) -> BB12<Square12> {
        unsafe {
            let attacks = RAYS[dir as usize][square];
            let mut blocked = attacks & &blockers;
            let block_square = blocked.pop_reverse();
            match block_square {
                Some(i) => attacks & &!&RAYS[dir as usize][i.index()],
                None => attacks,
            }
        }
    }

    fn between(square1: Square12, square2: Square12) -> BB12<Square12> {
        unsafe { BETWEEN_BB[square1.index()][square2.index()] }
    }

    fn get_pawn_moves(square: usize, color: Color) -> BB12<Square12> {
        match color {
            Color::White | Color::Black => unsafe {
                PAWN_MOVES[color.index()][square]
            },
            Color::NoColor => BB12::empty(),
        }
    }
}

const fn sliding_attacks(square: i32, deltas: &[i32]) -> BB12<Square12> {
    let mut attack = BB12::new(0, 0);
    let mut i = 0;
    let len = deltas.len();
    let mut diff_limit = 2;
    if deltas[0] == 49 {
        diff_limit = 4;
    }
    while i < len {
        let mut previous = square;
        loop {
            let sq = previous + deltas[i];
            let file_diff = (sq % 12) - (previous % 12);
            if file_diff > diff_limit || file_diff < -diff_limit || sq < 0 {
                break;
            } else if sq > 127 && sq < 144 {
                attack.0 .1 |= SQUARE_BB[sq as usize].0 .1;
                break;
            } else if sq > 127 {
                break;
            }

            let bb = 1 << sq;
            attack.0 .0 |= bb;
            if attack.0 .0 & bb != 0 {
                break;
            }
            previous = sq;
        }
        i += 1;
    }

    attack
}

const fn init_stepping_attacks(deltas: &[i32]) -> [BB12<Square12>; 144] {
    let mut table = [BB12::new(0, 0); 144];
    let mut sq = 0;
    while sq < 144 {
        table[sq] = sliding_attacks(sq as i32, deltas);
        sq += 1;
    }
    table
}

fn diagonal_ray(start: i32, delta: &[i32; 1], new_sq: i32) -> BB12<Square12> {
    let mut sq = start;
    let mut bb = BB12::empty();
    loop {
        let b = sliding_attacks(sq, delta);
        if b.len() == 0 {
            break;
        }
        bb |= &b;
        sq += new_sq;
    }
    bb
}

#[cfg(test)]
mod tests2 {
    use crate::{
        attacks::Ray,
        bitboard::BitBoard,
        shuuro12::{
            attacks12::{BLACK_PAWN_ATTACKS, WHITE_PAWN_ATTACKS},
            bitboard12::square_bb,
            board_defs::EMPTY_BB,
            square12::consts::*,
        },
        Color, Square,
    };

    use super::{
        Attacks, Attacks12, GIRAFFE_ATTACKS, KING_MOVES, KNIGHT_ATTACKS,
        PAWN_MOVES, RAYS,
    };

    #[test]
    fn pawn_moves() {
        Attacks12::init_pawn_moves();
        let ok_cases = [
            (A2, square_bb(&A3) | &square_bb(&A4), Color::White, true, 2),
            (L11, square_bb(&L12), Color::White, true, 1),
            (C12, EMPTY_BB, Color::White, false, 0),
            (G12, EMPTY_BB, Color::White, false, 0),
            (
                H11,
                square_bb(&H10) | &square_bb(&H9),
                Color::Black,
                true,
                2,
            ),
            (D4, square_bb(&D3), Color::Black, true, 1),
            (A2, square_bb(&A1), Color::Black, true, 1),
            (L1, EMPTY_BB, Color::Black, false, 0),
        ];

        for case in ok_cases {
            unsafe {
                let bb = PAWN_MOVES[case.2.index()][case.0.index()];
                let moves = bb & &case.1;
                assert_eq!(moves.is_any(), case.3);
                assert_eq!(bb.len(), case.4);
            };
        }
    }

    #[test]
    fn pawn_attacks() {
        let ok_cases = [
            (A1, [Some(B2), None], 1, Color::White),
            (D1, [Some(E2), Some(C2)], 2, Color::White),
            (F11, [Some(G12), Some(E12)], 2, Color::White),
            (C12, [None, None], 0, Color::White),
            (A12, [Some(B11), None], 1, Color::Black),
            (C12, [Some(B11), Some(D11)], 2, Color::Black),
            (G2, [Some(H1), Some(F1)], 2, Color::Black),
            (H1, [None, None], 0, Color::Black),
        ];

        for case in ok_cases {
            let sq = case.0.index();
            match case.3 {
                Color::White => {
                    let attacks = WHITE_PAWN_ATTACKS[sq];

                    for attack in case.1.into_iter().flatten() {
                        assert!((attacks & &attack).is_any());
                    }
                    assert_eq!(attacks.len(), case.2);
                }
                Color::Black => {
                    let attacks = BLACK_PAWN_ATTACKS[sq];
                    let mut count = 0;
                    for attack in case.1.into_iter().flatten() {
                        assert!((attacks & &attack).is_any());
                        count += 1;
                    }
                    assert_eq!(attacks.len(), count);
                }
                Color::NoColor => (),
            }
        }
    }

    #[test]
    fn knight_attacks() {
        let knight_cases = [
            (A1, vec![B3, C2]),
            (E4, vec![D2, F2, C3, G3, C5, G5, D6, F6]),
            (B11, vec![D12, D10, C9, A9]),
            (L8, vec![K10, J9, J7, K6]),
        ];
        for case in knight_cases {
            let sq = case.0.index();
            let attacks = KNIGHT_ATTACKS[sq];
            let capacity = case.1.len();
            for sq in case.1 {
                assert!((attacks & &sq).is_any());
            }
            assert_eq!(attacks.len(), capacity as u32);
            // assert!(false);
        }
    }

    #[test]
    fn king_attacks() {
        let king_cases = [
            (L1, vec![K1, K2, L2], Color::White),
            (L12, vec![K12, K11, L11], Color::White),
            (
                D11,
                vec![D12, C12, E12, E11, C11, C10, D10, E10],
                Color::Black,
            ),
            (A5, vec![A6, B6, B5, B4, A4], Color::Black),
        ];

        for case in king_cases {
            let sq = case.0.index();
            let attacks = KING_MOVES[sq];
            for attack in case.1 {
                assert!((attacks & &attack).is_any());
            }
        }
    }

    #[test]
    fn rays() {
        Attacks12::init();
        let ok_cases = [
            (A1, A12, 10, Ray::North),
            (D6, D12, 5, Ray::North),
            (L11, L12, 0, Ray::North),
            (E3, A3, 3, Ray::West),
            (G6, B6, 4, Ray::West),
            (L12, A12, 10, Ray::West),
            (H1, L1, 3, Ray::East),
            (C11, K11, 7, Ray::East),
            (F5, G5, 0, Ray::East),
            (E12, E3, 8, Ray::South),
            (K12, K10, 1, Ray::South),
            (F9, F4, 4, Ray::South),
        ];

        for case in ok_cases {
            unsafe {
                let ray = RAYS[case.3 as usize][case.0.index()];
                let between = Attacks12::between(case.0, case.1);
                let calc = ray & &between;
                assert_eq!(calc.len(), case.2);
            }
        }
    }

    #[test]
    fn giraffe_attacks() {
        let cases = [(&G5, vec![&F9, &H9, &C6, &K6, &C4, &K4, &F1, &H1])];
        for case in cases {
            let bb = GIRAFFE_ATTACKS[case.0.index()];
            for sq in case.1 {
                assert_eq!((bb & sq).len(), 1);
            }
        }
    }
}
