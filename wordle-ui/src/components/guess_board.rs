use wordle_solver::types::{Guess, Guesses, LetterState};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GuessBoardProps {
    pub max_guesses: usize,
    pub max_word_length: usize,
    pub guesses: Guesses,
    pub current_guess: String,
}

#[function_component(GuessBoard)]
pub fn guess_board(
    GuessBoardProps {
        max_word_length,
        max_guesses,
        guesses,
        current_guess,
    }: &GuessBoardProps,
) -> Html {
    let row_occupied = guesses.len() + 1;
    let current_row = (guesses.len() < *max_guesses).then(|| {
        html! {
            <CurrentGuessRow
                max_length={*max_word_length}
                current_guess={current_guess.clone()}
            />
        }
    });
    let placeholder_rows = (row_occupied..*max_guesses)
        .map(|_| html! {<PlaceholderRow max_length={*max_word_length} />});
    let guess_rows = guesses.iter().map(|guess| {
        html! { <GuessRow guess={guess.clone()} /> }
    });
    html! {
        <div class="guess-board">
            {
                guess_rows
                .chain(current_row)
                .chain(placeholder_rows)
                .collect::<Html>()
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct GuessRowProps {
    pub guess: Guess,
}

#[function_component(GuessRow)]
pub fn guess_row(GuessRowProps { guess }: &GuessRowProps) -> Html {
    html! {
        <div class="guess-row">
            {
                guess.iter().map(
                    |(c, state)| html! {<div class={classes!("guess-tile", letter_state_class(state))}>{c}</div>}
                ).collect::<Html>()
            }
        </div>
    }
}

fn letter_state_class(state: &LetterState) -> &'static str {
    use LetterState::*;
    match state {
        Incorrect => "incorrect",
        CorrectLetter => "correct-letter",
        CorrectPlacement => "correct-placement",
    }
}

#[derive(Properties, PartialEq)]
pub struct CurrentGuessRowProps {
    pub max_length: usize,
    pub current_guess: String,
}

#[function_component(CurrentGuessRow)]
pub fn current_guess_row(
    CurrentGuessRowProps {
        max_length,
        current_guess,
    }: &CurrentGuessRowProps,
) -> Html {
    let num_remaining = *max_length - current_guess.len();
    let remaining_spaces = (0..num_remaining).map(|_| ' ');
    html! {
        <div class="guess-row">
            {
                current_guess.chars().chain(remaining_spaces).map(
                    |c| html! {<div class="guess-tile">{c}</div>}
                ).collect::<Html>()
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct PlaceholderRowProps {
    pub max_length: usize,
}

#[function_component(PlaceholderRow)]
pub fn placeholder_row(PlaceholderRowProps { max_length }: &PlaceholderRowProps) -> Html {
    html! {<CurrentGuessRow max_length={*max_length} current_guess="" />}
}
