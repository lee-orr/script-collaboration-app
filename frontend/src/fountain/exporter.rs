use serde::de::IntoDeserializer;

use super::types::{Line, LineContent, Script, Title, TextAlignment};

pub fn export_fountain(source: &Script) -> String {
    let title = export_title(&source.title);
    let content = export_lines(&source.lines);
    let mut result = Vec::<String>::new();
    if title != "" {
        result.push(title);
    }
    if content != "" {
        result.push(content);
    }
    result.join("\n")
}

fn export_text_content(content: &Vec<LineContent>) -> String {
    content
        .into_iter()
        .map(|content| content.content.to_owned())
        .collect::<Vec<String>>()
        .join("")
}

fn export_title(title: &Title) -> String {
    let mut result: Vec<String> = Vec::new();

    if let Some(title) = &title.title {
        result.push(format!("Title: {}", export_text_content(title)));
    }

    if let Some(credit) = &title.credit {
        result.push(format!("Credit: {}", export_text_content(credit)));
    }

    if let Some(author) = &title.author {
        result.push(format!("Author: {}", export_text_content(author)));
    }

    if let Some(source) = &title.source {
        result.push(format!("Source: {}", export_text_content(source)));
    }

    if let Some(draft) = &title.draft {
        result.push(format!("Draft: {}", export_text_content(draft)));
    }

    if let Some(contact) = &title.contact {
        result.push(format!("Contact: {}", export_text_content(contact)));
    }

    result.join("\n")
}

fn export_line_content (line_content: &Vec<LineContent>) -> String {
    line_content.into_iter().map(|line| line.raw_content.clone()).collect::<String>()
}

