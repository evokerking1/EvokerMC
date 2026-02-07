# Scripting API Documentation

The EvokerMC Scripting API allows you to create mods using Lua or Python scripting languages.

## Lua Scripting

### Getting Started

Create a Lua script file (e.g., `myscript.lua`):

```lua
-- Log a message
game:log("Hello from Lua!")

-- Get list of worlds
local worlds = game:getWorlds()
for i, world in ipairs(worlds) do
    game:log("World: " .. world)
end
```

### Logging

```lua
game:log("Info message")
game:debug("Debug message")
game:warn("Warning message")
game:error("Error message")
```

### Block Operations

```lua
-- Set a block
game:setBlock("world", 100, 64, 200, "minecraft:stone")

-- Get a block
local block = game:getBlock("world", 100, 64, 200)
game:log("Block: " .. block)

-- Fill an area with blocks
game:fillBlocks("world", 0, 60, 0, 10, 70, 10, "minecraft:diamond_block")
```

### Entity Management

```lua
-- Spawn an entity
local uuid = game:spawnEntity("world", "minecraft:zombie", 100.0, 64.0, 200.0)
game:log("Spawned entity: " .. uuid)

-- Remove an entity
game:removeEntity(uuid)
```

### Player Interaction

```lua
-- Get online players
local players = game:getPlayers()
for i, player in ipairs(players) do
    game:log("Player: " .. player)
    game:sendMessage(player, "Hello!")
end

-- Broadcast message
game:broadcast("Server announcement!")

-- Teleport player
game:teleport("player123", "world", 0.0, 64.0, 0.0)
```

### Item Management

```lua
-- Give items to player
game:giveItem("player123", "minecraft:diamond", 64)

-- Remove items from player
game:removeItem("player123", "minecraft:dirt", 32)
```

### World Management

```lua
-- Get worlds
local worlds = game:getWorlds()

-- Get time
local time = game:getTime("world")
game:log("Current time: " .. time)

-- Set time
game:setTime("world", 6000) -- Noon

-- Get weather
local weather = game:getWeather("world")
game:log("Weather: " .. weather)

-- Set weather
game:setWeather("world", "rain")
```

### Command Execution

```lua
-- Execute a command
local result = game:executeCommand("/say Hello World")
game:log("Result: " .. result)
```

### Example: Simple Spawn Platform Script

```lua
-- Create a spawn platform when the script loads
game:log("Creating spawn platform...")

-- Fill a 10x1x10 platform at y=64
game:fillBlocks("world", -5, 64, -5, 5, 64, 5, "minecraft:stone")

-- Add walls
game:fillBlocks("world", -5, 65, -5, -5, 68, 5, "minecraft:stone_bricks")
game:fillBlocks("world", 5, 65, -5, 5, 68, 5, "minecraft:stone_bricks")
game:fillBlocks("world", -4, 65, -5, 4, 68, -5, "minecraft:stone_bricks")
game:fillBlocks("world", -4, 65, 5, 4, 68, 5, "minecraft:stone_bricks")

game:log("Spawn platform created!")
```

## Python Scripting

### Getting Started

Create a Python script file (e.g., `myscript.py`):

```python
import game

# Log a message
game.py_log("Hello from Python!")

# Your mod logic here
```

### Logging

```python
import game

game.py_log("Info message")
game.py_debug("Debug message")
game.py_warn("Warning message")
game.py_error("Error message")
```

### Example: Player Greeter

```python
import game

game.py_log("Player greeter script loaded!")

# This would be called on player join event
def greet_player(player_id):
    game.py_log(f"Player {player_id} joined!")
    # In full implementation, would send message to player
```

### Example: World Time Manager

```python
import game

def set_day():
    """Set time to day (6000 ticks)"""
    game.py_log("Setting time to day")
    # In full implementation:
    # game.set_time("world", 6000)

def set_night():
    """Set time to night (18000 ticks)"""
    game.py_log("Setting time to night")
    # In full implementation:
    # game.set_time("world", 18000)

game.py_log("Time manager loaded!")
```

## API Reference

### Logging Functions

- `game:log(message)` / `game.py_log(message)` - Log info message
- `game:debug(message)` / `game.py_debug(message)` - Log debug message
- `game:warn(message)` / `game.py_warn(message)` - Log warning
- `game:error(message)` / `game.py_error(message)` - Log error

### Block Functions

- `game:setBlock(world, x, y, z, block_id)` - Set block at position
- `game:getBlock(world, x, y, z)` - Get block at position
- `game:fillBlocks(world, x1, y1, z1, x2, y2, z2, block_id)` - Fill area with blocks

### Entity Functions

- `game:spawnEntity(world, entity_id, x, y, z)` - Spawn entity
- `game:removeEntity(uuid)` - Remove entity
- `game:getEntitiesNear(world, x, y, z, radius)` - Get nearby entities

### Player Functions

- `game:getPlayers()` - Get list of online players
- `game:sendMessage(player_id, message)` - Send message to player
- `game:broadcast(message)` - Broadcast to all players
- `game:teleport(player_id, world, x, y, z)` - Teleport player

### Item Functions

- `game:giveItem(player_id, item_id, count)` - Give items
- `game:removeItem(player_id, item_id, count)` - Remove items

### World Functions

- `game:getWorlds()` - Get loaded worlds
- `game:getTime(world)` - Get time of day (0-24000)
- `game:setTime(world, time)` - Set time of day
- `game:getWeather(world)` - Get weather
- `game:setWeather(world, weather)` - Set weather

### Utility Functions

- `game:executeCommand(command)` - Execute command
- `game:schedule(delay_ticks, task_id, data)` - Schedule delayed task

## Loading Scripts

Scripts are loaded by the mod loader:

- Place `.lua` files in the `scripts/lua/` directory
- Place `.py` files in the `scripts/python/` directory

Scripts are automatically executed when the mod loads.

## Best Practices

1. **Error Handling**: Use pcall() in Lua or try/except in Python
2. **Performance**: Avoid heavy computations in event handlers
3. **Testing**: Test scripts thoroughly before deploying
4. **Logging**: Use appropriate log levels (debug for development, info for production)
5. **Namespacing**: Prefix your custom IDs with your mod name

## Current Limitations

The scripting API is currently in development. Some features may be placeholder implementations that will be fully implemented in future versions.

## Examples Directory

See the `examples/` directory for more complete script examples:
- `examples/lua/spawn_platform.lua` - Creates a spawn platform
- `examples/lua/player_greeter.lua` - Greets players on join
- `examples/python/time_manager.py` - Manages world time
- `examples/python/economy.py` - Simple economy system
