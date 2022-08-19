use gloo::timers::callback::Timeout;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, InputEvent};
use yew::{html, Callback, Component, Properties};

use crate::fountain::types::LineContent;

use super::formatting_functions::*;

#[derive(Clone, PartialEq)]
pub enum TitleAlignment {
    Center,
    Left,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TitleElementEditorProps {
    pub tag: String,
    pub element_text: Option<Vec<LineContent>>,
    pub changed: Callback<(String, String)>,
    pub is_editor: bool,
    pub alignment: TitleAlignment,
}

pub struct TitleElementEditor {
    timeout: Option<Timeout>,
    inner_content: Option<String>
}

impl Component for TitleElementEditor {
    type Message = EditorMsg;

    type Properties = TitleElementEditorProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            timeout: None,
            inner_content: None
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            EditorMsg::ChangedContent(content) => {
                self.inner_content = Some(content);
                let timeout = self.timeout.take();
                self.timeout = None;

                if let Some(timeout) = timeout {
                    timeout.cancel();
                }

                let handle = {
                    let link = ctx.link().clone();
                    Timeout::new(2000, move || link.send_message(EditorMsg::ReadyToParse))
                };

                self.timeout = Some(handle);
            }
            EditorMsg::ReadyToParse => {
                if let Some(content) = &self.inner_content {
                    ctx.props().changed.emit((ctx.props().tag.clone(), content.clone()));
                }
            }
            EditorMsg::None => {},
        }
        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let TitleElementEditorProps {
            tag,
            element_text,
            is_editor,
            changed,
            alignment,
        } = &ctx.props();

        let is_editor = *is_editor;

        let onchange = ctx.link().callback(|event: InputEvent| {
            let target = event.target();
            if let Some(target) = target {
                let input = target.dyn_into::<HtmlElement>();
                if let Ok(input) = input {
                    return EditorMsg::ChangedContent(input.inner_text());
                }
            }
            EditorMsg::None
        });

        let classes = format!(
            "flex flex-row whitespace-pre-wrap {}",
            match alignment {
                TitleAlignment::Center => "justify-center",
                TitleAlignment::Left => "justify-start",
            }
        );

        html!(
            <>
                if is_editor {
                    {editor_tag(&format!("{}: ", tag))}
                }
                <span contenteditable={if is_editor {"true"} else {"false"}} oninput={onchange} class={classes}>
                if let Some(element_text) = element_text {
                    {format_text(&element_text, is_editor, is_editor)}
                }
                </span>
            </>
        )
    }
}
