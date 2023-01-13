mod namae;
mod animals;
mod adjectives;

use namae::generate_name;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <div class={classes!("container")}>
                <h1 class={classes!("text")}>
                    { generate_name() }
                </h1>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
