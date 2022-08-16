use super::types::{Line, Script, Title, LineContent, TextAlignment, CharacterLine};

pub fn parse_fountain(source: &str) -> Script {
    let source = parse_boneyard(source);
    let source = source
        .into_iter()
        .map(|(content, boneyard)| {
            if boneyard {
                vec![(content, boneyard)]
            } else {
                content
                    .lines()
                    .into_iter()
                    .map(|v| (v.to_owned(), boneyard))
                    .collect()
            }
        })
        .flatten()
        .collect::<Vec<(String, bool)>>();
    let slice = source
        .split(|(v, _)| v.trim() == "")
        .collect::<Vec<&[(String, bool)]>>();
    let (title, contains_title) = parse_title(&slice.get(0));
    let continued_slice = if contains_title {
        slice[1..]
            .to_owned()
            .into_iter()
            .map(|v| {
                let mut iter = v.iter().map(|v| v.to_owned()).collect::<Vec<_>>();
                iter.push(("".to_owned(), false));
                iter
            })
            .flatten()
            .collect::<Vec<(String, bool)>>()
    } else {
        source
    };

    Script {
        title,
        lines: parse_lines(&continued_slice),
    }
}

fn parse_boneyard(source: &str) -> Vec<(String, bool)> {
    let mut boneyard = false;
    source
        .split("/*")
        .map(|v| v.split("*/"))
        .flatten()
        .map(|v| {
            boneyard = !boneyard;
            (v.to_owned(), !boneyard)
        })
        .collect::<Vec<(String, bool)>>()
}

fn parse_scene_heading(line: &str) -> Option<String> {
    if line.starts_with(".") {
        Some(line[1..].to_owned())
    } else if line.starts_with("INT")
        || line.starts_with("EXT")
        || line.starts_with("EST")
        || line.starts_with("I/E")
        || line.starts_with("int")
        || line.starts_with("ext")
        || line.starts_with("est")
        || line.starts_with("i/e")
    {
        Some(line.to_owned())
    } else {
        None
    }
}

fn parse_character_heading(line: &str, previous_line: &Line) -> Option<String> {
    if matches!(previous_line, Line::Empty) && line.to_uppercase() == line {
        Some(line.to_owned())
    } else {
        None
    }
}

fn parse_dialog(line: &str, previous_line: &Line) -> Option<(String, bool)> {
    if match previous_line {
        Line::CharacterContent(_) => true,
        _ => false,
    } {
        Some((
            line.to_owned(),
            line.starts_with("(") && line.ends_with(")"),
        ))
    } else {
        None
    }
}

fn parse_lyrics(line: &str) -> Option<String> {
    if line.starts_with("~") {
        Some(line[1..].trim().to_owned())
    } else {
        None
    }
}

fn parse_transitions(line: &str, previous_line: &Line) -> Option<String> {
    if line.starts_with(">") && !line.ends_with("<") {
        Some(line[1..].trim().to_owned())
    } else if matches!(previous_line, Line::Empty)
        && line.to_uppercase() == line
        && line.ends_with("TO:")
    {
        Some(line.to_owned())
    } else {
        None
    }
}

fn parse_centered_text(line: &str) -> Option<String> {
    if line.starts_with(">") && line.ends_with("<") {
        Some(line[1..line.len() - 1].trim().to_owned())
    } else {
        None
    }
}

fn parse_page_break(line: &str) -> bool {
    if line.starts_with("===") {
        true
    } else {
        false
    }
}

