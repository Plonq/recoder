use crate::components::{TextInput, Textarea};
use crate::engine::{decode, encode, hmac_signature_b64, hmac_signature_hex};
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum Msg {
    SetAction(Action),
    SetHmacKey(String),
    SetHmacEncoding(HmacEncoding),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Action {
    Hmac,
}

impl Default for Action {
    fn default() -> Self {
        Action::Hmac
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum HmacEncoding {
    Hex,
    Base64,
}

impl Default for HmacEncoding {
    fn default() -> Self {
        HmacEncoding::Hex
    }
}

#[derive(Debug, PartialEq)]
struct HmacConfig {
    key: String,
    encoding: HmacEncoding,
}

impl Default for HmacConfig {
    fn default() -> Self {
        HmacConfig {
            key: String::default(),
            encoding: HmacEncoding::Hex,
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub input: String,
}

#[derive(Default, Debug)]
pub struct Hashing {
    action: Action,
    output: String,
    hmac_config: HmacConfig,
}

impl Component for Hashing {
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
            Msg::SetHmacKey(key) => {
                self.hmac_config.key = key;
                true
            }
            Msg::SetHmacEncoding(encoding) => {
                self.hmac_config.encoding = encoding;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let on_action_click = link.batch_callback(|e: Event| {
            let action_el = e.target_dyn_into::<HtmlInputElement>();
            action_el.map(|btn| match btn.value().as_str() {
                "hmac" => Msg::SetAction(Action::default()),
                _ => Msg::SetAction(Action::default()),
            })
        });

        let on_secret_input = link.callback(|value| Msg::SetHmacKey(value));

        let on_encoding_click = link.batch_callback(|e: Event| {
            let encoding_btn = e.target_dyn_into::<HtmlInputElement>();
            encoding_btn.map(|btn| match btn.value().as_str() {
                "hex" => Msg::SetHmacEncoding(HmacEncoding::Hex),
                "base64" => Msg::SetHmacEncoding(HmacEncoding::Base64),
                _ => Msg::SetHmacEncoding(HmacEncoding::Hex),
            })
        });

        let mut output = String::default();
        let mut error = false;
        let mut error_message = String::default();

        match &self.action {
            Action::Hmac => match &self.hmac_config.encoding {
                HmacEncoding::Hex => {
                    output = hmac_signature_hex(
                        self.hmac_config.key.as_str(),
                        ctx.props().input.as_str(),
                    );
                }
                HmacEncoding::Base64 => {
                    output = hmac_signature_b64(
                        self.hmac_config.key.as_str(),
                        ctx.props().input.as_str(),
                    );
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
                                value="hmac"
                                checked={self.action == Action::Hmac}
                                onchange={&on_action_click}
                            />
                            <span>{ "HMAC Digest" }</span>
                        </label>
                    </div>
                </div>
                <div class="controls">
                    if self.action == Action::Hmac {
                        <TextInput placeholder={"Enter secret key".to_string()} value={self.hmac_config.key.clone()} on_change={on_secret_input} />
                        {"Output as:"}
                        <label>
                            <input
                                type="radio"
                                name="hmac-encoding"
                                value="hex"
                                checked={self.hmac_config.encoding == HmacEncoding::Hex}
                                onchange={&on_encoding_click}
                            />
                            { "Hex" }
                        </label>
                        <label>
                            <input
                                type="radio"
                                name="hmac-encoding"
                                value="base64"
                                checked={self.hmac_config.encoding == HmacEncoding::Base64}
                                onchange={&on_encoding_click}
                            />
                            { "Base64" }
                        </label>
                    }
                </div>
                <div class="overlay-container">
                    <Textarea placeholder={"Output".to_string()} value={output} read_only={true} />
                    if error {
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
