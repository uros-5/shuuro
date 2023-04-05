use std::{
    marker::PhantomData,
    ops::{BitAnd, BitOr, Not},
};

pub use crate::attacks::Attacks;
use crate::{attacks::Ray, bitboard::BitBoard, Color, PieceType, Square};

use super::{
    bitboard12::{square_bb, BB12, SQUARE_BB},
    board_defs::{FILE_BB, RANK_BB},
    square12::Square12,
};

static mut NON_SLIDING_ATTACKS: [[[BB12<Square12>; 144]; 6]; 2] =
    [[[BB12::new([0, 0, 0, 0, 0, 0, 0, 0, 0]); 144]; 6]; 2];

static mut GIRAFFE_ATTACKS: [BB12<Square12>; 144] =
    [BB12::new([0, 0, 0, 0, 0, 0, 0, 0, 0]); 144];

static mut PAWN_MOVES: [[BB12<Square12>; 144]; 2] =
    [[BB12::new([0, 0, 0, 0, 0, 0, 0, 0, 0]); 144]; 2];

pub static mut RAYS: [[BB12<Square12>; 144]; 8] =
    [[BB12::new([0, 0, 0, 0, 0, 0, 0, 0, 0]); 144]; 8];

static mut BETWEEN_BB: [[BB12<Square12>; 144]; 144] =
    [[BB12::new([0, 0, 0, 0, 0, 0, 0, 0, 0]); 144]; 144];

#[derive(Clone, Copy, Debug, Default)]
pub struct Attacks12<S, B>
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

impl Attacks12<Square12, BB12<Square12>> {
    pub fn new() -> Self {
        Self {
            _a: PhantomData,
            _s: PhantomData,
        }
    }
}

impl Attacks<Square12, BB12<Square12>> for Attacks12<Square12, BB12<Square12>> {
    fn add_pawn_moves(current: Square12, next: Square12, color: &Color) {
        unsafe {
            PAWN_MOVES[color.index()][current.index()] |= &square_bb(&next);
        };
    }

    fn init_pawn_moves() {
        for color in Color::iter() {
            for sq in Square12::iter() {
                if let Some(up) = sq.upward(&color) {
                    Self::add_pawn_moves(sq, up, &color);
                    if sq.first_pawn_rank(color) {
                        if let Some(up) = up.upward(&color) {
                            Self::add_pawn_moves(sq, up, &color);
                        }
                    }
                }
            }
        }
    }

    fn init_pawn_attacks() {
        let add = |current: Square12, next: Square12, color: &Color| unsafe {
            NON_SLIDING_ATTACKS[color.index()][PieceType::Pawn.index()]
                [current.index()] |= &square_bb(&next);
        };
        for color in Color::iter() {
            for sq in Square12::iter() {
                for attack in sq.x(&color).into_iter().flatten() {
                    {
                        add(sq, attack, &color);
                    }
                }
            }
        }
    }

