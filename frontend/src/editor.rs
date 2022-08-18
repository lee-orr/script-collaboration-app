use wasm_bindgen::JsCast;
use web_sys::{console, HtmlTextAreaElement};
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

pub enum EditorMsg {
    None,
    ChangedContent(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct EditorProps {
    pub content: String,
    pub title: String,
    pub changed: Callback<String>,
}

pub struct Editor {}

const CONTENTEDITABLE: AttrValue = AttrValue::Static("true");

impl Component for Editor {
    type Message = EditorMsg;
    type Properties = EditorProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let EditorProps { content, title, .. } = ctx.props();
        let content = AttrValue::from(content.to_owned());

        let onchange = ctx.link().callback(|event: InputEvent| {
            let target = event.target();
            if let Some(target) = target {
                let input = target.dyn_into::<HtmlTextAreaElement>();
                if let Ok(input) = input {
                    return EditorMsg::ChangedContent(input.value());
                }
            }
            EditorMsg::None
        });

        html!(
            <div class="h-full w-full flex flex-col items-stretch justify-center gap-2 p-2">
                <div class="text-gray-100 font-medium">{&title}</div>
                // <div contenteditable={CONTENTEDITABLE} class="whitespace-pre bg-gray-700 text-gray-100 font-light" oninput={onchange}>
                //     {&content}
                // </div>
                <textarea class="bg-gray-700 text-gray-100 font-light flex-grow" oninput={onchange} value={content}/>
            </div>)
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            EditorMsg::ChangedContent(content) => {
                unsafe {
                    console::log_2(&"Content".into(), &content.clone().into());
                }
                ctx.props().changed.emit(content);
            }
            _ => {}
        }
        false
    }

    fn changed(&mut self, ctx: &yew::Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &yew::Context<Self>) {}
}
