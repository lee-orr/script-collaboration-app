use wasm_bindgen::JsCast;
use web_sys::{console, HtmlElement, InputEvent};
use yew::{html, virtual_dom::VNode, Callback, Component, Properties};

use super::types::{CharacterLine, Line, LineContent, Script, TextAlignment};

#[derive(Clone, PartialEq)]
pub enum DisplayMode {
    Display,
    DisplayNotes,
    Editor,
}

pub enum EditorMsg {
    None,
    ChangedContent(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct DisplayProps {
    pub script: Script,
    pub mode: DisplayMode,
    pub changed: Callback<String>,
}

pub struct Display {
    version: usize,
}

fn editor_tag(tag: &str) -> yew::Html {
    html!(<span class="text-gray-400 text-sm">{tag}</span>)
}

fn format_text(text: &Vec<LineContent>, is_editor: bool, display_notes: bool) -> yew::Html {
    if is_editor {
        let mut last_bold = false;
        let mut last_italic = false;
        let mut last_underline = false;
        let mut last_note = false;
        html!(<span>{text.into_iter().map(|LineContent { content, bold, italic, underline, note }| {
            let mut classes = "whitespace-pre-wraped".to_owned();
            if *underline {
                classes = format!("{} {}", classes, "underline");
            }
            if *bold {
                classes = format!("{} {}", classes, "font-bold");
            }
            if *italic {
                classes = format!("{} {}", classes, "italic");
            }
            if *note {
                if !display_notes {
                    classes = format!("{} {}", classes, "pl-1 pr-1 bg-red-800 text-red-400");
                } else {
                    classes = format!("{} {}", classes, "pl-1 pr-1 bg-gray-800 text-gray-400");
                }
                
            }

            let mut result = html!(<span class={classes}>{if content == "&nbsp;" { editor_tag(content) } else { html!(<>{&content}</>) }}</span>);

            if *underline != last_underline {
                last_underline = *underline;
                result = html!(<>{editor_tag("_")}{result}</>);
            }
            if *bold != last_bold {
                last_bold = *bold;
                result = html!(<>{editor_tag("**")}{result}</>);
            }
            if *italic != last_italic {
                last_italic = *italic;
                result = html!(<>{editor_tag("*")}{result}</>);
            }
            if *note && !last_note {
                last_note = true;
                result = html!(<>{editor_tag("[[")}{result}</>);
            }
            if !*note && last_note {
                last_note = true;
                result = html!(<>{editor_tag("]]")}{result}</>);
            }

            Some(result)
        }).flatten().collect::<Vec<_>>()}
        {
            if last_note {
                editor_tag("]]")
            } else {
                html!(<></>)
            }
        }
        {
            if last_underline {
                editor_tag("__")
            } else {
                html!(<></>)
            }
        }
        {
            if last_bold {
                editor_tag("**")
            } else {
                html!(<></>)
            }
        }
        {
            if last_italic {
                editor_tag("*")
            } else {
                html!(<></>)
            }
        }</span>)
    } else {
        html!(<span>{text.into_iter().map(|LineContent { content, bold, italic, underline, note }| {
            let mut classes = "whitespace-pre ".to_owned();
            if *underline {
                classes = format!("{} {}", classes, "underline");
            }
            if *bold {
                classes = format!("{} {}", classes, "font-bold");
            }
            if *italic {
                classes = format!("{} {}", classes, "italic");
            }
            if *note {
                if !display_notes {
                    return html!(<></>);
                } else {
                    classes = format!("{} {}", classes, "pl-1 pr-1 bg-gray-800 text-gray-400");
                }
            }
            html!(<span class={classes}>{if content == "&nbsp;" { html!(<><br/></>) } else { html!(<>{&content}</>) }}</span>)
        }).collect::<Vec<_>>()}</span>)
    }
}

fn view_title(script: &Script, is_editor: bool, display_notes: bool) -> yew::Html {
    let mut title_elements: Vec<yew::Html> = Vec::new();
    let title = &script.title;

    if let Some(title) = &title.title {
        if is_editor {
            title_elements.push(editor_tag("Title: "));
        }

        title_elements.push(html!(<span class="flex flex-row justify-center">{format_text(&title, is_editor, display_notes)}</span>))
    }

    if let Some(credit) = &title.credit {
        if is_editor {
            title_elements.push(editor_tag("Credit: "));
        }

        title_elements.push(html!(<span class="flex flex-row justify-center">{format_text(&credit, is_editor, display_notes)}</span>))
    }

    if let Some(author) = &title.author {
        if is_editor {
            title_elements.push(editor_tag("Author: "));
        }

        title_elements.push(html!(<span class="flex flex-row justify-center">{format_text(&author, is_editor, display_notes)}</span>))
    }
    if let Some(source) = &title.source {
        if is_editor {
            title_elements.push(editor_tag("Source: "));
        }

        title_elements.push(html!(<span class="flex flex-row justify-center">{format_text(&source, is_editor, display_notes)}</span>))
    }

    if let Some(date) = &title.draft {
        if is_editor {
            title_elements.push(editor_tag("Draft date: "));
        }

        title_elements.push(html!(<span class="flex flex-row justify-start">{format_text(&date, is_editor, display_notes)}</span>))
    }
    if let Some(contact) = &title.contact {
        if is_editor {
            title_elements.push(editor_tag("Contact: "));
        }

        title_elements.push(html!(<span class="flex flex-row justify-start">{format_text(&contact, is_editor, display_notes)}</span>))
    }

    if title_elements.len() > 0 {
        title_elements.push(html!(<div class="border-b flex-grow border-black m-2"><br/></div>))
    }

    html!(<div>{title_elements}</div>)
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
) -> (yew::Html, ChunkRelationship) {
    match line {
        Line::CharacterContent(text, line_type, character) => (
            view_character_content((&text, &line_type, &character), is_editor, display_notes),
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
            },
        ),
        Line::SceneHeading(scene) => (
            html!(<div class="flex flex-row justify-start uppercase pb-2">{display_scene_heading(scene, is_editor)}</div>),
            ChunkRelationship::None,
        ),
        Line::Action(action, centered) => (
            if *centered == TextAlignment::Center {
                html!(<div class="flex flex-row justify-center">{format_text(&action, is_editor, display_notes)}</div>)
            } else {
                html!(<div class="flex flex-row justify-start">{format_text(&action, is_editor, display_notes)}</div>)
            },
            ChunkRelationship::None,
        ),
        Line::Transition(transition) => (
            html!(<div class="flex flex-row justify-end uppercase pb-2 pt-2 pr-5 pl-5">{display_transition(transition, is_editor)}</div>),
            ChunkRelationship::None,
        ),
        Line::PageBreak => (
            if is_editor {
                html!(<>{editor_tag("===")}<div class="border-b flex-grow border-black m-2"/></>)
            } else {
                html!(<div class="border-b flex-grow border-black m-2"/>)
            },
            ChunkRelationship::None,
        ),
        Line::Boneyard(boneyard) => (
            if display_notes {
                html!(<div class="flex flex-row justify-right text-gray-400 border-gray-300 m-2 p-2 bg-gray-800">{if is_editor { "/*"} else { "" }}{format_text(&boneyard, is_editor, display_notes)}{if is_editor { "*/"} else { "" }}</div>)
            } else {
                html!(<></>)
            },
            ChunkRelationship::None,
        ),
        Line::Empty => (html!(<div>{"\u{00a0}"}</div>), if let Line::CharacterContent(_, _, _) = last_line {
                ChunkRelationship::SameChunk
        } else {
            ChunkRelationship::None
        }),
        _ => (html!(<>{"a"}</>), ChunkRelationship::None),
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

impl Component for Display {
    type Message = EditorMsg;

    type Properties = DisplayProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self { version: 0 }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            EditorMsg::ChangedContent(content) => {
                unsafe {
                    console::log_2(&"Content".into(), &content.clone().into());
                }
                ctx.props().changed.emit(content);
                self.version += 1;
            }
            _ => {}
        }
        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let DisplayProps {
            script,
            mode,
            changed,
        } = &ctx.props();
        let is_editor = mode == &DisplayMode::Editor;
        let display_notes = mode == &DisplayMode::DisplayNotes || mode == &DisplayMode::Editor;
        let mut last_meaningful_line = &Line::Empty;
        let mut last_line = &Line::Empty;
        let lines = script
            .lines
            .iter()
            .map(|line| {
                let last = last_meaningful_line;
                if line == &Line::Empty {
                    if last_line == &Line::Empty {
                        last_meaningful_line = &Line::Empty;
                    }
                } else {
                    last_meaningful_line = &line;
                }
                last_line = &line;
                
                view_line(&line, last, is_editor, display_notes)
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

        let onchange = ctx.link().callback(|event: InputEvent| {
            let target = event.target();
            if let Some(target) = target {
                let input = target.dyn_into::<HtmlElement>();
                if let Ok(input) = input {
                    let mut multiple_empty_lines = false;
                    let text: String = input.inner_text();
                    let filtered = text
                        .lines()
                        .into_iter()
                        .filter(|a| {
                            if a.is_empty() {
                                if multiple_empty_lines {
                                    return false;
                                } else {
                                    multiple_empty_lines = true;
                                    return true;
                                }
                            } else {
                                multiple_empty_lines = false;
                                return true;
                            }
                        })
                        .collect::<Vec<&str>>()
                        .join("\n");
                    return EditorMsg::ChangedContent(filtered);
                }
            }
            EditorMsg::None
        });

        html!(
            <div contenteditable={if is_editor {"true"} else {"false"}} oninput={onchange}>
                {view_title(&script, is_editor, display_notes)}
                {chunks}
            </div>
        )
    }
}
