mod board;
mod invalidmoveerror;
mod movement;
mod piece;
mod position;

fn main() {
    let b = board::Board::new();
    println!("{}", b);
}
