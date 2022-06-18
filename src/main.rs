mod editor;

use editor::Editor;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Editor>();
}
