use piece_moves::MoveGenerator;

mod board;
mod move_req;
mod piece_directions;
mod piece_enums;
mod piece_moves;

fn main() {
    let mut a = MoveGenerator::new(24);
    let b = &a.run();
    println!("{:?}", b);
}
