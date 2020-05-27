use std::error;
use std::fmt;

// type Result<T> = std::result::Result<T, InvalidMoveError>;

#[derive(Debug, Clone)]
pub enum InvalidMoveError {
    MoveSyntaxError,
    OutOfBoardMoveError,
    DestinationOccupiedError,
    NoPieceAtPositionError,
    NoPieceCapturedError,
    PieceHasNoSuchMoveError,
    NifuViolationError,
    NoMovePossibleAfterDropError,
    MandatoryPromotionError,
    UncoverCheckError,
    CheckmateByPawnDropError,
}
impl fmt::Display for InvalidMoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MoveSyntaxError => write!(f,"Move has an incorrect syntax"),
            OutOfBoardMoveError => write!(f,"The move uses squares outside of the board"),
            DestinationOccupiedError => write!(f,"The destination square is occupied"),
            NoPieceAtPositionError => write!(f,"No piece was found at the start location"),
            NoPieceCapturedError => write!(f,"Capture was indicated but no piece was captured"),
            PieceHasNoSuchMoveError => write!(f,"The piece cannot move in such a way"),
            NifuViolationError => write!(f,"A pawn was dropped in a column already occupied by a non-promoted pawn"),
            NoMovePossibleAfterDropError => write!(f,"The piece was dropped in a position but will never be able to move afterwards"),
            MandatoryPromotionError => write!(f,"The promotion of the piece is mandatory at this position but move do not provide it"),
            UncoverCheckError => write!(f,"The move uncovers the king"),
            CheckmateByPawnDropError => write!(f,"A checkmate cannot be given by dropping a pawn")
        }
    }
}

// This is important for other errors to wrap this one.
impl error::Error for InvalidMoveError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
