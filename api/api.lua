--!strict

----------
-- Cached functions.
local insert = table.insert


----------
-- Helper types.

export type Array<T> = {T}

----------
-- Standard minetest types.

export type BlockDefinition = {
  name: string,
  description: string,
  textures: Array<string>,
  drawtype: number
}

export type ItemDefinition = {
  name: string,
  description: string,
  readable_name: string,
  textures: Array<string>,
  drawtype: number
}

-- A fancy closure.
export type OnTick = (delta: number) -> nil

-- Singleton instances of raw data.
_G.blocks  = _G.blocks  or {}
_G.items   = _G.items   or {}
_G.on_tick = _G.on_tick or {}

local blocks:  {[string] : BlockDefinition} = _G.blocks
local items:   {[string] : ItemDefinition}  = _G.items
local on_tick: Array<OnTick>                = _G.on_tick

----------
-- Now we can ship the rest of the codebase back to the mod as a module.

-- We want to avoid piling up memory, so we're just going to overwrite.
-- This spends CPU to save memory.
-- This is also done like this so that linting works properly.
local minetest = {}

-- Mangle together the internal references.
minetest = _G.minetest or {}
_G.minetest = minetest

minetest.draw_type = {
  air       = 0,
  regular   = 1,
  block_box = 2,
  mesh      = 3
}

function minetest.register_block(definition: BlockDefinition)
  if (blocks[definition.name] ~= nil) then
    error(definition.name .. " is already a registered block.")
  end
  blocks[definition.name] = definition
  print("minetest: registered block [" .. definition.name .. "]")
end

function minetest.register_item(definition: ItemDefinition)
  if (items[definition.name] ~= nil) then
    error("error: " .. definition.name .. " is already a registered ")
  end
end

function minetest.register_on_tick(tick_closure: OnTick)
  insert(on_tick, tick_closure)
end


----------
-- API is returned as a module.

return minetest