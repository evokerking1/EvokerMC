//! UI system for main menu and in-game interfaces
//! 
//! Built with egui for immediate mode GUI

pub mod menu;
pub mod hud;
pub mod renderer;

pub use menu::{MainMenu, MenuState};
pub use hud::Hud;
pub use renderer::UiRenderer;
