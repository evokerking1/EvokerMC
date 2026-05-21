# Quick Start

Get EvokerMC up and running in a few minutes.

## 1. Build the Engine

```bash
git clone https://github.com/evokerking1/EvokerMC
cd EvokerMC
cargo build --release
```

## 2. Launch the Engine

```bash
cargo run --release
```

On startup the engine:

1. Initializes the logging system.
2. Loads the configuration from `config.json` (created on first run with defaults).
3. Scans the `mods/` directory and loads any discovered mods.
4. Initializes Forge/Neoforge compatibility layers.
5. Opens the main menu, ready for singleplayer or multiplayer.

## 3. Start a Singleplayer World

From the main menu select **Singleplayer**, then either:

- Choose an existing world from the list and click **Play Selected**, or
- Enter a name in the *Create New World* field and click **Create**.

The engine starts an **integrated server** locally and connects the client to it automatically — no separate server setup is needed for singleplayer.

## 4. Connect to a Multiplayer Server

From the main menu select **Multiplayer**:

- Click a **Saved Server** to connect immediately, or
- Type an address in the **Direct Connect** field (e.g. `play.example.com:25565`) and click **Connect**.

## 5. Add Your First Mod

Place a `.jar` or `.wasm` mod file into the `mods/` directory:

```bash
cp my-mod-1.0.0.jar mods/
```

Restart EvokerMC. The mod loader discovers and loads it automatically. You can enable or disable individual mods from **Main Menu → Mods**.

## 6. Add Custom Content (Data Packs)

Create a data pack to define new blocks, items, recipes, or entities:

```
data/
└── mypack/
    └── data/
        └── mypack/
            ├── blocks/
            │   └── ruby_ore.json
            └── items/
                └── ruby.json
```

Example block definition (`ruby_ore.json`):

```json
{
  "id": "mypack:ruby_ore",
  "name": "Ruby Ore",
  "properties": {
    "hardness": 3.0,
    "resistance": 3.0,
    "solid": true,
    "opaque": true
  },
  "textures": {
    "all": "blocks/ruby_ore"
  }
}
```

Content loads automatically when the engine starts. See the [Data-Driven Architecture](/architecture/data-driven) guide for the full schema reference.

## 7. Write a Script

Drop a Lua script in `scripts/lua/`:

```lua
-- scripts/lua/hello.lua
game:log("Hello from Lua!")

local players = game:getPlayers()
for _, player in ipairs(players) do
    game:sendMessage(player, "Welcome to EvokerMC!")
end
```

Scripts execute automatically when the mod/scripting system loads them. See the [Scripting API](/modding/scripting) guide for all available functions.

## Next Steps

| Topic | Guide |
|-------|-------|
| Architecture overview | [Architecture](/architecture/overview) |
| Creating mods | [Mod Development](/modding/development) |
| Data pack reference | [Data-Driven Architecture](/architecture/data-driven) |
| Scripting (Lua / Python) | [Scripting API](/modding/scripting) |
| Forge mod compatibility | [Forge Compatibility](/modding/forge) |
| Asset editor | [Asset Editor](/assets/overview) |
