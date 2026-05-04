# Event System

The `EventBus` in `evoker-core` is the primary communication mechanism between all engine subsystems and mods. It is entirely asynchronous and thread-safe.

## Core Concepts

### Events

Events are variants of the `Event` enum defined in `evoker_core::events`:

```rust
pub enum Event {
    // Game lifecycle
    GameStarted,
    GameStopped,
    GamePaused,
    GameResumed,

    // World
    WorldLoaded { world_name: String },
    WorldSaved  { world_name: String },
    WorldUnloaded { world_name: String },

    // Player
    PlayerJoined { player_id: String, player_name: String },
    PlayerLeft   { player_id: String },

    // UI
    MenuOpened { menu_type: String },
    MenuClosed { menu_type: String },

    // Mods
    ModLoaded   { mod_id: String, mod_name: String },
    ModUnloaded { mod_id: String },

    // Assets
    AssetLoaded   { asset_type: String, asset_id: String },
    AssetModified { asset_type: String, asset_id: String },

    // Network
    ServerStarted     { port: u16 },
    ServerStopped,
    ClientConnected   { server_address: String },
    ClientDisconnected,

    // Mods can define arbitrary events
    Custom { event_type: String, data: serde_json::Value },
}
```

### EventBus

`EventBus` stores a map of `event_type → Vec<Arc<dyn EventHandler>>` and an optional queue for deferred dispatch.

```rust
pub struct EventBus {
    handlers: Arc<DashMap<String, Vec<Arc<dyn EventHandler>>>>,
    event_queue: Arc<RwLock<Vec<Event>>>,
}
```

### EventHandler

Any type that implements `EventHandler` can subscribe to events:

```rust
#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle_event(&self, event: &Event) -> anyhow::Result<()>;
    fn event_types(&self) -> Vec<String>;
}
```

Return `"*"` from `event_types()` to receive every event.

## Registering a Handler

```rust
use evoker_core::{Event, EventBus, EventHandler};
use async_trait::async_trait;
use std::sync::Arc;

struct PlayerLogger;

#[async_trait]
impl EventHandler for PlayerLogger {
    async fn handle_event(&self, event: &Event) -> anyhow::Result<()> {
        if let Event::PlayerJoined { player_name, .. } = event {
            log::info!("{} joined the game", player_name);
        }
        Ok(())
    }

    fn event_types(&self) -> Vec<String> {
        vec!["PlayerJoined".to_string()]
    }
}

// Register
let bus = Arc::new(EventBus::new());
bus.register_handler(Arc::new(PlayerLogger));
```

## Emitting Events

### Immediate dispatch

`emit` calls all registered handlers before returning:

```rust
bus.emit(Event::GameStarted).await?;
```

### Deferred dispatch

`queue_event` places the event in a buffer. Call `process_queue` at a convenient point (e.g., each game tick) to dispatch all queued events:

```rust
// Queue (non-async, safe to call from sync context)
bus.queue_event(Event::PlayerLeft { player_id: id });

// Flush the queue — called once per game tick by the Game loop
bus.process_queue().await?;
```

## Custom Events (Mods)

Mods can emit and subscribe to application-defined events using the `Custom` variant:

### Emitting a custom event

```rust
// Using the Mod API
api.emit_custom_event("mymod:reward_given", serde_json::json!({
    "player": "player123",
    "amount": 100
})).await?;
```

### Handling a custom event

```rust
fn event_types(&self) -> Vec<String> {
    vec!["Custom".to_string()]
}

async fn handle_event(&self, event: &Event) -> anyhow::Result<()> {
    if let Event::Custom { event_type, data } = event {
        if event_type == "mymod:reward_given" {
            log::info!("Reward given: {:?}", data);
        }
    }
    Ok(())
}
```

## Thread Safety

`EventBus` uses `DashMap` (a concurrent hash map) and `parking_lot::RwLock` internally, so it can be shared freely across threads via `Arc<EventBus>`.

## See Also

- [Architecture Overview](/architecture/overview)
- [Mod API — Event Handling](/api/mod)
