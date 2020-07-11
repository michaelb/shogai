mod ai;
mod board;
mod invalidmoveerror;
mod movement;
mod piece;
mod position;

fn main() {
    // example of using the program
    let mut b = board::Board::new();

    loop {
        println!("");
        println!("{:?} turn", b.get_color());
        println!("{}", b);

        let mv;
        if b.get_turn() {
            mv = ai::get_move_from_human(&b);
        } else {
            mv = ai::greedy(&b);
        }

        println!("{:?} has chosen the move: {}", b.get_color(), mv);
        b = b.play_move_unchecked(&mv);
        if b.game_over() {
            println!("{:?} has lost the game!", b.get_color());
            println!("final disposition of the board is \n{}", b);

            break;
        }
    }
}
