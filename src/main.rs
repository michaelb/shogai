mod board;
mod invalidmoveerror;
mod movement;
mod piece;
mod position;

use std::thread::sleep;
use std::time;

fn main() {
    let mut b5 = board::Board::new().play_move("K5a-4b");

    loop {
        println!("");
        println!("{:?} turn", b5.get_color());
        println!("{}", b5);
        let mv = b5.iter_moves().next().unwrap();
        println!("{:?} has chosen the move: {}", b5.get_color(), mv);
        b5 = b5.play_move(&mv);
        if b5.game_over() {
            println!("{:?} has lost the game!", b5.get_color());
            println!("final disposition of the board is \n{}", b5);

            break;
        }
        let some_time = time::Duration::from_millis(100);
        sleep(some_time);
    }
}
