use log::info;
use ropey::Rope;
use web_sys::{HtmlElement, Range};
use yew::prelude::*;

pub struct Editor {
    node_ref: NodeRef,
    text: Rope,
    caret: u32,
}

pub enum EditorEvent {
    Keypress(KeyboardEvent),
}

impl Component for Editor {
    type Message = EditorEvent;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            text: Rope::from("toto nard"),
            node_ref: NodeRef::default(),
            caret: 0,
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            EditorEvent::Keypress(keyboard_event) => {
                let win = gloo::utils::window();

                let selection = win.get_selection().unwrap().unwrap();

                let position = if selection.range_count() > 0 {
                    let range = selection.get_range_at(0).unwrap();

                    range.start_offset().unwrap()
                } else {
                    0
                };

                info!("{} at {}", keyboard_event.key(), position);

                self.text.insert(position as usize, &keyboard_event.key());
                self.caret = position + 1;
                true
            }
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        let win = gloo::utils::window();
        let selection = win.get_selection().unwrap().unwrap();

        let elt = self
            .node_ref
            .cast::<HtmlElement>()
            .unwrap()
            .first_child()
            .unwrap();

        selection.remove_all_ranges().unwrap();
        let range = Range::new().unwrap();

        range.set_start(&elt, self.caret).unwrap();
        range.set_end(&elt, self.caret).unwrap();

        selection.add_range(&range).unwrap();
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let link = ctx.link();

        let key_pressed = link.callback(|e: KeyboardEvent| {
            e.prevent_default();
            EditorEvent::Keypress(e)
        });

        html! {
            <div type="text" class="editor" onkeypress={key_pressed} contenteditable="true" ref={self.node_ref.clone()}>
            {&self.text}
            </div>
        }
    }
}
