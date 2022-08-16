use yew::{Properties, Component, html};

use super::types::{Script, Line, TextAlignment, LineContent, CharacterLine};

#[derive(Clone, PartialEq, Properties)]
pub struct DisplayProps {
    pub script: Script
}

pub struct Display {}

fn format_text(text: &Vec<LineContent>) -> yew::Html {
    html!(<span>{text.into_iter().map(|LineContent { content, bold, italic, underline, note }| {
        let mut classes = "".to_owned();
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
            classes = format!("{} {}", classes, "pl-1 pr-1 bg-gray-800 text-gray-400")
        }
        html!(<span class={classes}>{if content == "&nbsp;" { html!(<><br/></>) } else { html!(<>{&content}</>) }}</span>)
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

    html!(<div class="whitespace-pre">{title_elements}</div>)
}

fn view_character_content(line: &(Vec<LineContent>, CharacterLine, String)) -> yew::Html {
    match line {
        (_, CharacterLine::CharacterHeading(_), character) =>  html!(<div class="flex flex-row justify-center uppercase pt-2">{&character}</div>),
        (content, CharacterLine::Dialogue, _) => html!(<div class="flex flex-row justify-center text-center pl-20 pr-20">{format_text(&content)}</div>),
        (content, CharacterLine::Parenthetical, _) => html!(<div class="flex flex-row justify-center text-center">{format_text(&content)}</div>),
        (content, CharacterLine::Lyrics, _) => html!(<div class="flex flex-row justify-center italic"><div class="text-start w-1/2">{format_text(&content)}</div></div>),
        (_, CharacterLine::Empty, _) => html!(<><br/><br/></>),
        _ => html!(<>{"C"}</>)
    }
}

fn separate_character_content(content: &Vec<(Vec<LineContent>, CharacterLine, String)>) -> yew::Html {
    let mut columns : Vec<Vec<yew::Html>> = vec![];
    let mut latest : Vec<&(Vec<LineContent>, CharacterLine, String)> = vec![];

    let mut last_character  = "";
    for line in content {
        if latest.len() == 0 {
            last_character = &line.2;
            latest.push(line);
        } else {
            if last_character == line.2 {
                latest.push(line);
            } else {
                columns.push(latest.into_iter().map(|c| view_character_content(c)).collect::<Vec<yew::Html>>());
                latest = vec![line];
                last_character = &line.2;
            }
        }
    }


    columns.push(latest.into_iter().map(|c| view_character_content(c)).collect::<Vec<yew::Html>>());



    html!(
        <div class="flex flex-row justify-center items-start">
            {columns.into_iter().map(|c| html!(<div class="flex flex-col justify-center">{c}</div>)).collect::<Vec<yew::Html>>()}
        </div>
    )
}

fn view_line(line: &Line) -> yew::Html {
    match line {
        Line::CharacterContent(content) => separate_character_content(content),
        Line::SceneHeading(scene) => html!(<div class="flex flex-row justify-start uppercase pb-2">{scene}</div>),
        Line::Action(action, centered) => if *centered == TextAlignment::Center { html!(<div class="flex flex-row justify-center">{format_text(&action)}</div>) } else { html!(<div class="flex flex-row justify-start">{format_text(&action)}</div>) },
        Line::Transition(transition) => html!(<div class="flex flex-row justify-end uppercase pb-2 pt-2 pr-5 pl-5">{&transition}</div>),
        Line::PageBreak => html!(<div class="border-b flex-grow border-black m-2"/>),
        Line::Boneyard(boneyard) => html!(<div class="flex flex-row justify-right text-gray-400 border-gray-300 m-2 p-2 bg-gray-800">{format_text(&boneyard)}</div>),
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