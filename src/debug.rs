use gloo::{console::info, events::EventListener};
use pulldown_cmark::{html::push_html, Parser};
use wasm_bindgen::JsCast;

use web_sys::{HtmlElement, HtmlTextAreaElement};
use yew::prelude::*;

pub struct EditorDebug {
    output_ref: NodeRef,
    input_ref: NodeRef,
    selection_listener: Option<EventListener>,
}

pub enum EditorDebugEvent {
    ContentChanged(String),
    SelectionChanged((u32, u32)),
}

impl Component for EditorDebug {
    type Message = EditorDebugEvent;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            output_ref: NodeRef::default(),
            input_ref: NodeRef::default(),
            selection_listener: None,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let document = gloo::utils::document();

            let link = ctx.link().clone();
            let input = self.input_ref.cast::<HtmlTextAreaElement>().unwrap();

            let listener = EventListener::new(&document, "selectionchange", move |e| {
                if let (Ok(Some(start)), Ok(Some(end))) =
                    (input.selection_start(), input.selection_end())
                {
                    link.send_message(EditorDebugEvent::SelectionChanged((start, end)));
                }
            });

            self.selection_listener = Some(listener);
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            EditorDebugEvent::ContentChanged(content) => {
                let parser = Parser::new(&content);

                let mut html_output = String::new();
                push_html(&mut html_output, parser);

                let elt = self.output_ref.cast::<HtmlElement>().unwrap();
                elt.set_inner_html(&html_output);

                true
            }
            EditorDebugEvent::SelectionChanged((start, end)) => {
                info!("Selection: {} - {}", start, end);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let content_changed = link.batch_callback(|e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();

            Some(EditorDebugEvent::ContentChanged(input.value()))
        });

        html! {
               <div style="display: flex; height: 100%">
                    <textarea style="flex: 1" oninput={content_changed}  ref={self.input_ref.clone()}/>
                    <div style="flex: 1;" ref={self.output_ref.clone()}></div>
               </div>
        }
    }
}
