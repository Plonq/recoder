use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::{Hashing, Header, TextEncoding, Textarea};

pub enum Msg {
    SetText(String),
    SetCategory(Category),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Category {
    Encoding,
    Hashing,
}

impl Default for Category {
    fn default() -> Self {
        Category::Encoding
    }
}

#[derive(Default, Debug)]
pub struct App {
    category: Category,
    input: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetText(text) => {
                self.input = text;
                true
            }
            Msg::SetCategory(category) => {
                self.category = category;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let set_text = link.callback(Msg::SetText);

        let on_category_click = link.batch_callback(|e: Event| {
            let category_el = e.target_dyn_into::<HtmlInputElement>();
            category_el.map(|btn| match btn.value().as_str() {
                "encoding" => Msg::SetCategory(Category::Encoding),
                "hashing" => Msg::SetCategory(Category::Hashing),
                _ => Msg::SetCategory(Category::Encoding),
            })
        });

        html! {
            <>
                <Header/>
                <main class="main">
                    <div class="row">
                        <Textarea placeholder={"Input".to_string()} value={self.input.clone()} on_change={set_text} />
                    </div>
                    <div class="row">
                        <div class="form-radio-group">
                            <label class="form-radio">
                                <input
                                    type="radio"
                                    name="category"
                                    value="encoding"
                                    checked={self.category == Category::Encoding}
                                    onchange={&on_category_click}
                                />
                                <span>{ "Text Encoding" }</span>
                            </label>
                            <label class="form-radio">
                                <input
                                    type="radio"
                                    name="category"
                                    value="hashing"
                                    checked={self.category == Category::Hashing}
                                    onchange={&on_category_click}
                                />
                                <span>{ "Hashing" }</span>
                            </label>
                        </div>
                    </div>
                    <div class="row">
                        if self.category == Category::Encoding {
                            <TextEncoding input={self.input.clone()}/>
                        }
                        else if self.category == Category::Hashing {
                            <Hashing input={self.input.clone()}/>
                        }
                    </div>
                </main>
                <div class="footer">
                    { "Powered by Rust, WebAssembly, and the Yew framework. " }
                    <a href="https://github.com/Plonq/recoder">{ "GitHub Repo" }</a>
                    { "." }
                </div>
            </>
        }
    }
}
