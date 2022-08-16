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

fn parse_character_heading(line: &str, previous_line: &Line) -> Option<(String, bool)> {
    let mut line = line.to_owned();
    let mut is_dual = false;
    if line.ends_with('^') {
        line.pop();
        line = line.trim().to_owned();
        is_dual = true;
    }
    if matches!(previous_line, Line::Empty) && line.to_uppercase() == line {
        Some((line.to_owned(), is_dual))
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
        (Line::Action(parse_content_formatting(&centered), TextAlignment::Center), None)
    } else if let Some(heading) = parse_scene_heading(v) {
        (Line::SceneHeading(heading), None)
    } else if let Some(transition) = parse_transitions(v, &previous_line) {
        (Line::Transition(transition), None)
    } else if let Some((character, is_dual)) = parse_character_heading(v, &previous_line) {
        (Line::CharacterContent(vec![(vec![], CharacterLine::CharacterHeading(is_dual), character.clone())]), Some(character))
    } else if let Some(lyrics) = parse_lyrics(v) {
        (Line::CharacterContent(vec![(parse_content_formatting(&lyrics), CharacterLine::Lyrics, current_character.to_owned())]), None)
    } else if let Some((dialogue, parenthetical)) = parse_dialog(v, &previous_line) {
        if parenthetical {
            (Line::CharacterContent(vec![(parse_content_formatting(&dialogue), CharacterLine::Parenthetical, current_character.to_owned())]), None)
        } else {
            (Line::CharacterContent(vec![(parse_content_formatting(&dialogue), CharacterLine::Dialogue, current_character.to_owned())]), None)
        }
    } else {
        (Line::Action(parse_content_formatting(&v), TextAlignment::Center), None)
    }
}

fn parse_lines(slice: &Vec<(String, bool)>) -> Vec<Line> {
    let mut previous_line = Line::Empty;
    let mut current_character = "".to_owned();
    let iterator = slice
        .iter()
        .map(|(v, boneyard)| {
            if *boneyard {
                return Line::Boneyard(parse_content_formatting(&v));
            }
            let (result, character) = parse_line(&v, &previous_line, &current_character);
            previous_line = result.clone();
            if let Some(character) = character {
                current_character = character;
            }
            result
        });

    let mut lines : Vec<Line> = Vec::new();

    for line in iterator {
        collapse_line(line, &mut lines);
    }

    lines
}

