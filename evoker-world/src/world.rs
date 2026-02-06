//! World representation and management

use crate::{Chunk, ChunkCoord, WorldStorage};
use dashmap::DashMap;
use evoker_core::{Event, EventBus};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;

/// World metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldInfo {
    /// World name
    pub name: String,
    /// World seed
    pub seed: i64,
    /// Game mode
    pub game_mode: String,
    /// Difficulty
    pub difficulty: String,
    /// Creation time (Unix timestamp)
    pub created_at: i64,
    /// Last played time (Unix timestamp)
    pub last_played: i64,
}

/// Main world structure
pub struct World {
    info: RwLock<WorldInfo>,
    chunks: Arc<DashMap<ChunkCoord, Arc<Chunk>>>,
    storage: Arc<WorldStorage>,
    event_bus: Arc<EventBus>,
    loaded: RwLock<bool>,
}

impl World {
    /// Create a new world
    pub fn new(
        name: String,
        seed: i64,
        storage_path: PathBuf,
        event_bus: Arc<EventBus>,
    ) -> anyhow::Result<Self> {
        let info = WorldInfo {
            name: name.clone(),
            seed,
            game_mode: "survival".to_string(),
            difficulty: "normal".to_string(),
            created_at: chrono::Utc::now().timestamp(),
            last_played: chrono::Utc::now().timestamp(),
        };
        
        let storage = Arc::new(WorldStorage::new(storage_path)?);
        
        Ok(Self {
            info: RwLock::new(info),
            chunks: Arc::new(DashMap::new()),
            storage,
            event_bus,
            loaded: RwLock::new(false),
        })
    }
    
    /// Load an existing world
    pub async fn load(
        name: String,
        storage_path: PathBuf,
        event_bus: Arc<EventBus>,
    ) -> anyhow::Result<Self> {
        let storage = Arc::new(WorldStorage::new(storage_path)?);
        let info = storage.load_world_info(&name).await?;
        
        let world = Self {
            info: RwLock::new(info),
            chunks: Arc::new(DashMap::new()),
            storage,
            event_bus: event_bus.clone(),
            loaded: RwLock::new(true),
        };
        
        event_bus.queue_event(Event::WorldLoaded {
            world_name: name,
        });
        
        Ok(world)
    }
    
    /// Save the world
    pub async fn save(&self) -> anyhow::Result<()> {
        let info = self.info.read().clone();
        
        log::info!("Saving world: {}", info.name);
        
        // Save world info
        self.storage.save_world_info(&info).await?;
        
        // Save all loaded chunks
        for chunk_entry in self.chunks.iter() {
            let coord = chunk_entry.key();
            let chunk = chunk_entry.value();
            self.storage.save_chunk(&info.name, *coord, chunk).await?;
        }
        
        self.event_bus.queue_event(Event::WorldSaved {
            world_name: info.name.clone(),
        });
        
        log::info!("World saved: {}", info.name);
        
        Ok(())
    }
    
    /// Unload the world
    pub async fn unload(&self) -> anyhow::Result<()> {
        // Save before unloading
        self.save().await?;
        
        let info = self.info.read();
        log::info!("Unloading world: {}", info.name);
        
        // Clear loaded chunks
        self.chunks.clear();
        
        *self.loaded.write() = false;
        
        self.event_bus.queue_event(Event::WorldUnloaded {
            world_name: info.name.clone(),
        });
        
        Ok(())
    }
    
    /// Get world info
    pub fn info(&self) -> WorldInfo {
        self.info.read().clone()
    }
    
    /// Load a chunk
    pub async fn load_chunk(&self, coord: ChunkCoord) -> anyhow::Result<Arc<Chunk>> {
        // Check if already loaded
        if let Some(chunk) = self.chunks.get(&coord) {
            return Ok(chunk.clone());
        }
        
        // Try to load from storage
        let info = self.info.read();
        let chunk = if let Ok(chunk) = self.storage.load_chunk(&info.name, coord).await {
            Arc::new(chunk)
        } else {
            // Generate new chunk
            Arc::new(Chunk::generate(coord, info.seed))
        };
        
        self.chunks.insert(coord, chunk.clone());
        
        Ok(chunk)
    }
    
    /// Unload a chunk
    pub async fn unload_chunk(&self, coord: ChunkCoord) -> anyhow::Result<()> {
        if let Some((_, chunk)) = self.chunks.remove(&coord) {
            let info = self.info.read();
            self.storage.save_chunk(&info.name, coord, &chunk).await?;
        }
        Ok(())
    }
    
    /// Check if world is loaded
    pub fn is_loaded(&self) -> bool {
        *self.loaded.read()
    }
}

// Add chrono as dependency
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_world_creation() {
        let event_bus = Arc::new(EventBus::new());
        let temp_dir = std::env::temp_dir().join("evoker_test");
        
        let world = World::new(
            "test_world".to_string(),
            12345,
            temp_dir.clone(),
            event_bus,
        );
        
        assert!(world.is_ok());
        
        // Cleanup
        let _ = std::fs::remove_dir_all(temp_dir);
    }
}
