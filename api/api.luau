--!strict

local x: boolean = true

local minetest = require("api/minetest")

minetest.register_block({
  name = "air",
  drawtype = minetest.draw_type.air,
})

-- Assembled definition test.
-- TODO: fix this somehow.
local definition: Block = {}
definition.name = "stone"
definition.draw_type = minetest.draw_type.block_box
minetest.register_block(definition)