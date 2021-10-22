use piece_moves::MoveGenerator;

use crate::board::Board;

mod board;
mod piece_directions;
mod piece_moves;

fn main() {
    let mut board = Board::new();
    board.add_data();
    let mut a = MoveGenerator::new(54, &board);
    a.run();
}
