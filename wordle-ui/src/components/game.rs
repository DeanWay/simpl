use super::guess_board::GuessBoard;
use super::keyboard::Keyboard;
use super::word_hints::WordHintsPopover;
use crate::dictionary::DICTIONARY;
use gloo_events::EventListener;
use gloo_utils::window;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;

use wordle_solver::constraint::word_matches;
use wordle_solver::game::WordleGame;
use wordle_solver::types::{GameCondition, Guesses};
use yew::events::KeyboardEvent;
use yew::prelude::*;
const MAX_GUESSES: usize = 6;
const MAX_WORD_LENGTH: usize = 5;

pub struct Game {
    current_guess: String,
    game: WordleGame,
    key_listener: Option<EventListener>,
    game_message: Option<String>,
    message_key: u8,
}

pub enum GameMessage {
    AddLetter(char),
    DeleteLetter,
    Submit,
}

impl Game {
    fn handle_submit(&mut self) -> bool {
        let s = self.game.secret_word().to_string();
        match self.game.make_guess(&self.current_guess.to_lowercase()) {
            Err(err) => {
                self.set_message(err);
            }
            Ok(_) => {
                self.current_guess = String::new();
            }
        };
        if self.game.game_condition() == GameCondition::Win {
            self.set_message("You Win!");
        }
        if self.game.game_condition() == GameCondition::Loss {
            self.set_message(&s);
        }
        true
    }

    fn handle_add_letter(&mut self, c: char) -> bool {
        let current_guess_incomplete = self.current_guess.len() < MAX_WORD_LENGTH;
        if self.still_playing() && current_guess_incomplete && c.is_ascii_alphabetic() {
            self.current_guess.push(c.to_ascii_lowercase());
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

    fn set_message(&mut self, msg: &str) {
        self.game_message.replace(msg.to_string());
        self.message_key = self.message_key.wrapping_add(1);
    }
}

impl Component for Game {
    type Message = GameMessage;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            current_guess: String::from(""),
            game: WordleGame::new_with_random_secret_word(DICTIONARY),
            key_listener: None,
            game_message: None,
            message_key: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        use GameMessage::*;
        match msg {
            AddLetter(c) => self.handle_add_letter(c),
            DeleteLetter => self.handle_delete(),
            Submit => self.handle_submit(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let guesses: Guesses = self.game.game_state().guesses.clone();
        let remaining_words: Vec<String> = DICTIONARY
            .iter()
            .filter(|word| word_matches(word, &self.game.game_state()))
            .map(|word| word.to_string())
            .collect();
        log::info!("{:?}", remaining_words);
        html! {
            <div class="game-container">
                <div
                    key={self.message_key}
                    class={classes!("game-message", self.game_message.as_ref().map(|_| "show"))}
                >
                    {self.game_message.as_ref().unwrap_or(&"".to_string())}
                </div>
                <GuessBoard
                    max_word_length={MAX_WORD_LENGTH}
                    max_guesses={MAX_GUESSES}
                    guesses={guesses}
                    current_guess={self.current_guess.clone()}
                />
                <Keyboard
                    letter_states={self.game.letter_states()}
                    on_key_press={ctx.link().callback(|c| GameMessage::AddLetter(c))}
                    on_delete={ctx.link().callback(|_| GameMessage::DeleteLetter)}
                    on_submit={ctx.link().callback(|_| GameMessage::Submit)}
                />
                <WordHintsPopover remaining_words={remaining_words}/>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        let onkeydown = ctx.link().callback(|e: KeyboardEvent| match e.key_code() {
            8 => GameMessage::DeleteLetter,
            13 => GameMessage::Submit,
            c => GameMessage::AddLetter((c as u8) as char),
        });
        let listener = EventListener::new(&window(), "keydown", move |event| {
            let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
            onkeydown.emit(event.clone())
        });
        self.key_listener.replace(listener);
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        if let Some(listener) = &mut self.key_listener {
            drop(listener);
            self.key_listener = None;
        }
    }
}
