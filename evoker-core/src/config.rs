//! Configuration management

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Game settings
    pub game: GameConfig,
    /// Network settings
    pub network: NetworkConfig,
    /// Graphics settings
    pub graphics: GraphicsConfig,
    /// Mod settings
    pub mods: ModConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    /// Game title
    pub title: String,
    /// Target tick rate (ticks per second)
    pub tick_rate: u32,
    /// Enable debug mode
    pub debug: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Default server port
    pub default_port: u16,
    /// Maximum players
    pub max_players: usize,
    /// Enable multiplayer
    pub enable_multiplayer: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsConfig {
    /// Render distance (in chunks)
    pub render_distance: u32,
    /// Enable VSync
    pub vsync: bool,
    /// Target FPS
    pub target_fps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModConfig {
    /// Mods directory
    pub mods_dir: PathBuf,
    /// Enable mod loading
    pub enable_mods: bool,
    /// Enable WASM mods
    pub enable_wasm: bool,
    /// Enable scripting
    pub enable_scripting: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            game: GameConfig {
                title: "EvokerMC".to_string(),
                tick_rate: 20,
                debug: false,
            },
            network: NetworkConfig {
                default_port: 25565,
                max_players: 100,
                enable_multiplayer: true,
            },
            graphics: GraphicsConfig {
                render_distance: 16,
                vsync: true,
                target_fps: 60,
            },
            mods: ModConfig {
                mods_dir: PathBuf::from("mods"),
                enable_mods: true,
                enable_wasm: true,
                enable_scripting: true,
            },
        }
    }
}

impl Config {
    /// Load configuration from file
    pub fn load(path: &PathBuf) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }
    
    /// Save configuration to file
    pub fn save(&self, path: &PathBuf) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
