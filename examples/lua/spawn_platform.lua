-- Spawn Platform Builder
-- Creates a nice spawn platform at spawn point

game:log("=== Spawn Platform Builder ===")
game:log("Building spawn platform...")

-- Configuration
local center_x = 0
local center_y = 64
local center_z = 0
local platform_size = 10
local wall_height = 4

-- Build the platform floor
game:log("Building platform floor...")
game:fillBlocks(
    "world",
    center_x - platform_size, center_y, center_z - platform_size,
    center_x + platform_size, center_y, center_z + platform_size,
    "minecraft:stone"
)

-- Add decorative border
game:fillBlocks(
    "world",
    center_x - platform_size, center_y, center_z - platform_size,
    center_x + platform_size, center_y, center_z - platform_size,
    "minecraft:stone_bricks"
)
game:fillBlocks(
    "world",
    center_x - platform_size, center_y, center_z + platform_size,
    center_x + platform_size, center_y, center_z + platform_size,
    "minecraft:stone_bricks"
)
game:fillBlocks(
    "world",
    center_x - platform_size, center_y, center_z - platform_size,
    center_x - platform_size, center_y, center_z + platform_size,
    "minecraft:stone_bricks"
)
game:fillBlocks(
    "world",
    center_x + platform_size, center_y, center_z - platform_size,
    center_x + platform_size, center_y, center_z + platform_size,
    "minecraft:stone_bricks"
)

-- Build walls
game:log("Building walls...")
for h = 1, wall_height do
    local y = center_y + h
    
    -- North wall
    game:fillBlocks(
        "world",
        center_x - platform_size, y, center_z - platform_size,
        center_x + platform_size, y, center_z - platform_size,
        "minecraft:stone_bricks"
    )
    
    -- South wall
    game:fillBlocks(
        "world",
        center_x - platform_size, y, center_z + platform_size,
        center_x + platform_size, y, center_z + platform_size,
        "minecraft:stone_bricks"
    )
    
    -- West wall
    game:fillBlocks(
        "world",
        center_x - platform_size, y, center_z - platform_size + 1,
        center_x - platform_size, y, center_z + platform_size - 1,
        "minecraft:stone_bricks"
    )
    
    -- East wall
    game:fillBlocks(
        "world",
        center_x + platform_size, y, center_z - platform_size + 1,
        center_x + platform_size, y, center_z + platform_size - 1,
        "minecraft:stone_bricks"
    )
end

-- Add torches at corners
local torch_y = center_y + wall_height
game:setBlock("world", center_x - platform_size + 1, torch_y, center_z - platform_size + 1, "minecraft:torch")
game:setBlock("world", center_x + platform_size - 1, torch_y, center_z - platform_size + 1, "minecraft:torch")
game:setBlock("world", center_x - platform_size + 1, torch_y, center_z + platform_size - 1, "minecraft:torch")
game:setBlock("world", center_x + platform_size - 1, torch_y, center_z + platform_size - 1, "minecraft:torch")

-- Add a welcome sign at center
game:setBlock("world", center_x, center_y + 1, center_z, "minecraft:oak_sign")

game:log("Spawn platform complete!")
game:broadcast("Spawn platform has been built!")
