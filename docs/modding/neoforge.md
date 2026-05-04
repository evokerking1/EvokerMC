# Neoforge Compatibility

EvokerMC includes a compatibility layer for **NeoForge** (the community-maintained Forge fork targeting Minecraft 1.20.1+). The `NeoforgeCompatLayer` in `evoker-compat/src/neoforge.rs` maps NeoForge events to EvokerMC's `EventBus`.

## Overview

NeoForge is API-compatible with Forge for most mod use cases, but uses different package names and a revised event bus. EvokerMC handles both independently:

```
NeoForge Event                    EvokerMC Event
──────────────────────────────────────────────────
PlayerEvent.PlayerLoggedInEvent  → PlayerJoined
PlayerEvent.PlayerLoggedOutEvent → PlayerLeft
ServerStartedEvent               → ServerStarted
ServerStoppingEvent              → ServerStopped
LevelEvent.Load                  → WorldLoaded
```

## Initialisation

NeoForge compatibility initialises alongside Forge at application startup:

```rust
let neoforge = NeoforgeCompatLayer::new(
    event_bus.clone(),
    mod_loader.clone(),
);
neoforge.init().await?;
```

## Loading a NeoForge Mod

NeoForge mods are JAR files. Place them in `mods/`:

```bash
cp my-neoforge-mod-1.0.0.jar mods/
```

The loader detects and initialises them just like regular JAR mods. See [JAR Mods](/modding/jar) for setup requirements.

## Differences from Forge Compatibility

| Feature | Forge | NeoForge |
|---------|-------|---------|
| Package prefix | `net.minecraftforge` | `net.neoforged` |
| Event bus | `MinecraftForge.EVENT_BUS` | `NeoForge.EVENT_BUS` |
| Mod metadata | `mods.toml` / `mod.json` | `neoforge.mods.toml` / `mod.json` |

EvokerMC's loader reads `mod.json` for both. If your NeoForge mod only ships `neoforge.mods.toml`, provide a companion `mod.json` with the same metadata.

## Current Limitations

- Partial event mapping — only the most commonly used events are bridged.
- Client-side NeoForge mods (renderers, GUIs) are not supported.
- NeoForge's new "registrar" system (`DeferredRegister`) is not yet fully emulated.

## See Also

- [JAR Mods](/modding/jar)
- [Forge Compatibility](/modding/forge)
- [JNI Integration](/jni-integration)
