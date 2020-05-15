use crate::position::*;

use std::str::FromStr;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Movement {
    pub start: Position,
    pub end: Position,
    pub promotion: bool,
}

impl ToString for Movement {
    fn to_string(&self) -> String {
        let mv = [self.start.to_string(), self.end.to_string()].join("-");
        if self.promotion {
            [mv, "+".to_string()].join("")
        } else {
            mv
        }
    }
}

impl FromStr for Movement {
    // probably panic if given a wrong format
    type Err = String;
    fn from_str(s: &str) -> Result<Movement, String> {
        let mut pr = false;
        let mut fs: Vec<char> = s.chars().filter(|&c| !(c >= 'A' && c <= 'Z')).collect();
        if *fs.last().unwrap() == '+' {
            pr = true;
            fs.pop();
        }
        let s1: String = fs[0..2].iter().collect();
        let s2: String = fs[2..4].iter().collect();
        let p1: Position = s1.parse().unwrap();
        let p2: Position = s2.parse().unwrap();
        Ok(Movement {
            start: p1,
            end: p2,
            promotion: pr,
        })
    }
}

#[cfg(test)]
mod test {

    use crate::movement::*;
    #[test]
    fn convertmoves() {
        for &pr in &[true, false] {
            for start in (0..80) {
                for end in (0..80) {
                    let mv = Movement {
                        start: Position(start),
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
