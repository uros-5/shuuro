use std::marker::PhantomData;

pub use crate::attacks::Attacks;
use crate::{attacks::Ray, bitboard::BitBoard, Color, PieceType, Square};

use super::{
    bitboard6::{BB6, SQUARE_BB},
    board_defs::{FILE_BB, RANK_BB},
    square6::Square6,
};

const KING_DELTAS: [i32; 8] = [7, 6, 5, 1, -7, -6, -5, -1];
const KNIGHT_DELTAS: [i32; 8] = [13, 11, 8, 4, -13, -11, -8, -4];
const GIRAFFE_DELTAS: [i32; 8] = [25, 23, 10, 2, -25, -23, -10, -2];
const WHITE_PAWN_DELTAS: [i32; 2] = [5, 7];
const BLACK_PAWN_DELTAS: [i32; 2] = [-5, -7];

const fn init_stepping_attacks(deltas: &[i32]) -> [BB6<Square6>; 36] {
    let mut table = [BB6::new(0); 36];
    let mut sq = 0;
    while sq < 36 {
        table[sq] = BB6::new(sliding_attacks(sq as i32, deltas));
        sq += 1;
    }
    table
}

pub fn sliding_attacks2(square: i32, deltas: &[i32]) -> u64 {
    let mut attack = 0;

    let mut i = 0;
    let len = deltas.len();
    while i < len {
        let mut previous = square;
        loop {
            let sq = previous + deltas[i];
            let file_diff = (sq % 6) - (previous % 6);
            // let file_diff = (sq & 0x6) - (previous & 0x6);
            if file_diff > 2 || file_diff < -2 || sq < 0 || sq > 63 {
                break;
            }
            let bb = 1 << sq;
            attack |= bb;
            if attack & bb != 0 {
                break;
            }
            previous = sq;
        }
        i += 1;
    }

    attack
}

const fn sliding_attacks(square: i32, deltas: &[i32]) -> u64 {
    let mut attack = 0;

    let mut i = 0;
    let len = deltas.len();
    while i < len {
        let mut previous = square;
        loop {
            let sq = previous + deltas[i];
            let file_diff = (sq % 6) - (previous % 6);
            // let file_diff = (sq & 0x5) - (previous & 0x5);
            if file_diff > 2 || file_diff < -2 || sq < 0 || sq > 35 {
                break;
            }
            let bb = 1 << sq;
            attack |= bb;
            if attack & bb != 0 {
                break;
            }
            previous = sq;
        }
        i += 1;
    }

    attack
}
pub static KNIGHT_ATTACKS: [BB6<Square6>; 36] =
    init_stepping_attacks(&KNIGHT_DELTAS);
pub static GIRAFFE_ATTACKS: [BB6<Square6>; 36] =
    init_stepping_attacks(&GIRAFFE_DELTAS);
pub static WHITE_PAWN_ATTACKS: [BB6<Square6>; 36] =
    init_stepping_attacks(&WHITE_PAWN_DELTAS);
pub static BLACK_PAWN_ATTACKS: [BB6<Square6>; 36] =
    init_stepping_attacks(&BLACK_PAWN_DELTAS);
pub static KING_MOVES: [BB6<Square6>; 36] = init_stepping_attacks(&KING_DELTAS);
static mut PAWN_MOVES: [[BB6<Square6>; 36]; 2] =
    [init_stepping_attacks(&[-6]), init_stepping_attacks(&[6])];

pub static mut RAYS: [[BB6<Square6>; 36]; 8] = [[BB6::new(0); 36]; 8];

static mut BETWEEN_BB: [[BB6<Square6>; 36]; 36] = [[BB6::new(0); 36]; 36];

#[derive(Clone)]
pub struct Attacks6<S, B>
where
    S: Square,
    B: BitBoard<S>,
{
    _a: PhantomData<B>,
    _s: PhantomData<S>,
}

