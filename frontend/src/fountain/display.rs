use yew::{Properties, Component, html};

use super::types::{Script, Line, TextAlignment, LineContent, CharacterLine};

#[derive(Clone, PartialEq, Properties)]
pub struct DisplayProps {
    pub script: Script,
    pub is_editor: bool,
}

pub struct Display {}

fn editor_tag(tag: &str) -> yew::Html {
    html!(<span class="text-gray-400 text-sm">{tag}</span>)
}

fn format_text(text: &Vec<LineContent>, is_editor: bool) -> yew::Html {
    if is_editor {
        let mut last_bold = false;
        let mut last_italic = false;
        let mut last_underline = false;
        let mut last_note = false;
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

            let mut result = html!(<span class={classes}>{if content == "&nbsp;" { html!(<><br/></>) } else { html!(<>{&content}</>) }}</span>);

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

            result
        }).collect::<Vec<_>>()}
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
}

fn view_title(script: &Script, is_editor: bool) -> yew::Html {
    let mut title_elements : Vec<yew::Html> = Vec::new();
    let title = &script.title;

    if let Some(title) = &title.title {
        if is_editor {
            title_elements.push(editor_tag("title: "));
        }
        title_elements.push(html!(<div class="flex flex-row justify-center"><h1>{format_text(&title, is_editor)}</h1></div>))
    }

    if let Some(credit) = &title.credit {
        if is_editor {
            title_elements.push(editor_tag("credit: "));
        }

        title_elements.push(html!(<div class="flex flex-row justify-center">{format_text(&credit, is_editor)}</div>))
    }

    if let Some(author) = &title.author {
        if is_editor {
            title_elements.push(editor_tag("author: "));
        }

        title_elements.push(html!(<div class="flex flex-row justify-center">{format_text(&author, is_editor)}</div>))
    }
    if let Some(source) = &title.source {
        if is_editor {
            title_elements.push(editor_tag("source: "));
        }

        title_elements.push(html!(<div class="flex flex-row justify-center">{format_text(&source, is_editor)}</div>))
    }

    if let Some(date) = &title.draft {
        if is_editor {
            title_elements.push(editor_tag("draft: "));
        }

        title_elements.push(html!(<div class="flex flex-row justify-start">{format_text(&date, is_editor)}</div>))
    }
    if let Some(contact) = &title.contact {
        if is_editor {
            title_elements.push(editor_tag("contact: "));
        }

        title_elements.push(html!(<div class="flex flex-row justify-start">{format_text(&contact, is_editor)}</div>))
    }

    if title_elements.len() > 0 {
        title_elements.push(html!(<div class="border-b flex-grow border-black m-2"/>))
    }

    html!(<div class="whitespace-pre">{title_elements}</div>)
}

fn view_character_content(line: &(Vec<LineContent>, CharacterLine, String), is_editor: bool) -> yew::Html {
    match line {
        (_, CharacterLine::CharacterHeading(_), character) =>  html!(<div class="flex flex-row justify-center uppercase pt-2">{&character}</div>),
        (content, CharacterLine::Dialogue, _) => html!(<div class="flex flex-row justify-center text-center pl-20 pr-20">{format_text(&content, is_editor)}</div>),
        (content, CharacterLine::Parenthetical, _) => html!(<div class="flex flex-row justify-center text-center">{format_text(&content, is_editor)}</div>),
        (content, CharacterLine::Lyrics, _) => html!(<div class="flex flex-row justify-center italic"><div class="text-start w-1/2">{
            if is_editor {
                editor_tag("~")
            } else {
                html!(<></>)
            }
        }{format_text(&content, is_editor)}</div></div>),
        (_, CharacterLine::Empty, _) => html!(<><br/><br/></>),
        _ => html!(<>{"C"}</>)
    }
}

fn separate_character_content(content: &Vec<(Vec<LineContent>, CharacterLine, String)>, is_editor: bool) -> yew::Html {
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
                columns.push(latest.into_iter().map(|c| view_character_content(c, is_editor)).collect::<Vec<yew::Html>>());
                latest = vec![line];
                last_character = &line.2;
            }
        }
    }


    columns.push(latest.into_iter().map(|c| view_character_content(c, is_editor)).collect::<Vec<yew::Html>>());



    html!(
        <div class="flex flex-row justify-center items-start">
            {columns.into_iter().map(|c| html!(<div class="flex flex-col justify-center">{c}</div>)).collect::<Vec<yew::Html>>()}
        </div>
    )
}

fn view_line(line: &Line, is_editor: bool) -> yew::Html {
    match line {
        Line::CharacterContent(content) => separate_character_content(content, is_editor),
        Line::SceneHeading(scene) => html!(<div class="flex flex-row justify-start uppercase pb-2">{display_scene_heading(scene, is_editor)}</div>),
        Line::Action(action, centered) => if *centered == TextAlignment::Center { html!(<div class="flex flex-row justify-center">{format_text(&action, is_editor)}</div>) } else { html!(<div class="flex flex-row justify-start">{format_text(&action, is_editor)}</div>) },
        Line::Transition(transition) => html!(<div class="flex flex-row justify-end uppercase pb-2 pt-2 pr-5 pl-5">{display_transition(transition, is_editor)}</div>),
        Line::PageBreak => {
            if is_editor {
                html!(<>{editor_tag("===")}<div class="border-b flex-grow border-black m-2"/></>)
            } else {
                html!(<div class="border-b flex-grow border-black m-2"/>)
            }
        },
        Line::Boneyard(boneyard) => html!(<div class="flex flex-row justify-right text-gray-400 border-gray-300 m-2 p-2 bg-gray-800">{"/*"}{format_text(&boneyard, is_editor)}{"*/"}</div>),
        Line::Empty => html!(<br/>),
        _ => html!(<>{"a"}</>)
    }
}

fn display_scene_heading(line: &String, is_editor: bool) -> yew::Html {
    if !is_editor || line.starts_with("INT")
    || line.starts_with("EXT")
    || line.starts_with("EST")
    || line.starts_with("I/E")
    || line.starts_with("int")
    || line.starts_with("ext")
    || line.starts_with("est")
    || line.starts_with("i/e") {
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
    type Message = ();

    type Properties = DisplayProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let DisplayProps {script, is_editor} = &ctx.props();
        let lines = script.lines.iter().map(|line| view_line(&line, *is_editor)).collect::<Vec<_>>();
        html!(<>
            {view_title(&script, *is_editor)}
            {lines}
        </>)
    }
}