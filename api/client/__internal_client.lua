--!strict

----------
-- Start by printing the running Lua VM version.
print("minetest client is running: " .. _VERSION)


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

--[[
* Rambly ideas:
*
* Might need some kind of client mods folder or something.
* Unless we can just trigger out client things via shipping
* lua code to the client marked with "client mod" or something.
*
* Walking view bobbing animation can be done procedurally.
* Maybe:
* minetest.set_client_walk_animation(procedure: (number) -> void)
*
* Digging/placing animation can be done procedurally.
* These can also be done separately!
* These could be customized per block!
* Maybe:
* minetest.set_dig_animation(procedure: (number) -> void)
* minetest.set_place_animation(procedure: (number) -> void)
* 
* todo: more ideas
*
* Make it so the end user can move the android touch screen
* buttons around in the settings menu.
*
* celeron55:
* "i think ideally the server should just tell the client a list of 
* key mappings with default keys and the user should be able to 
* remap them on the client"
*
* This is for accessibility for things like mobile clients.
*
* "this would allow mobile clients to still be useful even if a 
* game has many extra inputs - it could just list them as buttons
* in a menu or on an automatically generated top layer HUD or whatever.
* and of course let's not forget accessibility, mobile is a pretty
* good excercise in that too from the server and protocol perspective"
*
* "so if your game is very specifically about sitting and everyone
* needs to be able to sit at any moment, you'd add "sit" with the
* default key "x" or whatever, and then every client and user will
* be able to deal with that"
*
* todo: ^ figure out how to do this without causing spaghetti.
*
* grorp:
* "the touchscreen controls are really bad. what they need is not more
* buttons, but "sub"game control. "sub"game-defined, context-dependent
* touchscreen controls. otherwise, the controls will always feel like
* a low-effort port of the desktop controls."
*
* todo: ^ figure out how to do this without causing Olive Garden.
*
]]