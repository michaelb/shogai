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
    PromotionError,
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
            InvalidMoveError::PromotionError => write!(f,"The promotion of the piece is mandatory or impossible at this position but move do not provide the correct instruction"),
            InvalidMoveError::UncoverCheckError => write!(f,"The king cannot be left in check with the current rules"),
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
pub fn check_destination<'a>(mv: &'a str, b: &'a Board) -> Result<&'a str, InvalidMoveError> {
    let full_move: Movement = mv.parse().unwrap();

    let destination = full_move.end;

    if full_move.start == None {
        //the move is a drop
        if None == b.is_occupied_by(destination) {
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
            if let Some(p) = b.is_occupied_by(destination) {
                if p.color != current_player_color {
                    return Ok(mv);
                }
            }
            return Err(InvalidMoveError::NoPieceCapturedError);
        } else {
            //check if there is not one's own piece already there
            if let Some(p) = b.is_occupied_by(destination) {
                if p.color == current_player_color {
                    return Err(InvalidMoveError::DestinationOccupiedError);
                }
            }
            return Ok(mv);
        }
    }
}

pub fn check_start<'a>(mv: &'a str, b: &'a Board) -> Result<&'a str, InvalidMoveError> {
    let full_move: Movement = mv.parse().unwrap();
    if b.clone().into_iter().find(|p| {
        p.position == full_move.start
            && p.piecetype == full_move.piecetype
            && p.color == b.get_color()
    }) == None
    {
        //no such piece at given start position
        return Err(InvalidMoveError::NoPieceAtPositionError);
    }

    return Ok(mv);
}

///the move only moves from square next to each other, no need to check path (and path-checking has
///trouble with promotino so it's win-win
fn small_move(start: Position, end: Position) -> bool {
    let x = start.row() as i32 - end.row() as i32;
    let y = start.column() as i32 - end.column() as i32;
    return x.abs() <= 1 && y.abs() <= 1;
}

pub fn check_possible_move<'a>(mv: &'a str, b: &'a Board) -> Result<&'a str, InvalidMoveError> {
    if maybe_drop(mv) {
        return Ok(mv);
        //drop can be anywhere, special cases are already handled by the DestinationOccupied and
        //NoMoveAfterDrop checks
    }
    let full_move: Movement = mv.parse().unwrap();
    let start = full_move.start.unwrap();
    let piece = check_position(full_move.start.unwrap(), b.clone()).unwrap();
    if !piece.get_relative_moves().into_iter().any(|relative_move| {
        (relative_move.0 as i32, relative_move.1 as i32)
            == (
                (full_move.end.0 as i32 % 9 - start.0 as i32 % 9),
                (full_move.end.0 as i32 / 9 - start.0 as i32 / 9),
            )
    }) {
        return Err(InvalidMoveError::PieceHasNoSuchMoveError);
    }
    if full_move.piecetype == PieceType::Rook
        && !small_move(start, full_move.end)
        && !check_rook_path(start, full_move.end, b.clone())
    {
        return Err(InvalidMoveError::PieceHasNoSuchMoveError);
    }
    if full_move.piecetype == PieceType::Bishop
        && !check_bishop_path(start, full_move.end, b.clone())
        && !small_move(start, full_move.end)
    {
        return Err(InvalidMoveError::PieceHasNoSuchMoveError);
    }
    if full_move.piecetype == PieceType::Lance
        && !check_lance_path(start, full_move.end, b.clone())
        && !small_move(start, full_move.end)
    {
        return Err(InvalidMoveError::PieceHasNoSuchMoveError);
    }
    return Ok(mv);
}

