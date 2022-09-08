use wordle_game::types::LetterState;

pub fn letter_state_class(state: &LetterState) -> &'static str {
    use LetterState::*;
    match state {
        Incorrect => "incorrect",
        CorrectLetter => "correct-letter",
        CorrectPlacement => "correct-placement",
    }
}
