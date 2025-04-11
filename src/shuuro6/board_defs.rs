use super::bitboard6::BB6;
use super::square6::Square6;
pub const EMPTY_BB: BB6<Square6> = BB6::new(0);

pub const RANK_BB: [BB6<Square6>; 6] = {
    let mut masks = [BB6::new(0); 6];
    let mut i = 0;
    while i < 6 {
        masks[i] = BB6::new(0x3f << (i * 6));
        i += 1;
    }
    masks
};

const FILE_A: u64 = 0x41041041;

pub const FILE_BB: [BB6<Square6>; 6] = {
    let mut masks = [BB6::new(0); 6];
    let mut i = 0;
    while i < 6 {
        masks[i] = BB6::new(FILE_A << i);
        i += 1;
    }
    masks
};
