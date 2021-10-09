use piece_moves::MoveGenerator;

mod piece_moves;
mod board;
mod move_req;
mod piece_directions;
mod piece_enums;

fn main() {
    let mut a = MoveGenerator::new(128);
    let b = &a.run();
    println!("{:?}", b);
}
