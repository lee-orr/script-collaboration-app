extern crate reqwest_wasm;

use std::borrow::Borrow;
use fountain::parser::parse_fountain;
use fountain::types::Script;
use fountain::display::Display;
use yew::html::Scope;
use yew::prelude::*;
use crate::editor::Editor;
use crate::fountain::types::LineContent;

mod data;
pub mod fountain;
mod editor;

enum Msg {
    UpdateContent(String)
}

struct App {
    content: String,
    title: String,
    parsed: Option<Script>
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let content = include_str!("./fountain-sample.fountain");
        let parsed = parse_fountain(&content);
        let title = if let Some(title) = &parsed.title.title {
            title.to_owned()
        } else { vec![LineContent { content: format!("A Title"), ..Default::default()}] };
        Self {
            content: content.to_owned(),
            title: title.into_iter().map(|c| c.content.to_owned()).collect::<Vec<String>>().join(""),
            parsed: Some(parsed)
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link();

        match msg {
            Msg::UpdateContent(content) => {
                let parsed = parse_fountain(&content);
                if let Some(title) = &parsed.title.title {
                    self.title = title.into_iter().map(|c| c.content.to_owned()).collect::<Vec<String>>().join("");
                }
                self.parsed = Some(parsed);
                self.content = content;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let changed = ctx.link().callback(|value: String| Msg::UpdateContent(value));

        html! {
            <div class="h-screen bg-gray-600 w-full flex flex-row items-center justify-center gap-y-3">
                <div class="h-full w-1/2 flex flex-col justify-center">
                    <Editor changed={changed} content={self.content.to_owned()} title={self.title.to_owned()}/>
                </div>
                <div class="w-1/2 text-gray-100 h-full overflow-y-scroll">
                    // {display}
                    if let Some(parsed) = &self.parsed {
                        <Display script={parsed.clone()} is_editor={true}/>
                    }
                </div>
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
