# EvokerMC

Welcome to the EvokerMC documentation! EvokerMC is a highly moddable 3D block game engine inspired by Minecraft and Hytale.

## What is EvokerMC?

EvokerMC is a game engine written in Rust that provides a modular, extensible foundation for creating voxel-based games. The engine is designed from the ground up to be:

- **Highly Moddable**: All game content is provided by mods
- **Performance-Focused**: Written in Rust for maximum performance
- **Server-Authoritative**: Server controls all game logic for fairness and security
- **Cross-Platform**: Runs on Windows, Linux, and macOS

## Key Features

### Core Engine
- Event-driven architecture
- Flexible configuration system
- Efficient world management with chunking
- Save/Load/Quit functionality

### Multiplayer
- Built-in networking with QUIC protocol
- Server-authoritative game logic
- Integrated server for singleplayer (runs locally)
- Dedicated server support for multiplayer

### Modding System
- **JAR Mods**: Load Java-based mods for compatibility
- **WASM Mods**: Secure, sandboxed mods using WebAssembly
- **Lua Scripting**: Easy modding with Lua scripts
- **Python Scripting**: Powerful scripting with Python
- **Forge API Compatibility**: Run existing Minecraft Forge mods
- **Neoforge API Compatibility**: Run existing Minecraft Neoforge mods

### Asset Editor
- In-game asset editor similar to Hytale's editor
- Edit textures, models, and other assets in real-time
- Live preview of changes
- Export assets for use in mods

### UI System
- Main menu with world selection
- Multiplayer server browser
- Settings and configuration
- Mod manager
- In-game HUD

## Architecture

EvokerMC uses a **server-authoritative architecture**:

- **Singleplayer**: Runs an integrated server locally that the client connects to
- **Multiplayer**: Client connects to a dedicated server
- **Server Controls Everything**: The server is the source of truth for all game state
- **Client is "Dumb"**: Client only renders and sends input; server validates everything

This design ensures:
- Fair multiplayer gameplay
- No client-side cheating
- Consistent game state
- Easy testing (singleplayer and multiplayer use same code path)

## Getting Started

See the [Installation](/installation) guide to get EvokerMC running on your system.

For mod developers, check out the [Modding Guide](/modding/development).

## Project Status

EvokerMC is currently in **early development**. The core architecture and systems are in place, but many features are still being implemented.

## License

EvokerMC is dual-licensed under MIT OR Apache-2.0. See LICENSE file for details.