fn collapse_line(mut line: Line, lines: &mut Vec<Line>) {
    if lines.len() == 0 {
        lines.push(line);
        return;
    }
    let length = lines.len() - 1;
    let mut previous_line = lines.last();

    match (line, previous_line) {
        (Line::CharacterContent(mut new_line_content), Some(Line::CharacterContent(old_line_content))) => {
            let last_line_conent = &old_line_content.last();
            let first_new_line_content = &new_line_content.first();

            match (last_line_conent, first_new_line_content) {
                (Some((_,_,last_character)), Some((_,line_type,new_character))) => {
                    if last_character == new_character && line_type != &CharacterLine::CharacterHeading(false) || line_type == &CharacterLine::CharacterHeading(true) {
                        let mut collapsed_line = old_line_content.clone();
                        collapsed_line.append(&mut new_line_content);
                        lines[length] = Line::CharacterContent(collapsed_line);
                    } else {
                        lines.push(Line::CharacterContent(new_line_content))
                    }
                },
                _ => lines.push(Line::CharacterContent(new_line_content))
            }
        },
        (Line::Empty, Some(Line::CharacterContent(old_line_content))) => {
            let last_line_conent = &old_line_content.last();
                if let Some(    (_,_,last_character)) = last_line_conent {
                let mut collapsed_line = old_line_content.clone();
                collapsed_line.push((vec![], CharacterLine::Empty, last_character.clone()));
                lines[length] = Line::CharacterContent(collapsed_line);
            }
        },
        (line, _) => lines.push(line),
    };
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
                        title.meta.insert(key.to_owned(), parse_content_formatting(&value));
                        match key.as_str() {
                            "Title" => title.title = Some(parse_content_formatting(&value)),
                            "Credit" => title.credit = Some(parse_content_formatting(&value)),
                            "Author" => title.author = Some(parse_content_formatting(&value)),
                            "Source" => title.source = Some(parse_content_formatting(&value)),
                            "Draft date" => title.draft = Some(parse_content_formatting(&value)),
                            "Contact" => title.contact = Some(parse_content_formatting(&value)),
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
                title.meta.insert(key.to_owned(), parse_content_formatting(&value));
                match key.as_str() {
                    "Title" => title.title = Some(parse_content_formatting(&value)),
                    "Credit" => title.credit = Some(parse_content_formatting(&value)),
                    "Author" => title.author = Some(parse_content_formatting(&value)),
                    "Source" => title.source = Some(parse_content_formatting(&value)),
                    "Draft date" => title.draft = Some(parse_content_formatting(&value)),
                    "Contact" => title.contact = Some(parse_content_formatting(&value)),
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

fn parse_content_formatting(text: &str) -> Vec<LineContent> {

    let mut process_underline = false;
    let processing = text.replace("&nbsp;","\n").split("_").map(|v| {
        if v.len() == 0 {
            (v.to_owned(), false)
        } else if v.ends_with("\\") {
            (format!("{}_", v[0..v.len() - 1].to_owned()), process_underline)
        } else {
            process_underline = if process_underline { false} else { true};
            (v.to_owned(), !process_underline)
        }
    }).filter(|v| v.0.len() > 0).collect::<Vec<(String, bool)>>();

    let mut process_bold = false;
    let processing = processing.into_iter().enumerate().map(|(segment,(line, underline))| {
        let split = line.split("**").collect::<Vec<_>>();
        let len = split.len();
        split.into_iter().enumerate().map(|(i,v)| {
            if i == len - 1 {
                (v.to_owned(), underline, process_bold)
            } else if v.len() == 0 {
                (v.to_owned(), false, false)
            } else if v.ends_with("\\") {
                (format!("{}**", v[0..v.len() - 1].to_owned()), underline, process_bold)
            } else {
                process_bold = if process_bold { false} else { true};
                (v.to_owned(), underline, !process_bold)
            }
        }).collect::<Vec<(String, bool, bool)>>()
    }).flatten().filter(|v| v.0.len() > 0).collect::<Vec<(String, bool, bool)>>();

    let mut process_italic = false;
    let processing = processing.into_iter().map(|(line, underline, bold)| {
        let split = line.split("*").collect::<Vec<_>>();
        let len = split.len();
        split.into_iter().enumerate().map(|(i,v)| {
            if i == len - 1 {
                (v.to_owned(), underline, bold, process_italic)
            } else if v.len() == 0 {
                (v.to_owned(), false, false, false)
            } else if v.ends_with("\\") {
                (format!("{}*", v[0..v.len() - 1].to_owned()), underline, bold, process_italic)
            } else {
                process_italic = if process_italic { false} else { true};
                (v.to_owned(), underline, bold, !process_italic)
            }
        }).collect::<Vec<(String, bool, bool, bool)>>()
    }).flatten().filter(|v| v.0.len() > 0).collect::<Vec<(String, bool, bool, bool)>>();

    let mut processing_note = false;
    let processing = processing.into_iter().map(|(line, underline, bold, italic)| {
        let split = line.split("[[").collect::<Vec<_>>();
        let len = split.len();
        split.into_iter().enumerate().map(|(i,v)| {
            let split = v.split("]]").collect::<Vec<_>>();
            if split.len() <= 1 {
                vec!([(v.to_owned(), underline, bold, italic, false)])
            } else {
                split.into_iter().enumerate().map(|(i, v)| {
                    if i == 0 {
                        [(v.to_owned(), underline, bold, italic, true)]
                    } else {
                        [(v.to_owned(), underline, bold, italic, false)]
                    }
                }).collect::<Vec<_>>()
            }
        }).flatten().flatten().collect::<Vec<(String, bool, bool, bool, bool)>>()
    }).flatten().filter(|v| v.0.len() > 0).collect::<Vec<(String, bool, bool, bool, bool)>>();

    processing.into_iter().map(|(content, underline, bold, italic, note)| {
        LineContent {
            content,
            underline, 
            bold,
            italic,
            note,
            ..Default::default()
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use crate::fountain::{
        parser::parse_fountain,
        types::{CharacterLine, Line, TextAlignment, LineContent},
    };

    use super::{collapse_line, parse_content_formatting};

    #[test]
    fn empty_string_returns_empty_document() {
        let result = parse_fountain("");
        assert_eq!(result.title.title, None);
        assert_eq!(result.lines.len(), 0);
    }

    #[test]
    fn when_there_is_title_info_it_is_parsed_first() {
        let result = parse_fountain(
            "Title: test me
        something more
        Credit: Written by
        Author: Someone Special
        Unknown: Unknown?
        
        the actual script starts here",
        );

        assert_eq!(
            result.title.title.unwrap()[0].content,
            "test me\nsomething more"
        );
        assert_eq!(result.title.credit.unwrap()[0].content, "Written by");
        assert_eq!(result.title.author.unwrap()[0].content, "Someone Special");
        assert_eq!(result.title.meta.get("Unknown").unwrap()[0].content, "Unknown?");
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
        assert_eq!(result.lines.len(), 4);
        if let Line::Action(line, _) = &result.lines[1] {
            assert_eq!(line[0].content, "THIS IS NOT A CHARACTER")
        } else {
            assert!(false, "didn't parse line")
        }
        if let Line::CharacterContent(lines) = &result.lines[3] {
            if let (line, line_type, character) = &lines[0] {
                assert_eq!(character, "THIS IS A CHARACTER");
                assert_eq!(line_type, &CharacterLine::CharacterHeading(false));
            } else {
                panic!()
            }
            if let (line, line_type, character) = &lines[1] {
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
    fn correctly_parses_character_names_with_dual() {
        let result = parse_fountain(
            "some scene setting
            THIS IS NOT A CHARACTER
            
            THIS IS A CHARACTER ^
            testing some dialogue and shit",
        );
        assert_eq!(result.lines.len(), 4);
        if let Line::Action(line, _) = &result.lines[1] {
            assert_eq!(line[0].content, "THIS IS NOT A CHARACTER")
        } else {
            assert!(false, "didn't parse line")
        }
        if let Line::CharacterContent(lines) = &result.lines[3] {
            if let (line, line_type, character) = &lines[0] {
                assert_eq!(character, "THIS IS A CHARACTER");
                assert_eq!(line_type, &CharacterLine::CharacterHeading(true));
            } else {
                panic!()
            }
            if let (line, line_type, character) = &lines[1] {
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
        assert_eq!(result.lines.len(), 5);
        if let Line::Action(line, _) = &result.lines[1] {
            assert_eq!(line[0].content, "THIS IS NOT A CHARACTER")
        } else {
            assert!(false, "didn't parse line")
        }
        if let Line::CharacterContent(lines) = &result.lines[3] {
            if let (line, line_type, character) = &lines[0] {
                assert_eq!(character, "THIS IS A CHARACTER");
                assert_eq!(line_type, &CharacterLine::CharacterHeading(false));
            } else {
                panic!()
            }
            if let (line, line_type, character) = &lines[1] {
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
        assert_eq!(result.lines.len(), 2);
        if let Line::CharacterContent(lines) = &result.lines[0] {
            if let (line, line_type, character) = &lines[1] {
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
        assert_eq!(result.lines.len(), 4);
        if let Line::CharacterContent(lines) = &result.lines[0] {
            if let (line, line_type, character) = &lines[1] {
                assert_eq!(line[0].content, "(to everyone)");
                assert_eq!(character, "CHARACTER");
                assert_eq!(line_type, &CharacterLine::Parenthetical);
            } else {
                panic!()
            }if let (line, line_type, character) = &lines[2] {
                assert_eq!(line[0].content, "I am talking now!");
                assert_eq!(character, "CHARACTER");
                assert_eq!(line_type, &CharacterLine::Dialogue);
            } else {
                panic!()
            }
            if let (line, line_type, character) = &lines[3] {
                assert_eq!(line[0].content, "(aside)");
                assert_eq!(character, "CHARACTER");
                assert_eq!(line_type, &CharacterLine::Parenthetical);
            } else {
                panic!()
            }
            if let (line, line_type, character) = &lines[4] {
                assert_eq!(line[0].content, "I think...");
                assert_eq!(character, "CHARACTER");
                assert_eq!(line_type, &CharacterLine::Dialogue);
            } else {
                panic!()
            }
        } else {
            assert!(false, "didn't parse line");
        }

        if let Line::Action(line, _) = &result.lines[2] {
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
        assert_eq!(result.lines.len(), 1);
        if let Line::CharacterContent(lines) = &result.lines[0] {
            if let (line, line_type, character) = &lines[1] {
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

    #[test]
    fn collapse_two_dialogues_by_the_same_character() {
        let mut result = vec![Line::CharacterContent(vec![(vec![], CharacterLine::CharacterHeading(false), "character".to_owned())])];
        collapse_line(Line::CharacterContent(vec![(vec![LineContent { content: "test".to_owned(), ..Default::default()}], CharacterLine::Dialogue, "character".to_owned())]),  &mut result);

        assert_eq!(result.len(), 1);
        if let Line::CharacterContent(line) = &result[0] {
            assert_eq!(line.len(), 2);
        } else {
            panic!();
        }
    }

    #[test]
    fn dont_collapse_different_character_dialogues() {
        let mut result = vec![Line::CharacterContent(vec![(vec![], CharacterLine::CharacterHeading(false), "character".to_owned())])];
        collapse_line(Line::CharacterContent(vec![(vec![LineContent { content: "test".to_owned(), ..Default::default()}], CharacterLine::Dialogue, "character 2".to_owned())]),  &mut result);

        assert_eq!(result.len(), 2);
        if let Line::CharacterContent(line) = &result[0] {
            assert_eq!(line.len(), 1);
        } else {
            panic!();
        }
    }


    #[test]
    fn collapse_two_dialogues_by_different_characters_if_dual_dialogue() {
        let mut result = vec![Line::CharacterContent(vec![(vec![], CharacterLine::CharacterHeading(false), "character".to_owned())])];
        collapse_line(Line::CharacterContent(vec![(vec![], CharacterLine::CharacterHeading(true), "character 2".to_owned())]),  &mut result);

        assert_eq!(result.len(), 1);
        if let Line::CharacterContent(line) = &result[0] {
            assert_eq!(line.len(), 2);
        } else {
            panic!();
        }
    }

    #[test]
    fn with_no_formatting_returns_single_unformatted_content() {
        let result = parse_content_formatting("test line");
        
        assert_eq!(result.len(), 1);

        let content = &result[0];

        assert_eq!(content.content, "test line");
        assert_eq!(content.bold, false);
        assert_eq!(content.underline, false);
        assert_eq!(content.italic, false);
        assert_eq!(content.note, false);
    }

    #[test]
    fn parse_underline_correctly() {
        let result = parse_content_formatting("test _line_");
        
        assert_eq!(result.len(), 2);

        let content = &result[0];

        assert_eq!(content.content, "test ");
        assert_eq!(content.bold, false);
        assert_eq!(content.underline, false);
        assert_eq!(content.italic, false);
        assert_eq!(content.note, false);

        let content = &result[1];

        assert_eq!(content.content, "line");
        assert_eq!(content.bold, false);
        assert_eq!(content.underline, true);
        assert_eq!(content.italic, false);
        assert_eq!(content.note, false);
    }

    #[test]
    fn parse_bold_correctly() {
        let result = parse_content_formatting("test **line**");
        
        assert_eq!(result.len(), 2);

        let content = &result[0];

        assert_eq!(content.content, "test ");
        assert_eq!(content.bold, false);
        assert_eq!(content.underline, false);
        assert_eq!(content.italic, false);
        assert_eq!(content.note, false);

        let content = &result[1];

        assert_eq!(content.content, "line");
        assert_eq!(content.bold, true);
        assert_eq!(content.underline, false);
        assert_eq!(content.italic, false);
        assert_eq!(content.note, false);
    }

    #[test]
    fn parse_italic_correctly() {
        let result = parse_content_formatting("test *line*");
        
        assert_eq!(result.len(), 2);

        let content = &result[0];

        assert_eq!(content.content, "test ");
        assert_eq!(content.bold, false);
        assert_eq!(content.underline, false);
        assert_eq!(content.italic, false);
        assert_eq!(content.note, false);

        let content = &result[1];

        assert_eq!(content.content, "line");
        assert_eq!(content.bold, false);
        assert_eq!(content.underline, false);
        assert_eq!(content.italic, true);
        assert_eq!(content.note, false);
    }

    #[test]
    fn parse_notes_correctly() {
        let result = parse_content_formatting("test [[line]]");
        
        assert_eq!(result.len(), 2);

        let content = &result[0];

        assert_eq!(content.content, "test ");
        assert_eq!(content.bold, false);
        assert_eq!(content.underline, false);
        assert_eq!(content.italic, false);
        assert_eq!(content.note, false);

        let content = &result[1];

        assert_eq!(content.content, "line");
        assert_eq!(content.bold, false);
        assert_eq!(content.underline, false);
        assert_eq!(content.italic, false);
        assert_eq!(content.note, true);
    }
}
