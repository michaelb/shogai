use crate::position::Position;
use std::fmt;
use std::str::FromStr;

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
pub struct Piece {
    pub color: Color,
    pub piecetype: PieceType,
    pub promoted: bool,
    pub position: Option<Position>, // if a piece is not on the board, it can NOT be promoted
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
///though there is not really "color" in shogi, it is simpler
///to think and visualize with this
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

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
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
    ///I use the piece valuation from YSS 7.0 (1997), but scaled x100 to be integers. values of
    pub fn value(&self) -> i32 {
        match self.piecetype {
            PieceType::Pawn => {
                if self.position == None {
                    115
                } else if self.promoted {
                    420
                } else {
                    100
                }
            }
            PieceType::Lance => {
                if self.position == None {
                    480
                } else if self.promoted {
                    630
                } else {
                    430
                }
            }
            PieceType::Knight => {
                if self.position == None {
                    510
                } else if self.promoted {
                    640
                } else {
                    450
                }
            }
            PieceType::Silver => {
                if self.position == None {
                    720
                } else if self.promoted {
                    670
                } else {
                    640
                }
            }
            PieceType::Gold => {
                if self.position == None {
                    780
                } else {
                    690
                }
            }
            PieceType::Rook => {
                if self.position == None {
                    1270
                } else if self.promoted {
                    1300
                } else {
                    1040
                }
            }
            PieceType::Bishop => {
                if self.position == None {
                    1110
                } else if self.promoted {
                    1150
                } else {
                    890
                }
            }
            PieceType::King => 20126, // max value of all other pieces combined
        }
    }

    ///get relatives moves of a piece: but do not check if piece have to 'jump' over other pieces
    pub fn get_relative_moves(&self) -> Vec<(i16, i16)> {
        //beware to check that moves  not within a column make not the piece's "wrap around" the board in case the move is next to the border
        //

        //also, whether the piece has to jump over other piece is not checked!
        let mut possibles_moves: Vec<(i16, i16)> = match &self.piecetype {
            PieceType::Pawn => vec![(0, 1)],
            PieceType::King => vec![
                (1, 0),
                (-1, 0),
                (1, 1),
                (-1, 1),
                (0, 1),
                (0, -1),
                (1, -1),
                (-1, -1),
            ],
            PieceType::Rook => vec![
                (-8, 0),
                (-7, 0),
                (-6, 0),
                (-5, 0),
                (-4, 0),
                (-3, 0),
                (-2, 0),
                (-1, 0),
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (5, 0),
                (6, 0),
                (7, 0),
                (8, 0),
                (0, 1),
                (0, -1),
                (0, 2),
                (0, -2),
                (0, 3),
                (0, -3),
                (0, 4),
                (0, -4),
                (0, 5),
                (0, -5),
                (0, 6),
                (0, -6),
                (0, 7),
                (0, -7),
                (0, 8),
                (0, -8),
            ],
            PieceType::Bishop => vec![
                (1, 1),
                (2, 2),
                (3, 3),
                (4, 4),
                (5, 5),
                (6, 6),
                (7, 7),
                (8, 8),
                (-1, 1),
                (-2, 2),
                (-3, 3),
                (-4, 4),
                (-5, 5),
                (-6, 6),
                (-7, 7),
                (-8, 8),
                (1, -1),
                (2, -2),
                (3, -3),
                (4, -4),
                (5, -5),
                (6, -6),
                (7, -7),
                (8, -8),
                (-1, -1),
                (-2, -2),
                (-3, -3),
                (-4, -4),
                (-5, -5),
                (-6, -6),
                (-7, -7),
                (-8, -8),
            ],
            PieceType::Gold => vec![(1, 0), (-1, 0), (-1, 1), (0, 1), (1, 1), (0, -1)],
            PieceType::Silver => vec![(-1, 1), (0, 1), (1, 1), (1, -1), (-1, -1)],
            PieceType::Knight => vec![(-1, 2), (1, 2)],
            PieceType::Lance => vec![
                (0, 1),
                (0, 2),
                (0, 3),
                (0, 4),
                (0, 5),
                (0, 6),
                (0, 7),
                (0, 8),
            ],
        };

        //manage promotion
        if self.promoted {
            if self.piecetype == PieceType::Pawn
                || self.piecetype == PieceType::Knight
                || self.piecetype == PieceType::Lance
                || self.piecetype == PieceType::Silver
            {
                possibles_moves = vec![(1, 0), (-1, 0), (-1, 1), (0, 1), (1, 1), (0, -1)];
            } else if self.piecetype == PieceType::Rook {
                possibles_moves.append(&mut vec![(1, 1), (-1, 1), (1, -1), (-1, -1)]);
            } else {
                // bishop only one left, beacuse gold and king cannot be promoted
                possibles_moves.append(&mut vec![(1, 0), (-1, 0), (0, 1), (0, -1)]);
            }
        }

        let mut possibles_moves_colored;
        if self.color == Color::White {
            possibles_moves_colored = possibles_moves;
        } else {
            possibles_moves_colored = possibles_moves
                .iter()
                .map(|(m, n)| (-m, -n))
                .collect::<Vec<(i16, i16)>>();
        }

        possibles_moves_colored.shuffle(&mut thread_rng());

        //make those board-aware (see first comment)
        return possibles_moves_colored
            .into_iter()
            .filter(|&mv| {
                (self.position.unwrap().0 as i32 + mv.0 as i32) / 9
                    == self.position.unwrap().0 as i32 / 9
                    && (self.position.unwrap().0 as i32 + (mv.0 + 9 * mv.1) as i32) >= 0
                    && (self.position.unwrap().0 as i32 + (mv.0 + 9 * mv.1) as i32) <= 80
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
    /// display on terminal the piece, black pieces are colored in red and white as default color
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let symbol = &self.piecetype.to_string();
        let colorcodeprefix = match &self.color {
            Color::White => "",
            Color::Black => "\x1b[0;31m",
        };
        let colorcodesuffix = match &self.color {
            Color::White => "",
            Color::Black => "\x1b[0m",
        };
        if self.promoted {
            write!(f, "{}+{}{}", colorcodeprefix, symbol, colorcodesuffix)
        } else {
            write!(f, "{} {}{}", colorcodeprefix, symbol, colorcodesuffix)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::piece::*;
    use crate::position::*;
    /// for test purposes only
    fn move_to(p: &mut Piece, dest_square: Position) {
        p.position = Some(dest_square);
    }
    ///for test purposes only
    fn remove(p: &mut Piece) {
        p.color.invert();
        p.position = None;
    }

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

                move_to(&mut p1, c);
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
            remove(&mut p1);
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
