use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Line {
    Action(String),
    Character(String),
    Dialogue(String, String),
    Parenthetical(String),
    DualDialogue(Vec<(String, String)>),
    Lyrics(String, String),
    Transition(String),
    CenteredText(String),
    PageBreak,
    Note(String),
    Boneyard(String),
    Section(String),
    Synopsis(String),
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Title {
    pub title: Option<String>,
    pub credit: Option<String>,
    pub author: Option<String>,
    pub source: Option<String>,
    pub draft: Option<String>,
    pub contact: Option<String>,
    pub meta: HashMap<String, String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Script {
    pub title: Title,
    pub lines: Vec<Line>,
}
