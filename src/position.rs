use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Position(pub u16); //but must range between 0 and 80
                              //(81 position for 9*9 grid)

impl Position {
    pub fn row(&self) -> char {
        return ((self.0 / 9) as u8 + 'a' as u8) as char;
    }
    pub fn column(&self) -> char {
        return ((self.0 % 9) as u8 + 1 + '1' as u8) as char;
    }
}

impl ToString for Position {
    fn to_string(&self) -> String {
        let x_axis = (self.0 % 9) + 1;
        let y_axis = ((self.0 / 9) as u8 + 'a' as u8) as char;
        [x_axis.to_string(), y_axis.to_string()].join("")
    }
}

impl FromStr for Position {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        if s.len() != 2 {
            return Err(String::from("Invalid number of characters in string"));
        }
        let v: Vec<&str> = s.split("").filter(|&m| m != "").take(2).collect();
        let (x, y) = (v[0], v[1]);
        let p = (x.parse::<u8>().unwrap() as u8 - 1)
            + (y.parse::<char>().unwrap() as u8 - 'a' as u8) * 9;
        Ok(Position(p as u16))
    }
}

#[cfg(test)]
mod test {
    use crate::position::*;

    #[test]
    fn pos_to_string() {
        let pos = Position(10);
        assert_eq!(pos.to_string(), "2b");

        let pos2 = Position(0);
        assert_eq!(pos2.to_string(), "1a");

        let pos3 = Position(80);
        assert_eq!(pos3.to_string(), "9i");
    }

    #[test]
    fn convertback() {
        for i in 0..80 {
            let p0 = Position(i);
            let p0ts = p0.to_string();
            let p1: Position = Position::from_str(&p0ts).unwrap();

            assert_eq!(p1, p0);
        }
    }
}
