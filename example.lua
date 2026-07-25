-- example.lua
local win = gui.Window(100, 100, 640, 480, "Luza App")
local btn = gui.Button(280, 210, 80, 60, "Click Me")

btn:on_click(function()
    print("Button was clicked!")
end)

win:show()
gui.run()