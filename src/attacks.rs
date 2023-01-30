use crate::{bitboard::BitBoard, Color, PieceType, Square};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

#[derive(Clone, Copy, Debug)]
pub enum Ray {
    North = 0,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

pub trait Attacks<S, T>
where
    S: Square,
    T: BitBoard<S>,
    for<'b> &'b T: BitOr<&'b T, Output = T>,
    for<'b> &'b T: BitAnd<&'b T, Output = T>,
    for<'b> &'b T: BitOr<T, Output = T>,
    for<'b> &'b T: BitAndAssign<&'b T>,
    for<'b> &'b T: BitOrAssign<&'b T>,
    for<'b> &'b T: BitXor<&'b T, Output = T>,
    for<'b> &'b T: BitXorAssign<&'b T>,
{
    fn init_pawn_attacks();
    fn init_knight_attacks();
    fn init_king_attacks();

    fn init_north_ray();
    fn init_south_ray();
    fn init_east_ray();
    fn init_west_ray();
    fn init_north_east_ray();
    fn init_north_west_ray();
    fn init_south_east_ray();
    fn init_south_west_ray();
    fn init_between();

    fn get_non_sliding_attacks(piece_type: PieceType, square: impl Square, color: Color) -> T;

    fn get_sliding_attacks(piece_type: PieceType, square: S, blockers: T) -> T;

    fn get_positive_ray_attacks(dir: Ray, square: usize, blockers: T) -> T;

    fn get_negative_ray_attacks(dir: Ray, square: usize, blockers: T) -> T;

    fn get_bishop_attacks(square: usize, blockers: T) -> T {
        &(&Self::get_positive_ray_attacks(Ray::NorthWest, square, blockers)
            | &Self::get_positive_ray_attacks(Ray::NorthEast, square, blockers))
            | &(&Self::get_negative_ray_attacks(Ray::SouthWest, square, blockers)
                | &Self::get_negative_ray_attacks(Ray::SouthEast, square, blockers))
    }

    fn get_rook_attacks(square: usize, blockers: T) -> T {
        &(&Self::get_positive_ray_attacks(Ray::North, square, blockers)
            | &Self::get_positive_ray_attacks(Ray::East, square, blockers))
            | &(&Self::get_negative_ray_attacks(Ray::South, square, blockers)
                | &Self::get_negative_ray_attacks(Ray::West, square, blockers))
    }
}
