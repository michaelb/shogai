mod board;
mod invalidmoveerror;
mod movement;
mod piece;
mod position;

fn main() {
    let mut b = board::Board::empty();
    let p1: position::Position = "1f".parse().unwrap();
    b.add_piece(piece::Piece {
        color: piece::Color::White,
        piecetype: piece::PieceType::Pawn,
        promoted: false,
        position: Some(p1),
    });

    println!("{}", b);
    println!("{}", b.play_move("P1f-1g"));

    let b5 = board::Board::new();
    println!("{}", b5);
}
