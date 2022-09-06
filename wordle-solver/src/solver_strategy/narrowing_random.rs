#![allow(dead_code)]
use std::collections::HashSet;

use super::SolverStrategy;
use crate::{constraint, types::WordleGameState};
use rand::seq::IteratorRandom;

pub struct NarrowingRandomWordleSolver {
    dictionary: HashSet<String>,
}

impl NarrowingRandomWordleSolver {
    pub fn new(dictionary: &[&str]) -> Self {
        Self {
            dictionary: HashSet::from_iter(dictionary.iter().map(|s| s.to_string())),
        }
    }
}

impl SolverStrategy for NarrowingRandomWordleSolver {
    fn next_guess(&mut self, game_state: &WordleGameState) -> String {
        self.dictionary
            .iter()
            .filter(|word| constraint::word_matches(word, game_state))
            .choose(&mut rand::thread_rng())
            .expect("impossible to win if we run out of options")
            .to_string()
    }
}
