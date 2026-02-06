//! Modding system for loading and managing mods
//! 
//! Supports both JAR-based mods and WASM mods for maximum compatibility and security

pub mod jar;
pub mod wasm;
pub mod loader;
pub mod api;

pub use loader::{ModLoader, ModInfo, ModType};
pub use api::ModApi;
