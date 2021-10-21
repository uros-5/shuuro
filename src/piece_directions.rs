use crate::board::{Color, Piece, Searching};
use Piece::*;

pub fn get_directions(piece: &Piece, _color: &Color) -> Vec<i32> {
    const ORTHOGONAL: [i32; 4] = [-1, 1, -16, 16];
    const DIAGONAL: [i32; 4] = [-17, -15, 17, 15];
    const JUMP: [i32; 8] = [31, 33, 14, 18, -14, -18, -31, -33];
    match piece {
        King => {
            [ORTHOGONAL,DIAGONAL].concat()
        }
        Queen => {
            [ORTHOGONAL,DIAGONAL].concat()
        }
        Bishop => {
            DIAGONAL.to_vec()
        }
        Rook => {
            ORTHOGONAL.to_vec()
        }
        Pawn => {
            [ORTHOGONAL,DIAGONAL].concat()
        }
        Knight => {
            JUMP.to_vec()
        }
        _ => [].to_vec(),
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
