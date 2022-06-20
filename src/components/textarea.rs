use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue, UnwrapThrowExt};
use web_sys::{Event, HtmlTextAreaElement, InputEvent, MouseEvent};
use yew::prelude::*;
use yew_icons::{Icon, IconId};

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
    #[prop_or(false)]
    pub is_output: bool,
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlTextAreaElement = event_target.dyn_into().unwrap_throw();
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
        is_output,
    } = props.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change.emit(get_value_from_input_event(input_event));
    });

    let copy_to_clipboard = {
        let v = value.clone();
        move |_: MouseEvent| {
            let v = v.clone();
            wasm_bindgen_futures::spawn_local(async move {
                copy_to_clipboard(v).await.unwrap();
            });
        }
    };

    html! {
        <div class="textarea-container">
            <textarea class="textarea" {placeholder} readonly={read_only} value={value} {oninput} />
            if is_output {
                <div class="output-controls">
                    <button type="button" class="button-icon" aria-label="Copy output to clipboard" onclick={copy_to_clipboard}>
                        <Icon icon_id={IconId::FeatherCopy} />
                    </button>
                </div>
            }
        </div>
    }
}

#[wasm_bindgen(inline_js=r#"
export function copy_to_clipboard(value) {
    try {
        return window.navigator.clipboard.writeText(value);
    } catch(e) {
        console.log(e);
        return Promise.reject(e)
    }
}
"#)]
#[rustfmt::skip]
extern "C" { 
    #[wasm_bindgen(catch)]
    async fn copy_to_clipboard(value: String) -> Result<(), JsValue>;
}
