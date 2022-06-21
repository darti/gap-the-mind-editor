use pulldown_cmark::{html::push_html, Parser};

use web_sys::{HtmlElement, HtmlTextAreaElement};
use yew::prelude::*;

pub struct EditorDebug {
    output_ref: NodeRef,
}

pub enum EditorDebugEvent {
    ContentChanges(String),
}

impl Component for EditorDebug {
    type Message = EditorDebugEvent;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            output_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            EditorDebugEvent::ContentChanges(content) => {
                let parser = Parser::new(&content);

                let mut html_output = String::new();
                push_html(&mut html_output, parser);

                let elt = self.output_ref.cast::<HtmlElement>().unwrap();
                elt.set_inner_html(&html_output);

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let content_changes = link.batch_callback(|e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            Some(EditorDebugEvent::ContentChanges(input.value()))
        });

        html! {
               <div style="display: flex; height: 100%">
                    <textarea style="flex: 1" oninput={content_changes} />
                    <div style="flex: 1;" ref={self.output_ref.clone()}></div>
               </div>
        }
    }
}
