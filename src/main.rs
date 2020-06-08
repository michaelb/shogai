mod ai;
mod board;
mod invalidmoveerror;
mod movement;
mod piece;
mod position;

use std::io;
use std::io::*;

fn main() {
    game();
}

fn game() {
    let mut b5 = board::Board::new();

    loop {
        println!("");
        println!("{:?} turn", b5.get_color());
        println!("{}", b5);

        let mv;
        if b5.get_color() == piece::Color::White {
            // mv = get_move_from_human(b5.clone());
            mv = ai::greedy(b5.clone());
        } else {
            // mv = get_move_from_human(b5.clone());
            mv = ai::greedy(b5.clone());
        }

        println!("{:?} has chosen the move: {}", b5.clone().get_color(), mv);
        b5 = b5.play_move_unchecked(&mv);
        // if b5.game_over() {
        //     println!("{:?} has lost the game!", b5.get_color());
        //     println!("final disposition of the board is \n{}", b5);
        //
        //     break;
        // }
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
    // return input;
}
