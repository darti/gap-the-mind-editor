mod debug;
mod editor;

use debug::EditorDebug;
use editor::Editor;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<EditorDebug>();
}
