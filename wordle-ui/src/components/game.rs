use yew::prelude::*;

use crate::components::guess_board::GuessBoard;
use crate::components::keyboard::Keyboard;
use crate::dictionary::DICTIONARY;
use wordle_solver::game::WordleGame;
use wordle_solver::types::{GameCondition, Guesses};

const MAX_GUESSES: usize = 6;
const MAX_WORD_LENGTH: usize = 5;

pub struct App {
    current_guess: String,
    game: WordleGame,
}

pub enum AppMessage {
    AddLetter(char),
    DeleteLetter,
    Submit,
}

impl App {
    fn handle_submit(&mut self) -> Result<(), &str> {
        self.game.make_guess(&self.current_guess)?;
        self.current_guess = String::new();
        Ok(())
    }

    fn handle_add_letter(&mut self, c: char) -> bool {
        let current_guess_incomplete = self.current_guess.len() < MAX_WORD_LENGTH;
        if self.still_playing() && current_guess_incomplete && c.is_alphabetic() {
            self.current_guess.push(c);
            true
        } else {
            false
        }
    }

    fn handle_delete(&mut self) -> bool {
        if self.still_playing() {
            self.current_guess.pop();
            true
        } else {
            false
        }
    }

    fn still_playing(&self) -> bool {
        self.game.game_condition() == GameCondition::Playing
    }
}

impl Component for App {
    type Message = AppMessage;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {
            current_guess: String::from(""),
            game: WordleGame::new_with_random_secret_word(DICTIONARY),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        use AppMessage::*;
        match msg {
            AddLetter(c) => self.handle_add_letter(c),
            DeleteLetter => self.handle_delete(),
            Submit => match self.handle_submit() {
                Ok(_) => true,
                Err(err) => {
                    log::info!("{}", err);
                    false
                }
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let guesses: Guesses = self.game.game_state().guesses.clone();

        // let on_physical_key_press = {
        //     // let guesses = guesses.clone();
        //     let on_delete = on_delete.clone();
        //     let on_submit = on_submit.clone();
        //     let on_key_press = on_virtual_key_press.clone();
        //     Callback::from(move |e: KeyboardEvent| {
        //         log::info!("{:?}", e);
        //         log::info!("{:?}", e.key_code());
        //         match e.key_code() {
        //             8 => on_delete.emit(' '),
        //             13 => on_submit.emit(' '),
        //             c => on_key_press.emit((c as u8) as char),
        //         }
        //     })
        // };

        html! {
            <div>
                <GuessBoard
                    max_word_length={MAX_WORD_LENGTH}
                    max_guesses={MAX_GUESSES}
                    guesses={guesses}
                    current_guess={self.current_guess.clone()}
                />
                <Keyboard
                    on_key_press={ctx.link().callback(|c| AppMessage::AddLetter(c))}
                    on_delete={ctx.link().callback(|_| AppMessage::DeleteLetter)}
                    on_submit={ctx.link().callback(|_| AppMessage::Submit)}
                />
            </div>
        }
    }
}
