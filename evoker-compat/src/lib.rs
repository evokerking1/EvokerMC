//! Compatibility layers for Minecraft Forge and Neoforge APIs
//! 
//! Allows existing Minecraft mods to work with EvokerMC

pub mod forge;
pub mod neoforge;

pub use forge::ForgeCompatLayer;
pub use neoforge::NeoforgeCompatLayer;
