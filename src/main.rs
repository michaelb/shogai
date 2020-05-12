mod board;
mod piece;
mod position;

fn main() {
    let b = board::Board::new();
    println!("{}", b);
}
