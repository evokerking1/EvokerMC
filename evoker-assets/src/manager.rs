//! Asset manager

use crate::types::{Asset, AssetType};
use dashmap::DashMap;
use evoker_core::{Event, EventBus};
use std::path::PathBuf;
use std::sync::Arc;

/// Asset manager
pub struct AssetManager {
    assets_dir: PathBuf,
    loaded_assets: Arc<DashMap<String, Arc<Asset>>>,
    event_bus: Arc<EventBus>,
}

impl AssetManager {
    /// Create a new asset manager
    pub fn new(assets_dir: PathBuf, event_bus: Arc<EventBus>) -> anyhow::Result<Self> {
        std::fs::create_dir_all(&assets_dir)?;
        
        Ok(Self {
            assets_dir,
            loaded_assets: Arc::new(DashMap::new()),
            event_bus,
        })
    }
    
    /// Load an asset
    pub async fn load_asset(&self, id: &str) -> anyhow::Result<Arc<Asset>> {
        // Check if already loaded
        if let Some(asset) = self.loaded_assets.get(id) {
            return Ok(asset.clone());
        }
        
        // Load from disk
        let asset_path = self.assets_dir.join(format!("{}.json", id));
        let content = tokio::fs::read_to_string(&asset_path).await?;
        let mut asset: Asset = serde_json::from_str(&content)?;
        asset.load().await?;
        
        let asset = Arc::new(asset);
        self.loaded_assets.insert(id.to_string(), asset.clone());
        
        self.event_bus.queue_event(Event::AssetLoaded {
            asset_type: format!("{:?}", asset.asset_type),
            asset_id: id.to_string(),
        });
        
        Ok(asset)
    }
    
    /// Save an asset
    pub async fn save_asset(&self, asset: &Asset) -> anyhow::Result<()> {
        // Save metadata
        let asset_path = self.assets_dir.join(format!("{}.json", asset.id));
        let content = serde_json::to_string_pretty(asset)?;
        tokio::fs::write(asset_path, content).await?;
        
        // Save data
        asset.save().await?;
        
        Ok(())
    }
    
    /// List assets by type
    pub async fn list_assets_by_type(&self, asset_type: AssetType) -> anyhow::Result<Vec<String>> {
        let mut assets = Vec::new();
        
        let mut entries = tokio::fs::read_dir(&self.assets_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                if let Ok(content) = tokio::fs::read_to_string(&path).await {
                    if let Ok(asset) = serde_json::from_str::<Asset>(&content) {
                        if asset.asset_type == asset_type {
                            assets.push(asset.id);
                        }
                    }
                }
            }
        }
        
        Ok(assets)
    }
    
    /// Get loaded asset
    pub fn get_asset(&self, id: &str) -> Option<Arc<Asset>> {
        self.loaded_assets.get(id).map(|a| a.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_asset_manager() {
        let event_bus = Arc::new(EventBus::new());
        let temp_dir = std::env::temp_dir().join("evoker_assets_test");
        
        let manager = AssetManager::new(temp_dir.clone(), event_bus).unwrap();
        
        // Cleanup
        let _ = std::fs::remove_dir_all(temp_dir);
    }
}
