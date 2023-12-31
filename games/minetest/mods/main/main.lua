local minetest = require("api/api")

-- minetest.register_on_tick(function(delta: number)
--   print("Hello from LuauJIT! " .. tostring(delta))
-- end)

minetest.register_block({
  name = "minetest:stone",
  drawtype = minetest.draw_type.regular,
  description = "Stone",
  textures = {"default_stone.png"}
})

minetest.register_block({
  name = "minetest:dirt",
  drawtype = minetest.draw_type.regular,
  description = "Stone",
  textures = {"default_dirt.png"}
})

minetest.register_block({
  name = "minetest:grass",
  drawtype = minetest.draw_type.regular,
  description = "Stone",
  textures = {"default_stone.png"}
})

print("lua: minetest/main loaded")
