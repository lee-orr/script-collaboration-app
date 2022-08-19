pub mod formatting_functions;
mod line_editor;
mod title_element_editor;

use formatting_functions::*;
use line_editor::LineEditor;
use title_element_editor::{TitleAlignment, TitleElementEditor};
use wasm_bindgen::JsCast;
use web_sys::{console, HtmlElement, InputEvent};
use yew::{html, Callback, Component, Properties};

use super::types::{CharacterLine, Line, LineContent, Script, TextAlignment};

#[derive(Clone, PartialEq, Properties)]
pub struct DisplayProps {
    pub script: Script,
    pub mode: DisplayMode,
    pub changed_title_element: Callback<(String, String)>,
    pub changed_line: Callback<(usize, String)>,
}

pub struct Display {
    version: usize,
}

fn view_title(
    script: &Script,
    is_editor: bool,
    display_notes: bool,
    callback: Callback<(String, String)>,
) -> yew::Html {
    let mut title_elements: Vec<yew::Html> = Vec::new();
    let title = &script.title;

    title_elements.push(html!(<TitleElementEditor alignment={TitleAlignment::Center} changed={callback.clone()} is_editor={is_editor} tag={"Title".to_owned()} element_text={if let Some(title) = &title.title {
        title.clone()
    } else {
        Vec::new()
    }}/>));

    title_elements.push(html!(<TitleElementEditor alignment={TitleAlignment::Center} changed={callback.clone()} is_editor={is_editor} tag={"Credit".to_owned()} element_text={if let Some(credit) = &title.credit {
        credit.clone()
    } else {
        Vec::new()
    }}/>));

    title_elements.push(html!(<TitleElementEditor alignment={TitleAlignment::Center} changed={callback.clone()} is_editor={is_editor} tag={"Author".to_owned()} element_text={if let Some(author) = &title.author {
        author.clone()
    } else {
        Vec::new()
    }}/>));

    title_elements.push(html!(<TitleElementEditor alignment={TitleAlignment::Center} changed={callback.clone()} is_editor={is_editor} tag={"Source".to_owned()} element_text={if let Some(source) = &title.source {
        source.clone()
    } else {
        Vec::new()
    }}/>));

    title_elements.push(html!(<TitleElementEditor alignment={TitleAlignment::Left} changed={callback.clone()} is_editor={is_editor} tag={"Draft date".to_owned()} element_text={if let Some(draft) = &title.draft {
        draft.clone()
    } else {
        Vec::new()
    }}/>));

    title_elements.push(html!(<TitleElementEditor alignment={TitleAlignment::Left} changed={callback.clone()} is_editor={is_editor} tag={"Contact".to_owned()} element_text={if let Some(contact) = &title.contact {
        contact.clone()
    } else {
        Vec::new()
    }}/>));

    if title_elements.len() > 0 {
        title_elements.push(html!(<div class="border-b flex-grow border-black m-2"><br/></div>))
    }

    html!(<div class="flex flex-col gap-2">{title_elements}</div>)
}

#[derive(PartialEq, Eq)]
enum ChunkRelationship {
    None,
    SameChunk,
    ParallelChunk,
}

fn view_line(
    line: &Line,
    last_line: &Line,
    is_editor: bool,
    display_notes: bool,
    callback: Callback<(usize, String)>,
    line_id: usize
) -> (yew::Html, ChunkRelationship) {
    (
        html!(<LineEditor line_id={line_id} is_editor={is_editor} changed={callback} display_notes={display_notes} line={line.clone()}/>),
        match line {
            Line::CharacterContent(text, line_type, character) => {
                if let Line::CharacterContent(_, _, previous_character) = last_line {
                    if previous_character == character
                        && line_type != &CharacterLine::CharacterHeading(false)
                    {
                        ChunkRelationship::SameChunk
                    } else if line_type == &CharacterLine::CharacterHeading(true) {
                        ChunkRelationship::ParallelChunk
                    } else {
                        ChunkRelationship::None
                    }
                } else {
                    ChunkRelationship::None
                }
            }
            Line::Empty => {
                if let Line::CharacterContent(_, _, _) = last_line {
                    ChunkRelationship::SameChunk
                } else {
                    ChunkRelationship::None
                }
            }
            _ => ChunkRelationship::None,
        },
    )
}

pub enum FountainEditMsg {
    None,
    ChangedTitleElement(String, String),
    ChangedLineElement(usize, String)
}

impl Component for Display {
    type Message = FountainEditMsg;

    type Properties = DisplayProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self { version: 0 }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FountainEditMsg::ChangedTitleElement(tag, title) => {
                ctx.props().changed_title_element.emit((tag, title))
            },
            FountainEditMsg::ChangedLineElement(id, line) => {
                ctx.props().changed_line.emit((id, line))
            },
            _ => {}
        }
        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let DisplayProps {
            script,
            mode,
            changed_line: _,
            changed_title_element:_,
        } = &ctx.props();


        let line_changed = ctx.link().callback(|(id, line): (usize,String)| {
            FountainEditMsg::ChangedLineElement(id, line)
        });

        let title_change = ctx
            .link()
            .callback(|(tag, content): (String, String)| {
                FountainEditMsg::ChangedTitleElement(tag, content)
    });

        let is_editor = mode == &DisplayMode::Editor;
        let display_notes = mode == &DisplayMode::DisplayNotes || mode == &DisplayMode::Editor;
        let mut last_meaningful_line = &Line::Empty;
        let mut last_line = &Line::Empty;
        let lines = script
            .lines
            .iter()
            .enumerate()
            .map(|(line_id, line)| {
                let last = last_meaningful_line;
                if line == &Line::Empty {
                    if last_line == &Line::Empty {
                        last_meaningful_line = &Line::Empty;
                    }
                } else {
                    last_meaningful_line = &line;
                }
                last_line = &line;

                view_line(&line, last, is_editor, display_notes, line_changed.clone(), line_id)
            })
            .collect::<Vec<_>>();

        let mut last_chunk = vec![];
        let mut last_parallel_chunks = vec![];
        let mut chunks = vec![];

        for (line, part_of_previous_chunk) in lines.into_iter() {
            match part_of_previous_chunk {
                ChunkRelationship::ParallelChunk => {
                    last_parallel_chunks.push(last_chunk);
                    last_chunk = vec![line];
                }
                ChunkRelationship::SameChunk => {
                    last_chunk.push(line);
                }
                ChunkRelationship::None => {
                    last_parallel_chunks.push(last_chunk);
                    chunks.push(last_parallel_chunks);
                    last_parallel_chunks = vec![];
                    last_chunk = vec![line];
                }
            }
        }

        last_parallel_chunks.push(last_chunk);
        chunks.push(last_parallel_chunks);

        let chunks = chunks
        .into_iter()
        .map(|chunk| html!(
            <div class="flex flex-row w-full justify-center">
            {chunk
                .into_iter()
                .map(|lines| html!(<div class="flex flex-col items-streach flex-grow">{lines}</div>)).collect::<Vec<_>>()}
            </div>
        )).collect::<Vec<_>>();

        html!(
            <div>
                {view_title(&script, is_editor, display_notes, title_change)}
                {chunks}
            </div>
        )
    }
}
