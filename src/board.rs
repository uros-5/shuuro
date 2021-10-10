use std::collections::HashMap;

use crate::piece_enums::{Color, PieceName, ServerPiece};

pub fn add_pieces() -> HashMap<i32, ServerPiece> {
    let mut table2: HashMap<i32, ServerPiece> = HashMap::new();
    table2.insert(
        54,
        ServerPiece {
            color: Color::Blue,
            piece: PieceName::Queen,
        },
    );

    table2.insert(
        57,
        ServerPiece {
            color: Color::Blue,
            piece: PieceName::King,
        },
    );

    table2.insert(
        105,
        ServerPiece {
            color: Color::Red,
            piece: PieceName::Bishop,
        },
    );

    table2.insert(
        154,
        ServerPiece {
            color: Color::Red,
            piece: PieceName::King,
        },
    );

    table2.insert(
        48,
        ServerPiece {
            color: Color::Red,
            piece: PieceName::Rook,
        },
    );

    table2
}

pub fn get(id: i32, table: &HashMap<i32, ServerPiece>) -> &ServerPiece {
    let value = table.get(&id);
    match value {
        Some(i) => i,
        None => &ServerPiece {
            piece: PieceName::NoPiece,
            color: Color::NoColor,
        },
    }
}

pub fn piece_exist(id: i32, table: &HashMap<i32, ServerPiece>) -> (&ServerPiece, bool) {
    let piece = get(id, table);
    match piece.piece {
        PieceName::NoPiece => return (piece, false),
        PieceName::Plynth => return (piece, false),
        _ => return (piece, true),
    }
}

pub fn get_possible_range() -> Vec<i32> {
    let mut start = 0;
    let mut end = 12;
    let mut all_pos = Vec::<i32>::new();
    for _i in 0..12 {
        for i2 in start..end {
            all_pos.push(i2)
        }
        start += 16;
        end += 16;
    }
    all_pos
}
