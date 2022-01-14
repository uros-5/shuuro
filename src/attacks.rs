use crate::bitboard::BitBoard;
use crate::board_defs::{EMPTY_BB, FILE_BB, RANK_BB};
use crate::piece_type::PieceType;
use crate::SQUARE_BB;
use crate::{Color, Square};

#[derive(Clone, Copy)]
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

static mut NON_SLIDING_ATTACKS: [[[BitBoard; 144]; 6]; 2] =
    [[[BitBoard([0, 0, 0, 0, 0, 0, 0, 0, 0]); 144]; 6]; 2];

static mut RAYS: [[BitBoard; 144]; 8] = [[BitBoard([0, 0, 0, 0, 0, 0, 0, 0, 0]); 144]; 8];
pub fn init() {
    init_pawn_attacks();
    init_knight_attacks();
    init_king_attacks();

    init_north_ray();
    init_south_ray();
    init_east_ray();
    init_west_ray();
    init_north_east_ray();
    init_north_west_ray();
    init_south_east_ray();
    init_south_west_ray();
    init_between();
}

pub fn get_non_sliding_attacks(piece_type: PieceType, square: Square, color: Color) -> BitBoard {
    unsafe { NON_SLIDING_ATTACKS[color as usize][piece_type as usize][square.index()] }
}

pub fn get_sliding_attacks(piece_type: PieceType, square: Square, blockers: BitBoard) -> BitBoard {
    match piece_type {
        PieceType::Bishop => get_bishop_attacks(square.index(), blockers),
        PieceType::Rook => get_rook_attacks(square.index(), blockers),
        PieceType::Queen => {
            &get_bishop_attacks(square.index(), blockers)
                | &get_rook_attacks(square.index(), blockers)
        }
        _ => EMPTY_BB,
    }
}

fn get_bishop_attacks(square: usize, blockers: BitBoard) -> BitBoard {
    &(&get_positive_ray_attacks(Ray::NorthWest, square, blockers)
        | &get_positive_ray_attacks(Ray::NorthEast, square, blockers))
        | &(&get_negative_ray_attacks(Ray::SouthWest, square, blockers)
            | &get_negative_ray_attacks(Ray::SouthEast, square, blockers))
}

fn get_rook_attacks(square: usize, blockers: BitBoard) -> BitBoard {
    &(&get_positive_ray_attacks(Ray::North, square, blockers)
        | &get_positive_ray_attacks(Ray::East, square, blockers))
        | &(&get_negative_ray_attacks(Ray::South, square, blockers)
            | &get_negative_ray_attacks(Ray::West, square, blockers))
}

fn get_positive_ray_attacks(dir: Ray, square: usize, blockers: BitBoard) -> BitBoard {
    unsafe {
        let attacks = RAYS[dir as usize][square];
        let mut blocked = &attacks & &blockers;
        let block_square = blocked.pop();
        match block_square {
            Some(i) => &attacks & &!&RAYS[dir as usize][i.index() as usize],
            None => attacks,
        }
    }
}

fn get_negative_ray_attacks(dir: Ray, square: usize, blockers: BitBoard) -> BitBoard {
    unsafe {
        let attacks = RAYS[dir as usize][square];
        let mut blocked = &attacks & &blockers;
        let block_square = blocked.pop_reverse();
        match block_square {
            Some(i) => &attacks & &!&RAYS[dir as usize][i.index() as usize],
            None => attacks,
        }
    }
}

fn init_pawn_attacks() {
    fn add(index: i32, color: Color, attacks: BitBoard) {
        unsafe {
            NON_SLIDING_ATTACKS[color as usize][PieceType::Pawn as usize][index as usize] |=
                &attacks;
        }
    }
    let dirs: [i32; 6] = [11, 13, 12, -11, -13, -12];
    for i in 0..144 {
        for dir in dirs {
            let current_attack: BitBoard;
            let new_index = i + dir;
            if new_index > 0 && new_index < 144 {
                current_attack = SQUARE_BB[new_index as usize];
                match dir {
                    13 => add(i, Color::White, &current_attack & &!&RANK_BB[0]),
                    11 => add(i, Color::White, &current_attack & &!&RANK_BB[11]),
                    12 => add(i, Color::White, current_attack),
                    -11 => add(i, Color::Black, &current_attack & &!&RANK_BB[0]),
                    -13 => add(i, Color::Black, &current_attack & &!&RANK_BB[11]),
                    -12 => add(i, Color::Black, current_attack),
                    _ => (),
                }
            }
        }
    }
}

fn init_knight_attacks() {
    let dirs: [i32; 8] = [-10, 10, 23, 25, 14, -23, -25, -14];
    for i in 0..144 {
        let mut attacks: BitBoard = EMPTY_BB;
        for dir in dirs {
            let mut current_attack: BitBoard;
            let new_index = i + dir;
            if new_index > 0 && new_index < 144 {
                current_attack = SQUARE_BB[new_index as usize];
                match dir {
                    23 | -25 => current_attack &= &!&RANK_BB[11],
                    -23 | 25 => current_attack &= &!&RANK_BB[0],
                    10 | -14 => current_attack &= &!&(&RANK_BB[11] | &RANK_BB[10]),
                    -10 | 14 => current_attack &= &!&(&RANK_BB[0] | &RANK_BB[1]),
                    _ => (),
                }
                attacks = &attacks | &current_attack;
            }
        }
        unsafe {
            NON_SLIDING_ATTACKS[Color::White as usize][PieceType::Knight as usize][i as usize] =
                attacks;
            NON_SLIDING_ATTACKS[Color::Black as usize][PieceType::Knight as usize][i as usize] =
                attacks;
        }
    }
}

