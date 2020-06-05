use crate::board::*;
use crate::movement::*;
/// return the best move for a greedy algorithm
pub fn greedy(b: Board) -> String {
    b.iter_moves()
        .max_by_key(|mv| -b.play_move_unchecked(&mv).value())
        .unwrap()
}

// implements real ai down there
//
// use rurel::mdp::{Agent, State};
//
// impl State for Board {
//     type A = String;
//     fn reward(&self) -> f64 {
//         -self.value() as f64
//     }
//
//     fn actions(&self) -> Vec<String> {
//         let mut v = self.iter_moves().collect::<Vec<String>>();
//         println!("len v : {}", v.len());
//         if v.len() == 0 {
//             println!("{}", self);
//             println!("{:?}", self.get_color());
//         }
//         v.push(String::from("restart"));
//         v
//     }
// }
//
// struct MyAgent {
//     state: Board,
// }
//
// impl Agent<Board> for MyAgent {
//     fn current_state(&self) -> &Board {
//         &self.state
//     }
//     fn take_action(&mut self, action: &String) {
//         self.state = self.state.play_move(action);
//     }
// }
//
// use rurel::strategy::explore::RandomExploration;
// use rurel::strategy::learn::QLearning;
// use rurel::strategy::terminate::FixedIterations;
// use rurel::AgentTrainer;
//
// pub fn train() -> AgentTrainer<Board> {
//     let mut trainer = AgentTrainer::new();
//     let mut train_board = Board::new();
//     train_board.rules = Rules {
//         can_uncover_check: true,
//     };
//     let mut agent = MyAgent {
//         state: train_board.clone(),
//     };
//     trainer.train(
//         &mut agent,
//         &QLearning::new(0.2, 0.01, 500.),
//         &mut FixedIterations::new(999),
//         &RandomExploration::new(),
//     );
//     return trainer;
// }
//
// pub fn best_move<'a>(t: &AgentTrainer<Board>, b: &'a Board) -> String {
//     let mut best_score = 0;
//     let mut best_move = String::from("");
//     for mv in b.iter_moves() {
//         if let Some(score) = t.expected_value(b, &mv) {
//             println!("uwu");
//             if score as i64 >= best_score {
//                 best_score = score as i64;
//                 best_move = mv.to_string();
//             }
//         }
//     }
//     return best_move;
// }
