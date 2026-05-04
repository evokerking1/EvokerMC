# Creating Assets

This guide explains how to create and package assets for EvokerMC, whether for the base game or for a mod.

## Asset Directory Layout

```
assets/
├── blocks/          # Block textures (PNG)
├── items/           # Item textures (PNG)
├── entities/        # Entity models and textures
├── sounds/          # Sound effects (OGG/WAV)
├── music/           # Background music (OGG/MP3)
├── shaders/         # GLSL shader programs
├── scripts/         # Script files (Lua / Python)
└── config/          # JSON configuration assets
```

## Block Textures

Block textures are PNG images placed in `assets/blocks/`. The file name (without extension) becomes the texture ID used in block definitions.

Example: `assets/blocks/ruby_ore.png` → texture ID `blocks/ruby_ore`

In a block definition:

```json
{
  "id": "mymod:ruby_ore",
  "textures": {
    "all": "blocks/ruby_ore"
  }
}
```

Different faces can have different textures:

```json
{
  "textures": {
    "top":    "blocks/grass_top",
    "bottom": "blocks/dirt",
    "sides":  "blocks/grass_side"
  }
}
```

## Item Textures

Item textures follow the same convention under `assets/items/`.

```json
{
  "id": "mymod:ruby",
  "texture": "items/ruby"
}
```

## Entity Models

Entity models are JSON files describing geometry and bone hierarchy. Textures referenced by the model live in `assets/entities/`.

```json
{
  "id": "mymod:custom_mob",
  "model": "entities/custom_mob",
  "texture": "entities/custom_mob"
}
```

## Sounds

Place `.ogg` or `.wav` files in `assets/sounds/`. Reference them by ID in game events or scripts:

```lua
game:playSound("player_id", "sounds/my_sound")
```

## Packaging Assets in a Mod

Assets intended to ship with a JAR or WASM mod should be placed inside the archive and extracted at load time. A convention used by many mods is to include an `assets/` folder at the root of the JAR:

```
my-mod.jar
├── mod.json
└── assets/
    ├── blocks/
    │   └── ruby_ore.png
    └── items/
        └── ruby.png
```

The engine's asset manager will merge these assets with the global `assets/` folder when the mod is loaded.

## Using the In-Game Asset Editor

The [Asset Editor](/assets/overview) lets you create and modify assets live:

1. Open the editor overlay (default: **F10**).
2. Select the asset type from the sidebar.
3. Click **New Asset**, give it a name, and start editing.
4. Click **Export** to save the file to disk.

## Asset IDs

Asset IDs follow the convention `<type>/<name>`, matching the directory structure:

- `blocks/stone` → `assets/blocks/stone.png`
- `items/diamond_sword` → `assets/items/diamond_sword.png`
- `sounds/block_break` → `assets/sounds/block_break.ogg`

Mod-provided assets should be namespaced to avoid collisions: `mymod:blocks/ruby_ore`.

## See Also

- [Asset Editor Overview](/assets/overview)
- [Asset Types](/assets/types)
- [Data-Driven Architecture](/architecture/data-driven)
