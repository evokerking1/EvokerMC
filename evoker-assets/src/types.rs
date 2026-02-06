//! Asset types

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Asset type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetType {
    /// Block texture
    BlockTexture,
    /// Item texture
    ItemTexture,
    /// Entity model
    EntityModel,
    /// Sound effect
    Sound,
    /// Music
    Music,
    /// Shader
    Shader,
    /// Script
    Script,
    /// Configuration
    Config,
}

/// Asset metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    /// Asset ID
    pub id: String,
    /// Asset name
    pub name: String,
    /// Asset type
    pub asset_type: AssetType,
    /// Path to asset file
    pub path: PathBuf,
    /// Asset data (loaded in memory)
    #[serde(skip)]
    pub data: Vec<u8>,
}

impl Asset {
    /// Create a new asset
    pub fn new(id: String, name: String, asset_type: AssetType, path: PathBuf) -> Self {
        Self {
            id,
            name,
            asset_type,
            path,
            data: Vec::new(),
        }
    }
    
    /// Load asset data
    pub async fn load(&mut self) -> anyhow::Result<()> {
        self.data = tokio::fs::read(&self.path).await?;
        Ok(())
    }
    
    /// Save asset data
    pub async fn save(&self) -> anyhow::Result<()> {
        tokio::fs::write(&self.path, &self.data).await?;
        Ok(())
    }
}