    fn init_knight_attacks() {
        for sq in Square12::iter() {
            let mut bb = BB12::empty();
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
        for sq in Square12::iter() {
            let mut bb = BB12::empty();
            for attack in sq.giraffe().into_iter().flatten() {
                bb |= &attack;
            }
            unsafe {
                GIRAFFE_ATTACKS[sq.index()] |= &bb;
            }
        }
    }

    fn init_king_attacks() {
        for sq in Square12::iter() {
            let mut bb = BB12::empty();
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
        for sq in 0..144 {
            let file = &FILE_BB[sq % 12];
            let rank = sq / 12;
            let mut bb = file | &BB12::empty();
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
        for sq in 0..144 {
            let file = &FILE_BB[sq % 12];
            let rank = sq / 12;
            let mut bb = file | &BB12::empty();
            #[allow(clippy::needless_range_loop)]
            for j in rank..12 {
                bb &= &!&RANK_BB[j];
            }
            bb &= &!&SQUARE_BB[sq];
            unsafe {
                RAYS[Ray::South as usize][sq] = bb;
            }
        }
    }

    fn init_east_ray() {
        for sq in 0..144 {
            let rank = &RANK_BB[sq / 12];
            let mut bb = rank | &BB12::empty();
            #[allow(clippy::needless_range_loop)]
            for j in 0..sq % 12 {
                bb &= &!&FILE_BB[j];
            }
            bb &= &!&SQUARE_BB[sq];
            unsafe {
                RAYS[Ray::East as usize][sq] = bb;
            }
        }
    }

    fn init_west_ray() {
        for sq in 0..144 {
            let rank = &RANK_BB[sq / 12];
            let mut bb = rank | &BB12::empty();
            #[allow(clippy::needless_range_loop)]
            for j in sq % 12..12 {
                bb &= &!&FILE_BB[j];
            }
            bb &= &!&SQUARE_BB[sq];
            unsafe {
                RAYS[Ray::West as usize][sq] = bb;
            }
        }
    }

    fn init_north_east_ray() {
        for sq in Square12::iter() {
            let mut bb = BB12::empty();
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
        for sq in Square12::iter() {
            let mut bb = BB12::empty();
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
        for sq in Square12::iter() {
            let mut bb = BB12::empty();
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
        for sq in Square12::iter() {
            let mut bb = BB12::empty();
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
                            &Attacks12::get_sliding_attacks(
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
                            &Attacks12::get_sliding_attacks(
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
    ) -> BB12<Square12> {
        unsafe {
            NON_SLIDING_ATTACKS[color as usize][piece_type as usize]
                [square.index()]
        }
    }

    fn get_girrafe_attacks(square: &Square12) -> BB12<Square12> {
        unsafe { GIRAFFE_ATTACKS[square.index()] }
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
                &Attacks12::get_bishop_attacks(square.index(), blockers)
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
        blockers: BB12<Square12>,
    ) -> BB12<Square12> {
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

    fn between(square1: Square12, square2: Square12) -> BB12<Square12> {
        unsafe { BETWEEN_BB[square1.index()][square2.index()] }
    }

    fn get_pawn_moves(square: usize, color: Color) -> BB12<Square12> {
        unsafe {
            match color {
                Color::White | Color::Black => {
                    PAWN_MOVES[color.index()][square]
                }
                Color::NoColor => BB12::empty(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        attacks::Ray,
        bitboard::BitBoard,
        shuuro12::{
            bitboard12::square_bb, board_defs::EMPTY_BB, square12::consts::*,
        },
        Color, PieceType, Square,
    };

    use super::{Attacks, Attacks12, NON_SLIDING_ATTACKS, PAWN_MOVES, RAYS};

    #[test]
    fn pawn_moves() {
        Attacks12::init_pawn_moves();
        let ok_cases = [
            (A1, square_bb(&A2), Color::White, true, 1),
            (L11, square_bb(&L12), Color::White, true, 1),
            (C12, EMPTY_BB, Color::White, false, 0),
            (G12, EMPTY_BB, Color::White, false, 0),
            (H12, square_bb(&H11), Color::Black, true, 1),
            (D4, square_bb(&D3), Color::Black, true, 1),
            (A2, square_bb(&A1), Color::Black, true, 1),
            (L1, EMPTY_BB, Color::Black, false, 0),
        ];

        for case in ok_cases {
            unsafe {
                let bb = PAWN_MOVES[case.2.index()][case.0.index()];
                let moves = &bb & &case.1;
                assert_eq!(moves.is_any(), case.3);
                assert_eq!(bb.count(), case.4);
            };
        }
    }

    #[test]
    fn pawn_attacks() {
        Attacks12::init_pawn_attacks();
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
        Attacks12::init_knight_attacks();
        let knight_cases = [
            (A1, vec![B3, C2], Color::White),
            (E4, vec![D2, F2, C3, G3, C5, G5, D6, F6], Color::White),
            (B11, vec![D12, D10, C9, A9], Color::Black),
            (L8, vec![K10, J9, J7, K6], Color::Black),
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
        Attacks12::init_king_attacks();
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
                let ray = &RAYS[case.3 as usize][case.0.index()];
                let between = Attacks12::between(case.0, case.1);
                let calc = ray & &between;
                assert_eq!(calc.count(), case.2);
            }
        }
    }
}
