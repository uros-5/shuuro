use crate::piece_enums::PieceName;
use PieceName::*;

pub fn get_directions(piece: &PieceName) -> Vec<i32> {
    const ORTHOGONAL: [i32; 4] = [-1, 1, -16, 16];
    const DIAGONAL: [i32; 4] = [-17, -15, 17, 15];
    const JUMP: [i32; 8] = [31, 33, 14, 18, -14, -18, -31, -33];
    let mut vec1 = ORTHOGONAL.to_vec();
    let mut vec2 = DIAGONAL.to_vec();
    let mut vec3 = JUMP.to_vec();
    let mut calc = Vec::<i32>::new();
    match piece {
        King => {
            calc.append(&mut vec1);
            calc.append(&mut vec2);
            calc
        }
        Queen => {
            calc.append(&mut vec1);
            calc.append(&mut vec2);
            calc
        }
        Bishop => {
            calc.append(&mut vec2);
            calc
        }
        Rook => {
            calc.append(&mut vec1);
            calc
        }
        Pawn => {
            calc.append(&mut vec1);
            calc.append(&mut vec2);
            calc
        }
        Night => {
            calc.append(&mut vec3);
            calc
        }
        _ => calc,
    }
}

pub fn get_directions_length(piece: &PieceName) -> bool {
    match piece {
        King => false,
        Queen => true,
        Rook => true,
        Bishop => true,
        Night => false,
        Pawn => true,
        _ => false,
    }
}
