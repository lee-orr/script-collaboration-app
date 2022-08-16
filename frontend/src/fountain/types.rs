use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Eq, Debug)]
pub struct LineContent {
    pub content: String,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub note: bool
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum CharacterLine {
    CharacterHeading,
    Dialogue,
    Parenthetical,
    Lyrics
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum TextAlignment {
    Left,
    Center
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Line {
    Action(Vec<LineContent>, TextAlignment),
    SceneHeading(String),
    CharacterContent(Vec<(Vec<LineContent>, CharacterLine, String)>),
    Transition(String),
    PageBreak,
    Boneyard(Vec<LineContent>),
    Section(String),
    Synopsis(Vec<LineContent>),
    Empty,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
pub struct Title {
    pub title: Option<String>,
    pub credit: Option<String>,
    pub author: Option<String>,
    pub source: Option<String>,
    pub draft: Option<String>,
    pub contact: Option<String>,
    pub meta: HashMap<String, String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Properties)]
pub struct Script {
    pub title: Title,
    pub lines: Vec<Line>,
}