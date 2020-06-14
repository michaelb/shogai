use crate::board::*;
use crate::movement::*;
use crate::*;
use std::io::*;

/// return the best move for a greedy algorithm
pub fn greedy(b: &Board) -> String {
    b.iter_moves()
        .max_by_key(|mv| {
            //prefer drops though or else the bot will never drop any pieces
            -b.play_move_unchecked(&mv).value() + {
                if mv.parse::<Movement>().unwrap().start == None {
                    1000
                } else {
                    0
                }
            }
        })
        .unwrap()
}

/// ask user for input from the terminal
pub fn get_move_from_human(b: &board::Board) -> String {
    let mut input = String::new();
    print!("Type in your move:");
    let _ = stdout().flush();
    stdin()
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
        return get_move_from_human(&b);
    } else {
        return input;
    }
    // return input;
}
