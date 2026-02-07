-- Player Greeter
-- Welcomes players when they join and gives them a starter kit

game:log("=== Player Greeter Script Loaded ===")

-- Configuration
local starter_items = {
    {id = "minecraft:wooden_pickaxe", count = 1},
    {id = "minecraft:wooden_axe", count = 1},
    {id = "minecraft:wooden_shovel", count = 1},
    {id = "minecraft:bread", count = 16},
    {id = "minecraft:torch", count = 32}
}

local welcome_messages = {
    "Welcome to the server!",
    "Type /help for a list of commands",
    "Have fun and play nice!"
}

-- Function to greet a player
function greet_player(player_id, player_name)
    game:log("Greeting player: " .. player_name .. " (" .. player_id .. ")")
    
    -- Send welcome messages
    for i, message in ipairs(welcome_messages) do
        game:sendMessage(player_id, message)
    end
    
    -- Give starter items
    game:sendMessage(player_id, "Here's a starter kit to help you get started!")
    for i, item in ipairs(starter_items) do
        game:giveItem(player_id, item.id, item.count)
        game:log("Gave " .. item.count .. "x " .. item.id .. " to " .. player_name)
    end
    
    -- Broadcast join message to all players
    game:broadcast(player_name .. " has joined the server!")
    
    -- Teleport to spawn
    game:teleport(player_id, "world", 0.5, 65.0, 0.5)
    game:sendMessage(player_id, "Teleported to spawn!")
end

-- Function to say goodbye to a player
function farewell_player(player_id, player_name)
    game:log("Player leaving: " .. player_name .. " (" .. player_id .. ")")
    game:broadcast(player_name .. " has left the server. Goodbye!")
end

-- Example usage - in full implementation, these would be called by event handlers
-- greet_player("player123", "TestPlayer")
-- farewell_player("player123", "TestPlayer")

game:log("Player Greeter ready!")
