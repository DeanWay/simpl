use super::types::WordleGameState;
pub mod narrowing_random;
pub mod random;
pub trait SolverStrategy {
    fn next_guess(&mut self, game_state: &WordleGameState) -> String;
}
