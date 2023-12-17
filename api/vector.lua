-- Raw types.
export type Vec2 = {
  x: number,
  y: number
}

export type Vec3 = {
  x: number,
  y: number,
  z: number
}

-- Implementation.
local vector = {}

function vector.vec2(x: number, y: number): Vec2
  return {
    x = x,
    y = y
  }
end

function vector.vec3(x: number, y: number, z: number): Vec3
  return {
    x = x,
    y = y,
    z = z
  }
end


return vector