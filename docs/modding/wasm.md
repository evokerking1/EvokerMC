# WASM Mods

EvokerMC supports mods compiled to [WebAssembly](https://webassembly.org/) (WASM). WASM mods run in a secure sandbox provided by [Wasmtime](https://wasmtime.dev/), so they cannot access the host filesystem, network, or memory outside the sandbox without explicit permission.

## Why WASM?

- **Security**: Sandboxed execution — a misbehaving mod cannot crash or compromise the engine.
- **Cross-platform**: WASM binaries run on any OS without recompilation.
- **Multiple source languages**: Rust, C, C++, AssemblyScript, Go, and many others can compile to WASM.

## Creating a WASM Mod

### 1. Write the Module (Rust example)

```rust
// src/lib.rs
#[no_mangle]
pub extern "C" fn init() {
    // Mod initialisation logic
}

#[no_mangle]
pub extern "C" fn on_tick() {
    // Called each game tick
}
```

Compile to WASM:

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
# Output: target/wasm32-unknown-unknown/release/my_mod.wasm
```

### 2. Create the Metadata File

WASM mods require an accompanying JSON metadata file with the same base name:

`my_mod.wasm.json`:

```json
{
  "id":          "my-wasm-mod",
  "name":        "My WASM Mod",
  "version":     "1.0.0",
  "description": "A sandboxed WASM mod",
  "authors":     ["YourName"],
  "dependencies": []
}
```

### 3. Install the Mod

Place both files in `mods/`:

```
mods/
├── my_mod.wasm
└── my_mod.wasm.json
```

Start EvokerMC. The WASM loader discovers the pair and loads the module.

## How It Works

The `WasmModLoader` in `evoker-modding` uses Wasmtime's `Engine` and `Instance` APIs:

```rust
use evoker_modding::WasmModLoader;
use std::path::Path;

let loader = WasmModLoader::new()?;
loader.load(Path::new("mods/my_mod.wasm")).await?;
```

The module is instantiated without any imports by default. Host functions (engine API calls) will be provided through WASI or a custom linker in a future release.

## Current Limitations

- Host API bindings (calling engine functions from WASM) are not yet exposed. The WASM module is loaded and instantiated, but bidirectional communication is planned for a future version.
- WASI support is not enabled by default in this version.

## Security Model

- The WASM sandbox prevents arbitrary memory access, syscalls, and file I/O.
- Each module runs in its own `Store`, so modules cannot share memory.
- Future versions will expose a capability-based API so mods can request only the permissions they need.

## See Also

- [Mod Development Guide](/modding/development)
- [JAR Mods](/modding/jar)
- [Scripting](/modding/scripting)
