use crate::shuuro_rules::{bitboard::BitBoard, Color, PieceType, Square};
use std::ops::{BitAnd, BitOr, Not};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

pub trait Attacks<S, B>: Sized + Clone
where
    S: Square,
    B: BitBoard<S>,
    for<'b> &'b B: BitOr<&'b B, Output = B>,
    for<'a> &'a B: BitAnd<&'a B, Output = B>,
    for<'a> &'a B: Not<Output = B>,
    for<'a> &'a B: BitOr<&'a S, Output = B>,
    for<'a> &'a B: BitAnd<&'a S, Output = B>,
{
    fn init_pawn_moves();
    fn init_pawn_attacks();
    fn init_knight_attacks();
    fn init_giraffe_attacks();
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

    fn add_pawn_moves(current: S, next: S, color: &Color);

    fn init_rays() {}

    fn get_non_sliding_attacks(
        piece_type: PieceType,
        square: &S,
        color: Color,
        blockers: B,
    ) -> B;

    fn get_giraffe_attacks(square: &S) -> B;

    fn get_sliding_attacks(piece_type: PieceType, square: &S, blockers: B)
        -> B;

    fn get_positive_ray_attacks(dir: Ray, square: usize, blockers: B) -> B;

    fn get_negative_ray_attacks(dir: Ray, square: usize, blockers: B) -> B;

    fn get_bishop_attacks(square: usize, blockers: B) -> B {
        &(&Self::get_positive_ray_attacks(Ray::NorthWest, square, blockers)
            | &Self::get_positive_ray_attacks(Ray::NorthEast, square, blockers))
            | &(&Self::get_negative_ray_attacks(
                Ray::SouthWest,
                square,
                blockers,
            ) | &Self::get_negative_ray_attacks(
                Ray::SouthEast,
                square,
                blockers,
            ))
    }

    fn get_rook_attacks(square: usize, blockers: B) -> B {
        &(&Self::get_positive_ray_attacks(Ray::North, square, blockers)
            | &Self::get_positive_ray_attacks(Ray::East, square, blockers))
            | &(&Self::get_negative_ray_attacks(Ray::South, square, blockers)
                | &Self::get_negative_ray_attacks(Ray::West, square, blockers))
    }

    fn get_pawn_moves(square: usize, color: Color) -> B;

    fn init() {
        Self::init_pawn_attacks();
        Self::init_pawn_moves();
        Self::init_knight_attacks();
        Self::init_giraffe_attacks();
        Self::init_king_attacks();

        Self::init_north_ray();
        Self::init_south_ray();
        Self::init_east_ray();
        Self::init_west_ray();
        Self::init_north_east_ray();
        Self::init_north_west_ray();
        Self::init_south_east_ray();
        Self::init_south_west_ray();
        Self::init_between();
    }

    fn between(sq1: S, sq2: S) -> B;
}
