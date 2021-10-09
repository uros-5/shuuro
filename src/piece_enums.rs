#[derive(PartialEq, Debug)]
pub enum PieceName {
    King,
    Queen,
    Bishop,
    Rook,
    Pawn,
    Night,
    Plynth,
    NoPiece,
}

#[derive(PartialEq, Debug)]
pub enum Color {
    Red,
    Blue,
    NoColor,
}

#[derive(PartialEq, Debug)]
pub enum TypeOfSearch {
    Check,
    MyMoves,
}

pub struct ServerPiece {
    pub color: Color,
    pub piece: PieceName,
}

#[derive(Debug)]
pub struct PiecePins {
    pub start: bool,
    pub fix: Vec<i32>,
}

impl PiecePins {
    pub fn new() -> PiecePins {
        PiecePins {
            start: false,
            fix: Vec::<i32>::new(),
        }
    }
    pub fn reset(&mut self) {
        self.start = false;
        self.fix.clear();
    }
}
