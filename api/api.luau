--!strict

local x: boolean = true

local minetest = require("api/minetest")

minetest.register_block({
  name = "air",
  drawtype = minetest.draw_type.air,
})