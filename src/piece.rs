use crate::position::Position;
use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Piece {
    pub color: Color,
    pub piecetype: PieceType,
    pub promoted: bool,
    pub position: Option<Position>, // if a piece is not on the board, it can NOT be promoted
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Color {
    Black,
    White,
}
impl Color {
    fn invert(&mut self) {
        if *self == Color::Black {
            *self = Color::White;
        } else {
            *self = Color::Black;
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
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

impl Piece {
    fn move_to(&mut self, dest_square: Position) {
        self.position = Some(dest_square);
    }

    fn remove(&mut self) {
        self.color.invert();
        self.position = None;
    }
}
impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        let symbol = match &self {
            PieceType::Rook => "R",
            PieceType::Pawn => "P",
            PieceType::Bishop => "B",
            PieceType::Gold => "G",
            PieceType::Silver => "S",
            PieceType::Knight => "N",
            PieceType::Lance => "L",
            PieceType::King => "K",
        };
        write!(f, "{}", symbol)
    }
}

impl FromStr for PieceType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "R" => Ok(PieceType::Rook),
            "P" => Ok(PieceType::Pawn),
            "B" => Ok(PieceType::Bishop),
            "G" => Ok(PieceType::Gold),
            "S" => Ok(PieceType::Silver),
            "N" => Ok(PieceType::Knight),
            "L" => Ok(PieceType::Lance),
            "K" => Ok(PieceType::King),
            _ => Err(String::from("not a valid piece type")),
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let symbol = match &self.piecetype {
            PieceType::Rook => "R",
            PieceType::Pawn => "P",
            PieceType::Bishop => "B",
            PieceType::Gold => "G",
            PieceType::Silver => "S",
            PieceType::Knight => "N",
            PieceType::Lance => "L",
            PieceType::King => "K",
        };
        if self.promoted {
            write!(f, "+{}", symbol)
        } else {
            write!(f, " {}", symbol)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::piece::*;
    use crate::position::*;

    #[test]
    fn move_check() {
        for i in 0..80 {
            for j in 0..80 {
                let a = Position(i);
                let b = Position(j);
                let c = Position(j);
                let mut p1 = Piece {
                    color: Color::Black,
                    piecetype: PieceType::Pawn,
                    promoted: false,
                    position: Some(a),
                };
                let p2 = Piece {
                    color: Color::Black,
                    piecetype: PieceType::Pawn,
                    promoted: false,
                    position: Some(b),
                };

                p1.move_to(c);
                assert_eq!(p1, p2);
            }
        }
    }

    #[test]
    fn remove_check() {
        for i in 0..80 {
            let a = Position(i);
            let mut p1 = Piece {
                color: Color::Black,
                piecetype: PieceType::Pawn,
                promoted: false,
                position: Some(a),
            };
            p1.remove();
            assert_eq!(p1.position, None);
            assert_eq!(p1.color, Color::White);
        }
    }

    #[test]
    fn simpletostring() {
        let p1: PieceType = PieceType::Pawn;
        let _s1 = p1.to_string();
    }
}
