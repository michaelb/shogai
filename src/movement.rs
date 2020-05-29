use crate::invalidmoveerror::*;
use crate::piece::*;
use crate::position::*;
use std::fmt;

use std::str::FromStr;
///!respect the standard notation, found at https://en.wikipedia.org/wiki/Shogi_notation#Piece
///!(see: Western notation)
///!However, origin must always be written!
///! Examples:
///!Moving a pawn for square 1g to square 1f is written "P1g-1f"
///!If a opponent's piece is taken, the move can be written as P1gx1f. This will ensure an extra
///!check to make sure there is a opponent piece there
///!If the piece is to be promoted, the move should be written "P4d-4c+" ('+' at the en of the move)
///!The promotion status may be provided anytime but will trigger the check if promotion is
///!requested but the piece does not fulfill conditions to be promoted, or if promotion is
///!mandatory but the promotion was not requested
///! No extra + must be provided to move a promoted pawn after the promotion. No extra '=' must
///!(though it can) be provided if the piece can be promoted but the player choose not to
///!An example of a drop is written "P*3e"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Movement {
    pub piecetype: PieceType,
    pub start: Option<Position>,
    pub end: Position,
    pub promotion: bool,
    pub force_capture: bool,
}

impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.start == None {
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
    // probably panic if given a wrong format
    type Err = String;
    fn from_str(s: &str) -> Result<Movement, String> {
        let piecetype: PieceType = s.chars().next().unwrap().to_string().parse().unwrap();
        let mut pr = false;
        let fs: Vec<char> = s
            .chars()
            .filter(|&c| !(c >= 'A' && c <= 'Z') && c != '-' && c != 'x')
            .collect();
        // moving a piece across the board
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
}
