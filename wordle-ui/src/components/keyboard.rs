use std::collections::HashMap;

use wordle_solver::types::LetterState;
use yew::prelude::*;

use super::util::letter_state_class;

#[derive(Properties, PartialEq)]
pub struct KeyboardProps {
    pub letter_states: HashMap<char, LetterState>,
    pub on_key_press: Callback<char>,
    pub on_delete: Callback<char>,
    pub on_submit: Callback<char>,
}

#[function_component(Keyboard)]
pub fn keyboard(
    KeyboardProps {
        letter_states,
        on_key_press,
        on_delete,
        on_submit,
    }: &KeyboardProps,
) -> Html {
    let state = |c| letter_states.get(&c).cloned();
    html! {
        <div class="keyboard">
            <div class="row">
                <Key letter='q' state={state('q')} on_key_press={on_key_press.clone()}/>
                <Key letter='w' state={state('w')} on_key_press={on_key_press.clone()}/>
                <Key letter='e' state={state('e')} on_key_press={on_key_press.clone()}/>
                <Key letter='r' state={state('r')} on_key_press={on_key_press.clone()}/>
                <Key letter='t' state={state('t')} on_key_press={on_key_press.clone()}/>
                <Key letter='y' state={state('y')} on_key_press={on_key_press.clone()}/>
                <Key letter='u' state={state('u')} on_key_press={on_key_press.clone()}/>
                <Key letter='i' state={state('i')} on_key_press={on_key_press.clone()}/>
                <Key letter='o' state={state('o')} on_key_press={on_key_press.clone()}/>
                <Key letter='p' state={state('p')} on_key_press={on_key_press.clone()}/>
            </div>
            <div class="row">
                <Key letter='a' state={state('a')} on_key_press={on_key_press.clone()}/>
                <Key letter='s' state={state('s')} on_key_press={on_key_press.clone()}/>
                <Key letter='d' state={state('d')} on_key_press={on_key_press.clone()}/>
                <Key letter='f' state={state('f')} on_key_press={on_key_press.clone()}/>
                <Key letter='g' state={state('g')} on_key_press={on_key_press.clone()}/>
                <Key letter='h' state={state('h')} on_key_press={on_key_press.clone()}/>
                <Key letter='j' state={state('j')} on_key_press={on_key_press.clone()}/>
                <Key letter='k' state={state('k')} on_key_press={on_key_press.clone()}/>
                <Key letter='l' state={state('l')} on_key_press={on_key_press.clone()}/>
            </div>
            <div class="row">
                <Key letter='🆗' on_key_press={on_submit.clone()}/>
                <Key letter='z' state={state('z')} on_key_press={on_key_press.clone()}/>
                <Key letter='x' state={state('x')} on_key_press={on_key_press.clone()}/>
                <Key letter='c' state={state('c')} on_key_press={on_key_press.clone()}/>
                <Key letter='v' state={state('v')} on_key_press={on_key_press.clone()}/>
                <Key letter='b' state={state('b')} on_key_press={on_key_press.clone()}/>
                <Key letter='n' state={state('n')} on_key_press={on_key_press.clone()}/>
                <Key letter='m' state={state('m')} on_key_press={on_key_press.clone()}/>
                <Key letter='␡' classname="delete" on_key_press={on_delete.clone()}/>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct KeyProps {
    letter: char,
    classname: Option<String>,
    state: Option<LetterState>,
    on_key_press: Callback<char>,
}

#[function_component(Key)]
fn key(
    KeyProps {
        classname,
        letter,
        state,
        on_key_press,
    }: &KeyProps,
) -> Html {
    let letter = letter.clone();
    let on_key_press = on_key_press.clone();
    let state_class = state.map(|s| letter_state_class(&s));
    html! {
        <button
            class={classes!("keyboard-key", state_class, classname)}
            onclick={move |_| on_key_press.emit(letter)}
        >
            { letter }
        </button>
    }
}
