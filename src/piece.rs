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
    pub fn invert(&mut self) {
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
    /// for test purposes only
    fn move_to(&mut self, dest_square: Position) {
        self.position = Some(dest_square);
    }

    ///for test purposes only
    fn remove(&mut self) {
        self.color.invert();
        self.position = None;
    }
    ///get relatives moves of a piece: but do not check if piece have to 'jump' over other pieces
    pub fn get_relative_moves(&self) -> Vec<i32> {
        //beware to check that moves  not within a column make not the piece's "wrap around" the board in case the move is next to the border
        //
        //check position % 9 == (position + relat_mov%9)%9

        //also, whether the piece has to jump over other piece is not checked!
        let mut possibles_moves: Vec<i32> = match &self.piecetype {
            PieceType::Pawn => vec![9],
            PieceType::King => vec![1, -1, 10, 8, 9, -9, -8, -10],
            PieceType::Rook => vec![
                -8, -7, -6, -5, -4, -3, -2, -1, 1, 2, 3, 4, 5, 6, 7, 8, 9, -9, 18, -18, 27, -27,
                -36, 36, 45, -45, 63, -63, -72, 72,
            ],
            PieceType::Bishop => vec![
                10, 20, 30, 40, 50, 60, 70, 80, -10, -20, -30, -40, -50, -60, -70, -80, 8, 16, 24,
                32, 40, 48, 56, 64, -8, -16, -24, -32, -40, -48, -56, -64,
            ],
            PieceType::Gold => vec![1, -1, 8, 9, 10, -9],
            PieceType::Silver => vec![8, 9, 10, -8, -10],
            PieceType::Knight => vec![17, 19],
            PieceType::Lance => vec![9, 18, 27, 36, 45, 54, 63, 72],
        };

        //manage promotion
        if self.promoted {
            if self.piecetype == PieceType::Pawn
                || self.piecetype == PieceType::Lance
                || self.piecetype == PieceType::Knight
                || self.piecetype == PieceType::Lance
            {
                possibles_moves = vec![1, -1, 8, 9, 10, -9]
            } else {
                possibles_moves.append(&mut vec![1, -1, 8, 9, 10, -8, -9, -10]);
                possibles_moves.sort();
                possibles_moves.dedup();
            }
        }

        let possibles_moves_colored;
        if self.color == Color::White {
            possibles_moves_colored = possibles_moves;
        } else {
            possibles_moves_colored = possibles_moves.iter().map(|m| -m).collect::<Vec<i32>>();
        }

        //make those board-aware (see first comment)
        return possibles_moves_colored
            .into_iter()
            .filter(|&mv| {
                self.position.unwrap().0 as i32 % 9
                    == (self.position.unwrap().0 as i32 + mv % 9) % 9
            })
            .collect();
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
        let symbol = &self.piecetype.to_string();
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
