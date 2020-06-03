use crate::board::*;
use crate::movement::*;
/// return the best move for a greedy algorithm
pub fn greedy(b: Board) -> String {
    b.iter_moves()
        .max_by_key(|mv| {
            -b.play_move_unchecked(&mv).value() + {
                if mv.as_bytes()[1] == b'*' {
                    2000
                } else {
                    0
                }
            }
        })
        .unwrap()
}
