--!strict

----------
-- Start by printing the running Lua VM version.
print("minetest server is running: " .. _VERSION)


----------
-- Cached function references.
local clock: () -> number = os.clock;


----------
-- "Check" if a mod is trying to hijack __internal_server.
-- This is done naively with a ton of room for improvement.
-- This will need an engine side lock.

-- Let's start where it all began.
local current_time: number = 0;

if (_G.internal_created_do_not_modify) then 
  error("minetest: DO NOT import __internal_server into your mods!")
else
  local check: number | nil = _G.internal_creation_time_stamp_do_not_modify
  if (check ~= nil and check ~= current_time) then
    error("minetest: DO NOT import __internal_server into your mods!")
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

local on_tick: minetest.Array<minetest.OnTick> = _G.on_tick

local function do_on_tick(delta: number)
  for _,func in ipairs(on_tick) do
    func(delta)
  end
end

_G.engine_on_tick_function = function(delta: number)
  local time_stamp: number = clock()  

  if (old_time_stamp == time_stamp) then
    error("minetest: DO NOT run _G.on_tick in your mods!")
  end

  do_on_tick(delta)

  old_time_stamp = time_stamp
end