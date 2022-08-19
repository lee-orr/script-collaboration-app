use crate::fountain::types::Line;

use web_sys::console;

use super::parser::parse_content_formatting;

use super::types::Script;

impl Script {
    pub(crate) fn update_title_element(&mut self, tag: &str, content: String) {
        let content = if content.is_empty() {
            None
        } else {
            Some(parse_content_formatting(&content))
        };

        match tag {
            "Title" => self.title.title = content,
            "Credit" => self.title.credit = content,
            "Author" => self.title.author = content,
            "Source" => self.title.source = content,
            "Draft date" => self.title.draft = content,
            "Contact" => self.title.contact = content,
            _ => {
                if let Some(content) = content {
                    self.title.meta.insert(tag.to_owned(), content);
                } else {
                    self.title.meta.remove(tag);
                }
            }
        }

        unsafe {
            console::log_2(&"Title updated".into(), &serde_json::to_string(&self.title).unwrap().into());
        }
    }

    pub(crate) fn update_line(&mut self, line_id: usize, line: &str) {
        let mut lines = &mut self.lines;

        let mut previous_line = if lines.len() > line_id && line_id > 0 { &lines[line_id] } else { &Line::Empty };

        let mut current_id = line_id;
        

        unsafe {
            console::log_3(&"Line updated".into(), &line_id.into(), &line.into());
        }
    }
}
