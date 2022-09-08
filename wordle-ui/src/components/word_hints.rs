use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct WordHintsProps {
    pub remaining_words: Vec<String>,
}

#[function_component(WordHints)]
pub fn word_hints(props: &WordHintsProps) -> Html {
    html! {
        <div class="word-hints-scroll-container">
            {props.remaining_words.iter().map(|word| html! {<p>{word}</p>}).collect::<Html>()}
        </div>
    }
}

#[function_component(WordHintsPopover)]
pub fn word_hints_popover(props: &WordHintsProps) -> Html {
    let is_open = use_state(|| false);

    if *is_open {
        html! {
            <div class="word-hints-popover">
                <WordHints remaining_words={props.remaining_words.clone()}/>
                <button
                    class="show-hints-button"
                    onclick={Callback::from(move |_| is_open.set(false))}
                >
                    {"Hide Hints"}
                </button>
            </div>
        }
    } else {
        html! {
            <button
                class="show-hints-button"
                onclick={Callback::from(move |_| is_open.set(true))}
            >
                {"Show Hints"}
            </button>
        }
    }
}
