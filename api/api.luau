--!strict

print("minetest is running: " .. _VERSION)

-- Helper types.
export type Array<T> = {T}

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

-- Singleton instances of raw data.
_G.blocks = _G.blocks or {}
_G.items = _G.items or {}

local blocks: {[string] : BlockDefinition} = _G.blocks
local items: {[string] : ItemDefinition}  = _G.items

-- Now we can ship the rest of the codebase back to the mod as a module.

local minetest = {}

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

return minetest