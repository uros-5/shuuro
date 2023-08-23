use super::bitboard12::BB12;
use super::square12::Square12;

pub const EMPTY_BB: BB12<Square12> = BB12::new((0, 0));

const fn ranks() -> [BB12<Square12>; 12] {
    let mut all = [BB12::new((0, 0)); 12];
    let mut i = 0;
    while i < 10 {
        all[i] = BB12::new((0xFFF << (i * 12), 0));
        i += 1;
    }
    all[10] = BB12::new((0xFF << (10 * 12), 0xF));
    all[11] = BB12::new((0, 0xFFF << 4));
    all
}

pub const RANK_BB: [BB12<Square12>; 12] = ranks();

const fn files() -> [BB12<Square12>; 12] {
    let mut all = [BB12::new((0, 0)); 12];
    let mut i = 0;
    let file = 0x1001001001001001001001001001001;
    let file2 = 0x1001001001001001001001001001;
    while i < 12 {
        if i >= 8 {
            let c = 1 << (i - 8) | (1 << (4 + i));
            all[i] = BB12::new((file2 << i, c));
        } else {
            all[i] = BB12::new((file << i, 1 << (4 + i)));
        }
        i += 1;
    }
    all
}

pub const FILE_BB: [BB12<Square12>; 12] = files();
