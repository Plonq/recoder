use yew::prelude::*;

/// App header
#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <h1 class="title">{ "ReCoder" }</h1>
        </header>
    }
}
