mod editor;

use gloo::events::EventListener;
use log::info;
use web_sys::{Event, KeyboardEvent};

use wasm_bindgen::{prelude::*, JsCast};

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    let doc = gloo::utils::document();
    let body = doc.body().unwrap();

    let editor = doc.create_element("div").unwrap();
    editor.set_attribute("contenteditable", "true").unwrap();
    editor.set_class_name("editor");

    let button = doc.create_element("button").unwrap();
    button.set_inner_html("Click me!");

    let editor_update = EventListener::new(&button, "click", move |e: &Event| {
        let event = e.dyn_ref::<KeyboardEvent>().unwrap_throw();
        info!("{:?}", event);
        gloo::dialogs::alert("Hello, world!");
    });

    Box::new(editor_update);

    body.append_child(&editor).unwrap();
    body.append_child(&button).unwrap();

    info!("Exiting...");
}
