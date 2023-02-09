use super::bitboard12::B12;
use super::square12::Square12;

pub const EMPTY_BB: B12<Square12> = B12::new([0, 0, 0, 0, 0, 0, 0, 0, 0]);

const RANK1_BB: B12<Square12> = B12::new([0xFFF, 0, 0, 0, 0, 0, 0, 0, 0]);
const RANK2_BB: B12<Square12> = B12::new([0xFFF << 12, 0xFF, 0, 0, 0, 0, 0, 0, 0]);
const RANK3_BB: B12<Square12> = B12::new([0, 0xFF << 8, 0xF, 0, 0, 0, 0, 0, 0]);
const RANK4_BB: B12<Square12> = B12::new([0, 0, 0xFFF0, 0, 0, 0, 0, 0, 0]);
const RANK5_BB: B12<Square12> = B12::new([0, 0, 0, 0xFFF, 0, 0, 0, 0, 0]);
const RANK6_BB: B12<Square12> = B12::new([0, 0, 0, 0xFFF << 12, 0xFF, 0, 0, 0, 0]);
const RANK7_BB: B12<Square12> = B12::new([0, 0, 0, 0, 0xFF << 8, 0xF, 0, 0, 0]);
const RANK8_BB: B12<Square12> = B12::new([0, 0, 0, 0, 0, 0xFFF0, 0, 0, 0]);
const RANK9_BB: B12<Square12> = B12::new([0, 0, 0, 0, 0, 0, 0xFFF, 0, 0]);
const RANK10_BB: B12<Square12> = B12::new([0, 0, 0, 0, 0, 0, 0xFFF << 12, 0xFF, 0]);
const RANK11_BB: B12<Square12> = B12::new([0, 0, 0, 0, 0, 0, 0, 0xFF << 8, 0xF]);
const RANK12_BB: B12<Square12> = B12::new([0, 0, 0, 0, 0, 0, 0, 0, 0xFFF0]);

pub const RANK_BB: [B12<Square12>; 12] = [
    RANK1_BB, RANK2_BB, RANK3_BB, RANK4_BB, RANK5_BB, RANK6_BB, RANK7_BB, RANK8_BB, RANK9_BB,
    RANK10_BB, RANK11_BB, RANK12_BB,
];

const FILE1_BB: B12<Square12> = B12::new([
    0x1001,
    1 << 8,
    1 << 4,
    0x1001,
    1 << 8,
    1 << 4,
    0x1001,
    1 << 8,
    1 << 4,
]);
const FILE2_BB: B12<Square12> = B12::new([
    0x1001 << 1,
    1 << 9,
    1 << 5,
    0x1001 << 1,
    1 << 9,
    1 << 5,
    0x1001 << 1,
    1 << 9,
    1 << 5,
]);
const FILE3_BB: B12<Square12> = B12::new([
    0x1001 << 2,
    1 << 10,
    1 << 6,
    0x1001 << 2,
    1 << 10,
    1 << 6,
    0x1001 << 2,
    1 << 10,
    1 << 6,
]);
const FILE4_BB: B12<Square12> = B12::new([
    0x1001 << 3,
    1 << 11,
    1 << 7,
    0x1001 << 3,
    1 << 11,
    1 << 7,
    0x1001 << 3,
    1 << 11,
    1 << 7,
]);
const FILE5_BB: B12<Square12> = B12::new([
    0x1001 << 4,
    0x1001,
    1 << 8,
    0x1001 << 4,
    0x1001,
    1 << 8,
    0x1001 << 4,
    0x1001,
    1 << 8,
]);
const FILE6_BB: B12<Square12> = B12::new([
    0x1001 << 5,
    0x1001 << 1,
    1 << 9,
    0x1001 << 5,
    0x1001 << 1,
    1 << 9,
    0x1001 << 5,
    0x1001 << 1,
    1 << 9,
]);
const FILE7_BB: B12<Square12> = B12::new([
    0x1001 << 6,
    0x1001 << 2,
    1 << 10,
    0x1001 << 6,
    0x1001 << 2,
    1 << 10,
    0x1001 << 6,
    0x1001 << 2,
    1 << 10,
]);
const FILE8_BB: B12<Square12> = B12::new([
    0x1001 << 7,
    0x1001 << 3,
    1 << 11,
    0x1001 << 7,
    0x1001 << 3,
    1 << 11,
    0x1001 << 7,
    0x1001 << 3,
    1 << 11,
]);
const FILE9_BB: B12<Square12> = B12::new([
    1 << 8,
    1 << 4,
    0x1001,
    1 << 8,
    1 << 4,
    0x1001,
    1 << 8,
    1 << 4,
    0x1001,
]);
const FILE10_BB: B12<Square12> = B12::new([
    1 << 9,
    1 << 5,
    0x1001 << 1,
    1 << 9,
    1 << 5,
    0x1001 << 1,
    1 << 9,
    1 << 5,
    0x1001 << 1,
]);
const FILE11_BB: B12<Square12> = B12::new([
    1 << 10,
    1 << 6,
    0x1001 << 2,
    1 << 10,
    1 << 6,
    0x1001 << 2,
    1 << 10,
    1 << 6,
    0x1001 << 2,
]);
const FILE12_BB: B12<Square12> = B12::new([
    1 << 11,
    1 << 7,
    0x1001 << 3,
    1 << 11,
    1 << 7,
    0x1001 << 3,
    1 << 11,
    1 << 7,
    0x1001 << 3,
]);

pub const FILE_BB: [B12<Square12>; 12] = [
    FILE1_BB, FILE2_BB, FILE3_BB, FILE4_BB, FILE5_BB, FILE6_BB, FILE7_BB, FILE8_BB, FILE9_BB,
    FILE10_BB, FILE11_BB, FILE12_BB,
];
