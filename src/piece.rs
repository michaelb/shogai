pub struct Piece {
    color: Color,
    piecetype: PieceType,
    promoted: bool,
    position: Option<Position>,
}

pub enum Color {
    Black,
    White,
}

pub enum PieceType {
    Pawn,
    King,
    Rook,
    Bishop,
    Gold,
    Silver,
    Knight,
    Lance,
}
