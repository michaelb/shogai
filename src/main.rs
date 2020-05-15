mod board;
mod movement;
mod piece;
mod position;

fn main() {
    let b = board::Board::new();
    println!("{}", b);
    for &pr in &[true, false] {
        for start in (0..80) {
            for end in (0..80) {
                let mv = movement::Movement {
                    start: position::Position(start),
                    end: position::Position(end),
                    promotion: pr,
                };

                let s = mv.clone().to_string();
                assert_eq!(mv, s.parse::<movement::Movement>().unwrap());
            }
        }
    }
}
