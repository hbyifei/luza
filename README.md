# Luza - Lua 桌面应用宿主

用 Rust 编写的轻量级 Lua 脚本桌面应用运行环境，支持 FLTK 原生 GUI。

## 特性

- 运行 Lua 脚本并显示原生窗口
- 支持按钮点击回调绑定到 Lua 函数
- 单二进制部署，零依赖
- 跨平台（Windows / macOS / Linux）

## 快速开始

### 编译

cargo build --release

### 运行

luza example.lua

### 示例脚本

local win = gui.Window(100, 100, 640, 480, "Luza App")
local btn = gui.Button(280, 210, 80, 60, "Click Me")

btn:on_click(function()
    print("Button was clicked!")
end)

win:show()
gui.run()

## 项目结构

luza/
├── src/
│   ├── main.rs    # 入口，初始化 Lua 并加载脚本
│   └── gui.rs     # GUI 控件封装（Button, Window 等）
├── Cargo.toml
├── .gitignore
└── README.md

## 当前状态

- ✅ Lua 脚本加载与执行
- ✅ 原生窗口显示
- ✅ 按钮点击触发 Lua 回调
- 🚧 更多控件（Input, Choice 等）开发中
- 🚧 打包功能（luza build）规划中

## 技术栈

- Rust + FLTK（原生 GUI）
- mlua（Lua 5.4 嵌入）
- Arc + Mutex（线程安全共享 Lua 虚拟机）

## 许可证

MIT