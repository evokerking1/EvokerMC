# Scripting API Reference

Complete reference for all functions available in EvokerMC's Lua and Python scripting APIs.

## Lua vs Python

| Feature | Lua | Python |
|---------|-----|--------|
| Object syntax | `game:method(args)` | `game.function(args)` |
| Log prefix | `game:log` | `game.py_log` |
| Engine | mlua (Lua 5.4) | PyO3 (Python 3) |
| Script directory | `scripts/lua/` | `scripts/python/` |

---

## Logging

### Lua

```lua
game:log("message")     -- INFO
game:debug("message")   -- DEBUG
game:warn("message")    -- WARN
game:error("message")   -- ERROR
```

### Python

```python
import game
game.py_log("message")    # INFO
game.py_debug("message")  # DEBUG
game.py_warn("message")   # WARN
game.py_error("message")  # ERROR
```

---

## Block Functions

### `setBlock(world, x, y, z, block_id)`

Sets the block at the given world coordinates.

```lua
game:setBlock("world", 0, 64, 0, "minecraft:stone")
```

### `getBlock(world, x, y, z)` → `string`

Returns the block ID at the given position.

```lua
local id = game:getBlock("world", 0, 64, 0)
```

### `fillBlocks(world, x1, y1, z1, x2, y2, z2, block_id)`

Fills a rectangular volume with the specified block.

```lua
game:fillBlocks("world", -5, 64, -5, 5, 64, 5, "minecraft:stone")
```

---

## Entity Functions

### `spawnEntity(world, entity_id, x, y, z)` → `string`

Spawns an entity and returns its UUID.

```lua
local uuid = game:spawnEntity("world", "minecraft:zombie", 0.0, 64.0, 0.0)
```

### `removeEntity(uuid)`

Removes the entity with the given UUID.

```lua
game:removeEntity(uuid)
```

### `getEntitiesNear(world, x, y, z, radius)` → `table`

Returns a list of entity UUIDs within `radius` blocks of the position.

```lua
local near = game:getEntitiesNear("world", 0, 64, 0, 16)
```

---

## Player Functions

### `getPlayers()` → `table`

Returns a list of online player IDs.

```lua
local players = game:getPlayers()
```

### `sendMessage(player_id, message)`

Sends a chat message to a specific player.

```lua
game:sendMessage("player123", "Hello!")
```

### `broadcast(message)`

Sends a message to all online players.

```lua
game:broadcast("Server is restarting!")
```

### `teleport(player_id, world, x, y, z)`

Teleports a player to the specified location.

```lua
game:teleport("player123", "world", 0.0, 64.0, 0.0)
```

---

## Item Functions

### `giveItem(player_id, item_id, count)`

Gives items to a player's inventory.

```lua
game:giveItem("player123", "minecraft:diamond", 64)
```

### `removeItem(player_id, item_id, count)`

Removes items from a player's inventory.

```lua
game:removeItem("player123", "minecraft:dirt", 32)
```

---

## World Functions

### `getWorlds()` → `table`

Returns a list of loaded world names.

```lua
local worlds = game:getWorlds()
```

### `getTime(world)` → `number`

Returns the current time of day (0 – 24000 ticks). Noon = 6000, midnight = 18000.

```lua
local time = game:getTime("world")
```

### `setTime(world, time)`

Sets the time of day.

```lua
game:setTime("world", 6000)   -- noon
game:setTime("world", 18000)  -- midnight
```

### `getWeather(world)` → `string`

Returns the current weather: `"clear"`, `"rain"`, or `"thunder"`.

```lua
local weather = game:getWeather("world")
```

### `setWeather(world, weather)`

Sets the weather.

```lua
game:setWeather("world", "rain")
game:setWeather("world", "clear")
```

---

## Command Functions

### `executeCommand(command)` → `string`

Executes a command and returns the result string.

```lua
local result = game:executeCommand("/say Hello World")
```

---

## Scheduling

### `schedule(delay_ticks, task_id, data)`

Schedules a task to run after `delay_ticks` game ticks (1 tick = 50 ms at 20 TPS).

```lua
game:schedule(200, "my_task", { message = "10 seconds passed!" })
```

---

## Examples

### Spawn a 5×5 platform

```lua
game:fillBlocks("world", -2, 63, -2, 2, 63, 2, "minecraft:oak_planks")
game:log("Platform created!")
```

### Greet all online players

```lua
for _, player in ipairs(game:getPlayers()) do
    game:sendMessage(player, "Welcome to the server!")
end
```

### Python: toggle day/night

```python
import game

def day():
    game.py_log("Setting to day")
    # game.set_time("world", 6000)

def night():
    game.py_log("Setting to night")
    # game.set_time("world", 18000)
```

---

## See Also

- [Scripting Guide](/modding/scripting)
- [Mod API Reference](/api/mod)
- [Mod Development Guide](/modding/development)
