mod gui;

use std::sync::{Arc, Mutex};

fn main() {
    let lua = Arc::new(Mutex::new(mlua::Lua::new()));

    {
        let lua = lua.lock().unwrap();
        gui::register_gui_module(&lua).expect("Failed to register gui module");
    }

    if let Some(path) = std::env::args().nth(1) {
        let code = std::fs::read_to_string(&path).expect("Failed to read script file");
        let lua = lua.lock().unwrap();
        lua.load(&code).set_name("script.lua").exec()
            .expect("Lua script execution failed");
    } else {
        eprintln!("Usage: luza <script.lua>");
        std::process::exit(1);
    }
}