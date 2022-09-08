use super::game::Game;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="app-container">
            <Game />
        </div>
    }
}
