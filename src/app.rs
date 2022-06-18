use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::Textarea;
use crate::engine::*;

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

        let on_action_click = link.batch_callback(|e: Event| {
            let action_el = e.target_dyn_into::<HtmlInputElement>();
            action_el.map(|btn| match btn.value().as_str() {
                "encode" => Msg::SetAction(Action::Encode),
                "decode" => Msg::SetAction(Action::Decode),
                _ => Msg::SetAction(Action::Encode),
            })
        });

        let on_encoding_click = link.batch_callback(|e: Event| {
            let encoding_btn = e.target_dyn_into::<HtmlInputElement>();
            encoding_btn.map(|btn| match btn.value().as_str() {
                "base64" => Msg::SetEncoding(Encoding::Base64),
                "uri" => Msg::SetEncoding(Encoding::Uri),
                "hex" => Msg::SetEncoding(Encoding::Hex),
                "html" => Msg::SetEncoding(Encoding::Html),
                _ => Msg::SetEncoding(Encoding::Base64),
            })
        });

        html! {
            <div class="main">
                <h1 class="title">{ "ReCoder" }</h1>
                <div class="row">
                    <Textarea placeholder={"Input".to_string()} value={self.input.clone()} on_change={set_text} />
                </div>
                <div class="row">
                    <label>
                        <input
                            type="radio"
                            name="action"
                            value="encode"
                            checked={self.action == Action::Encode}
                            onchange={&on_action_click}
                        />
                        { "Encode" }
                    </label>
                    <label>
                        <input
                            type="radio"
                            name="action"
                            value="decode"
                            checked={self.action == Action::Decode}
                            onchange={&on_action_click}
                        />
                        { "Decode" }
                    </label>
                </div>
                <div class="row">
                    <div class="controls">
                        <label>
                            <input
                                type="radio"
                                name="encoding"
                                value="base64"
                                checked={self.encoding == Encoding::Base64}
                                onchange={&on_encoding_click}
                            />
                            { "Base64" }
                        </label>
                        <label>
                            <input
                                type="radio"
                                name="encoding"
                                value="uri"
                                checked={self.encoding == Encoding::Uri}
                                onchange={&on_encoding_click}
                            />
                            { "URI/URL" }
                        </label>
                        <label>
                            <input
                                type="radio"
                                name="encoding"
                                value="hex"
                                checked={self.encoding == Encoding::Hex}
                                onchange={&on_encoding_click}
                            />
                            { "Hex" }
                        </label>
                        <label>
                            <input
                                type="radio"
                                name="encoding"
                                value="html"
                                checked={self.encoding == Encoding::Html}
                                onchange={&on_encoding_click}
                            />
                            { "HTML" }
                        </label>
                    </div>
                </div>
                <div class="row">
                    <Textarea placeholder={"Output".to_string()} value={self.output.clone()} read_only={true} />
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
