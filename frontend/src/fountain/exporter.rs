use serde::de::IntoDeserializer;

use super::types::{Line, LineContent, Script, Title};

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

fn export_lines(lines: &Vec<Line>) -> String {
    lines
        .into_iter()
        .map(|line| match line {
            Line::Action(line, _) => line[0].content.clone(),
            Line::SceneHeading(heading) => format!(".{}", heading),
            _ => "".to_owned(),
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

#[cfg(test)]
mod tests {
    use super::export_fountain;
    use crate::fountain::types::{Line, LineContent, Script, Title};

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
}
