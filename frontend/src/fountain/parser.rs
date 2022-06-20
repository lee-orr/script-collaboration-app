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

fn parse_scene_heading(line: &str) -> Option<String> {
    if line.starts_with(".") {
        Some(line[1..].to_owned())
    } else if line.starts_with("INT") || line.starts_with("EXT") || line.starts_with("EST") || line.starts_with("I/E") ||
    line.starts_with("int") || line.starts_with("ext") || line.starts_with("est") || line.starts_with("i/e") {
        Some(line.to_owned())
    } else {
        None
    }
}

fn parse_lines(slice: &Vec<String>) -> Vec<Line>{
    slice
        .iter()
        .map(|v| {
            let v = v.trim();
            if let Some(headhing) = parse_scene_heading(v) {
                Line::SceneHeading(headhing)
            } else {
                Line::Action(v.to_owned())
            }
})
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

    #[test]
    fn defaults_to_action_if_no_other_option_exists() {
        let result = parse_fountain("well hello there
        how are you");
        assert_eq!(result.lines.len(), 2);
        if let Line::Action(line) = &result.lines[0] {
            assert_eq!(line, "well hello there")
        } else {
            assert!(false, "didn't parse line")
        }
        if let Line::Action(line) = &result.lines[1] {
            assert_eq!(line, "how are you")
        } else {
            assert!(false, "didn't parse line")
        }
    }

    #[test]
    fn correctly_parses_scene_heading() {
        let result = parse_fountain(".A HEADING
        INT Other Heading
EXT More Headings
EST Heading 4
INT./EXT Heading 5
INT/EXT Heading 6
I/E Heading 7
int Other Heading
ext More Headings
est Heading 4
int./ext Heading 5
int/ext Heading 6
i/e Heading 7");
assert_eq!(result.lines.len(), 13);

if let Line::SceneHeading(line) = &result.lines[0] {
    assert_eq!(line, "A HEADING")
} else {
    assert!(false, "didn't parse line")
}

if let Line::SceneHeading(line) = &result.lines[1] {
    assert_eq!(line, "INT Other Heading")
} else {
    assert!(false, "didn't parse line")
}

if let Line::SceneHeading(line) = &result.lines[2] {
    assert_eq!(line, "EXT More Headings")
} else {
    assert!(false, "didn't parse line")
}

if let Line::SceneHeading(line) = &result.lines[3] {
    assert_eq!(line, "EST Heading 4")
} else {
    assert!(false, "didn't parse line")
}


if let Line::SceneHeading(line) = &result.lines[4] {
    assert_eq!(line, "INT./EXT Heading 5")
} else {
    assert!(false, "didn't parse line")
}

if let Line::SceneHeading(line) = &result.lines[5] {
    assert_eq!(line, "INT/EXT Heading 6")
} else {
    assert!(false, "didn't parse line")
}


if let Line::SceneHeading(line) = &result.lines[6] {
    assert_eq!(line, "I/E Heading 7")
} else {
    assert!(false, "didn't parse line")
}


if let Line::SceneHeading(line) = &result.lines[7] {
    assert_eq!(line, "int Other Heading")
} else {
    assert!(false, "didn't parse line")
}


if let Line::SceneHeading(line) = &result.lines[8] {
    assert_eq!(line, "ext More Headings")
} else {
    assert!(false, "didn't parse line")
}


if let Line::SceneHeading(line) = &result.lines[9] {
    assert_eq!(line, "est Heading 4")
} else {
    assert!(false, "didn't parse line")
}


if let Line::SceneHeading(line) = &result.lines[10] {
    assert_eq!(line, "int./ext Heading 5")
} else {
    assert!(false, "didn't parse line")
}


if let Line::SceneHeading(line) = &result.lines[11] {
    assert_eq!(line, "int/ext Heading 6")
} else {
    assert!(false, "didn't parse line")
}


if let Line::SceneHeading(line) = &result.lines[12] {
    assert_eq!(line, "i/e Heading 7")
} else {
    assert!(false, "didn't parse line")
}
    }
}