fn init_king_attacks() {
    let dirs: [i32; 8] = [11, 12, 13, -13, -12, -11, 1, -1];
    for i in 0..144 {
        let mut attacks: BitBoard = EMPTY_BB;
        for dir in dirs {
            let mut current_attack: BitBoard;
            let new_index = i + dir;
            if new_index >= 0 && new_index < 144 {
                current_attack = SQUARE_BB[new_index as usize];
                match dir {
                    11 | -13 | -1 => current_attack &= &!&RANK_BB[11],
                    -11 | 13 | 1 => current_attack &= &!&RANK_BB[0],
                    _ => (),
                }
                attacks = &attacks | &current_attack;
            }
        }
        unsafe {
            NON_SLIDING_ATTACKS[Color::White as usize][PieceType::King as usize][i as usize] =
                attacks;
            NON_SLIDING_ATTACKS[Color::Black as usize][PieceType::King as usize][i as usize] =
                attacks;
        }
    }
}

fn init_north_ray() {
    for i in 0..144 {
        let calc_rank = i % 12;
        let file = i / 12;
        let mut rank = &RANK_BB[calc_rank] & &!&SQUARE_BB[i];
        for j in 0..file {
            rank &= &!&FILE_BB[j];
        }
        unsafe {
            RAYS[Ray::North as usize][i as usize] = rank;
        }
    }
}

fn init_south_ray() {
    for i in 0..144 {
        let calc_rank = i % 12;
        let file = i / 12;
        let mut rank = &RANK_BB[calc_rank] & &!&SQUARE_BB[i];
        for j in file..12 {
            rank &= &!&FILE_BB[j];
        }
        unsafe {
            RAYS[Ray::South as usize][i as usize] = rank;
        }
    }
}

fn init_east_ray() {
    for i in 0..144 {
        let calc_file = i / 12;
        let rank = i % 12;
        let mut file = &FILE_BB[calc_file] & &!&SQUARE_BB[i];
        for j in 0..rank {
            file &= &!&RANK_BB[j];
        }
        unsafe {
            RAYS[Ray::East as usize][i as usize] = file;
        }
    }
}

fn init_west_ray() {
    for i in 0..144 {
        let calc_file = i / 12;
        let rank = i % 12;
        let mut file = &FILE_BB[calc_file] & &!&SQUARE_BB[i];
        for j in rank..12 {
            file &= &!&RANK_BB[j];
        }
        unsafe {
            RAYS[Ray::West as usize][i as usize] = file;
        }
    }
}

fn init_north_east_ray() {
    for i in 0..144 {
        let mut all = EMPTY_BB;
        let mut new_index: i32 = i;
        loop {
            let file = new_index / 12;
            new_index = new_index + 13;
            if new_index >= 0 && new_index < 144 {
                if file + 1 == new_index / 12 {
                    all |= &SQUARE_BB[new_index as usize];
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        unsafe {
            RAYS[Ray::NorthEast as usize][i as usize] = all;
        }
    }
}

fn init_north_west_ray() {
    for i in 0..144 {
        let mut all = EMPTY_BB;
        let mut new_index: i32 = i;
        loop {
            let file = new_index / 12;
            new_index = new_index + 11;
            if new_index >= 0 && new_index < 144 {
                if file != new_index / 12 {
                    all |= &SQUARE_BB[new_index as usize];
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        unsafe {
            RAYS[Ray::NorthWest as usize][i as usize] = all;
        }
    }
}

fn init_south_east_ray() {
    for i in 0..144 {
        let mut all = EMPTY_BB;
        let mut new_index: i32 = i;
        loop {
            let file = new_index / 12;
            new_index = new_index - 11;
            if new_index >= 0 && new_index < 144 {
                if file != new_index / 12 {
                    all |= &SQUARE_BB[new_index as usize];
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        unsafe {
            RAYS[Ray::SouthEast as usize][i as usize] = all;
        }
    }
}

fn init_south_west_ray() {
    for i in 0..144 {
        let mut all = EMPTY_BB;
        let mut new_index: i32 = i;
        loop {
            let file = new_index / 12;
            new_index = new_index - 13;
            if new_index >= 0 && new_index < 144 {
                if file - 1 == new_index / 12 {
                    all |= &SQUARE_BB[new_index as usize];
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        unsafe {
            RAYS[Ray::SouthWest as usize][i as usize] = all;
        }
    }
}

static mut BETWEEN_BB: [[BitBoard; 144]; 144] = [[BitBoard([0, 0, 0, 0, 0, 0, 0, 0, 0]); 144]; 144];

fn init_between() {
    for from in Square::iter() {
        for to in Square::iter() {
            if from == to {
                continue;
            }

            let df = from.file() as i8 - to.file() as i8;
            let dr = from.rank() as i8 - to.rank() as i8;
            unsafe {
                if df == 0 || dr == 0 {
                    BETWEEN_BB[from.index()][to.index()] =
                        &get_sliding_attacks(PieceType::Rook, from, SQUARE_BB[to.index()])
                            & &get_sliding_attacks(PieceType::Rook, to, SQUARE_BB[from.index()]);
                } else if df.abs() == dr.abs() {
                    BETWEEN_BB[from.index()][to.index()] =
                        &get_sliding_attacks(PieceType::Bishop, from, SQUARE_BB[to.index()])
                            & &get_sliding_attacks(PieceType::Bishop, to, SQUARE_BB[from.index()]);
                }
            }
        }
    }
}

pub fn between(square1: Square, square2: Square) -> BitBoard {
    unsafe { BETWEEN_BB[square1.index()][square2.index()] }
}
