extern crate reqwest_wasm;

use crate::editor::Editor;
use crate::fountain::types::LineContent;
use fountain::display::{formatting_functions::DisplayMode, Display};
use fountain::exporter::export_fountain;
use fountain::parser::parse_fountain;
use fountain::types::Script;
use gloo::timers::callback::Timeout;
use web_sys::console;
use yew::prelude::*;
use fountain::script_update::*;

mod data;
mod editor;
pub mod fountain;

enum Msg {
    UpdateContent(String),
    SetDisplayMode(DisplayMode),
    UpdateTitle(String, String),
    UpdateLine(usize, String)
}

struct App {
    content: String,
    title: String,
    parsed: Script,
    mode: DisplayMode,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let content = include_str!("./fountain-sample.fountain");
        let parsed = parse_fountain(&content);
        let title = if let Some(title) = &parsed.title.title {
            title.to_owned()
        } else {
            vec![LineContent {
                content: format!("A Title"),
                ..Default::default()
            }]
        };
        Self {
            content: content.to_owned(),
            title: title
                .into_iter()
                .map(|c| c.content.to_owned())
                .collect::<Vec<String>>()
                .join(""),
            parsed: parsed,
            mode: DisplayMode::Editor,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link();

        match msg {
            Msg::UpdateTitle(tag, element) => {
                let parsed = &mut self.parsed;
                let updated = parsed.update_title_element(&tag, element);

                self.content = export_fountain(&updated);
                self.parsed = updated;
                true
            }
            Msg::UpdateLine(id, line) => {
                let parsed = &mut self.parsed;
                let updated =  parsed.update_line(id, &line);
                self.content = export_fountain(&updated);
                self.parsed = updated;
                true
            }
            Msg::UpdateContent(content) => {
                self.content = content;

                let parsed = parse_fountain(&self.content);
                if let Some(title) = &parsed.title.title {
                    self.title = title
                        .into_iter()
                        .map(|c| c.content.to_owned())
                        .collect::<Vec<String>>()
                        .join("");
                }
                unsafe {
                    console::log_2(
                        &"Parsed".into(),
                        &serde_json::to_string(&parsed).unwrap().into(),
                    );
                }
                self.parsed = parsed;
                true
            }
            Msg::SetDisplayMode(mode) => {
                self.mode = mode;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let changed = ctx
            .link()
            .callback(|value: String| Msg::UpdateContent(value));

        let mode = self.mode.clone();
        let set_mode = ctx.link().callback(move |_event: MouseEvent| {
            Msg::SetDisplayMode(if mode == DisplayMode::Editor {
                DisplayMode::Display
            } else {
                DisplayMode::Editor
            })
        });

        let changed_title_element = ctx.link().callback(|(tag, element): (String, String)| Msg::UpdateTitle(tag, element));
        let changed_line = ctx.link().callback(|(id, line): (usize, String)| Msg::UpdateLine(id, line));

        html! {
            <div class="h-screen bg-gray-600 w-full flex flex-row items-center justify-center gap-y-3">
                <div class="fixed top-0.5 right-0.5">
                    <button class="bg-gray-800 hover:bg-gray-700 text-gray-100 p-2" onclick={set_mode}>{ if &self.mode == &DisplayMode::Editor {
                        "Editor"
                    } else {
                        "Display"
                    }}</button>
                </div>
                <div class="w-2/5 text-gray-100 h-full whitespace-pre-wrap">
                    <Editor content={self.content.clone()} title={"Editor".to_owned()} changed={changed}/>
                </div>
                 <div class="w-2/5 text-gray-100 h-full overflow-y-scroll">
                    <Display changed_line={changed_line} changed_title_element={changed_title_element} script={self.parsed.clone()} mode ={self.mode.clone()}/>
                 </div>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
