# Modding Development Guide

Learn how to create mods for EvokerMC. Mods can add new blocks, items, entities, game mechanics, commands, and more.

## Mod Types

| Type | Language | File | Use case |
|------|----------|------|---------|
| **JAR mod** | Java | `.jar` | Large feature mods, Forge/Neoforge ports |
| **WASM mod** | Any → WebAssembly | `.wasm` + `.wasm.json` | Cross-platform, sandboxed mods |
| **Lua script** | Lua 5.4 | `.lua` | Simple scripts, event handlers |
| **Python script** | Python 3 | `.py` | Advanced scripts, data processing |

All mod types are placed in the `mods/` directory (scripts go in `scripts/lua/` or `scripts/python/`).

## Choosing a Mod Type

- **Start with Lua or Python scripts** if you want to react to events, manage world state, or add simple commands — no compilation needed.
- **Use WASM** for performance-critical or security-sensitive code that should run in a sandboxed environment.
- **Use JAR** if you are porting an existing Minecraft Forge mod or need the full JVM ecosystem.

## Your First Mod: Lua Script

1. Create `scripts/lua/hello_world.lua`:

```lua
game:log("Hello World mod loaded!")

-- Greet every player that is online when the script runs
local players = game:getPlayers()
for _, player in ipairs(players) do
    game:sendMessage(player, "Hello from my first mod!")
end
```

2. Start EvokerMC — the script runs automatically.

## Your First Mod: JAR Mod

1. Create a Java class:

```java
package com.example;

public class HelloMod {
    public HelloMod() {}

    public void init() {
        System.out.println("HelloMod initialized!");
    }
}
```

2. Package it as a JAR with a `mod.json` in the root:

```json
{
  "id":          "hello-mod",
  "name":        "Hello Mod",
  "version":     "1.0.0",
  "description": "A simple example mod",
  "authors":     ["YourName"],
  "dependencies": [],
  "main":        "com.example.HelloMod"
}
```

3. Place the JAR in `mods/` and start EvokerMC.

See [JAR Mods](/modding/jar) for the complete guide including JNI signatures and calling Java from Rust.

## Using the Mod API (Rust)

JAR and WASM mods written in Rust can access the `ModApi`:

```rust
use evoker_modding::ModApi;
use std::sync::Arc;

pub fn init(api: Arc<ModApi>) {
    // Register a new block
    api.register_block("mymod:ruby_ore", serde_json::json!({
        "hardness": 3.0,
        "solid": true,
        "opaque": true
    })).unwrap();

    // Spawn an entity at a fixed position
    tokio::spawn(async move {
        api.spawn_entity("world", "minecraft:zombie", 0.0, 64.0, 0.0)
            .await
            .unwrap();
    });
}
```

See the [Mod API Reference](/api/mod) for the full list of methods.

## Mod Dependencies

Declare dependencies in `mod.json`. The mod loader performs a **topological sort** before loading, so all dependencies are guaranteed to be loaded before your mod.

```json
{
  "id": "my-mod",
  "dependencies": ["some-library-mod", "another-mod"]
}
```

If a dependency is missing, the mod is skipped with an error message in the log.

## Dependency Cycles

Circular dependencies are detected at load time and reported as an error. No mods involved in the cycle are loaded.

## Managing Mods at Runtime

From the main menu go to **Mods** to see all installed mods and toggle them on or off. Disabled mods are not loaded on the next startup.

## See Also

- [JAR Mods](/modding/jar)
- [WASM Mods](/modding/wasm)
- [Scripting](/modding/scripting)
- [Forge Compatibility](/modding/forge)
- [Neoforge Compatibility](/modding/neoforge)
- [Mod API Reference](/api/mod)
