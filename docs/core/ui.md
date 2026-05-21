# UI System

The `evoker-ui` crate provides the game's user interface: the main menu, the in-game HUD, and the settings screens. It is built on [egui](https://github.com/emilk/egui), an immediate-mode GUI library.

## Components

### Main Menu

`MainMenu` is the entry point when no world is loaded. It manages its own internal state machine:

| State | Screen |
|-------|--------|
| `Main` | Top-level menu (Singleplayer / Multiplayer / Mods / Settings / Quit) |
| `Singleplayer` | World list — create, select, and delete worlds |
| `Multiplayer` | Server list and direct-connect input |
| `Settings` | Video and audio settings |
| `Mods` | List of installed mods with enable/disable toggles |

```rust
use evoker_ui::MainMenu;

let mut menu = MainMenu::new(event_bus.clone());

// Inside the egui render loop:
menu.render(ctx);
```

#### World Selection

```rust
// Get the currently selected world name
if let Some(world_name) = menu.selected_world() {
    app.start_singleplayer(world_name).await?;
}
```

#### Server Address

```rust
let address = menu.get_server_address();
app.connect_multiplayer(address).await?;
```

### HUD (Heads-Up Display)

`Hud` renders the in-game overlay while the world is running:

- **Crosshair** at the centre of the screen
- **Hotbar** showing the active item slots
- **Health / hunger bars**
- **Debug overlay** (F3-style information: FPS, coordinates, chunk info)

### Asset Editor

The in-game asset editor (from `evoker-assets`) is accessible while the game is running and allows live editing of textures, models, and sounds. See [Asset Editor](/assets/overview).

## UI Events

The menu emits events via `EventBus` in response to user actions:

| Event | Trigger |
|-------|---------|
| `MenuOpened { menu_type }` | Navigating to a sub-menu |
| `MenuClosed { menu_type }` | Pressing "Back" |
| `WorldLoaded { world_name }` | Clicking "Play Selected" |
| `ClientConnected { server_address }` | Clicking "Connect" in Multiplayer |
| `GameStopped` | Clicking "Quit" |

## Settings

The settings screen currently exposes:

| Setting | Default | Range |
|---------|---------|-------|
| Render distance | 16 chunks | 2 - 32 |
| VSync | enabled | on/off |
| Master volume | 0.8 | 0 – 1 |

Settings are logged on save and will be persisted to `config.json` in a future release.

## egui Integration

`evoker-ui` depends on `egui 0.29` and `winit 0.30`. The rendering backend (wgpu) is provided by a separate integration layer that is part of the planned 3D renderer (`evoker-renderer`, not yet implemented). Until then the UI is rendered using a software rasteriser suitable for development.

## See Also

- [Architecture Overview](/architecture/overview)
- [Asset Editor](/assets/overview)
