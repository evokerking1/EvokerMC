# Installation

This guide covers how to install EvokerMC on your system.

## Prerequisites

- **Rust 1.70+** — install via [rustup.rs](https://rustup.rs/)
- **Git**
- **Java Development Kit (JDK) 8+** *(optional, required only for Java/JAR mods)*

## Installing Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify the installation:

```bash
rustc --version
cargo --version
```

## Cloning the Repository

```bash
git clone https://github.com/evokerking1/EvokerMC
cd EvokerMC
```

## Building

### Debug Build

```bash
cargo build
```

### Release Build (recommended for playing)

```bash
cargo build --release
```

The compiled binary is placed in `target/release/evokermc`.

## Running

```bash
# From the project root
cargo run --release

# Or run the binary directly
./target/release/evokermc
```

## Setting Up Java (Optional)

Java is required only if you intend to load JAR-based mods or Forge/Neoforge compatibility mods.

### Linux

```bash
sudo apt install openjdk-17-jdk
export JAVA_HOME=/usr/lib/jvm/java-17-openjdk-amd64
export LD_LIBRARY_PATH=$JAVA_HOME/lib/server:$LD_LIBRARY_PATH
```

### Windows

```cmd
set JAVA_HOME=C:\Program Files\Java\jdk-17
set PATH=%JAVA_HOME%\bin\server;%PATH%
```

### macOS

```bash
export JAVA_HOME=/Library/Java/JavaVirtualMachines/jdk-17.jdk/Contents/Home
export DYLD_LIBRARY_PATH=$JAVA_HOME/lib/server:$DYLD_LIBRARY_PATH
```

## Directory Structure After Installation

```
EvokerMC/
├── target/release/evokermc   # Compiled binary
├── data/                     # Data packs (game content)
├── mods/                     # Mod files (.jar, .wasm)
├── worlds/                   # Saved worlds
├── assets/                   # Game assets
└── scripts/                  # Lua and Python scripts
    ├── lua/
    └── python/
```

## Verifying the Installation

Run the test suite to confirm everything is working:

```bash
cargo test
```

All tests should pass. If you encounter build errors, ensure your Rust toolchain is up to date:

```bash
rustup update stable
```

## Next Steps

- Follow the [Quick Start](/quick-start) guide to launch your first world.
- Learn about [Modding](/modding/development) to add mods.
- Explore [Data Packs](/architecture/data-driven) to customise game content.
