local minetest = require("api/api")

minetest.register_on_tick(function(delta: number)
  print("Hello from Luaujit! " .. tostring(delta))
end)

print("lua: minetest/main loaded")
