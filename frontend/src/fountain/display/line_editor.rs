use gloo::timers::callback::Timeout;
use wasm_bindgen::JsCast;
use web_sys::{InputEvent, HtmlElement};
use yew::{html, Component, Properties, Callback};

use crate::fountain::types::{Line, TextAlignment, CharacterLine,LineContent};

use super::formatting_functions::{editor_tag,format_text,EditorMsg};

#[derive(Clone, PartialEq, Properties)]
pub struct LineEditorProps {
    pub is_editor: bool,
    pub display_notes: bool,
    pub line: Line,
    pub line_id: usize,
    pub changed: Callback<(usize, String)>
}

pub struct LineEditor {
    timeout: Option<Timeout>,
    inner_content: Option<String>}

impl Component for LineEditor {
    type Message = EditorMsg;
    type Properties = LineEditorProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            timeout: None,
            inner_content: None
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            EditorMsg::ChangedContent(content) => {
                self.inner_content = Some(content);
                let timeout = self.timeout.take();
                self.timeout = None;

                if let Some(timeout) = timeout {
                    timeout.cancel();
                }

                let handle = {
                    let link = ctx.link().clone();
                    Timeout::new(2000, move || link.send_message(EditorMsg::ReadyToParse))
                };

                self.timeout = Some(handle);
            },

            EditorMsg::ReadyToParse => {
                if let Some(content) = &self.inner_content {
                    let line_id = ctx.props().line_id;
                    let callback = &ctx.props().changed;
                    callback.emit((line_id, content.clone()));
                }
            }
            _ => {}
        }
        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let LineEditorProps {
            is_editor,
            display_notes,
            line,
            line_id,
            changed
        } = ctx.props();

        let is_editor = *is_editor;
        let display_notes = *display_notes;

        let onchange = ctx.link().callback(|event: InputEvent| {
            let target = event.target();
            if let Some(target) = target {
                let input = target.dyn_into::<HtmlElement>();
                if let Ok(input) = input {
                    return EditorMsg::ChangedContent(input.inner_text());
                }
            }
            EditorMsg::None
        });

        html!(
        <div class="flex flex-row whitespace-pre-wrap" contenteditable={if is_editor {"true"} else {"false"}} oninput={onchange}>
            {
                match line {
                    Line::CharacterContent(text, line_type, character) => 
                        view_character_content((&text, &line_type, &character), is_editor, display_notes),
                    Line::SceneHeading(scene) => 
                        html!(<div class="flex flex-row justify-start uppercase pb-2">{display_scene_heading(scene, is_editor)}</div>),
                    Line::Action(action, centered) => 
                        if *centered == TextAlignment::Center {
                            html!(<div class="flex flex-row justify-center">{format_text(&action, is_editor, display_notes)}</div>)
                        } else {
                            html!(<div class="flex flex-row justify-start">{format_text(&action, is_editor, display_notes)}</div>)
                        },
                    Line::Action(action, centered) => 
                        if *centered == TextAlignment::Center {
                            html!(<div class="flex flex-row justify-center">{format_text(&action, is_editor, display_notes)}</div>)
                        } else {
                            html!(<div class="flex flex-row justify-start">{format_text(&action, is_editor, display_notes)}</div>)
                        },
                    Line::Transition(transition) => 
                        html!(<div class="flex flex-row justify-end uppercase pb-2 pt-2 pr-5 pl-5">{display_transition(transition, is_editor)}</div>),
                    Line::PageBreak => 
                    if is_editor {
                        html!(<>{editor_tag("===")}<div class="border-b flex-grow border-black m-2"/></>)
                    } else {
                        html!(<div class="border-b flex-grow border-black m-2"/>)
                    },
                    Line::Boneyard(boneyard) => 
                    if display_notes {
                        html!(<div class="flex flex-row justify-right text-gray-400 border-gray-300 m-2 p-2 bg-gray-800">{if is_editor { "/*"} else { "" }}{format_text(&boneyard, is_editor, display_notes)}{if is_editor { "*/"} else { "" }}</div>)
                    } else {
                        html!(<></>)
                    }
                    Line::Empty => html!(<span>{"\u{00a0}"}</span>),
                    _ => html!(<>{" "}</>)
                }
            }
        </div>)
    }
}

fn view_character_content(
    line: (&Vec<LineContent>, &CharacterLine, &str),
    is_editor: bool,
    display_notes: bool,
) -> yew::Html {
    match line {
        (_, CharacterLine::CharacterHeading(is_dual), character) => {
            html!(<div class="flex flex-row justify-center uppercase pt-2">{&character}{if is_editor && *is_dual { editor_tag("^")} else { html!(<></>) }}</div>)
        }
        (content, CharacterLine::Dialogue, _) => {
            html!(<div class="flex flex-row justify-center text-center pl-20 pr-20">{format_text(&content, is_editor, display_notes)}</div>)
        }
        (content, CharacterLine::Parenthetical, _) => {
            html!(<div class="flex flex-row justify-center text-center">{format_text(&content, is_editor, display_notes)}</div>)
        }
        (content, CharacterLine::Lyrics, _) => {
            html!(<div class="flex flex-row justify-center italic"><div class="text-start w-1/2">{
            if is_editor {
                editor_tag("~")
            } else {
                html!(<></>)
            }
        }{format_text(&content, is_editor, display_notes)}</div></div>)
        }
        _ => html!(<>{"C"}</>),
    }
}

fn display_scene_heading(line: &String, is_editor: bool) -> yew::Html {
    if !is_editor
        || line.starts_with("INT")
        || line.starts_with("EXT")
        || line.starts_with("EST")
        || line.starts_with("I/E")
        || line.starts_with("int")
        || line.starts_with("ext")
        || line.starts_with("est")
        || line.starts_with("i/e")
    {
        html!(<>{line}</>)
    } else {
        html!(<>{editor_tag(".")}{line}</>)
    }
}

fn display_transition(line: &String, is_editor: bool) -> yew::Html {
    if !is_editor || line.ends_with("TO:") {
        html!(<>{line}</>)
    } else {
        html!(<>{editor_tag(">")}{line}</>)
    }
}