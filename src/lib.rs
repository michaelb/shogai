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
//!    b = b.play_move_unchecked(&mv); //because check are done within ai and get_human_mvoe
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
/// provide user with a way to generate moves
pub mod ai;
pub mod board;
pub mod invalidmoveerror;
pub mod movement;
pub mod piece;
pub mod position;
