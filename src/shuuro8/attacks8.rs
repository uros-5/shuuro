use std::{
    marker::PhantomData,
    ops::{BitAnd, BitOr, Not},
};

pub use crate::attacks::Attacks;
use crate::{attacks::Ray, bitboard::BitBoard, Color, PieceType, Square};

use super::{
    bitboard8::{square_bb, BB8, SQUARE_BB},
    board_defs::{FILE_BB, RANK_BB},
    square8::Square8,
};

pub static mut NON_SLIDING_ATTACKS: [[[BB8<Square8>; 64]; 6]; 2] =
    [[[BB8::new(0); 64]; 6]; 2];

static mut GIRAFFE_ATTACKS: [BB8<Square8>; 64] = [BB8::new(0); 64];

static mut PAWN_MOVES: [[BB8<Square8>; 64]; 2] = [[BB8::new(0); 64]; 2];

pub static mut RAYS: [[BB8<Square8>; 64]; 8] = [[BB8::new(0); 64]; 8];

static mut BETWEEN_BB: [[BB8<Square8>; 64]; 64] = [[BB8::new(0); 64]; 64];

pub struct Attacks8<S, B>
where
    S: Square,
    B: BitBoard<S>,
    for<'b> &'b B: BitOr<&'b B, Output = B>,
    for<'a> &'a B: BitAnd<&'a B, Output = B>,
    for<'a> &'a B: Not<Output = B>,
    for<'a> &'a B: BitOr<&'a S, Output = B>,
    for<'a> &'a B: BitAnd<&'a S, Output = B>,
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

impl Attacks<Square8, BB8<Square8>> for Attacks8<Square8, BB8<Square8>> {
    fn init_pawn_moves() {
        let add = |current: Square8, next: Square8, color: &Color| unsafe {
            PAWN_MOVES[color.index()][current.index()] |= &square_bb(&next);
        };
        for color in Color::iter() {
            for sq in Square8::iter() {
                if let Some(up) = sq.upward(&color) {
                    add(sq, up, &color);
                }
            }
        }
    }

    fn init_pawn_attacks() {
        let add = |current: Square8, next: Square8, color: &Color| unsafe {
            NON_SLIDING_ATTACKS[color.index()][PieceType::Pawn.index()]
                [current.index()] |= &square_bb(&next);
        };
        for color in Color::iter() {
            for sq in Square8::iter() {
                for attack in sq.x(&color).into_iter().flatten() {
                    {
                        add(sq, attack, &color);
                    }
                }
            }
        }
    }

    fn init_knight_attacks() {
        for sq in Square8::iter() {
            let mut bb = BB8::empty();
            for attack in sq.knight().into_iter().flatten() {
                bb |= &attack;
            }
            unsafe {
                NON_SLIDING_ATTACKS[0][PieceType::Knight.index()]
                    [sq.index()] |= &bb;
                NON_SLIDING_ATTACKS[1][PieceType::Knight.index()]
                    [sq.index()] |= &bb;
            }
        }
    }

    fn init_girraffe_attacks() {
        for sq in Square8::iter() {
            let mut bb = BB8::empty();
            for attack in sq.giraffe().into_iter().flatten() {
                bb |= &attack;
            }
            unsafe {
                GIRAFFE_ATTACKS[sq.index()] |= &bb;
            }
        }
    }

    fn init_king_attacks() {
        for sq in Square8::iter() {
            let mut bb = BB8::empty();
            for x in [sq.x(&Color::White), sq.x(&Color::Black)] {
                for attack in x.into_iter().flatten() {
                    bb |= &attack;
                }
            }
            for attack in [sq.left(), sq.right(), sq.up(), sq.down()]
                .into_iter()
                .flatten()
            {
                bb |= &attack;
            }
            unsafe {
                NON_SLIDING_ATTACKS[0][PieceType::King.index()][sq.index()] |=
                    &bb;
                NON_SLIDING_ATTACKS[1][PieceType::King.index()][sq.index()] |=
                    &bb;
            }
        }
    }

    fn init_north_ray() {
        for sq in 0..64 {
            let file = &FILE_BB[sq % 8];
            let rank = sq / 8;
            let mut bb = file | &BB8::empty();
            #[allow(clippy::needless_range_loop)]
            for j in 0..rank {
                bb &= &!&RANK_BB[j];
            }
            bb &= &!&SQUARE_BB[sq];
            unsafe {
                RAYS[Ray::North as usize][sq] = bb;
            }
        }
    }

    fn init_south_ray() {
        for sq in 0..64 {
            let file = &FILE_BB[sq % 8];
            let rank = sq / 8;
            let mut bb = file | &BB8::empty();
            #[allow(clippy::needless_range_loop)]
            for j in rank..8 {
                bb &= &!&RANK_BB[j];
            }
            bb &= &!&SQUARE_BB[sq];
            unsafe {
                RAYS[Ray::South as usize][sq] = bb;
            }
        }
    }

    fn init_east_ray() {
        for sq in 0..64 {
            let rank = &RANK_BB[sq / 8];
            let mut bb = rank | &BB8::empty();
            #[allow(clippy::needless_range_loop)]
            for j in 0..sq % 8 {
                bb &= &!&FILE_BB[j];
            }
            bb &= &!&SQUARE_BB[sq];
            unsafe {
                RAYS[Ray::East as usize][sq] = bb;
            }
        }
    }

    fn init_west_ray() {
        for sq in 0..64 {
            let rank = &RANK_BB[sq / 8];
            let mut bb = rank | &BB8::empty();
            #[allow(clippy::needless_range_loop)]
            for j in sq % 8..8 {
                bb &= &!&FILE_BB[j];
            }
            bb &= &!&SQUARE_BB[sq];
            unsafe {
                RAYS[Ray::West as usize][sq] = bb;
            }
        }
    }

