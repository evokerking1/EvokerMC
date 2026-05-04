# Mod API Reference

The `ModApi` struct (from `evoker-modding`) is the primary interface available to JAR and WASM mods for interacting with the engine.

## Accessing the API

```rust
use evoker_modding::ModApi;
use std::sync::Arc;

pub fn init(api: Arc<ModApi>) {
    // Use the API here
}
```

## Block Management

### `register_block`

```rust
api.register_block("mymod:ruby_ore", serde_json::json!({
    "hardness": 3.0,
    "resistance": 3.0,
    "solid": true,
    "opaque": true
}))?;
```

### `set_block`

```rust
api.set_block("world", 100, 64, 200, "mymod:ruby_ore").await?;
```

### `get_block_at`

```rust
let block_id = api.get_block_at("world", 100, 64, 200)?;
println!("Block: {}", block_id);
```

## Item Management

### `register_item`

```rust
api.register_item("mymod:ruby", serde_json::json!({
    "max_stack_size": 64,
    "rarity": "uncommon"
}))?;
```

### `give_item`

```rust
api.give_item("player123", "mymod:ruby", 5).await?;
```

### `remove_item`

```rust
api.remove_item("player123", "mymod:ruby", 1).await?;
```

## Entity Management

### `register_entity`

```rust
api.register_entity("mymod:crystal_golem", serde_json::json!({
    "health": 40.0,
    "speed": 0.15,
    "hostile": false
}))?;
```

### `spawn_entity`

```rust
let uuid = api.spawn_entity("world", "mymod:crystal_golem", 0.0, 64.0, 0.0).await?;
println!("Spawned: {}", uuid);
```

### `remove_entity`

```rust
api.remove_entity(&uuid).await?;
```

## Recipe System

### `register_recipe`

```rust
api.register_recipe("mymod:ruby_block", serde_json::json!({
    "type": "crafting_shaped",
    "pattern": ["###", "###", "###"],
    "key": { "#": { "item": "mymod:ruby" } },
    "result": { "item": "mymod:ruby_block", "count": 1 }
}))?;
```

## Command System

### `register_command`

```rust
api.register_command("heal", "Restores the player to full health")?;
```

### `execute_command`

```rust
let result = api.execute_command("heal", vec!["player123".to_string()]).await?;
println!("Result: {}", result);
```

## Player Interaction

### `get_online_players`

```rust
for player in api.get_online_players() {
    println!("Online: {}", player);
}
```

### `send_message`

```rust
api.send_message("player123", "Hello!").await?;
```

### `broadcast`

```rust
api.broadcast("Server is restarting in 5 minutes.").await?;
```

## World Queries

### `get_worlds`

```rust
for world in api.get_worlds() {
    println!("World: {}", world);
}
```

### `is_world_loaded`

```rust
if api.is_world_loaded("world") {
    println!("World is loaded");
}
```

## Configuration

### `get_config`

```rust
let value = api.get_config("mymod.max_rubies")?;
```

### `set_config`

```rust
api.set_config("mymod.max_rubies", serde_json::json!(100))?;
```

## Data Persistence

### `save_data`

```rust
api.save_data("mymod.scores", serde_json::json!({
    "player123": 500
})).await?;
```

### `load_data`

```rust
let data = api.load_data("mymod.scores")?;
```

## Event Handling

### `register_event_handler`

```rust
use evoker_core::{Event, EventHandler};
use async_trait::async_trait;

struct MyHandler;

#[async_trait]
impl EventHandler for MyHandler {
    async fn handle_event(&self, event: &Event) -> anyhow::Result<()> {
        if let Event::PlayerJoined { player_name, .. } = event {
            println!("Welcome, {}!", player_name);
        }
        Ok(())
    }

    fn event_types(&self) -> Vec<String> {
        vec!["PlayerJoined".to_string()]
    }
}

api.register_event_handler(Arc::new(MyHandler));
```

### `emit_custom_event`

```rust
api.emit_custom_event("mymod:reward_given", serde_json::json!({
    "player": "player123",
    "amount": 100
})).await?;
```

## API Version

```rust
let version = api.version();
println!("Mod API version: {}", version);  // "0.1.0"
```

## Best Practices

1. Always handle errors from API calls (`?` or `.unwrap_or_else`).
2. Use namespaced IDs: `mymod:item_name` to avoid conflicts.
3. Prefer async methods for world and player interactions.
4. Unregister event handlers when your mod is unloaded.

## See Also

- [Mod Development Guide](/modding/development)
- [Event System](/architecture/events)
- [Scripting API Reference](/api/scripting)
