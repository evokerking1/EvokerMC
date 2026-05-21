# World Management

The `evoker-world` crate handles everything related to worlds: creation, persistence, chunk management, and terrain generation.

## Core Types

### `World`

The top-level world object. Each world has a name, a seed, and a collection of loaded chunks.

```rust
pub struct WorldInfo {
    pub name: String,
    pub seed: i64,
    pub game_mode: String,   // "survival" | "creative" | "adventure"
    pub difficulty: String,  // "peaceful" | "easy" | "normal" | "hard"
    pub created_at: i64,     // Unix timestamp
    pub last_played: i64,
}
```

### `Chunk`

Chunks are 16 x 256 x 16 blocks. They are the unit of loading and saving.

### `ChunkCoord`

A `(i32, i32)` pair identifying a chunk's position in chunk-space (not block-space).

## Creating a World

```rust
use evoker_world::World;
use std::path::PathBuf;
use std::sync::Arc;

let world = World::new(
    "my_world".to_string(),
    42,                              // seed
    PathBuf::from("worlds"),         // storage directory
    event_bus.clone(),
)?;
```

## Loading an Existing World

```rust
let world = World::load(
    "my_world".to_string(),
    PathBuf::from("worlds"),
    event_bus.clone(),
).await?;
```

If the world does not exist on disk, `load` returns an error. Use `or_else` to fall back to creation:

```rust
let world = World::load(name.clone(), path.clone(), event_bus.clone())
    .await
    .or_else(|_| World::new(name, seed, path, event_bus.clone()))?;
```

## Saving a World

```rust
world.save().await?;
```

Saves the `WorldInfo` metadata and all currently loaded chunks to disk.

## Unloading a World

```rust
world.unload().await?;
```

Automatically calls `save()` first, then clears all in-memory chunks and fires `WorldUnloaded`.

## Chunk Loading

Chunks are loaded on demand:

```rust
let coord = ChunkCoord(0, 0);   // chunk at (0, 0) in chunk-space
let chunk = world.load_chunk(coord).await?;
```

If the chunk exists on disk it is deserialized; otherwise it is generated procedurally from the world seed.

## Chunk Unloading

```rust
world.unload_chunk(coord).await?;
```

Saves the chunk to disk before removing it from the in-memory map.

## Events

| Event | Fired when |
|-------|-----------|
| `WorldLoaded { world_name }` | `World::load` completes |
| `WorldSaved  { world_name }` | `world.save()` completes |
| `WorldUnloaded { world_name }` | `world.unload()` completes |

## Storage Layout

```
worlds/
└── my_world/
    ├── world.json      # WorldInfo metadata
    └── chunks/
        ├── 0_0.json    # Chunk at (0, 0)
        ├── 0_1.json
        └── ...
```

## See Also

- [Architecture Overview](/architecture/overview)
- [Data-Driven Architecture](/architecture/data-driven)
- [Networking](/core/networking)
