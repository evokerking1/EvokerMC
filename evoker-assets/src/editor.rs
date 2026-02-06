//! In-game asset editor (like Hytale's editor)

use crate::types::{Asset, AssetType};
use crate::manager::AssetManager;
use evoker_core::{Event, EventBus};
use std::sync::Arc;

/// Asset editor
pub struct AssetEditor {
    asset_manager: Arc<AssetManager>,
    event_bus: Arc<EventBus>,
    current_asset: Option<String>,
}

impl AssetEditor {
    /// Create a new asset editor
    pub fn new(asset_manager: Arc<AssetManager>, event_bus: Arc<EventBus>) -> Self {
        Self {
            asset_manager,
            event_bus,
            current_asset: None,
        }
    }
    
    /// Open an asset for editing
    pub async fn open_asset(&mut self, asset_id: String) -> anyhow::Result<()> {
        let asset = self.asset_manager.load_asset(&asset_id).await?;
        self.current_asset = Some(asset_id.clone());
        
        log::info!("Opened asset for editing: {}", asset.name);
        
        Ok(())
    }
    
    /// Close current asset
    pub fn close_asset(&mut self) {
        self.current_asset = None;
    }
    
    /// Save current asset
    pub async fn save_current_asset(&self) -> anyhow::Result<()> {
        if let Some(asset_id) = &self.current_asset {
            if let Some(asset) = self.asset_manager.get_asset(asset_id) {
                self.asset_manager.save_asset(&asset).await?;
                
                self.event_bus.queue_event(Event::AssetModified {
                    asset_type: format!("{:?}", asset.asset_type),
                    asset_id: asset_id.clone(),
                });
                
                log::info!("Saved asset: {}", asset.name);
            }
        }
        
        Ok(())
    }
    
    /// Create a new asset
    pub async fn create_asset(
        &mut self,
        id: String,
        name: String,
        asset_type: AssetType,
    ) -> anyhow::Result<()> {
        let path = std::path::PathBuf::from(format!("assets/{}/{}", 
            format!("{:?}", asset_type).to_lowercase(), name));
        
        let asset = Asset::new(id.clone(), name, asset_type, path);
        self.asset_manager.save_asset(&asset).await?;
        
        self.current_asset = Some(id);
        
        Ok(())
    }
    
    /// Render editor UI
    pub fn render(&mut self, ctx: &egui::Context) {
        egui::Window::new("Asset Editor")
            .default_size([800.0, 600.0])
            .show(ctx, |ui| {
                ui.heading("Asset Editor");
                ui.separator();
                
                if let Some(asset_id) = &self.current_asset {
                    if let Some(asset) = self.asset_manager.get_asset(asset_id) {
                        ui.label(format!("Editing: {}", asset.name));
                        ui.label(format!("Type: {:?}", asset.asset_type));
                        ui.separator();
                        
                        // Asset-specific editing UI would go here
                        
                        if ui.button("Save").clicked() {
                            tokio::spawn({
                                let editor = self.asset_manager.clone();
                                let asset = asset.clone();
                                async move {
                                    let _ = editor.save_asset(&asset).await;
                                }
                            });
                        }
                    }
                } else {
                    ui.label("No asset open");
                }
            });
    }
    
    /// Get current asset
    pub fn current_asset(&self) -> Option<&String> {
        self.current_asset.as_ref()
    }
}
