use crate::piece::*;
use crate::position::*;
use std::fmt;

use std::str::FromStr;
/// respect the standard notation, found at https://en.wikipedia.org/wiki/Shogi_notation#Piece
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Movement {
    pub piecetype: PieceType,
    pub start: Option<Position>,
    pub end: Position,
    pub promotion: bool,
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
            let mv = [self.start.unwrap().to_string(), self.end.to_string()].join("-");
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
        let mut fs: Vec<char> = s
            .chars()
            .filter(|&c| !(c >= 'A' && c <= 'Z') && c != '-' && c != 'x')
            .collect();
        // moving a piece across the board
        if fs[0] != '*' {
            if *fs.last().unwrap() == '+' {
                pr = true;
            }

            let s1: String = fs[0..2].iter().collect();
            let s2: String = fs[2..4].iter().collect();
            let p1: Position = s1.parse().unwrap();
            let p2: Position = s2.parse().unwrap();

            Ok(Movement {
                piecetype: piecetype,
                start: Some(p1),
                end: p2,
                promotion: pr,
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
                    };

                    let s = mv.clone().to_string();
                    assert_eq!(mv, s.parse::<Movement>().unwrap());
                }
            }
        }
    }
}
