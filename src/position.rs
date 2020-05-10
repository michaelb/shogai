pub struct Position(u16); //but must range between 0 and 80
//(81 position for 9*9 grid)

impl ToString for Position {
    fn to_string(&self) -> String {
        let y_axis = self.0 % 9;
        let x_axis = (self.0 / 9 + 'a' as u16) as char;
        [x_axis.to_string(), y_axis.to_string()].join("")
    }
}

impl Position {
