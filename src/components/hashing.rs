use crate::components::{TextInput, Textarea};
use crate::engine::{
    blake2b512_hash, blake2s256_hash, hmac_digest_b64, hmac_digest_hex, md5_hash, sha1_hash,
    sha224_hash, sha256_hash, sha384_hash, sha512_hash,
};
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum Msg {
    SetAction(Action),
    SetHmacKey(String),
    SetHmacEncoding(HmacEncoding),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Action {
    Md5,
    Sha1,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    Blake2b512,
    Blake2s256,
    Hmac,
}

impl Default for Action {
    fn default() -> Self {
        Action::Md5
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

#[derive(Default)]
pub struct Hashing {
    action: Action,
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
                "md5" => Msg::SetAction(Action::Md5),
                "sha1" => Msg::SetAction(Action::Sha1),
                "sha224" => Msg::SetAction(Action::Sha224),
                "sha256" => Msg::SetAction(Action::Sha256),
                "sha384" => Msg::SetAction(Action::Sha384),
                "sha512" => Msg::SetAction(Action::Sha512),
                "blake2b512" => Msg::SetAction(Action::Blake2b512),
                "blake2s256" => Msg::SetAction(Action::Blake2s256),
                "hmac" => Msg::SetAction(Action::Hmac),
                _ => Msg::SetAction(Action::default()),
            })
        });

        let on_secret_input = link.callback(|value| Msg::SetHmacKey(value));

        let on_encoding_click = link.batch_callback(|e: Event| {
            let encoding_btn = e.target_dyn_into::<HtmlInputElement>();
            encoding_btn.map(|btn| match btn.value().as_str() {
                "hex" => Msg::SetHmacEncoding(HmacEncoding::Hex),
                "base64" => Msg::SetHmacEncoding(HmacEncoding::Base64),
                _ => Msg::SetHmacEncoding(HmacEncoding::default()),
            })
        });

        let output;
        let input = ctx.props().input.as_str();

        match &self.action {
            Action::Md5 => output = md5_hash(input),
            Action::Sha1 => output = sha1_hash(input),
            Action::Sha224 => output = sha224_hash(input),
            Action::Sha256 => output = sha256_hash(input),
            Action::Sha384 => output = sha384_hash(input),
            Action::Sha512 => output = sha512_hash(input),
            Action::Blake2b512 => output = blake2b512_hash(input),
            Action::Blake2s256 => output = blake2s256_hash(input),
            Action::Hmac => match &self.hmac_config.encoding {
                HmacEncoding::Hex => {
                    output = hmac_digest_hex(self.hmac_config.key.as_str(), input);
                }
                HmacEncoding::Base64 => {
                    output = hmac_digest_b64(self.hmac_config.key.as_str(), input);
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
                                value="md5"
                                checked={self.action == Action::Md5}
                                onchange={&on_action_click}
                            />
                            <span>{ "MD5" }</span>
                        </label>
                        <label class="form-radio">
                            <input
                                type="radio"
                                name="action"
                                value="sha1"
                                checked={self.action == Action::Sha1}
                                onchange={&on_action_click}
                            />
                            <span>{ "SHA-1" }</span>
                        </label>
                        <label class="form-radio">
                            <input
                                type="radio"
                                name="action"
                                value="sha224"
                                checked={self.action == Action::Sha224}
                                onchange={&on_action_click}
                            />
                            <span>{ "SHA-224" }</span>
                        </label>
                        <label class="form-radio">
                            <input
                                type="radio"
                                name="action"
                                value="sha256"
                                checked={self.action == Action::Sha256}
                                onchange={&on_action_click}
                            />
                            <span>{ "SHA-256" }</span>
                        </label>
                        <label class="form-radio">
                            <input
                                type="radio"
                                name="action"
                                value="sha384"
                                checked={self.action == Action::Sha384}
                                onchange={&on_action_click}
                            />
                            <span>{ "SHA-384" }</span>
                        </label>
                        <label class="form-radio">
                            <input
                                type="radio"
                                name="action"
                                value="sha512"
                                checked={self.action == Action::Sha512}
                                onchange={&on_action_click}
                            />
                            <span>{ "SHA-512" }</span>
                        </label>
                        <label class="form-radio">
                            <input
                                type="radio"
                                name="action"
                                value="blake2b512"
                                checked={self.action == Action::Blake2b512}
                                onchange={&on_action_click}
                            />
                            <span>{ "BLAKE2b-512" }</span>
                        </label>
                        <label class="form-radio">
                            <input
                                type="radio"
                                name="action"
                                value="blake2s256"
                                checked={self.action == Action::Blake2s256}
                                onchange={&on_action_click}
                            />
                            <span>{ "BLAKE2s-256" }</span>
                        </label>
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
                        <label class="custom-radio">
                            <input
                                type="radio"
                                name="hmac-encoding"
                                value="hex"
                                checked={self.hmac_config.encoding == HmacEncoding::Hex}
                                onchange={&on_encoding_click}
                            />
                            { "Hex" }
                        </label>
                        <label class="custom-radio">
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
                <div>
                    <Textarea placeholder={"Output".to_string()} value={output} read_only={true} is_output={true} />
                </div>
            </div>
        }
    }
}
