use crate::board::{Color, Piece, Searching};
use Piece::*;

pub fn get_directions(piece: &Piece, _color: &Color) -> Vec<i32> {
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
        Knight => {
            calc.append(&mut vec3);
            calc
        }
        _ => calc,
    }
}

pub fn get_directions_length(type_of: &Searching, piece_name: &Piece) -> bool {
    fn right_piece(piece: &Piece) -> bool {
        match piece {
            King => false,
            Queen => true,
            Rook => true,
            Bishop => true,
            Knight => false,
            Pawn => true,
            _ => false,
        }
    }
    match type_of {
        Searching::Check => right_piece(piece_name),
        Searching::Regular => right_piece(piece_name),
        Searching::Pin => true,
    }
}
