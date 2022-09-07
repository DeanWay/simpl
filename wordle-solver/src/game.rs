use crate::types::{GameCondition, Guess, Guesses, LetterState, WordleGameState};
use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};

pub struct WordleGame {
    dictionary: HashSet<String>,
    guesses: Guesses,
    secret_word: String,
    max_guesses: usize,
}

impl WordleGame {
    pub fn new(dictionary: &[&str], secret_word: &str) -> Self {
        let dictionary_set = HashSet::from_iter(dictionary.iter().map(|s| s.to_string()));
        let secret_word = secret_word.to_owned();
        assert!(dictionary_set.contains(&secret_word));

        Self {
            dictionary: dictionary_set,
            guesses: vec![],
            secret_word,
            max_guesses: 6,
        }
    }

    pub fn new_with_random_secret_word(dictionary: &[&str]) -> Self {
        let secret_word = dictionary.choose(&mut rand::thread_rng()).unwrap().clone();
        Self::new(dictionary, &secret_word)
    }

    pub fn make_guess(&mut self, guess: &str) -> Result<(), &'static str> {
        if !self.dictionary.contains(guess) {
            return Err("Invalid word");
        }
        if self.game_condition() != GameCondition::Playing {
            return Err("game is over");
        }
        if self.words_already_guessed().contains(&guess.to_string()) {
            return Err("Already guessed");
        }
        let guess_result = Self::check_guess(guess, &self.secret_word);
        self.guesses.push(guess_result);
        Ok(())
    }

    pub fn game_state(&self) -> WordleGameState {
        WordleGameState {
            guesses: &self.guesses,
            condition: self.game_condition(),
        }
    }

    pub fn game_condition(&self) -> GameCondition {
        let has_won = self.guesses.iter().any(|guess| {
            guess
                .iter()
                .all(|(_, res)| *res == LetterState::CorrectPlacement)
        });
        if has_won {
            GameCondition::Win
        } else if self.guesses.len() >= self.max_guesses {
            GameCondition::Loss
        } else {
            GameCondition::Playing
        }
    }

    pub fn secret_word(&self) -> &str {
        &self.secret_word
    }

    pub fn words_already_guessed(&self) -> Vec<String> {
        self.guesses
            .iter()
            .map(|guess| guess.iter().map(|(c, _)| c).collect::<String>())
            .collect()
    }

    pub fn letter_states(&self) -> HashMap<char, LetterState> {
        let flat_guesses = self.guesses.iter().flatten();
        let mut result = HashMap::new();
        for (c, state) in flat_guesses.clone() {
            if *state == LetterState::CorrectPlacement {
                result.insert(c.clone(), state.clone());
            }
        }
        for (c, state) in flat_guesses.clone() {
            if *state == LetterState::CorrectLetter && !result.contains_key(c) {
                result.insert(c.clone(), state.clone());
            }
        }
        for (c, state) in flat_guesses.clone() {
            if *state == LetterState::Incorrect && !result.contains_key(c) {
                result.insert(c.clone(), state.clone());
            }
        }
        result
    }

    fn check_guess(guess: &str, secret_word: &str) -> Guess {
        let mut guess_result = Vec::with_capacity(guess.len());
        let mut letter_counts = crate::util::unique_element_counts(secret_word.chars());
        let zipped = guess.chars().zip(secret_word.chars());
        for (guess_c, selected_c) in zipped {
            if guess_c == selected_c {
                guess_result.push((guess_c, LetterState::CorrectPlacement));
                *letter_counts.get_mut(&selected_c).unwrap() -= 1;
            } else {
                guess_result.push((guess_c, LetterState::Incorrect))
            }
        }

        for (i, guess_c) in guess.chars().enumerate() {
            if i >= guess_result.len() {
                println!("{} {:?}", guess, guess_result)
            }
            if let Some(count) = letter_counts.get_mut(&guess_c) {
                if *count > 0 && guess_result[i].1 != LetterState::CorrectPlacement {
                    guess_result[i] = (guess_c, LetterState::CorrectLetter);
                    *count -= 1;
                }
            }
        }
        guess_result
    }
}

#[cfg(test)]
mod test_get_guess_result {
    use super::{LetterState::*, WordleGame};

    #[test]
    fn repeated_correct_letter_picks_first() {
        let guess = "slate";
        let secret = "salad";
        let expected_result = vec![
            ('s', CorrectPlacement),
            ('l', CorrectLetter),
            ('a', CorrectLetter),
            ('t', Incorrect),
            ('e', Incorrect),
        ];
        assert_eq!(WordleGame::check_guess(guess, secret), expected_result)
    }

    #[test]
    fn all_incorrect() {
        let guess = "would";
        let secret = "crate";
        let expected_result = vec![
            ('w', Incorrect),
            ('o', Incorrect),
            ('u', Incorrect),
            ('l', Incorrect),
            ('d', Incorrect),
        ];
        assert_eq!(WordleGame::check_guess(guess, secret), expected_result)
    }

    #[test]
    fn all_correct_placement() {
        let guess = "slate";
        let secret = "slate";
        let expected_result = vec![
            ('s', CorrectPlacement),
            ('l', CorrectPlacement),
            ('a', CorrectPlacement),
            ('t', CorrectPlacement),
            ('e', CorrectPlacement),
        ];
        assert_eq!(WordleGame::check_guess(guess, secret), expected_result)
    }

    #[test]
    fn all_correct_letter() {
        let guess = "tares";
        let secret = "stare";
        let expected_result = vec![
            ('t', CorrectLetter),
            ('a', CorrectLetter),
            ('r', CorrectLetter),
            ('e', CorrectLetter),
            ('s', CorrectLetter),
        ];
        assert_eq!(WordleGame::check_guess(guess, secret), expected_result)
    }

    #[test]
    fn correct_placement_captures_letters() {
        let guess = "lllll";
        let secret = "hello";
        let expected_result = vec![
            ('l', Incorrect),
            ('l', Incorrect),
            ('l', CorrectPlacement),
            ('l', CorrectPlacement),
            ('l', Incorrect),
        ];
        assert_eq!(WordleGame::check_guess(guess, secret), expected_result)
    }

    #[test]
    fn correct_placement_and_correct_letter() {
        let guess = "llzll";
        let secret = "hello";
        let expected_result = vec![
            ('l', CorrectLetter),
            ('l', Incorrect),
            ('z', Incorrect),
            ('l', CorrectPlacement),
            ('l', Incorrect),
        ];
        assert_eq!(WordleGame::check_guess(guess, secret), expected_result)
    }
}
