local win = gui.Window(100, 100, 480, 320, "Luza Input Demo")

local name_input = gui.Input(140, 60, 220, 30, "姓名:")
name_input:set_value("请输入你的名字")

local result = gui.Input(140, 110, 220, 30, "结果:")
result:set_value("")

name_input:on_change(function()
    print("输入内容变为: " .. name_input:get_value())
end)

local btn = gui.Button(200, 170, 80, 40, "确定")
btn:on_click(function()
    local text = name_input:get_value()
    print("你好, " .. text .. "!")
    result:set_value("你好, " .. text .. "!")
end)

win:show()
gui.run()