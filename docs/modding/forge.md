# Forge Compatibility

EvokerMC includes a compatibility layer (`evoker-compat`) that maps Minecraft Forge events and APIs to their EvokerMC equivalents. This allows many existing Forge mods to be loaded without modification.

## Overview

The `ForgeCompatLayer` in `evoker-compat/src/forge.rs` bridges the gap between Forge's event system and EvokerMC's `EventBus`:

```
Forge Event              EvokerMC Event
────────────────────────────────────────
PlayerLoggedInEvent  →  PlayerJoined
PlayerLoggedOutEvent →  PlayerLeft
ServerStartedEvent   →  ServerStarted
ServerStoppingEvent  →  ServerStopped
WorldLoadEvent       →  WorldLoaded
```

## Initialisation

Forge compatibility is initialized automatically when the application starts:

```rust
let forge = ForgeCompatLayer::new(
    event_bus.clone(),
    mod_loader.clone(),
);
forge.init().await?;
```

After initialization, any Forge mod that subscribes to the mapped events will receive the corresponding EvokerMC events.

## Supported Forge Versions

The compatibility layer targets **Minecraft Forge** for Minecraft 1.20.x and later. Older versions may work but are not officially tested.

## Loading a Forge Mod

Forge mods are packaged as JAR files. Place the JAR in `mods/` just like any other JAR mod:

```bash
cp my-forge-mod-1.0.0.jar mods/
```

Requirements:

- The JAR must contain a valid `mod.json` or `META-INF/MANIFEST.MF` (see [JAR Mods](/modding/jar)).
- A JDK must be installed and `JAVA_HOME` set.

## Current Limitations

- Not all Forge events are mapped. Mods that rely on unmapped events will not receive them.
- Forge's item/block registry hooks are partially implemented. Mods that register content via `RegistryEvent` may need a data pack alongside them.
- Client-side-only Forge mods (rendering, GUI) are not supported; EvokerMC uses its own rendering pipeline.

## Troubleshooting

**Mod loads but events are not firing**

Check the engine log for `[ForgeCompatLayer]` messages. If a required Forge event class is not found, the mapping is silently skipped.

**ClassNotFoundException**

Ensure the Forge mod's JAR is self-contained (all dependencies bundled, or present as separate JARs in `mods/`).

## See Also

- [JAR Mods](/modding/jar)
- [JNI Integration](/jni-integration)
- [Neoforge Compatibility](/modding/neoforge)
