# Asset Editor Overview

EvokerMC includes an **in-game asset editor** inspired by Hytale's editor. It lets you create and modify game assets — textures, models, sounds, scripts, and more — while the engine is running, with changes reflected immediately.

## What Is the Asset Editor?

The asset editor is provided by the `evoker-assets` crate and is accessible at any time during gameplay (not only from the main menu). It is built on top of `egui` and integrates directly with the `AssetManager`.

Key capabilities:

- **Browse** all loaded assets by type.
- **Edit** asset data in real time (textures, model files, config, scripts).
- **Preview** changes without restarting the engine.
- **Export** edited assets for inclusion in a mod or data pack.

## Opening the Editor

The asset editor is exposed as a toggleable overlay. In a future release it will be bound to a keyboard shortcut (default: **F10**). Until then it can be opened programmatically:

```rust
use evoker_assets::AssetEditor;

let editor = AssetEditor::new(asset_manager.clone(), event_bus.clone());
// Render inside your egui frame:
editor.render(ctx, &mut asset_manager);
```

## Asset Manager

The `AssetManager` handles loading and caching assets from disk:

```rust
use evoker_assets::AssetManager;
use std::path::PathBuf;

let manager = AssetManager::new(PathBuf::from("assets"), event_bus.clone())?;
```

### Loading an Asset

```rust
let asset_id = manager.load_asset("blocks/stone", AssetType::BlockTexture).await?;
```

### Accessing a Loaded Asset

```rust
if let Some(asset) = manager.get_asset("blocks/stone") {
    println!("Asset: {} ({} bytes)", asset.name, asset.data.len());
}
```

## Live Editing Workflow

1. Open the asset editor overlay.
2. Select an asset type from the sidebar (e.g., *Block Textures*).
3. Click an asset to load it into the editor panel.
4. Modify the raw data or use the built-in editing tools.
5. Click **Apply** — the engine reloads the asset and emits an `AssetModified` event so any subscribed system (renderer, scripting API) can respond.

## Events

| Event | Fired when |
|-------|-----------|
| `AssetLoaded { asset_type, asset_id }` | An asset is successfully loaded |
| `AssetModified { asset_type, asset_id }` | An asset is modified via the editor |

## See Also

- [Creating Assets](/assets/creating)
- [Asset Types](/assets/types)
- [Architecture Overview](/architecture/overview)
