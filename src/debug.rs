use gloo::events::EventListener;
use log::info;
use pulldown_cmark::{html::push_html, Parser};

use wasm_bindgen::JsCast;

use web_sys::{HtmlElement, HtmlTextAreaElement};
use yew::prelude::*;

pub struct EditorDebug {
    output_ref: NodeRef,
    input_ref: NodeRef,
    selection_listener: Option<EventListener>,
    selection_input: Option<(u32, u32)>,
    selection_output: Option<(u32, u32)>,
}

pub enum EditorDebugEvent {
    ContentChanged(String),
    InputSelectionChanged((u32, u32)),
    OutputSelectionChanged((u32, u32)),
}

impl Component for EditorDebug {
    type Message = EditorDebugEvent;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            output_ref: NodeRef::default(),
            input_ref: NodeRef::default(),
            selection_listener: None,
            selection_input: None,
            selection_output: None,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let document = gloo::utils::document();

            let link = ctx.link().clone();
            let input = self.input_ref.cast::<HtmlElement>().unwrap();
            let input_textarea = self.input_ref.cast::<HtmlTextAreaElement>().unwrap();

            let output = self.output_ref.cast::<HtmlElement>().unwrap();

            let listener = EventListener::new(&document.clone(), "selectionchange", move |e| {
                let element = document
                    .active_element()
                    .map(|e| e.dyn_into::<HtmlElement>());

                if let Some(Ok(element)) = element {
                    if output == element {
                        let selection = gloo::utils::window().get_selection().unwrap().unwrap();

                        if selection.range_count() > 0 {
                            if let Ok(range) = selection.get_range_at(0) {
                                if let (Ok(start), Ok(end)) =
                                    (range.start_offset(), range.end_offset())
                                {
                                    link.send_message(EditorDebugEvent::OutputSelectionChanged((
                                        start, end,
                                    )));
                                }
                            }
                        };
                    }

                    if input == element {
                        if let (Ok(Some(start)), Ok(Some(end))) = (
                            input_textarea.selection_start(),
                            input_textarea.selection_end(),
                        ) {
                            link.send_message(EditorDebugEvent::InputSelectionChanged((
                                start, end,
                            )));
                        };
                    }
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
            EditorDebugEvent::InputSelectionChanged((start, end)) => {
                self.selection_input = Some((start, end));
                true
            }
            EditorDebugEvent::OutputSelectionChanged((start, end)) => {
                self.selection_output = Some((start, end));
                true
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
            <div style="display: flex; height: 100%; flex-direction: column">
               <div style="display: flex; flex: 2; height: 100%">
                    <textarea style="flex: 1" oninput={content_changed}  ref={self.input_ref.clone()}/>
                    <div style="flex: 1;" tabindex="0" ref={self.output_ref.clone()}></div>
               </div>
               <div style="flex: 1">
               if let Some((start, end)) = self.selection_input
               {
                     <div>
                          {"Input: "} <span>{start}</span> {" - "} <span>{end}</span>
                     </div>
               }

                if let Some((start, end)) = self.selection_output
               {
                     <div>
                          {"Output: "} <span>{start}</span> {" - "} <span>{end}</span>
                     </div>
               }
               </div>
            </div>
        }
    }
}