fn export_lines(lines: &Vec<Line>) -> String {
    lines
        .into_iter()
        .map(|line| export_line(line))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn export_line(line: &Line) -> String {
    match line {
        Line::Action(line, alignment) => {
            let content = export_line_content(line);

            match alignment {
                TextAlignment::Left => content,
                TextAlignment::Center => format!("> {} <", content)
            }
        },
        Line::SceneHeading(heading) => if heading.starts_with("INT")
        || heading.starts_with("EXT")
        || heading.starts_with("EST")
        || heading.starts_with("I/E")
        || heading.starts_with("int")
        || heading.starts_with("ext")
        || heading.starts_with("est")
        || heading.starts_with("i/e")
        || heading.starts_with(".") { heading.clone() } else { format!(".{}", heading) },
        Line::PageBreak => "===".to_owned(),
        Line::Transition(transition) => if transition.starts_with(">") { transition.clone() } else if transition.ends_with("TO:") {
            transition.to_uppercase().clone()
        } else { format!(">{}", transition) },
        Line::CharacterContent(content, line_type, character) => {
            let mut content = export_line_content(content);
            match line_type {
                super::types::CharacterLine::CharacterHeading(is_dual) => {
                    let content = content.to_uppercase().clone();
                    if *is_dual {
                        format!("{} ^", content)
                    } else {
                        content
                    }
                },
                super::types::CharacterLine::Dialogue => content,
                super::types::CharacterLine::Parenthetical => {
                    if !content.starts_with("(") {
                        content = format!("({}", content);
                    }
                    if !content.ends_with(")") {
                        content = format!("{})", content);
                    }
                    content
                },
                super::types::CharacterLine::Lyrics => {
                    if !content.starts_with("~") {
                        format!("~{}", content)
                    } else {
                        content
                    }
                },
            }
        },
        Line::Boneyard(content) => format!("/*\n{}\n*/", export_line_content(content)),
        Line::Section(_) => todo!(),
        Line::Synopsis(_) => todo!(),
        Line::Empty => "".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::export_fountain;
    use crate::fountain::types::{Line, LineContent, Script, Title, TextAlignment, CharacterLine};

    #[test]
    fn empty_script_results_in_empty_fountain() {
        let script = Script {
            title: Title::default(),
            lines: vec![],
        };

        let result = export_fountain(&script);

        assert_eq!("", result);
    }

    #[test]
    fn when_there_is_a_title_it_is_added_to_the_script() {
        let script = Script {
            title: Title {
                title: Some(vec![LineContent {
                    content: "A Show".to_owned(),
                    ..Default::default()
                }]),
                credit: Some(vec![LineContent {
                    content: "written by".to_owned(),
                    ..Default::default()
                }]),
                author: Some(vec![LineContent {
                    content: "An author".to_owned(),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            lines: vec![],
        };

        let result = export_fountain(&script);

        assert_eq!(
            "Title: A Show
Credit: written by
Author: An author",
            result
        );
    }

    #[test]
    fn exports_action_lines() {
        let script = Script {
            title: Title::default(),
            lines: vec![Line::Action(vec![LineContent { raw_content: "test me".to_owned(), ..Default::default() }], TextAlignment::Left)],
        };

        let result = export_fountain(&script);

        assert_eq!("test me", result);
    }

    #[test]
    fn exports_centered_action_lines() {
        let script = Script {
            title: Title::default(),
            lines: vec![Line::Action(vec![LineContent { raw_content: "test me".to_owned(), ..Default::default() }], TextAlignment::Center)],
        };

        let result = export_fountain(&script);

        assert_eq!("> test me <", result);
    }

    #[test]
    fn exports_empty_lines() {
        let script = Script {
            title: Title::default(),
            lines: vec![
                Line::Action(vec![LineContent { raw_content: "line".to_owned(), ..Default::default() }], TextAlignment::Left),
                Line::Empty,
                Line::Action(vec![LineContent { raw_content: "line 2".to_owned(), ..Default::default() }], TextAlignment::Left)]
        };

        let result = export_fountain(&script);

        assert_eq!("line

line 2", result);
    }

    #[test]
    fn exports_scene_headings() {
        let script = Script {
            title: Title::default(),
            lines: vec![Line::SceneHeading("Scene 1".to_owned()),Line::SceneHeading(".Scene 2".to_owned()), Line::SceneHeading("INT Scene 3".to_owned())],
        };

        let result = export_fountain(&script);

        assert_eq!(".Scene 1
.Scene 2
INT Scene 3", result);
    }

    #[test]
    fn exports_page_breaks() {
        let script = Script {
            title: Title::default(),
            lines: vec![
                Line::Action(vec![LineContent { raw_content: "line".to_owned(), ..Default::default() }], TextAlignment::Left),
                Line::PageBreak,
                Line::Action(vec![LineContent { raw_content: "line 2".to_owned(), ..Default::default() }], TextAlignment::Left)]
        };

        let result = export_fountain(&script);

        assert_eq!("line
===
line 2", result);
    }

    #[test]
    fn exports_transitions() {
        let script = Script {
            title: Title::default(),
            lines: vec![Line::Transition("Scene 1".to_owned()),Line::Transition("> Scene 2".to_owned()), Line::Transition("Scene 3 TO:".to_owned())],
        };

        let result = export_fountain(&script);

        assert_eq!(">Scene 1
> Scene 2
SCENE 3 TO:", result);
    }

    #[test]
    fn exports_boneyard() {
        let script = Script {
            title: Title::default(),
            lines: vec![Line::Boneyard(vec![LineContent { raw_content: "some content".to_owned(), ..Default::default() }])],
        };

        let result = export_fountain(&script);

        assert_eq!("/*
some content
*/", result);
    }

    #[test]
    fn exports_character_heading() {
        let script = Script {
            title: Title::default(),
            lines: vec![
                Line::CharacterContent(vec![LineContent { raw_content: "Character".to_owned(), ..Default::default() }], CharacterLine::CharacterHeading(false), "CHARACTER".to_owned()),
                Line::Empty,
                Line::CharacterContent(vec![LineContent { raw_content: "Character".to_owned(), ..Default::default() }], CharacterLine::CharacterHeading(true), "CHARACTER".to_owned())
            ],
        };

        let result = export_fountain(&script);

        assert_eq!("CHARACTER\n\nCHARACTER ^", result);
    }

    #[test]
    fn exports_dialogue() {
        let script = Script {
            title: Title::default(),
            lines: vec![
                Line::CharacterContent(vec![LineContent { raw_content: "dialogue".to_owned(), ..Default::default() }], CharacterLine::Dialogue, "CHARACTER".to_owned())            ],
        };

        let result = export_fountain(&script);

        assert_eq!("dialogue", result);
    }

    #[test]
    fn exports_parentheticals() {
        let script = Script {
            title: Title::default(),
            lines: vec![
                Line::CharacterContent(vec![LineContent { raw_content: "parens".to_owned(), ..Default::default() }], CharacterLine::Parenthetical, "CHARACTER".to_owned()),
                Line::CharacterContent(vec![LineContent { raw_content: "(parens)".to_owned(), ..Default::default() }], CharacterLine::Parenthetical, "CHARACTER".to_owned())
            ],
        };

        let result = export_fountain(&script);

        assert_eq!("(parens)\n(parens)", result);
    }

    #[test]
    fn exports_lyrics() {
        let script = Script {
            title: Title::default(),
            lines: vec![
                Line::CharacterContent(vec![LineContent { raw_content: "lyrics".to_owned(), ..Default::default() }], CharacterLine::Lyrics, "CHARACTER".to_owned())            ],
        };

        let result = export_fountain(&script);

        assert_eq!("~lyrics", result);
    }
}
