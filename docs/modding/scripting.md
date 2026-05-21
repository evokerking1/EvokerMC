# Scripting

EvokerMC supports scripting in **Lua 5.4** and **Python 3**. Scripts are the quickest way to add mod behaviour without any compilation step.

## Getting Started

### Lua

Place `.lua` files in `scripts/lua/`. They are executed automatically when the engine starts.

```lua
-- scripts/lua/hello.lua
game:log("Hello from Lua!")
```

### Python

Place `.py` files in `scripts/python/`. They are executed automatically when the engine starts.

```python
# scripts/python/hello.py
import game
game.py_log("Hello from Python!")
```

## Lua API Reference

### Logging

```lua
game:log("info message")
game:debug("debug message")
game:warn("warning message")
game:error("error message")
```

### Blocks

```lua
game:setBlock("world", x, y, z, "minecraft:stone")
local id = game:getBlock("world", x, y, z)
game:fillBlocks("world", x1, y1, z1, x2, y2, z2, "minecraft:stone")
```

### Entities

```lua
local uuid = game:spawnEntity("world", "minecraft:zombie", x, y, z)
game:removeEntity(uuid)
local nearby = game:getEntitiesNear("world", x, y, z, radius)
```

### Players

```lua
local players = game:getPlayers()
game:sendMessage(player_id, "Hello!")
game:broadcast("Server announcement!")
game:teleport(player_id, "world", x, y, z)
```

### Items

```lua
game:giveItem(player_id, "minecraft:diamond", 64)
game:removeItem(player_id, "minecraft:dirt", 32)
```

### World

```lua
local worlds = game:getWorlds()
local time = game:getTime("world")   -- 0–24000
game:setTime("world", 6000)          -- noon
local weather = game:getWeather("world")
game:setWeather("world", "rain")     -- "clear" | "rain" | "thunder"
```

### Commands

```lua
local result = game:executeCommand("/say Hello")
```

### Scheduling

```lua
game:schedule(100, "my_task", { key = "value" })  -- run after 100 ticks
```

## Python API Reference

### Logging

```python
import game
game.py_log("info")
game.py_debug("debug")
game.py_warn("warning")
game.py_error("error")
```

The Python API mirrors the Lua API with the `py_` prefix for logging functions and direct function calls (no method chaining).

## Examples

### Spawn Platform (Lua)

```lua
-- Create a 10×1×10 platform at y=64
game:fillBlocks("world", -5, 64, -5, 5, 64, 5, "minecraft:stone")
game:log("Spawn platform created!")
```

### Player Greeter (Lua)

```lua
local players = game:getPlayers()
for _, player in ipairs(players) do
    game:sendMessage(player, "Welcome to the server!")
end
```

### Time Manager (Python)

```python
import game

def set_day():
    game.py_log("Setting time to day")
    # game.set_time("world", 6000)

def set_night():
    game.py_log("Setting time to night")
    # game.set_time("world", 18000)

game.py_log("Time manager loaded!")
```

More examples are in the `examples/` directory:

- `examples/lua/spawn_platform.lua`
- `examples/lua/player_greeter.lua`
- `examples/python/time_manager.py`
- `examples/python/economy.py`

## Best Practices

1. Use `pcall()` (Lua) or `try/except` (Python) to guard against errors.
2. Keep event handlers fast — heavy computation blocks the game tick.
3. Prefix all custom IDs with your mod name: `mymod:item_name`.
4. Use `game:debug()` / `game.py_debug()` during development; switch to `game:log()` for production.

## Current Limitations

Some API functions are placeholder implementations and will be fully wired up in a future release. Check the function list above for features marked as "planned".

## See Also

- [Scripting API Reference](/api/scripting)
- [Mod Development Guide](/modding/development)