    fn init_north_east_ray() {
        for sq in Square8::iter() {
            let mut bb = BB8::empty();
            let mut sq_east = sq;
            while let Some(ne) = sq_east.nea() {
                bb |= &ne;
                sq_east = ne;
            }
            unsafe {
                RAYS[Ray::NorthEast as usize][sq.index()] = bb;
            }
        }
    }

    fn init_north_west_ray() {
        for sq in Square8::iter() {
            let mut bb = BB8::empty();
            let mut sq_west = sq;
            while let Some(w) = sq_west.nw() {
                bb |= &w;
                sq_west = w;
            }
            unsafe {
                RAYS[Ray::NorthWest as usize][sq.index()] = bb;
            }
        }
    }

    fn init_south_east_ray() {
        for sq in Square8::iter() {
            let mut bb = BB8::empty();
            let mut sq_west = sq;
            while let Some(w) = sq_west.se() {
                bb |= &w;
                sq_west = w;
            }
            unsafe {
                RAYS[Ray::SouthEast as usize][sq.index()] = bb;
            }
        }
    }

    fn init_south_west_ray() {
        for sq in Square8::iter() {
            let mut bb = BB8::empty();
            let mut sq_west = sq;
            while let Some(w) = sq_west.sw() {
                bb |= &w;
                sq_west = w;
            }
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
                            &Attacks8::get_sliding_attacks(
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
                            &Attacks8::get_sliding_attacks(
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
    ) -> BB8<Square8> {
        unsafe {
            NON_SLIDING_ATTACKS[color as usize][piece_type as usize]
                [square.index()]
        }
    }

    fn get_girrafe_attacks(square: &Square8) -> BB8<Square8> {
        unsafe { GIRAFFE_ATTACKS[square.index()] }
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
                &Attacks8::get_bishop_attacks(square.index(), blockers)
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
            let mut blocked = &attacks & &blockers;
            let block_square = blocked.pop();
            match block_square {
                Some(i) => &attacks & &!&RAYS[dir as usize][i.index()],
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
            let mut blocked = &attacks & &blockers;
            let block_square = blocked.pop_reverse();
            match block_square {
                Some(i) => &attacks & &!&RAYS[dir as usize][i.index()],
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

#[cfg(test)]
pub mod tests {

    use crate::{
        attacks::Ray,
        bitboard::BitBoard,
        shuuro8::{
            bitboard8::square_bb, board_defs::EMPTY_BB, square8::consts::*,
        },
        Color, PieceType, Square,
    };

    use super::{Attacks, Attacks8, NON_SLIDING_ATTACKS, PAWN_MOVES, RAYS};

    #[test]
    fn pawn_moves() {
        Attacks8::init_pawn_moves();
        let ok_cases = [
            (A1, square_bb(&A2), Color::White, 1),
            (H7, square_bb(&H8), Color::White, 1),
            (C8, EMPTY_BB, Color::White, 0),
            (G8, EMPTY_BB, Color::White, 0),
            (H8, EMPTY_BB, Color::Black, 0),
            (D4, square_bb(&D3), Color::Black, 1),
            (A2, square_bb(&A1), Color::Black, 1),
            (H1, EMPTY_BB, Color::Black, 0),
        ];
        for case in ok_cases {
            unsafe {
                let bb = PAWN_MOVES[case.2.index()][case.0.index()];

                assert_eq!((&bb & &case.1).count(), case.3);
            };
        }
    }

    #[test]
    fn pawn_attacks() {
        Attacks8::init_pawn_attacks();
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

        let pawn = PieceType::Pawn.index();

        for case in ok_cases {
            let color = case.3.index();
            let sq = case.0.index();
            unsafe {
                let attacks = NON_SLIDING_ATTACKS[color][pawn][sq];
                for attack in case.1.into_iter().flatten() {
                    assert!((&attacks & &attack).is_any());
                }
                assert_eq!(attacks.count(), case.2);
            }
        }
    }

    #[test]
    fn knight_attacks() {
        Attacks8::init_knight_attacks();
        let knight_cases = [
            (A1, vec![B3, C2], Color::White),
            (E4, vec![D2, F2, C3, G3, C5, G5, D6, F6], Color::White),
            (B7, vec![D8, D6, C5, A5], Color::Black),
            (H7, vec![F8, F6, G5], Color::Black),
        ];
        for case in knight_cases {
            let knight = PieceType::Knight as usize;
            let sq = case.0.index();
            let color = case.2 as usize;
            unsafe {
                let attacks = NON_SLIDING_ATTACKS[color][knight][sq];
                let capacity = case.1.len();
                for sq in case.1 {
                    assert!((&attacks & &sq).is_any());
                }
                assert_eq!(attacks.count(), capacity);
            }
        }
    }

    #[test]
    fn king_attacks() {
        Attacks8::init_king_attacks();
        let king_cases = [
            (H1, vec![H2, G2, G2], Color::White),
            (C8, vec![D8, B8, D7, B7, C7], Color::White),
            (D7, vec![C8, D8, E8, C7, E7, C6, D6, E6], Color::Black),
            (A5, vec![A6, B6, B5, B4, A4], Color::Black),
        ];

        for case in king_cases {
            let king = PieceType::King as usize;
            let color = case.2.index();
            let sq = case.0.index();
            unsafe {
                let attacks = NON_SLIDING_ATTACKS[color][king][sq];
                for attack in case.1 {
                    assert!((&attacks & &attack).is_any());
                }
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
                let calc = ray & &between;
                assert_eq!(calc.count(), case.2);
            }
        }
    }
}
