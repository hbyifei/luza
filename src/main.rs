use fltk::{app, button::Button, prelude::*, window::Window};

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 800, 600, "Luza Demo");
    let btn = Button::new(360, 270, 80, 60, "Click Me");
    wind.end();
    wind.show();

    // Lua engine
    let lua = mlua::Lua::new();

    if let Some(path) = std::env::args().nth(1) {
        if let Ok(code) = std::fs::read_to_string(&path) {
            let chunk = lua.load(&code).set_name("script.lua");
            if let Err(e) = chunk.exec() {
                eprintln!("Lua error: {}", e);
            }
        }
    }

    app.run().unwrap();
}