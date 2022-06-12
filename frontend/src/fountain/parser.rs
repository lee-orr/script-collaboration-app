use std::collections::HashMap;

use super::types::{Script, Title, Line};

pub fn parse_fountain(source: &str) -> Script {
    let source = source.lines().into_iter().map(|v| v.to_owned()).collect::<Vec<String>>();
    let slice = source.split(|v| v.trim() == "").collect::<Vec<&[String]>>();
    let (title, contains_title) = parse_title(&slice.get(0));
    let continued_slice = if contains_title {
        slice[1..].to_owned().into_iter().flatten().map(|v| v.to_owned()).collect::<Vec<String>>()
    } else {
        source
    };

    Script {
        title,
        lines: parse_lines(&continued_slice)
    }
}

fn parse_lines(slice: &Vec<String>) -> Vec<Line>{
    slice
        .iter()
        .map(|v| Line::Action(v.trim().to_owned()))
        .collect::<Vec<Line>>()
}

fn parse_title(source: &Option<&&[String]>) -> (Title, bool) {
    let mut title = Title::default();
    if let Some(section) = source {
    let first_line : Option<&String>= section.get(0);
    if first_line == None || first_line.unwrap().split(":").collect::<Vec::<_>>().len() != 2 {
        return (title, false);
    }

    let mut last_key : Option<String> = None;
    let mut last_value : Option<String> = None;

    for line in section.iter() {
        if line.len() == 0 { break; }
        let split : Vec<&str> = line.split(":").collect();
        if split.len() != 2 {
            if let Some(value) = last_value {
                last_value = Some(format!("{}\n{}", value, line.trim()))
            } else {
                last_value = Some(line.trim().to_owned());
            }
        } else {
            if let Some(key) = last_key {
                if let Some(value) = last_value {
                    title.meta.insert(key.to_owned(), value.to_owned());
                    match key.as_str() {
                        "Title" => title.title = Some(value),
                        "Credit" => title.credit = Some(value),
                        "Author" => title.author = Some(value),
                        "Source" => title.source = Some(value),
                        "Draft date" => title.draft = Some(value),
                        "Contact" => title.contact = Some(value),
                        _ => {
                            println!("Adding Unknown Key {} {}", &key, &value);
                        }
                    };
                }
                last_key = None;
                last_value = None;
            }
            let key = split[0].trim().to_owned();
            let value = split[1].trim().to_owned();
            last_key = Some(key);
            last_value = Some(value);
        }
    }
    if let Some(key) = last_key {
        if let Some(value) = last_value {
            title.meta.insert(key.to_owned(), value.to_owned());
            match key.as_str() {
                "Title" => title.title = Some(value),
                "Credit" => title.credit = Some(value),
                "Author" => title.author = Some(value),
                "Source" => title.source = Some(value),
                "Draft date" => title.draft = Some(value),
                "Contact" => title.contact = Some(value),
                _ => {
                    println!("Adding Unknown Key {} {}", &key, &value);
                }
            };
        }
    }
    (title, true)
} else {
    (title, false)
}
}

#[cfg(test)]
mod tests {
    use crate::fountain::{parser::parse_fountain, types::Line};

    #[test]
    fn empty_string_returns_empty_document() {
        let result = parse_fountain("");
        assert_eq!(result.title.title, None);
        assert_eq!(result.lines.len(), 0);
    }

    #[test]
    fn when_there_is_title_info_it_is_parsed_first() {
        let result = parse_fountain("Title: _**test me**_
        *something more*
        Credit: Written by
        Author: Someone Special
        Unknown: Unknown?
        
        the actual script starts here");

        assert_eq!(result.title.title.unwrap(), "_**test me**_\n*something more*");
        assert_eq!(result.title.credit.unwrap(), "Written by");
        assert_eq!(result.title.author.unwrap(), "Someone Special");
        assert_eq!(result.title.meta.get("Unknown").unwrap(), "Unknown?");
        assert_eq!(result.lines.len(), 1);
        if let Line::Action(line) = &result.lines[0] {
            assert_eq!(line, "the actual script starts here")
        } else {
            assert!(false, "didn't parse line")
        }
    }
}