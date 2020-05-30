use crate::board::*;
use crate::movement::*;
use crate::piece::*;
use crate::position::*;
use std::error;
use std::fmt;

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
#[allow(non_snake_case)]
#[allow(unused_variables)]
impl fmt::Display for InvalidMoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InvalidMoveError::MoveSyntaxError => write!(f,"Move has an incorrect syntax"),
            InvalidMoveError::OutOfBoardMoveError => write!(f,"The move uses squares outside of the board"),
            InvalidMoveError::DestinationOccupiedError => write!(f,"The destination square is occupied"),
            InvalidMoveError::NoPieceAtPositionError => write!(f,"No (such) piece was found at the start location"),
            InvalidMoveError::NoPieceCapturedError => write!(f,"Capture was indicated but no piece was captured"),
            InvalidMoveError::PieceHasNoSuchMoveError => write!(f,"The piece cannot move in such a way"),
            InvalidMoveError::NifuViolationError => write!(f,"A pawn was dropped in a column already occupied by a non-promoted pawn"),
            InvalidMoveError::NoMovePossibleAfterDropError => write!(f,"The piece was dropped in a position but will never be able to move afterwards"),
            InvalidMoveError::MandatoryPromotionError => write!(f,"The promotion of the piece is mandatory at this position but move do not provide it"),
            InvalidMoveError::UncoverCheckError => write!(f,"The move uncovers the king"),
            InvalidMoveError::CheckmateByPawnDropError => write!(f,"A checkmate cannot be given by dropping a pawn")
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

fn check_position(p: Position, b: Board) -> Option<Piece> {
    b.into_iter().find(|piece| piece.position == Some(p))
}

///check if destination is not occupied (or occupied by opponent)
pub fn check_destination(mv: &str, b: Board) -> Result<&str, InvalidMoveError> {
    let full_move: Movement = mv.parse().unwrap();

    let destination = full_move.end;

    if full_move.start == None {
        //the move is a drop
        if None == check_position(destination, b) {
            //ok the destination is empty
            return Ok(mv);
        } else {
            return Err(InvalidMoveError::DestinationOccupiedError);
        }
    } else {
        //the move is a normal move
        let current_player_color = b.get_color();
        if full_move.force_capture {
            // check there is an opponent piece there
            if let Some(p) = check_position(destination, b) {
                if p.color != current_player_color {
                    return Ok(mv);
                }
            }
            return Err(InvalidMoveError::NoPieceCapturedError);
        } else {
            //check if there is not one's own piece already there
            if let Some(p) = check_position(destination, b) {
                if p.color == current_player_color {
                    return Err(InvalidMoveError::NoPieceCapturedError);
                }
            }
            return Ok(mv);
        }
    }
}

pub fn check_start(mv: &str, b: Board) -> Result<&str, InvalidMoveError> {
    let full_move: Movement = mv.parse().unwrap();
    if b.into_iter()
        .find(|p| p.position == full_move.start && p.piecetype == full_move.piecetype)
        == None
    {
        //no such piece at given start position
        return Err(InvalidMoveError::NoPieceAtPositionError);
    }
    return Ok(mv);
}

pub fn check_possible_move(mv: &str, b: Board) -> Result<&str, InvalidMoveError> {
    Ok(mv)
    //Err(InvalidMoveError::PieceHasNoSuchMove)
    //TODO
}

pub fn check_nifu(mv: &str, b: Board) -> Result<&str, InvalidMoveError> {
    let full_move: Movement = mv.parse().unwrap();
    if full_move.piecetype != PieceType::Pawn || full_move.start != None {
        //not a pawn, not a drop
        return Ok(mv);
    }
    if let Some(_) = b.into_iter().filter(|p| p.position != None).find(|p| {
        p.piecetype == PieceType::Pawn && p.position.unwrap().0 % 9 == full_move.end.0 % 9
    }) {
        //two pawn on same column
        return Err(InvalidMoveError::NifuViolationError);
    }
    return Ok(mv);
}

pub fn check_move_possible_after_drop(mv: &str, b: Board) -> Result<&str, InvalidMoveError> {
    if !maybe_drop(mv) {
        //move not a drop so no check
        return Ok(mv);
    }

    let last_row;
    let before_last_row;
    if b.get_color() == Color::White {
        last_row = 'i';
        before_last_row = 'h';
    } else {
        last_row = 'a';
        before_last_row = 'b';
    }
    let full_move: Movement = mv.parse().unwrap();

    if full_move.piecetype == PieceType::Pawn && full_move.end.row() == last_row {
        return Err(InvalidMoveError::NoMovePossibleAfterDropError);
    } else if full_move.piecetype == PieceType::Lance && full_move.end.row() == last_row {
        return Err(InvalidMoveError::NoMovePossibleAfterDropError);
    } else if full_move.piecetype == PieceType::Knight
        && (full_move.end.row() == last_row || full_move.end.row() == before_last_row)
    {
        return Err(InvalidMoveError::NoMovePossibleAfterDropError);
    } else {
        return Ok(mv);
    }
}

pub fn check_mandatory_promotion(mv: &str, b: Board) -> Result<&str, InvalidMoveError> {
    Ok(mv)
    //Err(InvalidMoveError::MandatoryPromotionError);
    //TODO
}

pub fn check_uncover_check(mv: &str, b: Board) -> Result<&str, InvalidMoveError> {
    if b.rules.can_uncover_check {
        return Ok(mv);
    }

    //implement real mechanic here

    return Ok(mv);
    //Err(InvalidMoveError::UncoverCheckError)
    //TODO
}

pub fn check_checkmate_by_pawn_drop(mv: &str, b: Board) -> Result<&str, InvalidMoveError> {
    Ok(mv)
    //Err(InvalidMoveError::CheckmateByPawnDropError)
    //TODO
}
