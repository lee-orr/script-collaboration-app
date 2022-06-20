use yew::{Properties, Component, html};

use super::types::{Script, Line};

#[derive(Clone, PartialEq, Properties)]
pub struct DisplayProps {
    pub script: Script
}

pub struct Display {}

fn format_text(text: &str) -> yew::Html {
    html!(<>{text}</>)
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
        Line::Action(action) => html!(<div class="flex flex-row justify-start">{format_text(&action)}</div>),
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