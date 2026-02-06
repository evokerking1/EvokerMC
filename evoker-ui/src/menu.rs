//! Main menu implementation

use evoker_core::{Event, EventBus};
use std::sync::Arc;

/// Menu state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuState {
    Main,
    Singleplayer,
    Multiplayer,
    Settings,
    Mods,
}

/// Main menu
pub struct MainMenu {
    state: MenuState,
    event_bus: Arc<EventBus>,
}

impl MainMenu {
    /// Create a new main menu
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self {
            state: MenuState::Main,
            event_bus,
        }
    }
    
    /// Render main menu
    pub fn render(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("EvokerMC");
                ui.add_space(20.0);
                
                match self.state {
                    MenuState::Main => self.render_main_menu(ui),
                    MenuState::Singleplayer => self.render_singleplayer_menu(ui),
                    MenuState::Multiplayer => self.render_multiplayer_menu(ui),
                    MenuState::Settings => self.render_settings_menu(ui),
                    MenuState::Mods => self.render_mods_menu(ui),
                }
            });
        });
    }
    
    fn render_main_menu(&mut self, ui: &mut egui::Ui) {
        if ui.button("Singleplayer").clicked() {
            self.state = MenuState::Singleplayer;
            self.event_bus.queue_event(Event::MenuOpened {
                menu_type: "singleplayer".to_string(),
            });
        }
        
        if ui.button("Multiplayer").clicked() {
            self.state = MenuState::Multiplayer;
            self.event_bus.queue_event(Event::MenuOpened {
                menu_type: "multiplayer".to_string(),
            });
        }
        
        if ui.button("Mods").clicked() {
            self.state = MenuState::Mods;
            self.event_bus.queue_event(Event::MenuOpened {
                menu_type: "mods".to_string(),
            });
        }
        
        if ui.button("Settings").clicked() {
            self.state = MenuState::Settings;
            self.event_bus.queue_event(Event::MenuOpened {
                menu_type: "settings".to_string(),
            });
        }
        
        if ui.button("Quit").clicked() {
            self.event_bus.queue_event(Event::GameStopped);
        }
    }
    
    fn render_singleplayer_menu(&mut self, ui: &mut egui::Ui) {
        ui.label("Select or create a world");
        ui.add_space(10.0);
        
        // This would show list of worlds
        if ui.button("Create New World").clicked() {
            log::info!("Create new world");
        }
        
        ui.add_space(20.0);
        if ui.button("Back").clicked() {
            self.state = MenuState::Main;
        }
    }
    
    fn render_multiplayer_menu(&mut self, ui: &mut egui::Ui) {
        ui.label("Connect to a server");
        ui.add_space(10.0);
        
        // This would show server list
        if ui.button("Direct Connect").clicked() {
            log::info!("Direct connect");
        }
        
        ui.add_space(20.0);
        if ui.button("Back").clicked() {
            self.state = MenuState::Main;
        }
    }
    
    fn render_settings_menu(&mut self, ui: &mut egui::Ui) {
        ui.label("Settings");
        ui.add_space(10.0);
        
        // This would show settings options
        
        ui.add_space(20.0);
        if ui.button("Back").clicked() {
            self.state = MenuState::Main;
        }
    }
    
    fn render_mods_menu(&mut self, ui: &mut egui::Ui) {
        ui.label("Installed Mods");
        ui.add_space(10.0);
        
        // This would show list of mods
        
        ui.add_space(20.0);
        if ui.button("Back").clicked() {
            self.state = MenuState::Main;
        }
    }
    
    /// Get current menu state
    pub fn state(&self) -> MenuState {
        self.state
    }
}
