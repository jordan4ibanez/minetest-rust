--!strict

----------
-- Start by printing the running Lua VM version.
print("minetest is running: " .. _VERSION)

----------
-- "Check" if a mod is trying to hijack __internal.
-- This is done naively with a ton of room for improvement.
-- This will need an engine side lock.

local clock: () -> number = os.clock;

-- Let's start where it all began.
local current_time: number = 0;

if (_G.internal_created_do_not_modify) then 
  error("minetest: DO NOT import __internal into your mods!")
else
  local check: number | nil = _G.internal_creation_time_stamp_do_not_modify
  if (check ~= nil and check ~= current_time) then
    error("minetest: DO NOT import __internal into your mods!")
  end
end

_G.internal_created_do_not_modify = true
_G.internal_creation_time_stamp_do_not_modify = current_time

----------
-- Next we simply require the api to create and access the base implementation.

local minetest = require("api/api")

----------
-- Now we can create internalized procedures with defined components.
-- Bonus: We also have linting, woo!

local old_time_stamp: number = clock()

local on_step: minetest.Array<minetest.OnStep> = _G.on_step

local function do_on_step(delta: number)
  for _,func in ipairs(on_step) do
    func(delta)
  end
end

_G.engine_on_step_function = function(delta: number)
  local time_stamp: number = clock()  

  if (old_time_stamp == time_stamp) then
    error("minetest: DO NOT run _G.on_step in your mods!")
  end

  do_on_step(delta)

  old_time_stamp = time_stamp
end

-- "minetest: DO NOT run _G.on_step in your mods!"
