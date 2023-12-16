--!strict

-- Sample types.
export type Block = {
  name: string,
  drawtype: number
}

-- Singleton instances of raw data.
local blocks = _G.blocks;
local items  = _G.items;

-- Utilize a closure iterator to automate this process.
for _,container in ipairs({blocks, items}) do
  if (container == nil) then
    container = {}
  end
end

-- Now we can ship the rest of the codebase back to the mod as a module.
local minetest = {}

minetest.draw_type = {
  air       = 0,
  regular   = 1,
  block_box = 2
}

function minetest.register_block(definition: Block) 

end

function minetest.cool(): nil
  print("beans")
  return
end

return minetest