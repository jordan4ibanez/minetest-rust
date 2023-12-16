--!strict

print(_VERSION)
local x: boolean = true

local minetest = require("api/minetest")

minetest.register_block({
  name = "air",
  drawtype = minetest.draw_type.air,
})

-- Assembled definition test.
-- Assembly pattern is not type safe so that's why this errors
local definition: minetest.Block = {}
definition.name = "stone"
definition.drawtype = minetest.draw_type.regular
minetest.register_block(definition)

-- But htis doesn't :D
local boof: minetest.Block = {
  name = "stone",
  drawtype = minetest.draw_type.regular
}
