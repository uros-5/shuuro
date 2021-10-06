#[derive(PartialEq)]
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

#[derive(PartialEq)]
pub enum Color {
    Red,
    Blue,
    NoColor,
}

pub struct ServerPiece {
    pub color: Color,
    pub piece: PieceName,
}
