use base64::encode;
use yew::prelude::*;

use crate::textarea::Textarea;

pub enum Msg {
    Encode(String),
}

#[derive(Default, Debug)]
pub struct App {
    text: String,
    output: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Encode(text) => {
                self.text = text.clone();
                self.output = encode(text.clone().into_bytes());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let on_change = link.callback(Msg::Encode);
        html! {
            <div>
                <Textarea value={self.text.clone()} {on_change} />
                <textarea value={self.output.clone()} />
            </div>
        }
    }
}
