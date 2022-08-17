extern crate reqwest_wasm;

use std::borrow::Borrow;
use fountain::parser::parse_fountain;
use fountain::types::Script;
use fountain::display::{Display, DisplayMode};
use web_sys::console;
use yew::html::Scope;
use yew::prelude::*;
use crate::editor::Editor;
use crate::fountain::types::LineContent;
use gloo::timers::callback::{Timeout};

mod data;
pub mod fountain;
mod editor;

enum Msg {
    UpdateContent(String),
    ReadyToParse
}

struct App {
    content: String,
    title: String,
    parsed: Option<Script>,
    timeout: Option<Timeout>,
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
            parsed: Some(parsed),
            timeout: None
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link();

        match msg {
            Msg::UpdateContent(content) => {
                self.content = content;

                let mut timeout = self.timeout.take();
                self.timeout = None;

                if let Some(timeout) = timeout{
                    timeout.cancel();
                }

                let handle = {
                    let link = ctx.link().clone();
                    Timeout::new(500, move || link.send_message(Msg::ReadyToParse))
                };

                self.timeout = Some(handle);
                false
            },
            Msg::ReadyToParse => {
                let parsed = parse_fountain(&self.content);
                if let Some(title) = &parsed.title.title {
                    self.title = title.into_iter().map(|c| c.content.to_owned()).collect::<Vec<String>>().join("");
                }
                unsafe {
                    console::log_2(&"Parsed".into(), &serde_json::to_string(&parsed).unwrap().into());
                }
                self.parsed = Some(parsed);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let changed = ctx.link().callback(|value: String| Msg::UpdateContent(value));
        let changed_2 = ctx.link().callback(|value: String| Msg::UpdateContent(value));

        html! {
            <div class="h-screen bg-gray-600 w-full flex flex-row items-center justify-center gap-y-3">

            <div class="w-2/5 text-gray-100 h-full whitespace-pre-wrap">
                <Editor content={self.content.clone()} title={"Editor".to_owned()} changed={changed}/>
            </div>
                 <div class="w-2/5 text-gray-100 h-full overflow-y-scroll">
                    if let Some(parsed) = &self.parsed {
                        <Display changed={changed_2} script={parsed.clone()} mode ={DisplayMode::DisplayNotes}/>
                    } else {
                        <div>{"Failed to parse"}</div>
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
