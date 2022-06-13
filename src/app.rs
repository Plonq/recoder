use base64::{decode, encode};
use wasm_bindgen::JsValue;
use yew::prelude::*;

use crate::textarea::Textarea;

pub enum Msg {
    Encode(String),
    Decode(String),
}

#[derive(Default, Debug)]
pub struct App {
    decoded: String,
    encoded: String,
    decode_failed: bool,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Encode(decoded) => {
                self.decoded = decoded.clone();
                self.encoded = encode(decoded.clone().into_bytes());
                true
            },
            Msg::Decode(encoded) => {
                self.encoded = encoded.clone();
                if let Ok(decoded) = decode(encoded.clone()) {
                    self.decoded = String::from_utf8(decoded).unwrap();
                    self.decode_failed = false;
                } else {
                    self.decode_failed = true;
                }
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let encode = link.callback(Msg::Encode);
        let decode = link.callback(Msg::Decode);
        html! {
            <div class="main">
                <h1 class="title">{ "ReCoder" }</h1>
                <div class="row">
                    <Textarea value={self.decoded.clone()} on_change={encode} />
                    if self.decode_failed {
                        <div class="overlay"><span>{ "Decode Failed. Check your data." }</span></div>
                    }
                </div>
                <div class="row">
                    <Textarea value={self.encoded.clone()} on_change={decode} />
                </div>
            </div>
        }
    }
}
