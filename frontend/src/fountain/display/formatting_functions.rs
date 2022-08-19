use yew::{html, Properties};

use super::super::types::LineContent;

#[derive(Clone, PartialEq)]
pub enum DisplayMode {
    Display,
    DisplayNotes,
    Editor,
}

pub enum EditorMsg {
    None,
    ChangedContent(String),
    ReadyToParse,
}

pub(crate) fn editor_tag(tag: &str) -> yew::Html {
    html!(<span class="text-gray-400 text-sm">{tag}</span>)
}

pub(crate) fn format_text(text: &Vec<LineContent>, is_editor: bool, display_notes: bool) -> yew::Html {
    if is_editor {
        let mut last_bold = false;
        let mut last_italic = false;
        let mut last_underline = false;
        let mut last_note = false;
        html!(<span>{text.into_iter().map(|LineContent { content, bold, italic, underline, note, raw_content }| {
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
                classes = format!("{} {}", classes, "pl-1 pr-1 bg-gray-800 text-gray-400");
            }

            let mut result = html!(<span class={classes}>{if content == "&nbsp;" { editor_tag(content) } else { html!(<>{&raw_content}</>) }}</span>);

            Some(result)
        }).flatten().collect::<Vec<_>>()}</span>)
    } else {
        html!(<span>{text.into_iter().map(|LineContent { content, bold, italic, underline, note, raw_content }| {
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
                    return html!(<></>);
                } else {
                    classes = format!("{} {}", classes, "pl-1 pr-1 bg-gray-800 text-gray-400");
                }
            }
            html!(<span class={classes}>{if content == "&nbsp;" { html!(<><br/></>) } else { html!(<>{&content}</>) }}</span>)
        }).collect::<Vec<_>>()}</span>)
    }
}
