# Data-Driven Architecture

EvokerMC follows Minecraft's data-driven design philosophy where all game content is defined in JSON files rather than being hardcoded.

## Overview

The engine provides the **runtime and systems**, while **data packs provide the content**:

```
Engine (Rust Code)          Data Packs (JSON)
├── Registry System    ←──  ├── Blocks
├── Game Logic         ←──  ├── Items  
├── World Generation   ←──  ├── Entities
├── Rendering          ←──  ├── Recipes
└── Networking         ←──  └── Tags
```

## Data Pack Structure

Data packs follow Minecraft's structure:

```
data/
└── mypack/
    ├── pack.mcmeta                 # Pack metadata
    └── data/
        └── mypack/
            ├── blocks/             # Block definitions
            │   ├── stone.json
            │   └── grass_block.json
            ├── items/              # Item definitions
            │   ├── diamond_sword.json
            │   └── stick.json
            ├── entities/           # Entity definitions
            │   └── zombie.json
            ├── recipes/            # Crafting recipes
            │   ├── stick.json
            │   └── diamond_sword.json
            ├── loot_tables/        # Loot generation
            │   └── blocks/
            │       └── stone.json
            ├── tags/               # Object grouping
            │   ├── blocks/
            │   └── items/
            └── worldgen/           # World generation
                ├── biomes/
                └── structures/
```

## Defining Blocks

Example: `data/mypack/data/mypack/blocks/stone.json`

```json
{
  "id": "mypack:stone",
  "name": "Stone",
  "properties": {
    "hardness": 1.5,
    "resistance": 6.0,
    "solid": true,
    "opaque": true,
    "light_level": 0
  },
  "textures": {
    "all": "blocks/stone"
  },
  "loot_table": "mypack:blocks/stone"
}
```

## Defining Items

Example: `data/mypack/data/mypack/items/diamond_sword.json`

```json
{
  "id": "mypack:diamond_sword",
  "name": "Diamond Sword",
  "description": ["A powerful weapon"],
  "properties": {
    "max_stack_size": 1,
    "max_durability": 1561,
    "rarity": "rare"
  },
  "texture": "items/diamond_sword"
}
```

## Defining Recipes

Example: `data/mypack/data/mypack/recipes/stick.json`

```json
{
  "id": "mypack:stick",
  "type": "minecraft:crafting_shaped",
  "pattern": [
    "#",
    "#"
  ],
  "key": {
    "#": { "item": "mypack:oak_planks" }
  },
  "result": {
    "item": "mypack:stick",
    "count": 4
  }
}
```

## Defining Entities

Example: `data/mypack/data/mypack/entities/zombie.json`

```json
{
  "id": "mypack:zombie",
  "name": "Zombie",
  "entity_type": "mob",
  "attributes": {
    "max_health": 20.0,
    "movement_speed": 0.23,
    "attack_damage": 3.0
  },
  "behavior": {
    "behavior_type": "hostile",
    "goals": ["attack_player", "wander"]
  },
  "model": "entities/zombie",
  "loot_table": "mypack:entities/zombie"
}
```

## Registry System

All game objects are registered in typed registries:

```rust
// Engine code
let blocks = Registry::<BlockDefinition>::new("blocks");
let items = Registry::<ItemDefinition>::new("items");
let entities = Registry::<EntityDefinition>::new("entities");
let recipes = Registry::<RecipeDefinition>::new("recipes");

// Load from data packs
let loader = DataPackLoader::new("data");
loader.load_all_packs(&blocks, &items, &entities, &recipes).await?;

// Access registered objects
let stone = blocks.get(&RegistryKey::parse("mypack:stone")?);
```

## Hot-Reloading

Data packs can be reloaded without restarting:

```rust
// Reload all data packs
loader.load_all_packs(&blocks, &items, &entities, &recipes).await?;
```

## Benefits

1. **No Hardcoded Content**: Engine is content-agnostic
2. **Easy Modding**: Just add JSON files
3. **Hot-Reloadable**: Change content without recompiling
4. **Version Independent**: Data format can evolve separately from engine
5. **Mod Compatibility**: Multiple mods can provide data packs

## Creating Custom Content

To add new content, create a data pack:

1. Create directory structure: `data/mymod/`
2. Add `pack.mcmeta` with metadata
3. Create JSON definitions in `data/mymod/data/mymod/`
4. Place data pack in game's `data/` folder
5. Content loads automatically on game start

## See Also

- [Modding Guide](/modding/development)
- [Block API](/api/blocks)
- [Item API](/api/items)
