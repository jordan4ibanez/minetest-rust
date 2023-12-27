local minetest = require("api/api")

minetest.register_on_tick(function(delta: number)
  print("Hello from LuauJIT! " .. tostring(delta))
end)

print("lua: minetest/main loaded")
