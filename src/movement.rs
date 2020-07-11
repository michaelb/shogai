use crate::invalidmoveerror::*;
use crate::piece::*;
use crate::position::*;
use std::fmt;
use std::iter::once;

use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Movement {
    pub piecetype: PieceType,
    pub start: Option<Position>,
    pub end: Position,
    pub promotion: bool,
    pub force_capture: bool,
    pub withdraw: bool,
    pub restart: bool,
}

impl Movement {
    ///get possible absolute moves from pieces and relative movement
    pub fn from_relative(piece: &Piece, relative: (i16, i16)) -> impl Iterator<Item = String> {
        let move_non_promoting = Movement {
            piecetype: piece.piecetype,
            start: piece.position,
            end: Position(
                (piece.position.unwrap().0 as i32 + relative.0 as i32 + 9 * relative.1 as i32)
                    as u16,
            ),
            promotion: false,
            force_capture: false,
            withdraw: false,
            restart: false,
        };
        let move_promoting = Movement {
            piecetype: piece.piecetype,
            start: piece.position,
            end: Position(
                (piece.position.unwrap().0 as i32 + relative.0 as i32 + 9 * relative.1 as i32)
                    as u16,
            ),
            promotion: true,
            force_capture: false,
            withdraw: false,
            restart: false,
        };
        return once(move_promoting.to_string()).chain(once(move_non_promoting.to_string()));
    }
}

impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.restart {
            write!(f, "restart")
        } else if self.withdraw {
            write!(f, "withdraw")
        } else if self.start == None {
            //is a drop move
            write!(
                f,
                "{}",
                [
                    self.piecetype.to_string(),
                    String::from("*"),
                    self.end.to_string()
                ]
                .join("")
            )
        } else {
            //is a normal move
            let mut joiner = "-";
            if self.force_capture {
                joiner = "x";
            }
            let mv = [self.start.unwrap().to_string(), self.end.to_string()].join(joiner);
            if self.promotion {
                write!(
                    f,
                    "{}",
                    [self.piecetype.to_string(), mv, "+".to_string()].join("")
                )
            } else {
                write!(f, "{}", [self.piecetype.to_string(), mv].join(""))
            }
        }
    }
}

impl FromStr for Movement {
    // probably panic if given a wrong format, you better use check_move, or a part of it
    // before parsing. check_move is a bit heavy though
    type Err = String;
    fn from_str(s: &str) -> Result<Movement, String> {
        //special cases
        if s == "restart" {
            return Ok(Movement {
                piecetype: PieceType::Pawn,
                start: None,
                end: Position(0),
                promotion: false,
                force_capture: false,
                withdraw: false,
                restart: true,
            });
        }
        if s == "withdraw" {
            return Ok(Movement {
                piecetype: PieceType::Pawn,
                start: None,
                end: Position(0),
                promotion: false,
                force_capture: false,
                withdraw: true,
                restart: false,
            });
        }

        //parsing reals moves
        let piecetype: PieceType = s.chars().next().unwrap().to_string().parse().unwrap();
        let mut pr = false;
        let fs: Vec<char> = s
            .chars()
            .filter(|&c| !(c >= 'A' && c <= 'Z') && c != '-' && c != 'x')
            .collect();
        //
        // moving a piece across the board, not a drop
        if fs[0] != '*' {
            if *fs.last().unwrap() == '+' {
                pr = true;
            }
            let fc = match s.as_bytes()[3] {
                b'-' => false,
                b'x' => true,
                _ => panic!("{}", InvalidMoveError::MoveSyntaxError),
            };

            let s1: String = fs[0..2].iter().collect();
            let s2: String = fs[2..4].iter().collect();
            let p1: Position = s1.parse().unwrap();
            let p2: Position = s2.parse().unwrap();

            Ok(Movement {
                piecetype: piecetype,
                start: Some(p1),
                end: p2,
                promotion: pr,
                force_capture: fc,
                withdraw: false,
                restart: false,
            })
        } else {
            // drop movement
            let s1: String = fs[1..3].iter().collect();
            let p1: Position = s1.parse().unwrap();
            Ok(Movement {
                piecetype: piecetype,
                start: None,
                end: p1,
                promotion: pr,
                force_capture: false,
                withdraw: false,
                restart: false,
            })
        }
    }
}

#[cfg(test)]
mod test {

    use crate::movement::*;
    #[test]
    fn convertmoves() {
        for &pr in &[true, false] {
            for start in 0..80 {
                for end in 0..80 {
                    let mv = Movement {
                        piecetype: PieceType::Pawn,
                        start: Some(Position(start)),
                        end: Position(end),
                        promotion: pr,
                        force_capture: true,
                        withdraw: false,
                        restart: false,
                    };

                    let s = mv.clone().to_string();
                    assert_eq!(mv, s.parse::<Movement>().unwrap());
                }
            }
        }
    }

    #[test]
    fn testdrop() {
        let s = "P*8f".to_string();
        let mv = s.clone().parse::<Movement>().unwrap();
        let s2 = mv.to_string();
        assert_eq!(s, s2);
    }

    #[test]
    fn testrestart() {
        let s = "restart";
        let mv: Movement = s.clone().parse().unwrap();
        assert!(mv.restart);
        let s2 = mv.to_string();
        assert_eq!(s, s2);
    }
}
