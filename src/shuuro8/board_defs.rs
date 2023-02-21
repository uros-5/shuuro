use super::bitboard8::BB8;
use super::square8::Square8;
pub const EMPTY_BB: BB8<Square8> = BB8::new(0);

pub const RANK_BB: [BB8<Square8>; 8] = {
    let mut masks = [BB8::new(0); 8];
    let mut i = 0;
    while i < 8 {
        masks[i] = BB8::new(0xff << (i * 8));
        i += 1;
    }
    masks
};

const FILE_A: u64 = 0x0101_0101_0101_0101;

pub const FILE_BB: [BB8<Square8>; 8] = {
    let mut masks = [BB8::new(0); 8];
    let mut i = 0;
    while i < 8 {
        masks[i] = BB8::new(FILE_A << i);
        i += 1;
    }
    masks
};
