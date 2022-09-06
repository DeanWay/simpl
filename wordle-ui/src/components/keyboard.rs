use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct KeyboardProps {
    pub on_key_press: Callback<char>,
    pub on_delete: Callback<char>,
    pub on_submit: Callback<char>,
}

#[function_component(Keyboard)]
pub fn keyboard(
    KeyboardProps {
        on_key_press,
        on_delete,
        on_submit,
    }: &KeyboardProps,
) -> Html {
    html! {
        <div class="keyboard">
            <div class="row">
                <Key letter='q' on_key_press={on_key_press.clone()}/>
                <Key letter='w' on_key_press={on_key_press.clone()}/>
                <Key letter='e' on_key_press={on_key_press.clone()}/>
                <Key letter='r' on_key_press={on_key_press.clone()}/>
                <Key letter='t' on_key_press={on_key_press.clone()}/>
                <Key letter='y' on_key_press={on_key_press.clone()}/>
                <Key letter='u' on_key_press={on_key_press.clone()}/>
                <Key letter='i' on_key_press={on_key_press.clone()}/>
                <Key letter='o' on_key_press={on_key_press.clone()}/>
                <Key letter='p' on_key_press={on_key_press.clone()}/>
            </div>
            <div class="row">
                <Key letter='a' on_key_press={on_key_press.clone()}/>
                <Key letter='s' on_key_press={on_key_press.clone()}/>
                <Key letter='d' on_key_press={on_key_press.clone()}/>
                <Key letter='f' on_key_press={on_key_press.clone()}/>
                <Key letter='g' on_key_press={on_key_press.clone()}/>
                <Key letter='h' on_key_press={on_key_press.clone()}/>
                <Key letter='j' on_key_press={on_key_press.clone()}/>
                <Key letter='k' on_key_press={on_key_press.clone()}/>
                <Key letter='l' on_key_press={on_key_press.clone()}/>
            </div>
            <div class="row">
                <Key letter='z' on_key_press={on_key_press.clone()}/>
                <Key letter='x' on_key_press={on_key_press.clone()}/>
                <Key letter='c' on_key_press={on_key_press.clone()}/>
                <Key letter='v' on_key_press={on_key_press.clone()}/>
                <Key letter='b' on_key_press={on_key_press.clone()}/>
                <Key letter='n' on_key_press={on_key_press.clone()}/>
                <Key letter='m' on_key_press={on_key_press.clone()}/>
                <Key letter='␡' on_key_press={on_delete.clone()}/>
                <Key letter='🆗' on_key_press={on_submit.clone()}/>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct KeyProps {
    letter: char,
    on_key_press: Callback<char>,
}

#[function_component(Key)]
fn key(
    KeyProps {
        letter,
        on_key_press,
    }: &KeyProps,
) -> Html {
    let letter = letter.clone();
    let on_key_press = on_key_press.clone();
    html! {
        <button
            class="keyboard-key"
            onclick={move |_| on_key_press.emit(letter)}
        >
            { letter }
        </button>
    }
}
