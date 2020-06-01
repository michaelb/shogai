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
        b5 = b5.play_move(&mv);
        println!("{}", mv);
        let one_sec = time::Duration::from_secs(1);
        sleep(one_sec);
    }
}
