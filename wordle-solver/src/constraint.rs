use crate::{
    types::{Guess, LetterState::*, WordleGameState},
    util,
};

pub fn word_matches(word: &str, game_state: &WordleGameState) -> bool {
    game_state
        .guesses
        .iter()
        .all(|constraint| word_satisfies_contraint(word, constraint))
}

pub fn word_satisfies_contraint(word: &str, guess_result: &Guess) -> bool {
    let word_letter_counts = util::unique_element_counts(word.chars());

    // word should have at least all the correct letters
    let guess_correct_letter_counts = util::unique_element_counts(
        guess_result
            .iter()
            .filter(|(_, res)| *res == CorrectLetter || *res == CorrectPlacement)
            .map(|(c, _)| *c),
    );

    if !guess_correct_letter_counts
        .iter()
        .all(|(c, count)| match word_letter_counts.get(c) {
            None => false,
            Some(word_c_count) => word_c_count >= count,
        })
    {
        return false;
    }

    // word should not have more than known max letters
    // (in the case we have a correct and incorrect for the same letter we know the max)
    for (c, res) in guess_result {
        if *res == Incorrect {
            match guess_correct_letter_counts.get(c) {
                None => continue,
                Some(count) => {
                    if word_letter_counts.get(c).unwrap() != count {
                        return false;
                    }
                }
            };
        }
    }

    // correct placements should match at the same position
    // otherwise should not match at the same position
    for (word_c, (guess_c, letter_res)) in word.chars().zip(guess_result) {
        match letter_res {
            CorrectPlacement => {
                if word_c != *guess_c {
                    return false;
                }
            }
            Incorrect | CorrectLetter => {
                if word_c == *guess_c {
                    return false;
                }
            }
        };
    }
    true
}

#[cfg(test)]
mod test_word_satisfies_contraint {
    use super::word_satisfies_contraint;
    use crate::types::LetterState::*;

    #[test]
    fn matches_all_correct_placements() {
        let res = word_satisfies_contraint(
            "hello",
            &vec![
                ('h', CorrectPlacement),
                ('e', CorrectPlacement),
                ('l', CorrectPlacement),
                ('l', CorrectPlacement),
                ('o', CorrectPlacement),
            ],
        );
        assert!(res);
    }

    #[test]
    fn matches_some_correct_placements() {
        let res = word_satisfies_contraint(
            "there",
            &vec![
                ('t', CorrectPlacement),
                ('h', CorrectPlacement),
                ('i', Incorrect),
                ('r', CorrectPlacement),
                ('d', Incorrect),
            ],
        );
        assert!(res);
    }

    #[test]
    fn matches_correct_letters() {
        let res = word_satisfies_contraint(
            "tares",
            &vec![
                ('s', CorrectLetter),
                ('t', CorrectLetter),
                ('a', CorrectLetter),
                ('r', CorrectLetter),
                ('e', CorrectLetter),
            ],
        );
        assert!(res);
    }
}
