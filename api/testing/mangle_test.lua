--!strict

local minetest = require("api/api")

local function ref_test()
  print("testing 123 is this thing on?")
end

minetest.register_on_step(ref_test)

return nil
