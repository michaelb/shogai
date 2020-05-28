use crate::board::*;
use crate::movement::*;
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

fn maybe_drop(mv: &str) -> bool {
    if mv.len() == 4 && mv.as_bytes()[1] == b'*' {
        true
    } else {
        false
    }
}

fn maybe_normal_move(mv: &str) -> bool {
    if ((mv.len() == 7 && mv.as_bytes()[6] == b'+') || (mv.len() == 6))
        && (mv.as_bytes()[3] == b'-' || mv.as_bytes()[3] == b'x')
    {
        true
    } else {
        false
    }
}

pub fn check_syntax(mv: &str) -> Result<&str, InvalidMoveError> {
    if !(maybe_drop(mv) || maybe_normal_move(mv)) {
        return Err(InvalidMoveError::MoveSyntaxError);
    }

    let first_char = mv.chars().next().unwrap();
    if first_char != 'P'
        && first_char != 'K'
        && first_char != 'R'
        && first_char != 'B'
        && first_char != 'G'
        && first_char != 'S'
        && first_char != 'N'
        && first_char != 'L'
    {
        return Err(InvalidMoveError::MoveSyntaxError);
    }

    return Ok(mv);
}

/// Check if all the squares invloved fit into the shogiban
pub fn check_in_board(mv: &str) -> Result<&str, InvalidMoveError> {
    let mut in_board = true;
    if maybe_drop(mv) {
        in_board &= mv.as_bytes()[2] as char >= '1' && mv.as_bytes()[2] as char <= '9';
        in_board &= mv.as_bytes()[3] as char >= 'a' && mv.as_bytes()[3] as char <= 'i';
    }

    if maybe_normal_move(mv) {
        in_board &= mv.as_bytes()[1] as char >= '1' && mv.as_bytes()[1] as char <= '9';
        in_board &= mv.as_bytes()[2] as char >= 'a' && mv.as_bytes()[2] as char <= 'i';

        in_board &= mv.as_bytes()[4] as char >= '1' && mv.as_bytes()[4] as char <= '9';
        in_board &= mv.as_bytes()[5] as char >= 'a' && mv.as_bytes()[5] as char <= 'i';
    }

    if in_board {
        Ok(mv)
    } else {
        Err(InvalidMoveError::OutOfBoardMoveError)
    }
}
