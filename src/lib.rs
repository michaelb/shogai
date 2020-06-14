//! Shogai is a rust shogi helper. It can handle the game mechanic and even provides a small cli
//! interface, as well as a very simple (and stupid) bot

//! # Examples
//!
//! imports :
//! ```
//! //minimum required to make work the following
//! use shogai::board::Board;
//! use shogai::ai::{greedy, get_move_from_human};
//! // complete, may allow finner control over move-checking or allow easy situation generation
//! use shogai::board::*;
//! use shogai::piece::*;
//! use shogai::invalidmoveerror::*;
//! ```
//!
//!
//! Playing against the ai:
//! ```no_run
//! # use shogai::board::Board;
//! # use shogai::ai::{greedy, get_move_from_human};
//!let mut b = Board::new();
//!
//!loop {
//!    println!("");
//!    println!("{:?} turn", b.get_color());
//!    println!("{}", b);
//!
//!    let mv: String;
//!    if b.get_turn() {
//!        mv = get_move_from_human(&b);
//!    } else {
//!        mv = greedy(&b);
//!    }
//!
//!    println!("{:?} has chosen the move: {}", b.get_color(), mv);
//!    b = b.play_move_unchecked(&mv); //because checks are done within greedy and get_human_move
//!    if b.game_over() {
//!        println!("{:?} has lost the game!", b.get_color());
//!        println!("final disposition of the board is \n{}", b);
//!
//!        break;
//!    }
//!}
//!```
//!
//! Help a human player choose a move (filter illegal moves):
//! ```
//! # use shogai::board::Board;
//! # use shogai::ai::{greedy, get_move_from_human};
//! let mut b = Board::new();
//! println!("{}", b);
//! println!("choose within : {:?}", b.iter_moves().collect::<Vec<_>>());
//! println!("Type in your move:");
//! //...read user input... or use ai::get_move_from_human
//!
//!```
//!
//! Play pre-determined moves:
//! ```
//! # use shogai::board::Board;
//! # use shogai::ai::{greedy, get_move_from_human};
//! let first_player_move = "P9c-9d";
//! let second_player_move = "K5i-5h";
//!
//! let mut b = Board::new();
//! println!("{}", b);
//! b = b.play_move(&first_player_move);
//! println!("{}", b);
//! //equivalent
//! assert!(b.check_move(&second_player_move).is_ok());
//! b.play_move_unchecked(&second_player_move);
//! println!("{}", b);
//! ```
//!

#[warn(missing_docs)]
#[warn(missing_doc_code_examples)]
/// Provide user with a way to generate moves
pub mod ai;
/// Manage and manipulate information about the shogi board (shogiban) state
pub mod board;
/// Check a move against a board, to ensure legality. Different functions are used to check
/// differents types of incorrect moves.
pub mod invalidmoveerror;
/// Handles the conversion from string literal to computer-understandable movement structure
///respect the [standard notation](https://en.wikipedia.org/wiki/Shogi_notation#Piece)
///(see: Western notation);
///
///For quick reminder:
///
///normal move [Piece type][start]-[end][optionnal promotion]
///
///drop        [Piece type]*[end]
///
///However, origin must always be written! (implicit start position not allowed)
///
///
///If a opponent's piece is taken, the move *can*, *optionnaly*, be written as (eg) P1gx1f (note the x instead of the -). This will ensure an extra
///check to make sure there is a opponent piece there.
///
///If the piece is to be promoted, the move should be written "P4d-4c+" ('+' at the end of the
///move).
///The promotion status may be provided anytime but will trigger the check (and panic) if promotion is
///requested but the piece does not fulfill conditions to be promoted, or if promotion is
///mandatory but the promotion was not requested.
/// No extra + is required to move a promoted pawn after the promotion. No extra '=' must
///be provided if the piece can be promoted but the player choose not to.
///
///# Examples
///Dropping a pawn at position 3e: "P*3e"
///
///
///Moving the king from 5i to 4h : "K5i-4h"
///
///Taking an opponent pawn with a Lance: "L9ax9f"
///
///Also taking an opponent pawn with a Lance: "L9a-9f"
///
pub mod movement;
/// Represent shogi pieces
pub mod piece;
/// Represent square of the shogiban
pub mod position;