pub fn parse_line(
    line: &str,
    previous_line: &Line,
    current_character: &str,
) -> (Line, Option<String>) {
    let v = line.trim();
    if v == "" {
        (Line::Empty, None)
    } else if parse_page_break(v) {
        (Line::PageBreak, None)
    } else if let Some(centered) = parse_centered_text(v) {
        (Line::Action(vec![LineContent{content: centered, ..Default::default()}], TextAlignment::Center), None)
    } else if let Some(heading) = parse_scene_heading(v) {
        (Line::SceneHeading(heading), None)
    } else if let Some(transition) = parse_transitions(v, &previous_line) {
        (Line::Transition(transition), None)
    } else if let Some(character) = parse_character_heading(v, &previous_line) {
        (Line::CharacterContent(vec![(vec![], CharacterLine::CharacterHeading, character.clone())]), Some(character))
    } else if let Some(lyrics) = parse_lyrics(v) {
        (Line::CharacterContent(vec![(vec![LineContent{content: lyrics, ..Default::default()}], CharacterLine::Lyrics, current_character.to_owned())]), None)
    } else if let Some((dialogue, parenthetical)) = parse_dialog(v, &previous_line) {
        if parenthetical {
            (Line::CharacterContent(vec![(vec![LineContent{content: dialogue, ..Default::default()}], CharacterLine::Parenthetical, current_character.to_owned())]), None)
        } else {
            (Line::CharacterContent(vec![(vec![LineContent{content: dialogue, ..Default::default()}], CharacterLine::Dialogue, current_character.to_owned())]), None)
        }
    } else {
        (Line::Action(vec![LineContent{content: v.to_owned(), ..Default::default()}], TextAlignment::Center), None)
    }
}

fn parse_lines(slice: &Vec<(String, bool)>) -> Vec<Line> {
    let mut previous_line = Line::Empty;
    let mut current_character = "".to_owned();
    slice
        .iter()
        .map(|(v, boneyard)| {
            if *boneyard {
                return Line::Boneyard(vec![LineContent{content: v.to_owned(), ..Default::default()}]);
            }
            let (result, character) = parse_line(v, &previous_line, &current_character);
            previous_line = result.clone();
            if let Some(character) = character {
                current_character = character;
            }
            result
        })
        .collect::<Vec<Line>>()
}

