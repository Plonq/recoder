use yew::prelude::*;

/// App header
#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <>
            <header>
                <h1 class="sr-only">{ "ReCoder" }</h1>
                <img src="assets/recoder-logo.png" />
            </header>
            <p class="site-description">
                { "A simple tool for text encoding, decoding, and hashing. 100% client-side." }
            </p>
        </>
    }
}
