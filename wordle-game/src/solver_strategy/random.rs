use super::SolverStrategy;
use crate::types::WordleGameState;
use rand::seq::SliceRandom;
pub struct RandomWordleSolver {
    dictionary: Vec<String>,
}

impl RandomWordleSolver {
    pub fn new(dictionary: Vec<String>) -> Self {
        Self { dictionary }
    }
}

impl SolverStrategy for RandomWordleSolver {
    fn next_guess(&mut self, _: &WordleGameState) -> String {
        self.dictionary
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone()
    }
}
