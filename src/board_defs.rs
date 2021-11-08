use crate::bitboard::BitBoard;

pub const EMPTY_BB: BitBoard = BitBoard([0, 0, 0, 0, 0, 0, 0, 0, 0]);

const FILE1_BB: BitBoard = BitBoard([0xFFF, 0, 0, 0, 0, 0, 0, 0, 0]);
const FILE2_BB: BitBoard = BitBoard([0xFFF << 12, 0xFF, 0, 0, 0, 0, 0, 0, 0]);
const FILE3_BB: BitBoard = BitBoard([0, 0xFF << 8, 0xF, 0, 0, 0, 0, 0, 0]);
const FILE4_BB: BitBoard = BitBoard([0, 0, 0xFFF0, 0, 0, 0, 0, 0, 0]);
const FILE5_BB: BitBoard = BitBoard([0, 0, 0, 0xFFF, 0, 0, 0, 0, 0]);
const FILE6_BB: BitBoard = BitBoard([0, 0, 0, 0xFFF << 12, 0xFF, 0, 0, 0, 0]);
const FILE7_BB: BitBoard = BitBoard([0, 0, 0, 0, 0xFF << 8, 0xF, 0, 0, 0]);
const FILE8_BB: BitBoard = BitBoard([0, 0, 0, 0, 0, 0xFFF0, 0, 0, 0]);
const FILE9_BB: BitBoard = BitBoard([0, 0, 0, 0, 0, 0, 0xFFF, 0, 0]);
const FILE10_BB: BitBoard = BitBoard([0, 0, 0, 0, 0, 0, 0xFFF << 12, 0xFF, 0]);
const FILE11_BB: BitBoard = BitBoard([0, 0, 0, 0, 0, 0, 0, 0xFF << 8, 0xF]);
const FILE12_BB: BitBoard = BitBoard([0, 0, 0, 0, 0, 0, 0, 0, 0xFFF0]);

pub const FILE_BB: [BitBoard; 12] = [
    FILE1_BB, FILE2_BB, FILE3_BB, FILE4_BB, FILE5_BB, FILE6_BB, FILE7_BB, FILE8_BB, FILE9_BB,
    FILE10_BB, FILE11_BB, FILE12_BB,
];

const RANK1_BB: BitBoard = BitBoard([
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
const RANK2_BB: BitBoard = BitBoard([
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
const RANK3_BB: BitBoard = BitBoard([
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
const RANK4_BB: BitBoard = BitBoard([
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
const RANK5_BB: BitBoard = BitBoard([
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
const RANK6_BB: BitBoard = BitBoard([
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
const RANK7_BB: BitBoard = BitBoard([
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
const RANK8_BB: BitBoard = BitBoard([
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
const RANK9_BB: BitBoard = BitBoard([
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
const RANK10_BB: BitBoard = BitBoard([
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
const RANK11_BB: BitBoard = BitBoard([
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
const RANK12_BB: BitBoard = BitBoard([
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

pub const RANK_BB: [BitBoard; 12] = [
    RANK1_BB, RANK2_BB, RANK3_BB, RANK4_BB, RANK5_BB, RANK6_BB, RANK7_BB, RANK8_BB, RANK9_BB,
    RANK10_BB, RANK11_BB, RANK12_BB,
];
