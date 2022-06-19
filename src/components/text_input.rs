use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::prelude::*;

fn create_default_on_change_callback() -> Callback<String> {
    Callback::from(|_| ())
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: String,
    #[prop_or_else(create_default_on_change_callback)]
    pub on_change: Callback<String>,
    #[prop_or("Type here".to_string())]
    pub placeholder: String,
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

/// Controlled Textarea Component
#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let Props {
        value,
        on_change,
        placeholder,
    } = props.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change.emit(get_value_from_input_event(input_event));
    });

    html! {
        <input type="text" class="text-input" {placeholder} {value} {oninput} />
    }
}
