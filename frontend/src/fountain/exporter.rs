use super::types::{Script, Title, Line};

pub fn export_fountain(source: &Script) -> String {
    let title = export_title(&source.title);
    let content = export_lines(&source.lines);
    let mut result = Vec::<String>::new();
    if title != "" { result.push(title); }
    if content != "" { result.push(content); }
    result.join("\n")
}

fn export_title(title: &Title) -> String {
    let mut result : Vec<String> = Vec::new();

    if let Some(title) = &title.title {
        result.push(format!("Title: {}", title));
    }

    if let Some(credit) = &title.credit {
        result.push(format!("Credit: {}", credit));
    }

    if let Some(author) = &title.author {
        result.push(format!("Author: {}", author));
    }

    if let Some(source) = &title.source {
        result.push(format!("Source: {}", source));
    }

    if let Some(draft) = &title.draft {
        result.push(format!("Draft: {}", draft));
    }

    if let Some(contact) = &title.contact {
        result.push(format!("Contact: {}", contact));
    }

    result.join("\n")
}

fn export_lines(lines: &Vec<Line>) -> String {
    lines.into_iter().map(|line| {
        match line {
            Line::Action(line, _) => line[0].content.clone(),
            Line::SceneHeading(heading) => format!(".{}", heading),
            _ => "".to_owned()
        }
}).collect::<Vec<_>>().join("\n\n")
}

#[cfg(test)]
mod tests {
    use crate::fountain::types::{Script, Title, Line};
    use super::export_fountain;

    #[test]
    fn empty_script_results_in_empty_fountain() {
        let script = Script {
            title: Title::default(),
            lines: vec![]
        };

        let result = export_fountain(&script);

        assert_eq!("", result);
    }

    #[test]
    fn when_there_is_a_title_it_is_added_to_the_script() {
        let script = Script {
            title: Title {
                title: Some("A Show".to_owned()),
                credit: Some("written by".to_owned()),
                author: Some("An author".to_owned()),
                ..Default::default()
            },
            lines: vec![]
        };

        let result = export_fountain(&script);

        assert_eq!("Title: A Show
Credit: written by
Author: An author", result);
    }
}