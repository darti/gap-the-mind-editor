use log::info;
use ropey::Rope;
use yew::prelude::*;

pub struct Editor {
    node_ref: NodeRef,
    text: Rope,
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
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            EditorEvent::Keypress(keyboard_event) => {
                info!("MSG: {:?}", keyboard_event);

                let win = gloo::utils::window();
                let pos = win.get_selection();

                info!("POS: {:?}", pos);

                self.text.insert(0, &keyboard_event.key());
                true
            }
        }
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
