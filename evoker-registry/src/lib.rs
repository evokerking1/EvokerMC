//! Data-driven registry system
//! 
//! Similar to Minecraft's registry system, all game objects are registered
//! and loaded from JSON data files.

pub mod registry;
pub mod datapack;
pub mod blocks;
pub mod items;
pub mod entities;
pub mod recipes;

pub use registry::{Registry, RegistryKey};
pub use datapack::DataPack;
pub use blocks::BlockDefinition;
pub use items::ItemDefinition;
pub use entities::EntityDefinition;
pub use recipes::RecipeDefinition;
