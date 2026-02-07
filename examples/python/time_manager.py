"""
World Time Manager
Manages day/night cycles and provides time control commands
"""
import game

# Time constants (in ticks, 20 ticks = 1 second)
TIME_DAY = 6000      # Noon
TIME_NIGHT = 18000   # Midnight
TIME_SUNRISE = 23000
TIME_SUNSET = 13000

# Configuration
DEFAULT_WORLD = "world"

def log(message):
    """Helper function for logging"""
    game.py_log(f"[TimeManager] {message}")

def set_time(world, time):
    """Set the time in a world"""
    log(f"Setting time in {world} to {time}")
    # In full implementation:
    # game.set_time(world, time)

def set_day(world=DEFAULT_WORLD):
    """Set time to day (noon)"""
    log(f"Setting {world} to day")
    set_time(world, TIME_DAY)
    # In full implementation:
    # game.broadcast(f"Time set to day in {world}")

def set_night(world=DEFAULT_WORLD):
    """Set time to night (midnight)"""
    log(f"Setting {world} to night")
    set_time(world, TIME_NIGHT)
    # In full implementation:
    # game.broadcast(f"Time set to night in {world}")

def set_sunrise(world=DEFAULT_WORLD):
    """Set time to sunrise"""
    log(f"Setting {world} to sunrise")
    set_time(world, TIME_SUNRISE)

def set_sunset(world=DEFAULT_WORLD):
    """Set time to sunset"""
    log(f"Setting {world} to sunset")
    set_time(world, TIME_SUNSET)

def get_time_of_day(ticks):
    """Convert tick time to a readable string"""
    if 0 <= ticks < 6000:
        return "Morning"
    elif 6000 <= ticks < 12000:
        return "Afternoon"
    elif 12000 <= ticks < 18000:
        return "Evening"
    else:
        return "Night"

def monitor_time(world=DEFAULT_WORLD):
    """Monitor and log current time"""
    # In full implementation:
    # current_time = game.get_time(world)
    # time_of_day = get_time_of_day(current_time)
    # log(f"Current time in {world}: {current_time} ticks ({time_of_day})")
    log(f"Monitoring time in {world}")

# Command handlers
def handle_time_command(args):
    """Handle time-related commands"""
    if not args:
        log("Usage: time [day|night|sunrise|sunset|get]")
        return
    
    command = args[0].lower()
    world = args[1] if len(args) > 1 else DEFAULT_WORLD
    
    if command == "day":
        set_day(world)
    elif command == "night":
        set_night(world)
    elif command == "sunrise":
        set_sunrise(world)
    elif command == "sunset":
        set_sunset(world)
    elif command == "get":
        monitor_time(world)
    else:
        log(f"Unknown time command: {command}")

# Initialize
log("World Time Manager loaded!")
log("Available commands:")
log("  - time day [world] - Set time to day")
log("  - time night [world] - Set time to night")
log("  - time sunrise [world] - Set time to sunrise")
log("  - time sunset [world] - Set time to sunset")
log("  - time get [world] - Get current time")

# Example usage
# handle_time_command(["day"])
# handle_time_command(["night", "world"])
