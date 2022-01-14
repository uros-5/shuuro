use crate::{get_non_sliding_attacks, BitBoard, Color, PieceType, Square, EMPTY_BB};
use rand::prelude::*;
fn two_plynths(ranks: &[char; 6], files: &[u8; 6]) -> BitBoard {
    let bb;
    let mut rang = thread_rng();
    let mut gen_square = || -> Square {
        let rank = rang.gen_range(0..6);
        let file = rang.gen_range(0..6);
        let square = &format!("{}{}", ranks[rank], files[file])[..];
        Square::from_sfen(square).unwrap()
    };
    let sq_1 = gen_square();
    bb = &EMPTY_BB | sq_1;
    let outside = get_non_sliding_attacks(PieceType::King, sq_1, Color::White);
    loop {
        let sq_2 = gen_square();
        if (&outside & sq_2).is_empty() {
            return &bb | sq_2;
        }
    }
}

pub fn generate_plinths() -> BitBoard {
    let left = ['a', 'b', 'c', 'd', 'e', 'f'];
    let right = ['g', 'h', 'i', 'j', 'k', 'l'];
    let mut bb = EMPTY_BB;
    for i in [left, right] {
        for j in [[1, 2, 3, 4, 5, 6], [7, 8, 9, 10, 11, 12]] {
            loop {
                let new_bb = two_plynths(&i, &j);
                if new_bb.count() == 2 {
                    bb |= &new_bb;
                    break;
                }
            }
        }
    }
    bb
}

#[cfg(test)]
mod tests {
    use crate::init;
    use crate::plinths_set::generate_plinths;
    #[test]
    fn generate_all_plinths() {
        init();
        let bb = generate_plinths();
        assert_eq!(bb.count(), 8);
    }
}
