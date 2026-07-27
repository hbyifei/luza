use fltk::{app, button::Button, button::CheckButton, input::Input, prelude::*, window::Window};
use mlua::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

// ========== Button ==========

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

// ========== Window ==========

#[derive(Clone)]
pub struct LuaWindow {
    inner: Rc<RefCell<Window>>,
    ended: Rc<RefCell<bool>>,
}

impl LuaWindow {
    pub fn new(x: i32, y: i32, w: i32, h: i32, title: &str) -> Self {
        let win = Window::new(x, y, w, h, title);
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

// ========== Input 输入框 ==========

#[derive(Clone)]
pub struct LuaInput {
    inner: Rc<RefCell<Input>>,
}

impl LuaInput {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let input = Input::new(x, y, w, h, Some(label));
        Self {
            inner: Rc::new(RefCell::new(input)),
        }
    }

    pub fn get_value(&self) -> String {
        self.inner.borrow().value()
    }

    pub fn set_value(&self, val: String) {
        self.inner.borrow_mut().set_value(&val);
    }

    pub fn on_change(&self, func: mlua::Function) -> LuaResult<()> {
        let mut input = self.inner.borrow().clone();
        let func = Rc::new(func);
        input.set_callback({
            let func = func.clone();
            move |_| {
                if let Err(e) = func.call::<()>(()) {
                    eprintln!("Lua callback error: {}", e);
                }
            }
        });
        *self.inner.borrow_mut() = input;
        Ok(())
    }
}

impl mlua::UserData for LuaInput {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get_value", |_, this, ()| {
            Ok(this.get_value())
        });
        methods.add_method("set_value", |_, this, val: String| {
            this.set_value(val);
            Ok(())
        });
        methods.add_method_mut("on_change", |_, this, func: mlua::Function| {
            this.on_change(func)
        });
    }
}

// ========== CheckButton 复选框 ==========

#[derive(Clone)]
pub struct LuaCheckButton {
    inner: Rc<RefCell<CheckButton>>,
}

impl LuaCheckButton {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let cb = CheckButton::new(x, y, w, h, label);
        Self {
            inner: Rc::new(RefCell::new(cb)),
        }
    }

    pub fn is_checked(&self) -> bool {
        self.inner.borrow().is_set()
    }

    pub fn set_checked(&self, checked: bool) {
        self.inner.borrow_mut().set_value(checked);
    }

    pub fn on_toggle(&self, func: mlua::Function) -> LuaResult<()> {
        let mut cb = self.inner.borrow().clone();
        let func = Rc::new(func);
        cb.set_callback({
            let func = func.clone();
            move |_| {
                if let Err(e) = func.call::<()>(()) {
                    eprintln!("Lua callback error: {}", e);
                }
            }
        });
        *self.inner.borrow_mut() = cb;
        Ok(())
    }
}

impl mlua::UserData for LuaCheckButton {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("is_checked", |_, this, ()| {
            Ok(this.is_checked())
        });
        methods.add_method("set_checked", |_, this, checked: bool| {
            this.set_checked(checked);
            Ok(())
        });
        methods.add_method_mut("on_toggle", |_, this, func: mlua::Function| {
            this.on_toggle(func)
        });
    }
}

// ========== 注册 gui 模块 ==========

pub fn register_gui_module(lua: &Lua) -> LuaResult<()> {
    let gui = lua.create_table()?;

    // gui.Button(x, y, w, h, label)
    let button_ctor = lua.create_function(|_, (x, y, w, h, label): (i32, i32, i32, i32, String)| {
        Ok(LuaButton::new(x, y, w, h, &label))
    })?;
    gui.set("Button", button_ctor)?;

    // gui.Window(x, y, w, h, title)
    let window_ctor = lua.create_function(|_, (x, y, w, h, title): (i32, i32, i32, i32, String)| {
        Ok(LuaWindow::new(x, y, w, h, &title))
    })?;
    gui.set("Window", window_ctor)?;

    // gui.Input(x, y, w, h, label)
    let input_ctor = lua.create_function(|_, (x, y, w, h, label): (i32, i32, i32, i32, String)| {
        Ok(LuaInput::new(x, y, w, h, &label))
    })?;
    gui.set("Input", input_ctor)?;

    // gui.CheckButton(x, y, w, h, label)
    let cb_ctor = lua.create_function(|_, (x, y, w, h, label): (i32, i32, i32, i32, String)| {
        Ok(LuaCheckButton::new(x, y, w, h, &label))
    })?;
    gui.set("CheckButton", cb_ctor)?;

    // gui.run()
    let run_fn = lua.create_function(|_, ()| {
        app::App::default().run().unwrap();
        Ok(())
    })?;
    gui.set("run", run_fn)?;

    lua.globals().set("gui", gui)?;
    Ok(())
}