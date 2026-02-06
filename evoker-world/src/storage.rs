//! World storage system

use crate::{Chunk, ChunkCoord, WorldInfo};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

/// Storage format for world data
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StorageFormat {
    /// JSON format (human-readable, inefficient)
    Json,
    /// Binary format (efficient)
    Binary,
}

/// World storage manager
pub struct WorldStorage {
    base_path: PathBuf,
    format: StorageFormat,
}

impl WorldStorage {
    /// Create a new storage manager
    pub fn new(base_path: PathBuf) -> anyhow::Result<Self> {
        std::fs::create_dir_all(&base_path)?;
        
        Ok(Self {
            base_path,
            format: StorageFormat::Json,
        })
    }
    
    /// Get world directory path
    fn world_path(&self, world_name: &str) -> PathBuf {
        self.base_path.join(world_name)
    }
    
    /// Get world info file path
    fn world_info_path(&self, world_name: &str) -> PathBuf {
        self.world_path(world_name).join("world.json")
    }
    
    /// Get chunk file path
    fn chunk_path(&self, world_name: &str, coord: ChunkCoord) -> PathBuf {
        let region_x = coord.x >> 5; // Divide by 32
        let region_z = coord.z >> 5;
        
        self.world_path(world_name)
            .join("chunks")
            .join(format!("r.{}.{}", region_x, region_z))
            .join(format!("c.{}.{}.json", coord.x, coord.z))
    }
    
    /// Save world info
    pub async fn save_world_info(&self, info: &WorldInfo) -> anyhow::Result<()> {
        let path = self.world_info_path(&info.name);
        
        // Ensure directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        let content = serde_json::to_string_pretty(info)?;
        fs::write(path, content).await?;
        
        Ok(())
    }
    
    /// Load world info
    pub async fn load_world_info(&self, world_name: &str) -> anyhow::Result<WorldInfo> {
        let path = self.world_info_path(world_name);
        let content = fs::read_to_string(path).await?;
        let info: WorldInfo = serde_json::from_str(&content)?;
        Ok(info)
    }
    
    /// Save chunk
    pub async fn save_chunk(
        &self,
        world_name: &str,
        coord: ChunkCoord,
        chunk: &Chunk,
    ) -> anyhow::Result<()> {
        let path = self.chunk_path(world_name, coord);
        
        // Ensure directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        let content = serde_json::to_string(chunk)?;
        fs::write(path, content).await?;
        
        Ok(())
    }
    
    /// Load chunk
    pub async fn load_chunk(
        &self,
        world_name: &str,
        coord: ChunkCoord,
    ) -> anyhow::Result<Chunk> {
        let path = self.chunk_path(world_name, coord);
        let content = fs::read_to_string(path).await?;
        let chunk: Chunk = serde_json::from_str(&content)?;
        Ok(chunk)
    }
    
    /// List all worlds
    pub async fn list_worlds(&self) -> anyhow::Result<Vec<String>> {
        let mut worlds = Vec::new();
        
        let mut entries = fs::read_dir(&self.base_path).await?;
        while let Some(entry) = entries.next_entry().await? {
            if entry.file_type().await?.is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    worlds.push(name.to_string());
                }
            }
        }
        
        Ok(worlds)
    }
    
    /// Delete a world
    pub async fn delete_world(&self, world_name: &str) -> anyhow::Result<()> {
        let path = self.world_path(world_name);
        fs::remove_dir_all(path).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_storage() {
        let temp_dir = std::env::temp_dir().join("evoker_storage_test");
        let storage = WorldStorage::new(temp_dir.clone()).unwrap();
        
        let info = WorldInfo {
            name: "test".to_string(),
            seed: 12345,
            game_mode: "survival".to_string(),
            difficulty: "normal".to_string(),
            created_at: 0,
            last_played: 0,
        };
        
        storage.save_world_info(&info).await.unwrap();
        let loaded = storage.load_world_info("test").await.unwrap();
        
        assert_eq!(info.name, loaded.name);
        assert_eq!(info.seed, loaded.seed);
        
        // Cleanup
        let _ = std::fs::remove_dir_all(temp_dir);
    }
}
