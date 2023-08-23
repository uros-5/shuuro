use super::bitboard12::BB12;
use super::square12::Square12;

pub const EMPTY_BB: BB12<Square12> = BB12::new((0, 0));

const RANK1_BB: BB12<Square12> = BB12::new((0, 0));
const RANK2_BB: BB12<Square12> = BB12::new((0, 0));
const RANK3_BB: BB12<Square12> = BB12::new((0, 0));
const RANK4_BB: BB12<Square12> = BB12::new((0, 0));
const RANK5_BB: BB12<Square12> = BB12::new((0, 0));
const RANK6_BB: BB12<Square12> = BB12::new((0, 0));
const RANK7_BB: BB12<Square12> = BB12::new((0, 0));
const RANK8_BB: BB12<Square12> = BB12::new((0, 0));
const RANK9_BB: BB12<Square12> = BB12::new((0, 0));
const RANK10_BB: BB12<Square12> = BB12::new((0, 0));
const RANK11_BB: BB12<Square12> = BB12::new((0, 0));
const RANK12_BB: BB12<Square12> = BB12::new((0, 0));

pub const RANK_BB: [BB12<Square12>; 12] = [
    RANK1_BB, RANK2_BB, RANK3_BB, RANK4_BB, RANK5_BB, RANK6_BB, RANK7_BB,
    RANK8_BB, RANK9_BB, RANK10_BB, RANK11_BB, RANK12_BB,
];

const FILE1_BB: BB12<Square12> = BB12::new((0, 0));
const FILE2_BB: BB12<Square12> = BB12::new((0, 0));
const FILE3_BB: BB12<Square12> = BB12::new((0, 0));
const FILE4_BB: BB12<Square12> = BB12::new((0, 0));
const FILE5_BB: BB12<Square12> = BB12::new((0, 0));
const FILE6_BB: BB12<Square12> = BB12::new((0, 0));
const FILE7_BB: BB12<Square12> = BB12::new((0, 0));
const FILE8_BB: BB12<Square12> = BB12::new((0, 0));
const FILE9_BB: BB12<Square12> = BB12::new((0, 0));
const FILE10_BB: BB12<Square12> = BB12::new((0, 0));
const FILE11_BB: BB12<Square12> = BB12::new((0, 0));
const FILE12_BB: BB12<Square12> = BB12::new((0, 0));

pub const FILE_BB: [BB12<Square12>; 12] = [
    FILE1_BB, FILE2_BB, FILE3_BB, FILE4_BB, FILE5_BB, FILE6_BB, FILE7_BB,
    FILE8_BB, FILE9_BB, FILE10_BB, FILE11_BB, FILE12_BB,
];
