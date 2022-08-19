use crate::fountain::types::Line;

use web_sys::console;

use super::exporter::export_line;
use super::parser::{parse_content_formatting, parse_line};

use super::types::Script;

impl Script {
    pub(crate) fn update_title_element(&self, tag: &str, content: String) -> Self {
        let mut script = self.clone();
        let content = if content.is_empty() {
            None
        } else {
            Some(parse_content_formatting(&content))
        };

        match tag {
            "Title" => script.title.title = content,
            "Credit" => script.title.credit = content,
            "Author" => script.title.author = content,
            "Source" => script.title.source = content,
            "Draft date" => script.title.draft = content,
            "Contact" => script.title.contact = content,
            _ => {
                if let Some(content) = content {
                    script.title.meta.insert(tag.to_owned(), content);
                } else {
                    script.title.meta.remove(tag);
                }
            }
        }

        unsafe {
            console::log_2(&"Title updated".into(), &serde_json::to_string(&script.title).unwrap().into());
        }

        script
    }

    pub(crate) fn update_line(&self, line_id: usize, line: &str) -> Self{
        let mut script = self.clone();
        let mut lines = &mut script.lines;

        lines.remove(line_id);
        let mut new_lines = line.lines().collect::<Vec<_>>();

        let mut current_id = line_id;
        let mut previous_line = if lines.len() > line_id && line_id > 0 { lines[line_id - 1].clone() } else { Line::Empty };

        if !line.trim().is_empty() || new_lines.len() > 1 {   

            for line in new_lines.into_iter() {
                let (new_line, character) = parse_line(line, &previous_line, if let Line::CharacterContent(_, _, character) = &previous_line { character } else { "" });

                unsafe {
                    console::log_4(&"Line updated".into(), &current_id.into(), &line.into(), &serde_json::to_string(&previous_line).unwrap().into());
                }

                previous_line = new_line.clone();
                lines.insert(current_id, new_line);
                current_id += 1;
            }
        }

        loop {
            unsafe {
            console::log_2(&"Checking line".into(), &current_id.into());
        }
            if let Some(next_line) = lines.get(current_id) {
                let exported = export_line(&next_line);
                unsafe {
                    console::log_2(&"Found".into(), &(&exported).into());
                    console::log_2(&"Previous is".into(), &serde_json::to_string(&previous_line).unwrap().into());
                }
                let (new_line, character) = parse_line(&exported, &previous_line, if let Line::CharacterContent(_, _, character) = &previous_line { character } else { "" });
                unsafe {
                    console::log_2(&"Parsed into".into(), &serde_json::to_string(&new_line).unwrap().into());
                }

                if &new_line != next_line {
                    previous_line.clone();
                    lines[current_id] = new_line;
                    current_id += 1;
                } else {
                    break;
                }

            } else {
                break;
            }
        }

        script
    }
}
