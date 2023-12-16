local minetest = require("api.minetest")

print(minetest)

local x: boolean = true

function test(input: number): nil
  print(input)
end

test(1235)

type Point = {
  x: number,
  y: number
}

-- Autocomplete is amazing.
function type_consumer(input: Point): nil
  print(input.x, input.y)
end

-- Doesn't get confused. Excellent.
local Point = {
  new = function(): Point
    return {
      x = 0,
      y = 0
    }
  end
}

-- AMAZING!
type_consumer(Point.new())