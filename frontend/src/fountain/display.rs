use yew::{Properties, Component, html};

use super::types::{Script, Line};

#[derive(Clone, PartialEq, Properties)]
pub struct DisplayProps {
    pub script: Script
}

pub struct Display {}

fn format_text(text: &str) -> yew::Html {
    let mut process_underline = false;
    let processing = text.replace("&nbsp;","\n").split("_").map(|v| {
        if v.len() == 0 {
            (v.to_owned(), false)
        } else if v.ends_with("\\") {
            (format!("{}_", v[0..v.len() - 1].to_owned()), process_underline)
        } else {
            process_underline = if process_underline { false} else { true};
            (v.to_owned(), !process_underline)
        }
    }).filter(|v| v.0.len() > 0).collect::<Vec<(String, bool)>>();

    let mut process_bold = false;
    let processing = processing.into_iter().enumerate().map(|(segment,(line, underline))| {
        let line = if line.starts_with("**") { format!(" {}", line)} else { line };
        let line = if line.ends_with("**") { format!("{} ", line)} else { line };
        let split = line.split("**").collect::<Vec<_>>();
        let len = split.len();
        split.into_iter().enumerate().map(|(i,v)| {
            if i == len - 1 {
                (v.to_owned(), underline, process_bold)
            } else if v.len() == 0 {
                (v.to_owned(), false, false)
            } else if v.ends_with("\\") {
                (format!("{}**", v[0..v.len() - 1].to_owned()), underline, process_bold)
            } else {
                process_bold = if process_bold { false} else { true};
                (v.to_owned(), underline, !process_bold)
            }
        }).collect::<Vec<(String, bool, bool)>>()
    }).flatten().filter(|v| v.0.len() > 0).collect::<Vec<(String, bool, bool)>>();

    let mut process_italic = false;
    let processing = processing.into_iter().map(|(line, underline, bold)| {
        let line = if line.starts_with("*") { format!(" {}", line)} else { line };
        let line = if line.ends_with("*") { format!("{} ", line)} else { line };
        let split = line.split("*").collect::<Vec<_>>();
        let len = split.len();
        split.into_iter().enumerate().map(|(i,v)| {
            if i == len - 1 {
                (v.to_owned(), underline, bold, process_italic)
            } else if v.len() == 0 {
                (v.to_owned(), false, false, false)
            } else if v.ends_with("\\") {
                (format!("{}*", v[0..v.len() - 1].to_owned()), underline, bold, process_italic)
            } else {
                process_italic = if process_italic { false} else { true};
                (v.to_owned(), underline, bold, !process_italic)
            }
        }).collect::<Vec<(String, bool, bool, bool)>>()
    }).flatten().filter(|v| v.0.len() > 0).collect::<Vec<(String, bool, bool, bool)>>();

    let mut processing_note = false;
    let processing = processing.into_iter().map(|(line, underline, bold, italic)| {
        let line = if line.starts_with("[[") { format!(" {}", line)} else { line };
        let line = if line.ends_with("]]") { format!("{} ", line)} else { line };
        let split = line.split("[[").collect::<Vec<_>>();
        let len = split.len();
        split.into_iter().enumerate().map(|(i,v)| {
            let split = v.split("]]").collect::<Vec<_>>();
            if split.len() <= 1 {
                vec!([(v.to_owned(), underline, bold, italic, false)])
            } else {
                split.into_iter().enumerate().map(|(i, v)| {
                    if i == 0 {
                        [(v.to_owned(), underline, bold, italic, true)]
                    } else {
                        [(v.to_owned(), underline, bold, italic, false)]
                    }
                }).collect::<Vec<_>>()
            }
        }).flatten().flatten().collect::<Vec<(String, bool, bool, bool, bool)>>()
    }).flatten().filter(|v| v.0.len() > 0).collect::<Vec<(String, bool, bool, bool, bool)>>();

    html!(<span>{processing.into_iter().map(|(line, underline, bold, italic, note)| {
        let mut classes = "".to_owned();
        if underline {
            classes = format!("{} {}", classes, "underline");
        }
        if bold {
            classes = format!("{} {}", classes, "font-bold");
        }
        if italic {
            classes = format!("{} {}", classes, "italic");
        }
        if note {
            classes = format!("{} {}", classes, "pl-1 pr-1 bg-gray-800 text-gray-400")
        }
        html!(<span class={classes}>{line}</span>)
    }).collect::<Vec<_>>()}</span>)
}

fn view_title(script: &Script) -> yew::Html {
    let mut title_elements : Vec<yew::Html> = Vec::new();
    let title = &script.title;

    if let Some(title) = &title.title {
        title_elements.push(html!(<div class="flex flex-row justify-center"><h1>{format_text(&title)}</h1></div>))
    }

    if let Some(credit) = &title.credit {
        title_elements.push(html!(<div class="flex flex-row justify-center">{format_text(&credit)}</div>))
    }

    if let Some(author) = &title.author {
        title_elements.push(html!(<div class="flex flex-row justify-center">{format_text(&author)}</div>))
    }
    if let Some(source) = &title.source {
        title_elements.push(html!(<div class="flex flex-row justify-center">{format_text(&source)}</div>))
    }

    if let Some(date) = &title.draft {
        title_elements.push(html!(<div class="flex flex-row justify-start">{format_text(&date)}</div>))
    }
    if let Some(contact) = &title.contact {
        title_elements.push(html!(<div class="flex flex-row justify-start">{format_text(&contact)}</div>))
    }

    if title_elements.len() > 0 {
        title_elements.push(html!(<div class="border-b flex-grow border-black m-2"/>))
    }

    html!(<>{title_elements}</>)
}

fn view_line(line: &Line) -> yew::Html {
    match line {
        Line::Parenthetical(dialogue) => html!(<div class="flex flex-row justify-center">{format_text(&dialogue)}</div>),
        Line::Dialogue(dialogue, _) => html!(<div class="flex flex-row justify-center text-center pl-20 pr-20">{format_text(&dialogue)}</div>),
        Line::Character(character) => html!(<div class="flex flex-row justify-center uppercase pt-2">{format_text(&character)}</div>),
        Line::SceneHeading(scene) => html!(<div class="flex flex-row justify-start uppercase pb-2">{format_text(&scene)}</div>),
        Line::Action(action, centered) => if *centered { html!(<div class="flex flex-row justify-center">{format_text(&action)}</div>) } else { html!(<div class="flex flex-row justify-start">{format_text(&action)}</div>) },
        Line::Lyrics(lyric, _) => html!(<div class="flex flex-row justify-center italic"><div class="text-start w-1/2">{format_text(&lyric)}</div></div>),
        Line::Transition(transition) => html!(<div class="flex flex-row justify-end uppercase pb-2 pt-2 pr-5 pl-5">{format_text(&transition)}</div>),
        Line::PageBreak => html!(<div class="border-b flex-grow border-black m-2"/>),
        Line::Boneyard(boneyard) => html!(<div class="flex flex-row justify-right text-gray-400 border-gray-300 m-2 p-2 bg-gray-800">{boneyard}</div>),
        Line::Empty => html!(<br/>),
        _ => html!(<>{"a"}</>)
    }
}

impl Component for Display {
    type Message = ();

    type Properties = DisplayProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let script = &ctx.props().script;
        let lines = script.lines.iter().map(|line| view_line(&line)).collect::<Vec<_>>();
        html!(<>
            {view_title(&script)}
            {lines}
        </>)
    }
}