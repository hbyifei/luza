use fltk::{app, window::Window, prelude::*};
use mlua::Lua;

fn main() {
    // 创建 Lua 虚拟机
    let lua = Lua::new();

    // 注册 Rust 函数到 Lua，让 Lua 能创建窗口
    let create_window = lua
        .create_function(|_, (width, height): (i32, i32)| {
            let app = app::App::default();
            let mut win = Window::new(100, 100, width, height, "Luza Demo");
            win.end();
            win.show();
            app.run().unwrap();
            Ok(())
        })
        .unwrap();

    lua.globals().set("create_window", create_window).unwrap();

    // 执行 Lua 脚本
    lua.eval::<()>(
        r#"
        print("Hello from Lua!")
        create_window(400, 300)
        "#,
    )
    .unwrap();
}