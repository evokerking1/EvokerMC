# Mod API Documentation

The EvokerMC Mod API provides a comprehensive interface for mods to interact with the game engine.

## Getting Started

The Mod API is accessed through the `ModApi` struct, which is provided to your mod during initialization.

```rust
use evoker_modding::ModApi;
use std::sync::Arc;

// In your mod's initialization
pub fn init(api: Arc<ModApi>) {
    // Use the API
    api.register_block("mymod:custom_block", serde_json::json!({
        "hardness": 2.0,
        "solid": true
    })).unwrap();
}
```

## Block Management

### Register a Block

```rust
api.register_block("mymod:stone", serde_json::json!({
    "hardness": 1.5,
    "resistance": 6.0,
    "solid": true,
    "opaque": true
}))?;
```

### Set Block in World

```rust
api.set_block("world", 100, 64, 200, "mymod:custom_block").await?;
```

### Get Block at Position

```rust
let block_id = api.get_block_at("world", 100, 64, 200)?;
println!("Block at position: {}", block_id);
```

## Item Management

### Register an Item

```rust
api.register_item("mymod:diamond_sword", serde_json::json!({
    "max_stack_size": 1,
    "max_durability": 1561,
    "rarity": "rare"
}))?;
```

### Give Item to Player

```rust
api.give_item("player123", "mymod:diamond_sword", 1).await?;
```

### Remove Item from Player

```rust
api.remove_item("player123", "mymod:wood", 64).await?;
```

## Entity Management

### Register an Entity

```rust
api.register_entity("mymod:zombie", serde_json::json!({
    "health": 20.0,
    "speed": 0.25,
    "hostile": true
}))?;
```

### Spawn Entity

```rust
let entity_uuid = api.spawn_entity("world", "mymod:zombie", 100.0, 64.0, 200.0).await?;
println!("Spawned entity: {}", entity_uuid);
```

### Remove Entity

```rust
api.remove_entity(&entity_uuid).await?;
```

## Recipe System

### Register a Crafting Recipe

```rust
api.register_recipe("mymod:stick", serde_json::json!({
    "type": "crafting_shaped",
    "pattern": ["#", "#"],
    "key": {
        "#": { "item": "minecraft:oak_planks" }
    },
    "result": {
        "item": "minecraft:stick",
        "count": 4
    }
}))?;
```

## Command System

### Register a Command

```rust
api.register_command("heal", "Heals the player to full health")?;
```

### Execute a Command

```rust
let result = api.execute_command("heal", vec!["player123".to_string()]).await?;
println!("Command result: {}", result);
```

## Configuration

### Get Configuration Value

```rust
let value = api.get_config("mymod.some_setting")?;
```

### Set Configuration Value

```rust
api.set_config("mymod.some_setting", serde_json::json!("new_value"))?;
```

## Data Persistence

### Save Mod Data

```rust
api.save_data("mymod.player_scores", serde_json::json!({
    "player123": 1000,
    "player456": 500
})).await?;
```

### Load Mod Data

```rust
let data = api.load_data("mymod.player_scores")?;
```

## World Queries

### Get Loaded Worlds

```rust
let worlds = api.get_worlds();
for world in worlds {
    println!("World: {}", world);
}
```

### Check if World is Loaded

```rust
if api.is_world_loaded("world") {
    println!("World is loaded!");
}
```

## Player Interaction

### Get Online Players

```rust
let players = api.get_online_players();
for player in players {
    println!("Player: {}", player);
}
```

### Send Message to Player

```rust
api.send_message("player123", "Hello from mod!").await?;
```

### Broadcast Message

```rust
api.broadcast("Server announcement!").await?;
```

## Event Handling

### Register Event Handler

```rust
use evoker_core::EventHandler;
use async_trait::async_trait;

struct MyEventHandler;

#[async_trait]
impl EventHandler for MyEventHandler {
    async fn handle_event(&self, event: &Event) -> anyhow::Result<()> {
        // Handle the event
        Ok(())
    }
    
    fn event_types(&self) -> Vec<String> {
        vec!["PlayerJoined".to_string()]
    }
}

api.register_event_handler(Arc::new(MyEventHandler));
```

### Emit Custom Event

```rust
api.emit_custom_event("mymod:custom_event", serde_json::json!({
    "data": "value"
})).await?;
```

## Best Practices

1. **Error Handling**: Always handle errors properly in your mod code
2. **Async Operations**: Many API methods are async - use `.await` appropriately
3. **Resource IDs**: Use namespaced IDs like `mymod:item_name` for all resources
4. **Event Cleanup**: Unregister event handlers when your mod is unloaded
5. **Testing**: Test your mod thoroughly before distribution

## API Version

Check the API version:

```rust
let version = api.version();
println!("Mod API version: {}", version);
```

Current version: `0.1.0`
