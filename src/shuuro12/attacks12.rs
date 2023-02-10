use std::{
    marker::PhantomData,
    ops::{BitAnd, BitOr, Not},
};

pub use crate::attacks::Attacks;
use crate::{bitboard::BitBoard, Color, PieceType, Square};

use super::{
    bitboard12::{square_bb, B12, SQUARE_BB},
    square12::Square12,
};

static mut NON_SLIDING_ATTACKS: [[[B12<Square12>; 144]; 6]; 2] =
    [[[B12::new([0, 0, 0, 0, 0, 0, 0, 0, 0]); 144]; 6]; 2];

static mut PAWN_MOVES: [[B12<Square12>; 144]; 2] =
    [[B12::new([0, 0, 0, 0, 0, 0, 0, 0, 0]); 144]; 2];

static mut RAYS: [[B12<Square12>; 144]; 8] = [[B12::new([0, 0, 0, 0, 0, 0, 0, 0, 0]); 144]; 8];

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

impl Attacks12<Square12, B12<Square12>> {
    pub fn new() -> Self {
        Self {
            _a: PhantomData,
            _s: PhantomData,
        }
    }
}

impl Attacks<Square12, B12<Square12>> for Attacks12<Square12, B12<Square12>> {
    fn init_pawn_moves() {
        let add = |current: Square12, next: Square12, color: &Color| unsafe {
            PAWN_MOVES[color.index()][current.index()] |= &square_bb(&next);
        };
        for color in Color::iter() {
            for sq in Square12::iter() {
                if let Some(up) = sq.upward(&color) {
                    add(sq, up, &color);
                }
            }
        }
    }

    fn init_pawn_attacks() {
        let add = |current: Square12, next: Square12, color: &Color| unsafe {
            NON_SLIDING_ATTACKS[color.index()][PieceType::Pawn.index()][current.index()] |=
                &square_bb(&next);
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
        todo!()
    }

    fn init_king_attacks() {
        todo!()
    }

    fn init_north_ray() {
        todo!()
    }

    fn init_south_ray() {
        todo!()
    }

    fn init_east_ray() {
        todo!()
    }

    fn init_west_ray() {
        todo!()
    }

    fn init_north_east_ray() {
        todo!()
    }

    fn init_north_west_ray() {
        todo!()
    }

    fn init_south_east_ray() {
        todo!()
    }

    fn init_south_west_ray() {
        todo!()
    }

    fn init_between() {
        todo!()
    }

    fn get_non_sliding_attacks(
        piece_type: crate::PieceType,
        square: &Square12,
        color: crate::Color,
    ) -> B12<Square12> {
        todo!()
    }

    fn get_sliding_attacks(
        piece_type: crate::PieceType,
        square: &Square12,
        blockers: B12<Square12>,
    ) -> B12<Square12> {
        todo!()
    }

    fn get_positive_ray_attacks(
        dir: crate::attacks::Ray,
        square: usize,
        blockers: B12<Square12>,
    ) -> B12<Square12> {
        todo!()
    }

    fn get_negative_ray_attacks(
        dir: crate::attacks::Ray,
        square: usize,
        blockers: B12<Square12>,
    ) -> B12<Square12> {
        todo!()
    }

    fn between(sq1: Square12, sq2: Square12) -> B12<Square12> {
        todo!()
    }
}

pub trait Abc {
    fn jeste();
}

impl Abc for B12<Square12> {
    fn jeste() {
        println!("jeste");
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bitboard::BitBoard,
        shuuro12::{
            bitboard12::{square_bb, B12},
            board_defs::EMPTY_BB,
            square12::{consts::*, Square12},
        },
        Color, PieceType, Square,
    };

    use super::{Attacks, Attacks12, NON_SLIDING_ATTACKS, PAWN_MOVES};

    #[test]
    fn pawn_moves() {
        Attacks12::init_pawn_moves();
        let ok_cases = [
            (A1, square_bb(&A2), Color::White, 1),
            (L11, square_bb(&L12), Color::White, 1),
            (C12, EMPTY_BB, Color::White, 0),
            (G12, EMPTY_BB, Color::White, 0),
            (H12, square_bb(&H11), Color::Black, 1),
            (D4, square_bb(&D3), Color::Black, 1),
            (A2, square_bb(&A1), Color::Black, 1),
            (L1, EMPTY_BB, Color::Black, 0),
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
}