fn parse_title(source: &Option<&&[(String, bool)]>) -> (Title, bool) {
    let mut title = Title::default();
    if let Some(section) = source {
        let first_line: Option<&(String, bool)> = section.get(0);
        if first_line == None || first_line.unwrap().0.split(":").collect::<Vec<_>>().len() != 2 {
            return (title, false);
        }

        let mut last_key: Option<String> = None;
        let mut last_value: Option<String> = None;

        for (line, _) in section.iter() {
            if line.len() == 0 {
                break;
            }
            let split: Vec<&str> = line.split(":").collect();
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
    use crate::fountain::{
        parser::parse_fountain,
        types::{CharacterLine, Line, TextAlignment},
    };

    #[test]
    fn empty_string_returns_empty_document() {
        let result = parse_fountain("");
        assert_eq!(result.title.title, None);
        assert_eq!(result.lines.len(), 0);
    }

    #[test]
    fn when_there_is_title_info_it_is_parsed_first() {
        let result = parse_fountain(
            "Title: _**test me**_
        *something more*
        Credit: Written by
        Author: Someone Special
        Unknown: Unknown?
        
        the actual script starts here",
        );

        assert_eq!(
            result.title.title.unwrap(),
            "_**test me**_\n*something more*"
        );
        assert_eq!(result.title.credit.unwrap(), "Written by");
        assert_eq!(result.title.author.unwrap(), "Someone Special");
        assert_eq!(result.title.meta.get("Unknown").unwrap(), "Unknown?");
        assert_eq!(result.lines.len(), 2);
        if let Line::Action(line, _) = &result.lines[0] {
            assert_eq!(line[0].content, "the actual script starts here")
        } else {
            assert!(false, "didn't parse line")
        }
    }

    #[test]
    fn defaults_to_action_if_no_other_option_exists() {
        let result = parse_fountain(
            "well hello there
        how are you",
        );
        assert_eq!(result.lines.len(), 2);
        if let Line::Action(line, _) = &result.lines[0] {
            assert_eq!(line[0].content, "well hello there")
        } else {
            assert!(false, "didn't parse line")
        }
        if let Line::Action(line, _) = &result.lines[1] {
            assert_eq!(line[0].content, "how are you")
        } else {
            assert!(false, "didn't parse line")
        }
    }

    #[test]
    fn correctly_parses_scene_heading() {
        let result = parse_fountain(
            ".A HEADING
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
i/e Heading 7",
        );
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

    #[test]
    fn correctly_parses_character_names() {
        let result = parse_fountain(
            "some scene setting
            THIS IS NOT A CHARACTER
            
            THIS IS A CHARACTER
            testing some dialogue and shit",
        );
        assert_eq!(result.lines.len(), 5);
        if let Line::Action(line, _) = &result.lines[1] {
            assert_eq!(line[0].content, "THIS IS NOT A CHARACTER")
        } else {
            assert!(false, "didn't parse line")
        }
        if let Line::CharacterContent(lines) = &result.lines[3] {
            if let (line, line_type, character) = &lines[0] {
                assert_eq!(character, "THIS IS A CHARACTER");
                assert_eq!(line_type, &CharacterLine::CharacterHeading);
            } else {
                panic!()
            }
        } else {
            assert!(false, "didn't parse line")
        }
        if let Line::CharacterContent(lines) = &result.lines[4] {
            if let (line, line_type, character) = &lines[0] {
                assert_eq!(line[0].content, "testing some dialogue and shit");
                assert_eq!(character, "THIS IS A CHARACTER");
                assert_eq!(line_type, &CharacterLine::Dialogue);
            } else {
                panic!()
            }
        } else {
            assert!(false, "didn't parse line")
        }
    }

    #[test]
    fn correctly_parses_character_names_when_there_is_a_title() {
        let result = parse_fountain(
            "Title: WHAT
            
            some scene setting
            THIS IS NOT A CHARACTER
            
            THIS IS A CHARACTER
            testing some dialogue and shit",
        );
        assert_eq!(result.lines.len(), 6);
        if let Line::Action(line, _) = &result.lines[1] {
            assert_eq!(line[0].content, "THIS IS NOT A CHARACTER")
        } else {
            assert!(false, "didn't parse line")
        }
        if let Line::CharacterContent(lines) = &result.lines[3] {
            if let (line, line_type, character) = &lines[0] {
                assert_eq!(character, "THIS IS A CHARACTER");
                assert_eq!(line_type, &CharacterLine::CharacterHeading);
            } else {
                panic!()
            }
        } else {
            assert!(false, "didn't parse line")
        }
        if let Line::CharacterContent(lines) = &result.lines[4] {
            if let (line, line_type, character) = &lines[0] {
                assert_eq!(line[0].content, "testing some dialogue and shit");
                assert_eq!(character, "THIS IS A CHARACTER");
                assert_eq!(line_type, &CharacterLine::Dialogue);
            } else {
                panic!()
            }
        } else {
            assert!(false, "didn't parse line")
        }
    }

    #[test]
    fn correctly_parses_dialog() {
        let result = parse_fountain(
            "Title: WHAT
            
            CHARACTER
            I am talking now...",
        );
        assert_eq!(result.lines.len(), 3);
        if let Line::CharacterContent(lines) = &result.lines[1] {
            if let (line, line_type, character) = &lines[0] {
                assert_eq!(line[0].content, "I am talking now...");
                assert_eq!(character, "CHARACTER");
                assert_eq!(line_type, &CharacterLine::Dialogue);
            } else {
                panic!()
            }
        } else {
            assert!(false, "didn't parse line");
        }
    }

    #[test]
    fn correctly_parses_parenthetical() {
        let result = parse_fountain(
            "Title: WHAT
            
            CHARACTER
            (to everyone)
            I am talking now!
            (aside)
            I think...
            
            and an action",
        );
        assert_eq!(result.lines.len(), 8);
        if let Line::CharacterContent(lines) = &result.lines[1] {
            if let (line, line_type, character) = &lines[0] {
                assert_eq!(line[0].content, "(to everyone)");
                assert_eq!(character, "CHARACTER");
                assert_eq!(line_type, &CharacterLine::Parenthetical);
            } else {
                panic!()
            }
        } else {
            assert!(false, "didn't parse line");
        }
        if let Line::CharacterContent(lines) = &result.lines[2] {
            if let (line, line_type, character) = &lines[0] {
                assert_eq!(line[0].content, "I am talking now!");
                assert_eq!(character, "CHARACTER");
                assert_eq!(line_type, &CharacterLine::Dialogue);
            } else {
                panic!()
            }
        } else {
            assert!(false, "didn't parse line");
        }
        if let Line::CharacterContent(lines) = &result.lines[3] {
            if let (line, line_type, character) = &lines[0] {
                assert_eq!(line[0].content, "(aside)");
                assert_eq!(character, "CHARACTER");
                assert_eq!(line_type, &CharacterLine::Parenthetical);
            } else {
                panic!()
            }
        } else {
            assert!(false, "didn't parse line");
        }
        if let Line::CharacterContent(lines) = &result.lines[4] {
            if let (line, line_type, character) = &lines[0] {
                assert_eq!(line[0].content, "I think...");
                assert_eq!(character, "CHARACTER");
                assert_eq!(line_type, &CharacterLine::Dialogue);
            } else {
                panic!()
            }
        } else {
            assert!(false, "didn't parse line");
        }

        if let Line::Action(line, _) = &result.lines[6] {
            assert_eq!(line[0].content, "and an action");
        } else {
            assert!(false, "didn't parse line");
        }
    }

    #[test]
    fn correctly_parse_lyrics() {
        let result = parse_fountain(
            "CHARACTER
        ~ part of a song",
        );
        assert_eq!(result.lines.len(), 2);
        if let Line::CharacterContent(lines) = &result.lines[1] {
            if let (line, line_type, character) = &lines[0] {
                assert_eq!(line[0].content, "part of a song");
                assert_eq!(character, "CHARACTER");
                assert_eq!(line_type, &CharacterLine::Lyrics);
            } else {
                panic!()
            }
        } else {
            assert!(false, "didn't parse line");
        }
    }

    #[test]
    fn correctly_parse_transitions() {
        let result = parse_fountain(
            "some action

        CUT TO:
        
        another action
        
        > Another transition
        
        wow",
        );

        assert_eq!(result.lines.len(), 9);
        if let Line::Transition(line) = &result.lines[2] {
            assert_eq!(line, "CUT TO:");
        } else {
            assert!(false, "didn't parse line");
        }
        if let Line::Transition(line) = &result.lines[6] {
            assert_eq!(line, "Another transition");
        } else {
            assert!(false, "didn't parse line");
        }
    }

    #[test]
    fn correctly_parse_centered_text() {
        let result = parse_fountain("> HEY THERE <");
        assert_eq!(result.lines.len(), 1);
        if let Line::Action(line, centered) = &result.lines[0] {
            assert_eq!(line[0].content, "HEY THERE");
            assert_eq!(centered, &TextAlignment::Center);
        } else {
            assert!(false, "didn't parse line");
        }
    }

    #[test]
    fn correctly_parse_page_breaks() {
        let result = parse_fountain("===");
        assert_eq!(result.lines.len(), 1);
        if let Line::PageBreak = &result.lines[0] {
            assert!(true);
        } else {
            assert!(false, "didn't parse line");
        }
    }

    #[test]
    fn can_parse_the_boneyard() {
        let result = parse_fountain(
            "testing /* wehawe
            wthawet
            wtrwat */ something",
        );

        assert_eq!(result.lines.len(), 3);
        if let Line::Action(line, _) = &result.lines[0] {
            assert_eq!(line[0].content, "testing");
        } else {
            assert!(false, "didn't parse line");
        }
        if let Line::Action(line, _) = &result.lines[2] {
            assert_eq!(line[0].content, "something");
        } else {
            assert!(false, "didn't parse line");
        }
        if let Line::Boneyard(line) = &result.lines[1] {
            assert_eq!(
                line[0].content,
                " wehawe
            wthawet
            wtrwat "
            );
        } else {
            assert!(false, "didn't parse line");
        }
    }
}
