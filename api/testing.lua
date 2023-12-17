local minetest = require("api/api")
local vector = require("api/vector")

local vec_test: vector.Vec2 = vector.vec2(1, 2)

minetest.register_on_step(function(delta: number)
  print("hello from LuauJIT: " .. tostring(delta))
end)

require("api/testing/mangle_test")


minetest.register_on_step(function()
  print("test2")
end)
