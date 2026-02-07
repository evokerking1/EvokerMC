# EvokerMC Scripting Examples

This directory contains example scripts demonstrating the EvokerMC Scripting API.

## Lua Examples

### spawn_platform.lua
Creates a decorative spawn platform with walls and torches at the world spawn point.

**Features:**
- Builds a stone platform with decorative border
- Adds stone brick walls around the platform
- Places torches at corners for lighting
- Adds a welcome sign at the center

**Usage:**
```bash
# Place in scripts/lua/ directory
# Script will run automatically when the server starts
```

### player_greeter.lua
Welcomes players when they join the server and gives them a starter kit.

**Features:**
- Sends welcome messages to new players
- Gives starter tools and resources
- Broadcasts join/leave messages
- Teleports players to spawn

**Starter Kit:**
- Wooden pickaxe
- Wooden axe
- Wooden shovel
- 16 bread
- 32 torches

**Usage:**
```bash
# Place in scripts/lua/ directory
# Functions will be called by event handlers
```

## Python Examples

### time_manager.py
Manages world time with convenient commands for setting day, night, and other times.

**Features:**
- Set time to day, night, sunrise, or sunset
- Get current time and time of day
- Support for multiple worlds
- Time constants for easy reference

**Commands:**
- `time day [world]` - Set time to noon
- `time night [world]` - Set time to midnight
- `time sunrise [world]` - Set time to sunrise
- `time sunset [world]` - Set time to sunset
- `time get [world]` - Get current time

**Usage:**
```python
import time_manager

# Set time to day
time_manager.set_day()

# Set time to night in specific world
time_manager.set_night("world_nether")

# Handle time command
time_manager.handle_time_command(["day"])
```

### economy.py
Implements a simple economy system with currency and transactions.

**Features:**
- Player balance management
- Money transfers between players
- Item buying and selling
- Economy leaderboard
- Persistent balance tracking (in memory)

**Currency:**
- Name: "coins"
- Starting balance: 100

**Commands:**
- `/balance` - Check your balance
- `/pay <player> <amount>` - Send money to another player
- `/buy <item>` - Buy an item from the shop
- `/sell <item>` - Sell an item
- `/baltop` - Show economy leaderboard

**Usage:**
```python
import economy

# Check balance
economy.handle_balance_command("player123")

# Add money (e.g., from mining)
economy.add_money("player123", 50)

# Transfer money
economy.transfer_money("player123", "player456", 25)

# Buy an item
economy.buy_item("player123", "minecraft:diamond_sword", 100)

# Show leaderboard
economy.show_leaderboard()
```

## Running the Examples

### Lua Scripts

1. Create the scripts directory if it doesn't exist:
   ```bash
   mkdir -p scripts/lua
   ```

2. Copy the Lua scripts:
   ```bash
   cp examples/lua/*.lua scripts/lua/
   ```

3. Scripts will be loaded automatically when the server starts

### Python Scripts

1. Create the scripts directory if it doesn't exist:
   ```bash
   mkdir -p scripts/python
   ```

2. Copy the Python scripts:
   ```bash
   cp examples/python/*.py scripts/python/
   ```

3. Import and use the scripts in your mod code:
   ```python
   import time_manager
   import economy
   ```

## Extending the Examples

These examples serve as a starting point. You can extend them by:

1. **Adding event handlers** - React to player actions, world events, etc.
2. **Persistent storage** - Save data between server restarts
3. **Configuration files** - Make settings customizable
4. **Integration** - Combine multiple scripts for complex functionality
5. **Custom commands** - Add new commands and features

## API Documentation

For complete API documentation, see:
- [Mod API Documentation](../docs/modding/mod-api.md)
- [Scripting API Documentation](../docs/modding/scripting-api.md)

## Support

For help with scripting:
- Check the documentation in `docs/modding/`
- Look at example implementations
- Review the source code in `evoker-scripting/src/`

## Contributing

To contribute new examples:
1. Create a well-commented script
2. Add documentation in this README
3. Test thoroughly
4. Submit a pull request

## License

These examples are provided under the same license as EvokerMC (MIT OR Apache-2.0).
