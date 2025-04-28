use thiserror::Error;

/// The error type for SFEN serialize/deserialize operations.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum SfenError {
    #[error("data fields are missing")]
    MissingDataFields,

    #[error("an illegal piece notation is found")]
    IllegalPieceType,

    #[error("the side to move needs to be white or black")]
    IllegalSideToMove,

    #[error("this square does not exist")]
    IllegalPieceFound,

    #[error("an illegal move count notation is found")]
    IllegalMoveCount(#[from] std::num::ParseIntError),

    #[error("an illegal move notation is found")]
    IllegalMove,

    #[error("an illegal board state notation is found")]
    IllegalBoardState,

    #[error("check on first move is found")]
    IllegalFirstMove,

    #[error("plinths can contain only knights")]
    IllegalPieceTypeOnPlynth,

    #[error("board size is bigger")]
    UnknownFile,
}

/// Represents an error occurred during making a move.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum MoveError {
    #[error("the king is in check")]
    InCheck,

    #[error("perpetual check detected")]
    Draw,

    #[error("stalemate detected")]
    DrawByStalemate,

    #[error("Insufficient material detected")]
    DrawByInsufficientMaterial,

    #[error("not your turn")]
    EnemysTurn,

    #[error("the piece can not move anymore")]
    NonMovablePiece,

    #[error("the move is inconsistent with the current position: {0}")]
    Inconsistent(&'static str),

    #[error("repetition detected")]
    RepetitionDraw,

    #[error("first move error")]
    FirstMoveError,
}