///return true if the path is clear, false if a piece is blocking the way
pub fn check_bishop_path(start: Position, end: Position, b: Board) -> bool {
    let direction = if (end.0 as i32 - start.0 as i32) > 0 {
        if (end.0 as i32 - start.0 as i32) % 8 == 0 {
            8
        } else {
            10
        }
    } else {
        if (start.0 as i32 - end.0 as i32) % 8 == 0 {
            -8
        } else {
            -10
        }
    };
    let mut counter = start.0 as i32 + direction;
    while counter != end.0 as i32 {
        if !(None == check_position(Position(counter as u16), b.clone())) {
            return false;
        }
        counter += direction;
    }
    return true;
}
///return true if the path is clear, false if a piece is blocking the way
pub fn check_rook_path(start: Position, end: Position, b: Board) -> bool {
    let direction;
    if start.column() == end.column() {
        //vertical move
        direction = if end.0 > start.0 { 9 } else { -9 };
    } else {
        //horizontal move
        direction = if end.0 > start.0 { 1 } else { -1 };
    }
    let mut counter = start.0 as i32 + direction;
    while counter != end.0 as i32 {
        if !(None == check_position(Position(counter as u16), b.clone())) {
            return false;
        }
        counter += direction;
    }
    return true;
}
///return true if the path is clear, false if a piece is blocking the way
pub fn check_lance_path(start: Position, end: Position, b: Board) -> bool {
    let direction = if end.0 > start.0 { 9 } else { -9 };
    let mut counter = start.0 as i32 + direction;
    while counter != end.0 as i32 {
        if !(None == check_position(Position(counter as u16), b.clone())) {
            return false;
        }
        counter += direction;
    }
    return true;
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

pub fn check_promotion(mv: &str, b: Board) -> Result<&str, InvalidMoveError> {
    //TODO :proper test
    if !maybe_normal_move(mv) {
        //move not a normal move but a drop so no check
        return Ok(mv);
    }
    let last_row;
    let before_last_row;
    let third_row;
    if b.get_color() == Color::White {
        last_row = 'i';
        before_last_row = 'h';
        third_row = 'g';
    } else {
        last_row = 'a';
        before_last_row = 'b';
        third_row = 'c';
    }
    let full_move: Movement = mv.parse().unwrap();

    //cannot promote gold or king
    if full_move.promotion
        && (full_move.piecetype == PieceType::Gold || full_move.piecetype == PieceType::King)
    {
        return Err(InvalidMoveError::PromotionError);
    }

    if full_move.promotion {
        //promotion is asked
        if (full_move.end.row() != last_row
            && full_move.end.row() != before_last_row
            && full_move.end.row() != third_row)
            && (full_move.start.unwrap().row() != last_row
                && full_move.start.unwrap().row() != before_last_row
                && full_move.start.unwrap().row() != third_row)
        {
            return Err(InvalidMoveError::PromotionError);
        }
        return Ok(mv);
    } else {
        //promotion not asked
        if let Some(piece) = check_position(full_move.start.unwrap(), b) {
            if !piece.promoted {
                if (full_move.piecetype == PieceType::Pawn && full_move.end.row() == last_row)
                    || (full_move.piecetype == PieceType::Lance && full_move.end.row() == last_row)
                    || (full_move.piecetype == PieceType::Knight
                        && (full_move.end.row() == last_row
                            || full_move.end.row() == before_last_row))
                {
                    return Err(InvalidMoveError::PromotionError);
                }
            }
        }
        return Ok(mv);
    }
}

///allow to uncover check, else consider the move invalid if it uncovers a check and do not take
///the opponent king
pub fn check_uncover_check(mv: &str, b: Board) -> Result<&str, InvalidMoveError> {
    if b.rules.can_uncover_check {
        return Ok(mv);
    }

    //implement real mechanic here
    let my_color = b.get_color();
    let mut opponent_color;
    {
        opponent_color = my_color.clone();
        opponent_color.invert();
    }
    if !b.contains(PieceType::King, opponent_color) {
        return Ok(mv); // we are just taking the opponent King so nothing else to check
    }

    let board_after_my_move = b.play_move_unchecked(mv);
    for opponent_move in board_after_my_move.iter_moves_partial_check() {
        let future_board = board_after_my_move.play_move_unchecked(&opponent_move);
        if !future_board.contains(PieceType::King, my_color) {
            //opponent has taken our king
            return Err(InvalidMoveError::UncoverCheckError);
        }
    }

    return Ok(mv);
}

pub fn check_checkmate_by_pawn_drop(mv: &str, b: Board) -> Result<&str, InvalidMoveError> {
    let full_move: Movement = mv.parse().unwrap();
    //ok if not a pawn or if not a drop
    if !(None == full_move.start) || full_move.piecetype != PieceType::Pawn {
        return Ok(mv);
    }

    let board_after_my_move = b.play_move_unchecked(mv);
    if board_after_my_move.game_over() {
        return Err(InvalidMoveError::CheckmateByPawnDropError);
    } else {
        Ok(mv)
    }
}
