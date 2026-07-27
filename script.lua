local win = gui.Window(100, 100, 420, 220, "Luza CheckButton Demo")

local cb1 = gui.CheckButton(50, 45, 150, 30, "开启音效")
cb1:set_checked(true)

local cb2 = gui.CheckButton(50, 80, 150, 30, "自动更新")

local status = gui.Input(50, 125, 230, 26, "状态:")
status:set_value("音效: 开启 | 更新: 关闭")

cb1:on_toggle(function()
    local s1 = "音效: " .. (cb1:is_checked() and "开启" or "关闭")
    local s2 = "更新: " .. (cb2:is_checked() and "开启" or "关闭")
    status:set_value(s1 .. " | " .. s2)
    print("音效状态: " .. tostring(cb1:is_checked()))
end)

cb2:on_toggle(function()
    local s1 = "音效: " .. (cb1:is_checked() and "开启" or "关闭")
    local s2 = "更新: " .. (cb2:is_checked() and "开启" or "关闭")
    status:set_value(s1 .. " | " .. s2)
    print("更新状态: " .. tostring(cb2:is_checked()))
end)

local btn = gui.Button(290, 175, 80, 28, "确定")
btn:on_click(function()
    print("最终设置: 音效=" .. tostring(cb1:is_checked()) .. ", 更新=" .. tostring(cb2:is_checked()))
end)

win:show()
gui.run()