# EvokerMC

A highly moddable, data-driven 3D block game engine inspired by Minecraft and Hytale.

## Features

🎮 **Core Engine**
- Server-authoritative architecture
- Integrated server for singleplayer
- Event-driven system
- World management (save/load/quit)
- Main menu and UI system

🌍 **Data-Driven Design** (Like Minecraft)
- All content defined in JSON data files
- Blocks, items, entities, recipes in data packs
- Hot-reloadable configurations
- Registry system for all game objects
- No hardcoded game content

🔌 **Modding System**
- JAR mods (Java compatibility)
- WASM mods (secure sandboxing)
- Lua scripting
- Python scripting
- Minecraft Forge API compatibility
- Minecraft Neoforge API compatibility

🎨 **Asset Editor**
- In-game asset editor like Hytale
- Live editing of textures, models, sounds
- Real-time preview
- Export assets for mods

🌐 **Multiplayer**
- Built-in networking with QUIC
- Dedicated server support
- Server-controlled gameplay

## Architecture

EvokerMC uses a **server-authoritative architecture**:
- Singleplayer runs an integrated local server
- Client connects to server (local or remote)
- Server controls all game logic
- Fair multiplayer, no client-side cheating

## Data-Driven Design

Everything in EvokerMC is defined through data files:

```
data/
├── blocks/           # Block definitions
├── items/            # Item definitions
├── entities/         # Entity definitions
├── recipes/          # Crafting recipes
├── loot_tables/      # Loot generation
├── worldgen/         # World generation
│   ├── biomes/       # Biome definitions
│   └── structures/   # Structure templates
└── tags/             # Tag definitions
```

Example block definition:
```json
{
  "id": "stone",
  "name": "Stone",
  "properties": {
    "hardness": 1.5,
    "resistance": 6.0,
    "solid": true,
    "opaque": true
  },
  "textures": {
    "all": "blocks/stone"
  }
}
```

## Building

```bash
# Build the project
cargo build --release

# Run tests
cargo test

# Generate documentation
cargo doc --open
```

## Documentation

Visit the [documentation site](https://docs.evokermc.dev) (built with Zensical).

## License

Dual-licensed under MIT OR Apache-2.0.