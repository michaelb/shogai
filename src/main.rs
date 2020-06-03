mod ai;
mod board;
mod invalidmoveerror;
mod movement;
mod piece;
mod position;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;
use std::io::*;
use std::thread::sleep;
use std::time;

fn main() {
    let mut b5 = board::Board::empty();
    // b5.set(piece::Color::White);
    b5.add_piece(piece::Piece {
        color: piece::Color::White,
        piecetype: piece::PieceType::Bishop,
        promoted: false,
        position: Some(position::Position(0)),
    });
    b5.add_piece(piece::Piece {
        color: piece::Color::White,
        piecetype: piece::PieceType::Bishop,
        promoted: false,
        position: Some(position::Position(70)),
    });
    loop {
        println!("");
        println!("{:?} turn", b5.get_color());
        println!("{}", b5);
        b5.piece_set.shuffle(&mut thread_rng());

        let mv;
        b5.turn = piece::Color::White;
        mv = get_move_from_human(b5.clone());
        if b5.get_color() == piece::Color::White {
            // mv = get_move_from_human(b5.clone());
            // mv = ai::greedy(b5.clone());
            // mv = b5.iter_moves().next().unwrap();
        } else {
            // mv = ai::greedy(b5.clone());
        }
        println!("len: {}", b5.iter_moves().count());

        println!("{:?} has chosen the move: {}", b5.clone().get_color(), mv);
        b5 = b5.play_move_unchecked(&mv);
        // if b5.game_over() {
        //     println!("{:?} has lost the game!", b5.get_color());
        //     println!("final disposition of the board is \n{}", b5);
        //
        //     break;
        // }
        // let some_time = time::Duration::from_millis(100);
        // sleep(some_time);
    }
}

fn get_move_from_human(b: board::Board) -> String {
    let mut input = String::new();
    print!("Type in your move:");
    let _ = stdout().flush();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }
    if let Err(e) = b.check_move(&input) {
        println!("Not a valid move: {}", e);
        return get_move_from_human(b);
    } else {
        return input;
    }
}
