//! Scripting API exposed to scripts

use evoker_core::{Event, EventBus};
use serde_json::Value;
use std::sync::Arc;

/// Script API
pub struct ScriptApi {
    event_bus: Arc<EventBus>,
}

impl ScriptApi {
    /// Create a new script API
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self { event_bus }
    }
    
    // ============================================================================
    // Logging
    // ============================================================================
    
    /// Log a message
    pub fn log(&self, message: &str) {
        log::info!("[Script] {}", message);
    }
    
    /// Log a debug message
    pub fn debug(&self, message: &str) {
        log::debug!("[Script] {}", message);
    }
    
    /// Log a warning message
    pub fn warn(&self, message: &str) {
        log::warn!("[Script] {}", message);
    }
    
    /// Log an error message
    pub fn error(&self, message: &str) {
        log::error!("[Script] {}", message);
    }
    
    // ============================================================================
    // Events
    // ============================================================================
    
    /// Emit an event
    pub async fn emit_event(&self, event_type: String, data: serde_json::Value) -> anyhow::Result<()> {
        self.event_bus.emit(Event::Custom { event_type, data }).await
    }
    
    // ============================================================================
    // Block Management
    // ============================================================================
    
    /// Set block at position
    pub async fn set_block(&self, world: &str, x: i32, y: i32, z: i32, block_id: &str) -> anyhow::Result<()> {
        log::debug!("[Script] Setting block at ({}, {}, {}) to {} in world {}", x, y, z, block_id, world);
        
        self.event_bus.emit(Event::Custom {
            event_type: "BlockPlaced".to_string(),
            data: serde_json::json!({
                "world": world,
                "x": x,
                "y": y,
                "z": z,
                "block_id": block_id,
            }),
        }).await?;
        
        Ok(())
    }
    
    /// Get block at position
    pub fn get_block(&self, world: &str, x: i32, y: i32, z: i32) -> anyhow::Result<String> {
        log::debug!("[Script] Getting block at ({}, {}, {}) in world {}", x, y, z, world);
        
        // Return placeholder - in full implementation, would query world data
        Ok("evokermc:air".to_string())
    }
    
    /// Fill region with blocks
    pub async fn fill_blocks(
        &self,
        world: &str,
        x1: i32, y1: i32, z1: i32,
        x2: i32, y2: i32, z2: i32,
        block_id: &str
    ) -> anyhow::Result<()> {
        log::info!("[Script] Filling blocks from ({},{},{}) to ({},{},{}) with {}", 
                   x1, y1, z1, x2, y2, z2, block_id);
        
        self.event_bus.emit(Event::Custom {
            event_type: "BlocksFilled".to_string(),
            data: serde_json::json!({
                "world": world,
                "x1": x1, "y1": y1, "z1": z1,
                "x2": x2, "y2": y2, "z2": z2,
                "block_id": block_id,
            }),
        }).await?;
        
        Ok(())
    }
    
    // ============================================================================
    // Entity Management
    // ============================================================================
    
    /// Spawn entity at position
    pub async fn spawn_entity(&self, world: &str, entity_id: &str, x: f64, y: f64, z: f64) -> anyhow::Result<String> {
        log::info!("[Script] Spawning entity {} at ({}, {}, {}) in world {}", entity_id, x, y, z, world);
        
        let entity_uuid = uuid::Uuid::new_v4().to_string();
        
        self.event_bus.emit(Event::Custom {
            event_type: "EntitySpawned".to_string(),
            data: serde_json::json!({
                "world": world,
                "entity_id": entity_id,
                "entity_uuid": entity_uuid,
                "x": x,
                "y": y,
                "z": z,
            }),
        }).await?;
        
        Ok(entity_uuid)
    }
    
    /// Remove entity
    pub async fn remove_entity(&self, entity_uuid: &str) -> anyhow::Result<()> {
        log::info!("[Script] Removing entity {}", entity_uuid);
        
        self.event_bus.emit(Event::Custom {
            event_type: "EntityRemoved".to_string(),
            data: serde_json::json!({
                "entity_uuid": entity_uuid,
            }),
        }).await?;
        
        Ok(())
    }
    
    /// Get entities in radius
    pub fn get_entities_near(&self, world: &str, x: f64, y: f64, z: f64, radius: f64) -> Vec<String> {
        log::debug!("[Script] Getting entities near ({}, {}, {}) within radius {} in world {}", 
                    x, y, z, radius, world);
        
        // Return placeholder - in full implementation, would query world entities
        vec![]
    }
    
    // ============================================================================
    // Item/Inventory Management
    // ============================================================================
    
    /// Give item to player
    pub async fn give_item(&self, player_id: &str, item_id: &str, count: u32) -> anyhow::Result<()> {
        log::info!("[Script] Giving {} x{} to player {}", item_id, count, player_id);
        
        self.event_bus.emit(Event::Custom {
            event_type: "ItemGiven".to_string(),
            data: serde_json::json!({
                "player_id": player_id,
                "item_id": item_id,
                "count": count,
            }),
        }).await?;
        
        Ok(())
    }
    
    /// Remove item from player
    pub async fn remove_item(&self, player_id: &str, item_id: &str, count: u32) -> anyhow::Result<()> {
        log::info!("[Script] Removing {} x{} from player {}", item_id, count, player_id);
        
        self.event_bus.emit(Event::Custom {
            event_type: "ItemRemoved".to_string(),
            data: serde_json::json!({
                "player_id": player_id,
                "item_id": item_id,
                "count": count,
            }),
        }).await?;
        
        Ok(())
    }
    
    // ============================================================================
    // Player Management
    // ============================================================================
    
    /// Get online players
    pub fn get_players(&self) -> Vec<String> {
        log::debug!("[Script] Getting online players");
        
        // Return placeholder - in full implementation, would query player manager
        vec![]
    }
    
    /// Send message to player
    pub async fn send_message(&self, player_id: &str, message: &str) -> anyhow::Result<()> {
        log::info!("[Script] Sending message to player {}: {}", player_id, message);
        
        self.event_bus.emit(Event::Custom {
            event_type: "PlayerMessage".to_string(),
            data: serde_json::json!({
                "player_id": player_id,
                "message": message,
            }),
        }).await?;
        
        Ok(())
    }
    
    /// Broadcast message to all players
    pub async fn broadcast(&self, message: &str) -> anyhow::Result<()> {
        log::info!("[Script] Broadcasting message: {}", message);
        
        self.event_bus.emit(Event::Custom {
            event_type: "BroadcastMessage".to_string(),
            data: serde_json::json!({
                "message": message,
            }),
        }).await?;
        
        Ok(())
    }
    
    /// Teleport player to position
    pub async fn teleport(&self, player_id: &str, world: &str, x: f64, y: f64, z: f64) -> anyhow::Result<()> {
        log::info!("[Script] Teleporting player {} to ({}, {}, {}) in world {}", 
                   player_id, x, y, z, world);
        
        self.event_bus.emit(Event::Custom {
            event_type: "PlayerTeleported".to_string(),
            data: serde_json::json!({
                "player_id": player_id,
                "world": world,
                "x": x,
                "y": y,
                "z": z,
            }),
        }).await?;
        
        Ok(())
    }
    
    // ============================================================================
    // World Queries
    // ============================================================================
    
    /// Get loaded worlds
    pub fn get_worlds(&self) -> Vec<String> {
        log::debug!("[Script] Getting loaded worlds");
        
        // Return placeholder - in full implementation, would query world manager
        vec![]
    }
    
    /// Get time of day in world (0-24000)
    pub fn get_time(&self, world: &str) -> i64 {
        log::debug!("[Script] Getting time in world {}", world);
        
        // Return placeholder - in full implementation, would query world data
        0
    }
    
    /// Set time of day in world (0-24000)
    pub async fn set_time(&self, world: &str, time: i64) -> anyhow::Result<()> {
        log::info!("[Script] Setting time to {} in world {}", time, world);
        
        self.event_bus.emit(Event::Custom {
            event_type: "TimeSet".to_string(),
            data: serde_json::json!({
                "world": world,
                "time": time,
            }),
        }).await?;
        
        Ok(())
    }
    
    /// Get weather in world
    pub fn get_weather(&self, world: &str) -> String {
        log::debug!("[Script] Getting weather in world {}", world);
        
        // Return placeholder - in full implementation, would query world data
        "clear".to_string()
    }
    
    /// Set weather in world
    pub async fn set_weather(&self, world: &str, weather: &str) -> anyhow::Result<()> {
        log::info!("[Script] Setting weather to {} in world {}", weather, world);
        
        self.event_bus.emit(Event::Custom {
            event_type: "WeatherSet".to_string(),
            data: serde_json::json!({
                "world": world,
                "weather": weather,
            }),
        }).await?;
        
        Ok(())
    }
    
    // ============================================================================
    // Data Persistence
    // ============================================================================
    
    /// Save script data
    pub async fn save_data(&self, key: &str, data: Value) -> anyhow::Result<()> {
        log::debug!("[Script] Saving data for key: {}", key);
        
        self.event_bus.emit(Event::Custom {
            event_type: "ScriptDataSaved".to_string(),
            data: serde_json::json!({
                "key": key,
                "data": data,
            }),
        }).await?;
        
        Ok(())
    }
    
    /// Load script data
    pub fn load_data(&self, key: &str) -> anyhow::Result<Value> {
        log::debug!("[Script] Loading data for key: {}", key);
        
        // Return placeholder - in full implementation, would read from persistent storage
        Ok(Value::Null)
    }
    
    // ============================================================================
    // Utility Functions
    // ============================================================================
    
    /// Execute a command
    pub async fn execute_command(&self, command: &str) -> anyhow::Result<String> {
        log::info!("[Script] Executing command: {}", command);
        
        self.event_bus.emit(Event::Custom {
            event_type: "CommandExecuted".to_string(),
            data: serde_json::json!({
                "command": command,
            }),
        }).await?;
        
        Ok("Command executed successfully".to_string())
    }
    
    /// Schedule a delayed task (in ticks, 20 ticks = 1 second)
    pub async fn schedule(&self, delay_ticks: u32, task_id: &str, data: Value) -> anyhow::Result<()> {
        log::info!("[Script] Scheduling task {} in {} ticks", task_id, delay_ticks);
        
        self.event_bus.emit(Event::Custom {
            event_type: "TaskScheduled".to_string(),
            data: serde_json::json!({
                "delay_ticks": delay_ticks,
                "task_id": task_id,
                "data": data,
            }),
        }).await?;
        
        Ok(())
    }
}
