# Asset Types

EvokerMC's asset system (`evoker-assets`) recognises the following asset types, represented by the `AssetType` enum.

## BlockTexture

PNG images applied to block faces.

- **Directory**: `assets/blocks/`
- **Format**: PNG (recommended 16×16, power-of-two sizes)
- **Usage**: Referenced in block JSON definitions via the `textures` field.

```json
{ "textures": { "all": "blocks/stone" } }
```

## ItemTexture

PNG images displayed in the inventory and HUD hotbar.

- **Directory**: `assets/items/`
- **Format**: PNG (recommended 16×16)
- **Usage**: Referenced in item JSON definitions via the `texture` field.

```json
{ "texture": "items/diamond_sword" }
```

## EntityModel

3D model data for entities (mobs, players, items in the world).

- **Directory**: `assets/entities/`
- **Format**: JSON geometry descriptor (format TBD; Bedrock-compatible format planned)
- **Usage**: Referenced in entity JSON definitions via the `model` field.

```json
{ "model": "entities/zombie" }
```

## Sound

Short sound effects played in response to game events (block break, item use, etc.).

- **Directory**: `assets/sounds/`
- **Format**: OGG Vorbis or WAV
- **Usage**: Played via the scripting API or sound events.

```lua
game:playSound(player_id, "sounds/block_break")
```

## Music

Background music tracks.

- **Directory**: `assets/music/`
- **Format**: OGG Vorbis or MP3
- **Usage**: Played by the audio system based on biome or game state.

## Shader

GLSL shader programs for the 3D renderer (wgpu).

- **Directory**: `assets/shaders/`
- **Format**: GLSL (`.vert`, `.frag`) or WGSL (`.wgsl`)
- **Status**: The renderer is not yet implemented; shaders are reserved for future use.

## Script

Lua or Python script files.

- **Directory**: `assets/scripts/` (or `scripts/lua/`, `scripts/python/` for standalone scripts)
- **Format**: `.lua`, `.py`
- **Usage**: Loaded by the scripting engine at startup.

## Config

JSON configuration files.

- **Directory**: `assets/config/`
- **Format**: JSON
- **Usage**: Read by mods and engine subsystems for configuration.

## Summary Table

| Type | `AssetType` variant | Directory | Format |
|------|---------------------|-----------|--------|
| Block texture | `BlockTexture` | `assets/blocks/` | PNG |
| Item texture | `ItemTexture` | `assets/items/` | PNG |
| Entity model | `EntityModel` | `assets/entities/` | JSON |
| Sound effect | `Sound` | `assets/sounds/` | OGG/WAV |
| Music | `Music` | `assets/music/` | OGG/MP3 |
| Shader | `Shader` | `assets/shaders/` | GLSL/WGSL |
| Script | `Script` | `assets/scripts/` | `.lua`/`.py` |
| Config | `Config` | `assets/config/` | JSON |

## See Also

- [Asset Editor Overview](/assets/overview)
- [Creating Assets](/assets/creating)
