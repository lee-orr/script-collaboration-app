extern crate reqwest_wasm;

use std::borrow::Borrow;
use yew::html::Scope;
use yew::prelude::*;
use crate::editor::Editor;

mod data;
pub mod fountain;
mod editor;

enum Msg {
    UpdateContent(String)
}

struct App {
    content: String,
    title: String
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            content: format!("Some content..."),
            title: format!("A Title")
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link();

        match msg {
            Msg::UpdateContent(content) => {
                self.content = content;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let changed = ctx.link().callback(|value: String| Msg::UpdateContent(value));

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let div = document.create_element("div").unwrap();
        div.set_inner_html(&self.content.clone());

        let display = Html::VRef(div.into());

        html! {
            <div class="h-screen bg-gray-600 w-full flex flex-row items-center justify-center gap-y-3">
                <Editor changed={changed} content={self.content.to_owned()} title={self.title.to_owned()}/>
                {display}
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
