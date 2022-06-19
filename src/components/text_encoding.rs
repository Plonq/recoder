use crate::components::Textarea;
use crate::engine::{decode, encode, Encoding};
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum Msg {
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

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub input: String,
}

#[derive(Default, Debug)]
pub struct TextEncoding {
    action: Action,
    output: String,
    encoding: Encoding,
}

impl Component for TextEncoding {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetAction(action) => {
                self.action = action;
                true
            }
            Msg::SetEncoding(encoding) => {
                self.encoding = encoding;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

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

        let mut output = String::default();
        let mut decode_failed = false;
        let mut error_message = String::default();

        match self.action {
            Action::Encode => {
                output = encode(ctx.props().input.clone(), &self.encoding);
            }
            Action::Decode => match decode(ctx.props().input.clone(), &self.encoding) {
                Ok(decoded) => {
                    output = decoded;
                }
                Err(e) => {
                    decode_failed = true;
                    error_message = e;
                }
            },
        }

        html! {
            <div class="action-component">
                <div class="control">
                    <div class="form-radio-group">
                        <label class="form-radio">
                            <input
                                type="radio"
                                name="action"
                                value="encode"
                                checked={self.action == Action::Encode}
                                onchange={&on_action_click}
                            />
                            <span>{ "Encode" }</span>
                        </label>
                        <label class="form-radio">
                            <input
                                type="radio"
                                name="action"
                                value="decode"
                                checked={self.action == Action::Decode}
                                onchange={&on_action_click}
                            />
                            <span>{ "Decode" }</span>
                        </label>
                    </div>
                </div>
                <div class="controls">
                    <div class="form-radio-group">
                        <label class="form-radio">
                            <input
                                type="radio"
                                name="encoding"
                                value="base64"
                                checked={self.encoding == Encoding::Base64}
                                onchange={&on_encoding_click}
                            />
                            <span>{ "Base64" }</span>
                        </label>
                        <label class="form-radio">
                            <input
                                type="radio"
                                name="encoding"
                                value="uri"
                                checked={self.encoding == Encoding::Uri}
                                onchange={&on_encoding_click}
                            />
                            <span>{ "URI/URL" }</span>
                        </label>
                        <label class="form-radio">
                            <input
                                type="radio"
                                name="encoding"
                                value="hex"
                                checked={self.encoding == Encoding::Hex}
                                onchange={&on_encoding_click}
                            />
                            <span>{ "Hex" }</span>
                        </label>
                        <label class="form-radio">
                            <input
                                type="radio"
                                name="encoding"
                                value="html"
                                checked={self.encoding == Encoding::Html}
                                onchange={&on_encoding_click}
                            />
                            <span>{ "HTML" }</span>
                        </label>
                    </div>
                </div>
                <div class="overlay-container">
                    <Textarea placeholder={"Output".to_string()} value={output} read_only={true} />
                    if decode_failed {
                        <div class="overlay">
                            <div class="content">
                                <div><strong>{ "Decode Failed" }</strong></div>
                                <div><em>{ error_message }</em></div>
                            </div>
                        </div>
                    }
                </div>
            </div>
        }
    }
}
