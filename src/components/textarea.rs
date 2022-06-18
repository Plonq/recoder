use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlTextAreaElement, InputEvent};
use yew::prelude::*;

fn create_default_on_change_callback() -> Callback<String> {
    Callback::from(|_| ())
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: String,
    #[prop_or_else(create_default_on_change_callback)]
    pub on_change: Callback<String>,
    #[prop_or(false)]
    pub read_only: bool,
    #[prop_or("Type here".to_string())]
    pub placeholder: String,
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlTextAreaElement = event_target.dyn_into().unwrap_throw();
    web_sys::console::log_1(&target.value().into());
    target.value()
}

/// Controlled Textarea Component
#[function_component(Textarea)]
pub fn textarea(props: &Props) -> Html {
    let Props {
        value,
        on_change,
        read_only,
        placeholder,
    } = props.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change.emit(get_value_from_input_event(input_event));
    });

    html! {
        <textarea class="textarea" {placeholder} rows="20" readonly={read_only} {value} {oninput} />
    }
}
