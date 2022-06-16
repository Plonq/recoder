use crate::encoding_decoding::{decode, encode, Encoding};
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;

use crate::textarea::Textarea;

pub enum Msg {
    SetText(String),
    SetAction(Action),
    SetEncoding(Encoding),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Action {
    Encode,
    Decode,
}

impl Default for Action {
    fn default() -> Self {
        Action::Encode
    }
}

#[derive(Default, Debug)]
pub struct App {
    action: Action,
    input: String,
    output: String,
    decode_failed: bool,
    error_message: String,
    encoding: Encoding,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetText(text) => match &self.action {
                Action::Encode => {
                    self.input = text.clone();
                }
                Action::Decode => {
                    self.input = text.clone();
                }
            },
            Msg::SetAction(action) => {
                self.action = action;
            }
            Msg::SetEncoding(encoding) => {
                self.encoding = encoding;
            }
        }

        match self.action {
            Action::Encode => {
                self.output = encode(self.input.clone(), &self.encoding);
                self.decode_failed = false;
            }
            Action::Decode => match decode(self.input.clone(), &self.encoding) {
                Ok(decoded) => {
                    self.output = decoded;
                    self.decode_failed = false;
                }
                Err(e) => {
                    self.decode_failed = true;
                    self.error_message = e;
                }
            },
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let set_text = link.callback(Msg::SetText);
        let set_encode = link.batch_callback(|e: Event| {
            let encode_radio = e.target_dyn_into::<HtmlInputElement>();
            if let Some(encode_radio) = encode_radio {
                if encode_radio.checked() {
                    Some(Msg::SetAction(Action::Encode))
                } else {
                    None
                }
            } else {
                None
            }
        });
        let set_decode = link.batch_callback(|e: Event| {
            let decode_radio = e.target_dyn_into::<HtmlInputElement>();
            if let Some(decode_radio) = decode_radio {
                if decode_radio.checked() {
                    Some(Msg::SetAction(Action::Decode))
                } else {
                    None
                }
            } else {
                None
            }
        });

        let on_encoding_click = link.batch_callback(|e: MouseEvent| {
            let encoding_btn = e.target_dyn_into::<HtmlElement>();
            encoding_btn.map(
                |btn| match btn.get_attribute("data-value").unwrap().as_str() {
                    "base64" => Msg::SetEncoding(Encoding::Base64),
                    "uri" => Msg::SetEncoding(Encoding::Uri),
                    _ => Msg::SetEncoding(Encoding::Base64),
                },
            )
        });

        html! {
            <div class="main">
                <h1 class="title">{ "ReCoder" }</h1>
                <div class="row">
                    <Textarea value={self.input.clone()} on_change={set_text} />
                </div>
                <div class="row">
                    <label>
                        <input
                            type="radio"
                            name="action"
                            checked={self.action == Action::Encode}
                            onchange={set_encode}
                        />
                        { "Encode" }
                    </label>
                    <label>
                        <input
                            type="radio"
                            name="action"
                            checked={self.action == Action::Decode}
                            onchange={set_decode}
                        />
                        { "Decode" }
                    </label>
                </div>
                <div class="row">
                    <div class="controls">
                        <button type="button" onclick={&on_encoding_click} data-value="base64">{ "Base64" }</button>
                        <button type="button" onclick={&on_encoding_click} data-value="uri">{ "URI" }</button>
                    </div>
                </div>
                <div class="row">
                    <Textarea value={self.output.clone()} read_only={true} />
                    if self.decode_failed {
                        <div class="overlay">
                            <div class="content">
                                <div><strong>{ "Decode Failed" }</strong></div>
                                <div><em>{ self.error_message.clone() }</em></div>
                            </div>
                        </div>
                    }
                </div>
            </div>
        }
    }
}
