--!strict

-- Sample types.
export type Block = {
  name: string,
  drawtype: number
}

-- Singleton instances of raw data.
_G.blocks = _G.blocks or {}
_G.items = _G.items or {}

local blocks: {[string] : Block} = _G.blocks
local items  = _G.items


-- Now we can ship the rest of the codebase back to the mod as a module.
local minetest = {}

minetest.draw_type = {
  air       = 0,
  regular   = 1,
  block_box = 2
}

function minetest.register_block(definition: Block)
  if (blocks[definition.name] ~= nil) then
    error(definition.name .. " is already a registered block.")
  end
  blocks[definition.name] = definition
  print("minetest: registered block [" .. definition.name .. "]")
end

function minetest.cool(): nil
  print("beans")
  return
end

return minetest