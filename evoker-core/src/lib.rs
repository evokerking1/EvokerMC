//! EvokerMC Core Engine
//! 
//! This module provides the foundational architecture for the EvokerMC game engine.
//! It includes the event system, game loop, and core abstractions.

pub mod events;
pub mod game;
pub mod config;

pub use events::{Event, EventBus, EventHandler};
pub use game::{Game, GameState};
pub use config::Config;

/// Result type for core operations
pub type Result<T> = anyhow::Result<T>;

/// Core engine initialization
pub async fn init() -> Result<()> {
    env_logger::init();
    log::info!("EvokerMC Core Engine initialized");
    Ok(())
}
