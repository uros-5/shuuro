use std::marker::PhantomData;

pub use crate::attacks::Attacks;
use crate::{attacks::Ray, bitboard::BitBoard, Color, PieceType, Square};

use super::{
    bitboard8::{BB8, SQUARE_BB},
    board_defs::{FILE_BB, RANK_BB},
    square8::Square8,
};

const KING_DELTAS: [i32; 8] = [9, 8, 7, 1, -9, -8, -7, -1];
const KNIGHT_DELTAS: [i32; 8] = [17, 15, 10, 6, -17, -15, -10, -6];
const GIRAFFE_DELTAS: [i32; 8] = [33, 31, 12, 4, -33, -31, -12, -4];
const WHITE_PAWN_DELTAS: [i32; 2] = [7, 9];
const BLACK_PAWN_DELTAS: [i32; 2] = [-7, -9];

const fn init_stepping_attacks(deltas: &[i32]) -> [BB8<Square8>; 64] {
    let mut table = [BB8::new(0); 64];
    let mut sq = 0;
    while sq < 64 {
        table[sq] = BB8::new(sliding_attacks(sq as i32, deltas));
        sq += 1;
    }
    table
}

const fn sliding_attacks(square: i32, deltas: &[i32]) -> u64 {
    let mut attack = 0;

    let mut i = 0;
    let len = deltas.len();
    while i < len {
        let mut previous = square;
        loop {
            let sq = previous + deltas[i];
            let file_diff = (sq & 0x7) - (previous & 0x7);
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

pub static KNIGHT_ATTACKS: [BB8<Square8>; 64] =
    init_stepping_attacks(&KNIGHT_DELTAS);
pub static GIRAFFE_ATTACKS: [BB8<Square8>; 64] =
    init_stepping_attacks(&GIRAFFE_DELTAS);
pub static WHITE_PAWN_ATTACKS: [BB8<Square8>; 64] =
    init_stepping_attacks(&WHITE_PAWN_DELTAS);
pub static BLACK_PAWN_ATTACKS: [BB8<Square8>; 64] =
    init_stepping_attacks(&BLACK_PAWN_DELTAS);
pub static KING_MOVES: [BB8<Square8>; 64] = init_stepping_attacks(&KING_DELTAS);
static mut PAWN_MOVES: [[BB8<Square8>; 64]; 2] =
    [init_stepping_attacks(&[8]), init_stepping_attacks(&[-8])];

pub static mut RAYS: [[BB8<Square8>; 64]; 8] = [[BB8::new(0); 64]; 8];

static mut BETWEEN_BB: [[BB8<Square8>; 64]; 64] = [[BB8::new(0); 64]; 64];

#[derive(Clone)]
pub struct Attacks8<S, B>
where
    S: Square,
    B: BitBoard<S>,
{
    _a: PhantomData<B>,
    _s: PhantomData<S>,
}

impl Attacks8<Square8, BB8<Square8>> {
    pub fn new() -> Self {
        Self {
            _a: PhantomData,
            _s: PhantomData,
        }
    }
}

impl Default for Attacks8<Square8, BB8<Square8>> {
    fn default() -> Self {
        Self::new()
    }
}

impl Attacks<Square8, BB8<Square8>> for Attacks8<Square8, BB8<Square8>> {
    fn init_pawn_moves() {
        for color in [(Color::White, 0, 8), (Color::Black, 56, -8_isize)] {
            let index = color.0.index();
            match color.0 {
                Color::NoColor => (),
                _ => {
                    for i in color.1..color.1 + 8 {
                        unsafe {
                            PAWN_MOVES[index][i as usize] = BB8::empty();
                        }
                    }
                    let start = color.1 + color.2;
                    let end = start + 8;
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
        let empty = &BB8::empty();
        for sq in 0..64 {
            let file = FILE_BB[sq % 8];
            let rank = sq / 8;
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
        let empty = &BB8::empty();
        for sq in 0..64 {
            let file = FILE_BB[sq % 8];
            let rank = sq / 8;
            let mut bb = file | empty;
            (rank..8).for_each(|j| {
                bb &= &!&RANK_BB[j];
            });
            bb &= &!&SQUARE_BB[sq];
            unsafe {
                RAYS[Ray::South as usize][sq] = bb;
            }
        }
    }

    fn init_east_ray() {
        let empty = &BB8::empty();
        for sq in 0..64 {
            let rank = RANK_BB[sq / 8];
            let mut bb = rank | empty;
            (0..sq % 8).for_each(|j| {
                bb &= &!&FILE_BB[j];
            });
            bb &= &!&SQUARE_BB[sq];
            unsafe {
                RAYS[Ray::East as usize][sq] = bb;
            }
        }
    }

    fn init_west_ray() {
        let empty = &BB8::empty();
        for sq in 0..64 {
            let rank = RANK_BB[sq / 8];
            let mut bb = rank | empty;
            (sq % 8..8).for_each(|j| {
                bb &= &!&FILE_BB[j];
            });
            bb &= &!&SQUARE_BB[sq];
            unsafe {
                RAYS[Ray::West as usize][sq] = bb;
            }
        }
    }

    fn init_north_east_ray() {
        let delta = &[9];
        for sq in Square8::iter() {
            let bb = diagonal_ray(sq.index() as i32, delta, 9);
            unsafe {
                RAYS[Ray::NorthEast as usize][sq.index()] = bb;
            }
        }
    }

    fn init_north_west_ray() {
        let delta = &[7];
        for sq in Square8::iter() {
            let bb = diagonal_ray(sq.index() as i32, delta, 7);
            unsafe {
                RAYS[Ray::NorthWest as usize][sq.index()] = bb;
            }
        }
    }

    fn init_south_east_ray() {
        let delta = &[-7];
        for sq in Square8::iter() {
            let bb = diagonal_ray(sq.index() as i32, delta, -7);
            unsafe {
                RAYS[Ray::SouthEast as usize][sq.index()] = bb;
            }
        }
    }

    fn init_south_west_ray() {
        let delta = &[-9];
        for sq in Square8::iter() {
            let bb = diagonal_ray(sq.index() as i32, delta, -9);
            unsafe {
                RAYS[Ray::SouthWest as usize][sq.index()] = bb;
            }
        }
    }

    fn init_between() {
        for from in Square8::iter() {
            for to in Square8::iter() {
                if from == to {
                    continue;
                }
                let df = from.file() as i8 - to.file() as i8;
                let dr = from.rank() as i8 - to.rank() as i8;
                unsafe {
                    if df == 0 || dr == 0 {
                        BETWEEN_BB[from.index()][to.index()] =
                            Attacks8::get_sliding_attacks(
                                PieceType::Rook,
                                &from,
                                SQUARE_BB[to.index()],
                            ) & &Attacks8::get_sliding_attacks(
                                PieceType::Rook,
                                &to,
                                SQUARE_BB[from.index()],
                            );
                    } else if df.abs() == dr.abs() {
                        BETWEEN_BB[from.index()][to.index()] =
                            Attacks8::get_sliding_attacks(
                                PieceType::Bishop,
                                &from,
                                SQUARE_BB[to.index()],
                            ) & &Attacks8::get_sliding_attacks(
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
        square: &Square8,
        color: Color,
        _blockers: BB8<Square8>,
    ) -> BB8<Square8> {
        match piece_type {
            PieceType::King => KING_MOVES[square.index()],
            PieceType::Knight => KNIGHT_ATTACKS[square.index()],
            PieceType::Giraffe => GIRAFFE_ATTACKS[square.index()],
            PieceType::Pawn => match color {
                Color::Black => BLACK_PAWN_ATTACKS[square.index()],
                Color::White => WHITE_PAWN_ATTACKS[square.index()],
                Color::NoColor => BB8::empty(),
            },
            _ => BB8::empty(),
        }
    }

    fn get_giraffe_attacks(square: &Square8) -> BB8<Square8> {
        GIRAFFE_ATTACKS[square.index()]
    }

    fn get_sliding_attacks(
        piece_type: PieceType,
        square: &Square8,
        blockers: BB8<Square8>,
    ) -> BB8<Square8> {
        match piece_type {
            PieceType::Bishop | PieceType::ArchBishop => {
                Attacks8::get_bishop_attacks(square.index(), blockers)
            }
            PieceType::Rook | PieceType::Chancellor => {
                Attacks8::get_rook_attacks(square.index(), blockers)
            }
            PieceType::Queen => {
                Attacks8::get_bishop_attacks(square.index(), blockers)
                    | &Attacks8::get_rook_attacks(square.index(), blockers)
            }
            _ => BB8::empty(),
        }
    }

    fn get_positive_ray_attacks(
        dir: Ray,
        square: usize,
        blockers: BB8<Square8>,
    ) -> BB8<Square8> {
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
        blockers: BB8<Square8>,
    ) -> BB8<Square8> {
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

    fn between(square1: Square8, square2: Square8) -> BB8<Square8> {
        unsafe { BETWEEN_BB[square1.index()][square2.index()] }
    }

    fn get_pawn_moves(square: usize, color: Color) -> BB8<Square8> {
        unsafe {
            match color {
                Color::White | Color::Black => {
                    PAWN_MOVES[color.index()][square]
                }
                Color::NoColor => BB8::empty(),
            }
        }
    }
}

fn diagonal_ray(start: i32, delta: &[i32; 1], new_sq: i32) -> BB8<Square8> {
    let mut sq = start;
    let mut bb = BB8::new(0);
    loop {
        let b = BB8::new(sliding_attacks(sq, delta));
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
        shuuro8::{
            attacks8::{
                BLACK_PAWN_ATTACKS, KNIGHT_ATTACKS, WHITE_PAWN_ATTACKS,
            },
            bitboard8::square_bb,
            board_defs::EMPTY_BB,
            square8::consts::*,
        },
        Color, Square,
    };

    use super::{Attacks, Attacks8, KING_MOVES, PAWN_MOVES, RAYS};

    #[test]
    fn pawn_moves() {
        Attacks8::init_pawn_moves();
        let ok_cases = [
            (A1, EMPTY_BB, Color::White, false, 0),
            (
                A2,
                (square_bb(&A3) | &square_bb(&A4)),
                Color::White,
                true,
                2,
            ),
            (H7, square_bb(&H8), Color::White, true, 1),
            (C8, EMPTY_BB, Color::White, false, 0),
            (G8, EMPTY_BB, Color::White, false, 0),
            (D7, square_bb(&D6) | &square_bb(&D5), Color::Black, true, 2),
            (H8, EMPTY_BB, Color::Black, false, 0),
            (D4, square_bb(&D3), Color::Black, true, 1),
            (A2, square_bb(&A1), Color::Black, true, 1),
            (H1, EMPTY_BB, Color::Black, false, 0),
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
            (F7, [Some(G8), Some(E8)], 2, Color::White),
            (C8, [None, None], 0, Color::White),
            (A7, [Some(B6), None], 1, Color::Black),
            (C7, [Some(B6), Some(D6)], 2, Color::Black),
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
            (E4, vec![D2, F2, C3, G3, C5, G5, D6, F6], Color::White),
            (B7, vec![D8, D6, C5, A5], Color::Black),
            (H7, vec![F8, F6, G5], Color::Black),
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
            (H1, vec![H2, G2, G2], Color::White),
            (C8, vec![D8, B8, D7, B7, C7], Color::White),
            (D7, vec![C8, D8, E8, C7, E7, C6, D6, E6], Color::Black),
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
        Attacks8::init();
        let ok_cases = [
            (A1, A8, 6, Ray::North),
            (D6, D8, 1, Ray::North),
            (H7, H8, 0, Ray::North),
            (E3, A3, 3, Ray::West),
            (G6, B6, 4, Ray::West),
            (H8, A8, 6, Ray::West),
            (D1, H1, 3, Ray::East),
            (C8, H8, 4, Ray::East),
            (F5, G5, 0, Ray::East),
            (E8, E3, 4, Ray::South),
            (B8, B6, 1, Ray::South),
            (F7, F4, 2, Ray::South),
        ];

        for case in ok_cases {
            unsafe {
                let ray = &RAYS[case.3 as usize][case.0.index()];
                let between = Attacks8::between(case.0, case.1);
                let calc = *ray & &between;
                assert_eq!(calc.count(), case.2);
            }
        }
    }
}
