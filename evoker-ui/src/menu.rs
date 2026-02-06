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
    // World list state
    world_list: Vec<String>,
    selected_world: Option<usize>,
    new_world_name: String,
    // Server list state
    server_address: String,
    saved_servers: Vec<(String, String)>, // (name, address)
    // Settings state
    render_distance: f32,
    vsync: bool,
    volume: f32,
    // Mods list state
    mod_list: Vec<(String, bool)>, // (mod_name, enabled)
}

impl MainMenu {
    /// Create a new main menu
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self {
            state: MenuState::Main,
            event_bus,
            world_list: vec!["World 1".to_string(), "World 2".to_string()],
            selected_world: None,
            new_world_name: String::new(),
            server_address: "localhost:25565".to_string(),
            saved_servers: vec![
                ("Local Server".to_string(), "localhost:25565".to_string()),
                ("Example Server".to_string(), "play.example.com:25565".to_string()),
            ],
            render_distance: 16.0,
            vsync: true,
            volume: 0.8,
            mod_list: vec![
                ("Example Mod".to_string(), true),
                ("Another Mod".to_string(), false),
            ],
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
        
        // World list
        egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
            for (i, world) in self.world_list.iter().enumerate() {
                let is_selected = self.selected_world == Some(i);
                if ui.selectable_label(is_selected, world).clicked() {
                    self.selected_world = Some(i);
                }
            }
        });
        
        ui.add_space(10.0);
        
        // World actions
        ui.horizontal(|ui| {
            if ui.button("Play Selected").clicked() {
                if let Some(idx) = self.selected_world {
                    let world_name = self.world_list[idx].clone();
                    log::info!("Starting world: {}", world_name);
                    self.event_bus.queue_event(Event::WorldLoaded {
                        world_name,
                    });
                }
            }
            
            if ui.button("Delete").clicked() {
                if let Some(idx) = self.selected_world {
                    self.world_list.remove(idx);
                    self.selected_world = None;
                }
            }
        });
        
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);
        
        // Create new world
        ui.label("Create New World");
        ui.horizontal(|ui| {
            ui.label("World Name:");
            ui.text_edit_singleline(&mut self.new_world_name);
        });
        
        if ui.button("Create").clicked() && !self.new_world_name.is_empty() {
            self.world_list.push(self.new_world_name.clone());
            log::info!("Created new world: {}", self.new_world_name);
            self.new_world_name.clear();
        }
        
        ui.add_space(20.0);
        if ui.button("Back").clicked() {
            self.state = MenuState::Main;
        }
    }
    
    fn render_multiplayer_menu(&mut self, ui: &mut egui::Ui) {
        ui.label("Connect to a server");
        ui.add_space(10.0);
        
        // Saved servers list
        ui.label("Saved Servers:");
        egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
            for (name, address) in &self.saved_servers {
                if ui.button(format!("{} ({})", name, address)).clicked() {
                    self.server_address = address.clone();
                    log::info!("Connecting to saved server: {}", address);
                    self.event_bus.queue_event(Event::ClientConnected {
                        server_address: address.clone(),
                    });
                }
            }
        });
        
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);
        
        // Direct connect
        ui.label("Direct Connect:");
        ui.horizontal(|ui| {
            ui.label("Server Address:");
            ui.text_edit_singleline(&mut self.server_address);
        });
        
        if ui.button("Connect").clicked() && !self.server_address.is_empty() {
            log::info!("Connecting to server: {}", self.server_address);
            self.event_bus.queue_event(Event::ClientConnected {
                server_address: self.server_address.clone(),
            });
        }
        
        ui.add_space(20.0);
        if ui.button("Back").clicked() {
            self.state = MenuState::Main;
        }
    }
    
    fn render_settings_menu(&mut self, ui: &mut egui::Ui) {
        ui.label("Settings");
        ui.add_space(10.0);
        
        // Video settings
        ui.group(|ui| {
            ui.label("Video Settings");
            ui.add_space(5.0);
            
            ui.horizontal(|ui| {
                ui.label("Render Distance:");
                ui.add(egui::Slider::new(&mut self.render_distance, 2.0..=32.0).suffix(" chunks"));
            });
            
            ui.checkbox(&mut self.vsync, "VSync");
        });
        
        ui.add_space(10.0);
        
        // Audio settings
        ui.group(|ui| {
            ui.label("Audio Settings");
            ui.add_space(5.0);
            
            ui.horizontal(|ui| {
                ui.label("Master Volume:");
                ui.add(egui::Slider::new(&mut self.volume, 0.0..=1.0).suffix("%"));
            });
        });
        
        ui.add_space(20.0);
        
        // Save/Reset buttons
        ui.horizontal(|ui| {
            if ui.button("Save Settings").clicked() {
                log::info!("Saving settings: render_distance={}, vsync={}, volume={}", 
                          self.render_distance, self.vsync, self.volume);
            }
            
            if ui.button("Reset to Defaults").clicked() {
                self.render_distance = 16.0;
                self.vsync = true;
                self.volume = 0.8;
            }
        });
        
        ui.add_space(20.0);
        if ui.button("Back").clicked() {
            self.state = MenuState::Main;
        }
    }
    
    fn render_mods_menu(&mut self, ui: &mut egui::Ui) {
        ui.label("Installed Mods");
        ui.add_space(10.0);
        
        // Mods list with enable/disable
        egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
            for (mod_name, enabled) in &mut self.mod_list {
                ui.horizontal(|ui| {
                    ui.checkbox(enabled, "");
                    ui.label(mod_name.as_str());
                    
                    if ui.small_button("Configure").clicked() {
                        log::info!("Configuring mod: {}", mod_name);
                    }
                });
            }
        });
        
        ui.add_space(10.0);
        
        // Mod actions
        ui.horizontal(|ui| {
            if ui.button("Open Mods Folder").clicked() {
                log::info!("Opening mods folder");
            }
            
            if ui.button("Get More Mods").clicked() {
                log::info!("Opening mod repository");
            }
        });
        
        ui.add_space(20.0);
        if ui.button("Back").clicked() {
            self.state = MenuState::Main;
        }
    }
    
    /// Get current menu state
    pub fn state(&self) -> MenuState {
        self.state
    }
    
    /// Get selected world
    pub fn selected_world(&self) -> Option<String> {
        self.selected_world.and_then(|idx| self.world_list.get(idx).cloned())
    }
    
    /// Get server address
    pub fn get_server_address(&self) -> String {
        self.server_address.clone()
    }
}