impl Attacks6<Square6, BB6<Square6>> {
    pub fn new() -> Self {
        Self {
            _a: PhantomData,
            _s: PhantomData,
        }
    }
}

impl Default for Attacks6<Square6, BB6<Square6>> {
    fn default() -> Self {
        Self::new()
    }
}

impl Attacks<Square6, BB6<Square6>> for Attacks6<Square6, BB6<Square6>> {
    fn init_pawn_moves() {
        for color in [(Color::White, 0, 6), (Color::Black, 30, -6_isize)] {
            let index = color.0.index();
            match color.0 {
                Color::NoColor => (),
                _ => {
                    for i in color.1..color.1 + 6 {
                        unsafe {
                            PAWN_MOVES[index][i as usize] = BB6::empty();
                        }
                    }
                    let start = color.1 + color.2;
                    let end = start + 6;
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
        let empty = &BB6::empty();
        for sq in 0..36 {
            let file = FILE_BB[sq % 6];
            let rank = sq / 6;
            let mut bb = file | empty;
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
        let empty = &BB6::empty();
        for sq in 0..36 {
            let file = FILE_BB[sq % 6];
            let rank = sq / 6;
            let mut bb = file | empty;
            (rank..6).for_each(|j| {
                bb &= &!&RANK_BB[j];
            });
            bb &= &!&SQUARE_BB[sq];
            unsafe {
                RAYS[Ray::South as usize][sq] = bb;
            }
        }
    }

    fn init_east_ray() {
        let empty = &BB6::empty();
        for sq in 0..36 {
            let rank = RANK_BB[sq / 6];
            let mut bb = rank | empty;
            (0..sq % 6).for_each(|j| {
                bb &= &!&FILE_BB[j];
            });
            bb &= &!&SQUARE_BB[sq];
            unsafe {
                RAYS[Ray::East as usize][sq] = bb;
            }
        }
    }

    fn init_west_ray() {
        let empty = &BB6::empty();
        for sq in 0..36 {
            let rank = RANK_BB[sq / 6];
            let mut bb = rank | empty;
            (sq % 6..6).for_each(|j| {
                bb &= &!&FILE_BB[j];
            });
            bb &= &!&SQUARE_BB[sq];
            unsafe {
                RAYS[Ray::West as usize][sq] = bb;
            }
        }
    }

    fn init_north_east_ray() {
        let delta = &[7];
        for sq in Square6::iter() {
            let bb = diagonal_ray(sq.index() as i32, delta, 7);
            unsafe {
                RAYS[Ray::NorthEast as usize][sq.index()] = bb;
            }
        }
    }

    fn init_north_west_ray() {
        let delta = &[5];
        for sq in Square6::iter() {
            let bb = diagonal_ray(sq.index() as i32, delta, 5);
            unsafe {
                RAYS[Ray::NorthWest as usize][sq.index()] = bb;
            }
        }
    }

    fn init_south_east_ray() {
        let delta = &[-5];
        for sq in Square6::iter() {
            let bb = diagonal_ray(sq.index() as i32, delta, -5);
            unsafe {
                RAYS[Ray::SouthEast as usize][sq.index()] = bb;
            }
        }
    }

    fn init_south_west_ray() {
        let delta = &[-7];
        for sq in Square6::iter() {
            let bb = diagonal_ray(sq.index() as i32, delta, -7);
            unsafe {
                RAYS[Ray::SouthWest as usize][sq.index()] = bb;
            }
        }
    }

    fn init_between() {
        for from in Square6::iter() {
            for to in Square6::iter() {
                if from == to {
                    continue;
                }
                let df = from.file() as i8 - to.file() as i8;
                let dr = from.rank() as i8 - to.rank() as i8;
                unsafe {
                    if df == 0 || dr == 0 {
                        BETWEEN_BB[from.index()][to.index()] =
                            Attacks6::get_sliding_attacks(
                                PieceType::Rook,
                                &from,
                                SQUARE_BB[to.index()],
                            ) & &Attacks6::get_sliding_attacks(
                                PieceType::Rook,
                                &to,
                                SQUARE_BB[from.index()],
                            );
                    } else if df.abs() == dr.abs() {
                        BETWEEN_BB[from.index()][to.index()] =
                            Attacks6::get_sliding_attacks(
                                PieceType::Bishop,
                                &from,
                                SQUARE_BB[to.index()],
                            ) & &Attacks6::get_sliding_attacks(
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
        square: &Square6,
        color: Color,
        _blockers: BB6<Square6>,
    ) -> BB6<Square6> {
        match piece_type {
            PieceType::King => KING_MOVES[square.index()],
            PieceType::Knight => KNIGHT_ATTACKS[square.index()],
            PieceType::Giraffe => GIRAFFE_ATTACKS[square.index()],
            PieceType::Pawn => match color {
                Color::Black => BLACK_PAWN_ATTACKS[square.index()],
                Color::White => WHITE_PAWN_ATTACKS[square.index()],
                Color::NoColor => BB6::empty(),
            },
            _ => BB6::empty(),
        }
    }

    fn get_giraffe_attacks(square: &Square6) -> BB6<Square6> {
        GIRAFFE_ATTACKS[square.index()]
    }

    fn get_sliding_attacks(
        piece_type: PieceType,
        square: &Square6,
        blockers: BB6<Square6>,
    ) -> BB6<Square6> {
        match piece_type {
            PieceType::Bishop | PieceType::ArchBishop => {
                Attacks6::get_bishop_attacks(square.index(), blockers)
            }
            PieceType::Rook | PieceType::Chancellor => {
                Attacks6::get_rook_attacks(square.index(), blockers)
            }
            PieceType::Queen => {
                Attacks6::get_bishop_attacks(square.index(), blockers)
                    | &Attacks6::get_rook_attacks(square.index(), blockers)
            }
            _ => BB6::empty(),
        }
    }

    fn get_positive_ray_attacks(
        dir: Ray,
        square: usize,
        blockers: BB6<Square6>,
    ) -> BB6<Square6> {
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
        blockers: BB6<Square6>,
    ) -> BB6<Square6> {
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

    fn between(square1: Square6, square2: Square6) -> BB6<Square6> {
        unsafe { BETWEEN_BB[square1.index()][square2.index()] }
    }

    fn get_pawn_moves(square: usize, color: Color) -> BB6<Square6> {
        unsafe {
            match color {
                Color::White | Color::Black => {
                    PAWN_MOVES[color.index()][square]
                }
                Color::NoColor => BB6::empty(),
            }
        }
    }
}

fn diagonal_ray(start: i32, delta: &[i32; 1], new_sq: i32) -> BB6<Square6> {
    let mut sq = start;
    let mut bb = BB6::new(0);
    loop {
        let b = BB6::new(sliding_attacks(sq, delta));
        if b.len() == 0 {
            break;
        }
        bb |= &b;
        sq += new_sq;
    }
    bb
}

#[cfg(test)]
pub mod tests {

    use crate::{
        attacks::Ray,
        bitboard::BitBoard,
        shuuro6::{
            attacks6::{
                BLACK_PAWN_ATTACKS, KNIGHT_ATTACKS, WHITE_PAWN_ATTACKS,
            },
            bitboard6::square_bb,
            board_defs::EMPTY_BB,
            square6::consts::*,
        },
        Color, Square,
    };

    use super::{Attacks, Attacks6, KING_MOVES, PAWN_MOVES, RAYS};

    #[test]
    fn pawn_moves() {
        Attacks6::init_pawn_moves();
        let ok_cases = [
            (A1, EMPTY_BB, Color::White, false, 0),
            (
                A2,
                (square_bb(&A3) | &square_bb(&A4)),
                Color::White,
                true,
                2,
            ),
            (F5, square_bb(&F6), Color::White, true, 1),
            (C6, EMPTY_BB, Color::White, false, 0),
            (E6, EMPTY_BB, Color::White, false, 0),
            (D5, square_bb(&D4) | &square_bb(&D3), Color::Black, true, 2),
            (F6, EMPTY_BB, Color::Black, false, 0),
            (D4, square_bb(&D3), Color::Black, true, 1),
            (A2, square_bb(&A1), Color::Black, true, 1),
            (F1, EMPTY_BB, Color::Black, false, 0),
        ];
        for case in ok_cases {
            unsafe {
                let bb = PAWN_MOVES[case.2.index()][case.0.index()];
                let moves = bb & &case.1;
                assert_eq!(moves.is_any(), case.3);
                assert_eq!(bb.count(), case.4);
            };
        }
    }

    #[test]
    fn pawn_attacks() {
        let ok_cases = [
            (A1, [Some(B2), None], 1, Color::White),
            (D1, [Some(E2), Some(C2)], 2, Color::White),
            (E5, [Some(F6), Some(D6)], 2, Color::White),
            (C6, [None, None], 0, Color::White),
            (A5, [Some(B4), None], 1, Color::Black),
            (C5, [Some(B4), Some(D4)], 2, Color::Black),
            (E2, [Some(D1), Some(F1)], 2, Color::Black),
            (F1, [None, None], 0, Color::Black),
        ];

        for case in ok_cases {
            let sq = case.0.index();
            match case.3 {
                Color::White => {
                    let attacks = WHITE_PAWN_ATTACKS[sq];
                    for attack in case.1.into_iter().flatten() {
                        assert!((attacks & &attack).is_any());
                    }
                    assert_eq!(attacks.count(), case.2);
                }
                Color::Black => {
                    let attacks = BLACK_PAWN_ATTACKS[sq];
                    for attack in case.1.into_iter().flatten() {
                        assert!((attacks & &attack).is_any());
                    }
                    assert_eq!(attacks.count(), case.2);
                }
                Color::NoColor => (),
            }
        }
    }

    #[test]
    fn knight_attacks() {
        let knight_cases = [
            (A1, vec![B3, C2], Color::White),
            (E4, vec![F6, D6, C5, C3, D2, F2], Color::White),
            (B5, vec![D6, D4, C3, A3], Color::Black),
            (F5, vec![D6, D4, E3], Color::Black),
        ];
        for case in knight_cases {
            let sq = case.0.index();
            let attacks = KNIGHT_ATTACKS[sq];
            let capacity = case.1.len();
            for sq in case.1 {
                assert!((attacks & &sq).is_any());
            }
            assert_eq!(attacks.count(), capacity);
        }
    }

    #[test]
    fn king_attacks() {
        let king_cases = [
            (F1, vec![E1, E2, F2], Color::White),
            (C6, vec![B6, B5, C5, D5, D6], Color::White),
            (D5, vec![C6, D6, E6, C5, E5, C4, D4, E4], Color::Black),
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
        Attacks6::init();
        let ok_cases = [
            (A1, A6, 4, Ray::North),
            (D4, D6, 1, Ray::North),
            (F5, F6, 0, Ray::North),
            (E3, A3, 3, Ray::West),
            (E6, B6, 2, Ray::West),
            (F6, A6, 4, Ray::West),
            (B1, F1, 3, Ray::East),
            (C6, F6, 2, Ray::East),
            (F5, E5, 0, Ray::East),
            (E6, E3, 2, Ray::South),
            (B6, B4, 1, Ray::South),
            (F6, F3, 2, Ray::South),
        ];

        for case in ok_cases {
            unsafe {
                let ray = &RAYS[case.3 as usize][case.0.index()];
                let between = Attacks6::between(case.0, case.1);
                let calc = *ray & &between;
                assert_eq!(calc.count(), case.2);
            }
        }
    }
}
