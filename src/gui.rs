use fltk::{app, button::Button, prelude::*, window::Window};
use mlua::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct LuaButton {
    inner: Rc<RefCell<Button>>,
}

impl LuaButton {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let btn = Button::new(x, y, w, h, label);
        Self {
            inner: Rc::new(RefCell::new(btn)),
        }
    }

    pub fn on_click(&self, func: mlua::Function) -> LuaResult<()> {
        let mut btn = self.inner.borrow().clone();
        let func = Rc::new(func);
        btn.set_callback({
            let func = func.clone();
            move |_| {
                if let Err(e) = func.call::<()>(()) {
                    eprintln!("Lua callback error: {}", e);
                }
            }
        });
        *self.inner.borrow_mut() = btn;
        Ok(())
    }
}

impl mlua::UserData for LuaButton {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("on_click", |_, this, func: mlua::Function| {
            this.on_click(func)
        });
    }
}

#[derive(Clone)]
pub struct LuaWindow {
    inner: Rc<RefCell<Window>>,
    ended: Rc<RefCell<bool>>,
}

impl LuaWindow {
    pub fn new(x: i32, y: i32, w: i32, h: i32, title: &str) -> Self {
        let win = Window::new(x, y, w, h, title);
        // 注意：此处不调用 end()，让子控件可以添加
        Self {
            inner: Rc::new(RefCell::new(win)),
            ended: Rc::new(RefCell::new(false)),
        }
    }

    pub fn show(&self) {
        let mut ended = self.ended.borrow_mut();
        if !*ended {
            self.inner.borrow_mut().end();
            *ended = true;
        }
        self.inner.borrow_mut().show();
    }
}

impl mlua::UserData for LuaWindow {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("show", |_, this, ()| {
            this.show();
            Ok(())
        });
    }
}

pub fn register_gui_module(lua: &Lua) -> LuaResult<()> {
    let gui = lua.create_table()?;

    let button_ctor = lua.create_function(|_, (x, y, w, h, label): (i32, i32, i32, i32, String)| {
        Ok(LuaButton::new(x, y, w, h, &label))
    })?;
    gui.set("Button", button_ctor)?;

    let window_ctor = lua.create_function(|_, (x, y, w, h, title): (i32, i32, i32, i32, String)| {
        Ok(LuaWindow::new(x, y, w, h, &title))
    })?;
    gui.set("Window", window_ctor)?;

    let run_fn = lua.create_function(|_, ()| {
        app::App::default().run().unwrap();
        Ok(())
    })?;
    gui.set("run", run_fn)?;

    lua.globals().set("gui", gui)?;
    Ok(())
}