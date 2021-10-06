use crate::board::{add_pieces, get, get_possible_range, piece_exist};
use crate::piece_directions::{get_directions, get_directions_length};
use crate::piece_enums::{Color, PieceName, ServerPiece};

pub fn generate_moves(current_pos: i32) -> Vec<i32> {
    let mut table = add_pieces();
    let possible_range = get_possible_range();
    let mut possible_positions = Vec::<i32>::new();
    if possible_range.contains(&current_pos) {
        let (item, exist) = piece_exist(current_pos, &table);
        println!("{}", exist);
        if exist {
            let directions = get_directions(&item.piece);
            let directions_length = get_directions_length(&item.piece);
            let mut new_position = current_pos;
            for direction in &directions {
                new_position = current_pos;
                loop {
                    new_position += direction;
                    if !possible_range.contains(&new_position) {
                        break;
                    }
                    if new_position == current_pos {
                        break;
                    }
                    let (other_piece, exist) = piece_exist(new_position, &table);
                    if exist {
                        if item.color == other_piece.color {
                            break;
                        } else {
                            possible_positions.push(new_position);
                            break;
                        }
                    } else {
                        if other_piece.piece == PieceName::Plynth {
                            break;
                        }
                    }
                    possible_positions.push(new_position);
                    if directions_length == false {
                        break;
                    }
                }
            }
        }
    }
    possible_positions
}
