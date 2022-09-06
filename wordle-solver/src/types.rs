#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LetterState {
    Incorrect,
    CorrectLetter,
    CorrectPlacement,
}

pub type Guess = Vec<(char, LetterState)>;

pub type Guesses = Vec<Guess>;

#[derive(Debug)]
pub struct WordleGameState<'a> {
    pub guesses: &'a Guesses,
    pub condition: GameCondition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameCondition {
    Win,
    Loss,
    Playing,
}
