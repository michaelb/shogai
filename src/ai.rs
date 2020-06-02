use crate::board::*;
/// return the best move for a greedy algorithm
pub fn greedy(b: Board) -> String {
    b.iter_moves()
        .max_by_key(|mv| -b.play_move(&mv).value())
        .unwrap()
}
