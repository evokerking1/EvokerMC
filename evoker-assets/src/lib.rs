//! Asset management and in-game editor system
//! 
//! Similar to Hytale's asset editor, allows editing assets in-game

pub mod manager;
pub mod editor;
pub mod types;

pub use manager::AssetManager;
pub use editor::AssetEditor;
pub use types::{Asset, AssetType};
