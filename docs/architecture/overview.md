# Architecture Overview

EvokerMC is a modular, data-driven game engine written in Rust. This page gives a high-level map of how all the pieces fit together.

## Crate Structure

The engine is split into focused workspace crates, each with a single responsibility:

| Crate | Purpose |
|-------|---------|
| `evoker-core` | Event bus, game loop, configuration |
| `evoker-world` | World save/load, chunks, terrain generation |
| `evoker-network` | QUIC-based server and client |
| `evoker-modding` | JAR and WASM mod loading, Mod API |
| `evoker-scripting` | Lua 5.4 and Python 3 scripting engines |
| `evoker-ui` | Main menu, HUD, settings — built on egui |
| `evoker-assets` | Asset manager and in-game asset editor |
| `evoker-compat` | Forge and Neoforge compatibility layers |
| `evoker-registry` | Data-driven registry and data pack loader |

## High-Level Diagram

```
┌──────────────────────────────────────────────────────┐
│                     Application                      │
│  ┌────────────┐  ┌──────────┐  ┌──────────────────┐ │
│  │  Main Menu │  │   HUD    │  │  Asset Editor    │ │
│  │ (evoker-ui)│  │(evoker-ui)│  │(evoker-assets)   │ │
│  └────────────┘  └──────────┘  └──────────────────┘ │
│                                                      │
│  ┌─────────────────────────────────────────────────┐ │
│  │             evoker-core (EventBus, Game)        │ │
│  └─────────────────────────────────────────────────┘ │
│                                                      │
│  ┌───────────┐  ┌─────────────┐  ┌───────────────┐  │
│  │evoker-    │  │evoker-      │  │evoker-        │  │
│  │world      │  │network      │  │modding        │  │
│  └───────────┘  └─────────────┘  └───────┬───────┘  │
│                                          │           │
│                  ┌───────────────────────┤           │
│                  │                       │           │
│         ┌────────┴──────┐  ┌─────────────┴────────┐  │
│         │evoker-        │  │evoker-scripting      │  │
│         │compat         │  │(Lua / Python)         │  │
│         └───────────────┘  └──────────────────────┘  │
└──────────────────────────────────────────────────────┘
```

## Communication: The Event Bus

All crates communicate through a central `EventBus` (from `evoker-core`). Handlers register for specific event types and are called asynchronously when an event is emitted.

Built-in event categories:

- **Game**: `GameStarted`, `GameStopped`, `GamePaused`, `GameResumed`
- **World**: `WorldLoaded`, `WorldSaved`, `WorldUnloaded`
- **Player**: `PlayerJoined`, `PlayerLeft`
- **Network**: `ServerStarted`, `ServerStopped`, `ClientConnected`, `ClientDisconnected`
- **Mod**: `ModLoaded`, `ModUnloaded`
- **Asset**: `AssetLoaded`, `AssetModified`
- **UI**: `MenuOpened`, `MenuClosed`
- **Custom**: `Custom { event_type, data }` — for mod-defined events

See [Event System](/architecture/events) for usage details.

## Server-Authoritative Design

The most important architectural decision is that **the server owns all game state**:

- In **singleplayer** an integrated server runs locally; the client connects to it via loopback just as it would a remote server.
- In **multiplayer** the client connects to a dedicated remote server.
- The **client only renders and sends input**; all validation and simulation happen server-side.

This eliminates a whole class of cheats and keeps the codebase simple — singleplayer and multiplayer share the same code path.

See [Server-Authoritative Design](/architecture/server-authoritative) and [Integrated Server](/architecture/integrated-server) for details.

## Data-Driven Content

No game content is hardcoded in the engine. Everything — blocks, items, entities, recipes, world generation — is declared in JSON data packs loaded at startup by `evoker-registry`.

See [Data-Driven Architecture](/architecture/data-driven) for the full schema reference.

## Modding

Mods can extend the engine in four ways:

1. **JAR mods** — Java classes loaded via JNI (`evoker-modding`)
2. **WASM mods** — WebAssembly modules sandboxed by Wasmtime (`evoker-modding`)
3. **Lua scripts** — interpreted by mlua (`evoker-scripting`)
4. **Python scripts** — interpreted by PyO3 (`evoker-scripting`)

Forge and Neoforge event mappings live in `evoker-compat`, enabling many existing Minecraft mods to run with minimal changes.
