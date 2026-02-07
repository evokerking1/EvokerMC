//! Mod API for interacting with the game engine

use evoker_core::{Event, EventBus};
use serde_json::Value;
use std::sync::Arc;

/// Mod API interface
pub struct ModApi {
    event_bus: Arc<EventBus>,
}

impl ModApi {
    /// Create a new mod API instance
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self { event_bus }
    }
    
    /// Register event handler
    pub fn register_event_handler(&self, handler: Arc<dyn evoker_core::EventHandler>) {
        self.event_bus.register_handler(handler);
    }
    
    /// Emit custom event
    pub async fn emit_custom_event(&self, event_type: String, data: serde_json::Value) -> anyhow::Result<()> {
        self.event_bus.emit(Event::Custom { event_type, data }).await
    }
    
    /// Get API version
    pub fn version(&self) -> &str {
        "0.1.0"
    }
    
    // ============================================================================
    // Block Registration and Management
    // ============================================================================
    
    /// Register a new block type
    pub fn register_block(&self, block_id: &str, properties: Value) -> anyhow::Result<()> {
        log::info!("Registering block: {}", block_id);
        
        // Emit block registration event
        self.event_bus.queue_event(Event::Custom {
            event_type: "BlockRegistered".to_string(),
            data: serde_json::json!({
                "block_id": block_id,
                "properties": properties,
            }),
        });
        
        Ok(())
    }
    
    /// Get block properties
    pub fn get_block(&self, block_id: &str) -> anyhow::Result<Value> {
        log::debug!("Getting block properties for: {}", block_id);
        
        // Return placeholder - in full implementation, would query registry
        Ok(serde_json::json!({
            "id": block_id,
            "exists": true,
        }))
    }
    
    /// Set block at position in world
    pub async fn set_block(&self, world: &str, x: i32, y: i32, z: i32, block_id: &str) -> anyhow::Result<()> {
        log::debug!("Setting block at ({}, {}, {}) to {} in world {}", x, y, z, block_id, world);
        
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
    
    /// Get block at position in world
    pub fn get_block_at(&self, world: &str, x: i32, y: i32, z: i32) -> anyhow::Result<String> {
        log::debug!("Getting block at ({}, {}, {}) in world {}", x, y, z, world);
        
        // Return placeholder - in full implementation, would query world data
        Ok("evokermc:air".to_string())
    }
    
    // ============================================================================
    // Item Registration and Management
    // ============================================================================
    
    /// Register a new item type
    pub fn register_item(&self, item_id: &str, properties: Value) -> anyhow::Result<()> {
        log::info!("Registering item: {}", item_id);
        
        self.event_bus.queue_event(Event::Custom {
            event_type: "ItemRegistered".to_string(),
            data: serde_json::json!({
                "item_id": item_id,
                "properties": properties,
            }),
        });
        
        Ok(())
    }
    
    /// Give item to player
    pub async fn give_item(&self, player_id: &str, item_id: &str, count: u32) -> anyhow::Result<()> {
        log::info!("Giving {} x{} to player {}", item_id, count, player_id);
        
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
    
    /// Remove item from player inventory
    pub async fn remove_item(&self, player_id: &str, item_id: &str, count: u32) -> anyhow::Result<()> {
        log::info!("Removing {} x{} from player {}", item_id, count, player_id);
        
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
    // Entity Management
    // ============================================================================
    
    /// Register a new entity type
    pub fn register_entity(&self, entity_id: &str, properties: Value) -> anyhow::Result<()> {
        log::info!("Registering entity: {}", entity_id);
        
        self.event_bus.queue_event(Event::Custom {
            event_type: "EntityRegistered".to_string(),
            data: serde_json::json!({
                "entity_id": entity_id,
                "properties": properties,
            }),
        });
        
        Ok(())
    }
    
    /// Spawn entity at position
    pub async fn spawn_entity(&self, world: &str, entity_id: &str, x: f64, y: f64, z: f64) -> anyhow::Result<String> {
        log::info!("Spawning entity {} at ({}, {}, {}) in world {}", entity_id, x, y, z, world);
        
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
    
    /// Remove entity by UUID
    pub async fn remove_entity(&self, entity_uuid: &str) -> anyhow::Result<()> {
        log::info!("Removing entity {}", entity_uuid);
        
        self.event_bus.emit(Event::Custom {
            event_type: "EntityRemoved".to_string(),
            data: serde_json::json!({
                "entity_uuid": entity_uuid,
            }),
        }).await?;
        
        Ok(())
    }
    
    // ============================================================================
    // Recipe Management
    // ============================================================================
    
    /// Register a crafting recipe
    pub fn register_recipe(&self, recipe_id: &str, recipe_data: Value) -> anyhow::Result<()> {
        log::info!("Registering recipe: {}", recipe_id);
        
        self.event_bus.queue_event(Event::Custom {
            event_type: "RecipeRegistered".to_string(),
            data: serde_json::json!({
                "recipe_id": recipe_id,
                "recipe_data": recipe_data,
            }),
        });
        
        Ok(())
    }
    
    // ============================================================================
    // Command Registration
    // ============================================================================
    
    /// Register a command
    pub fn register_command(&self, command: &str, description: &str) -> anyhow::Result<()> {
        log::info!("Registering command: /{} - {}", command, description);
        
        self.event_bus.queue_event(Event::Custom {
            event_type: "CommandRegistered".to_string(),
            data: serde_json::json!({
                "command": command,
                "description": description,
            }),
        });
        
        Ok(())
    }
    
    /// Execute a command
    pub async fn execute_command(&self, command: &str, args: Vec<String>) -> anyhow::Result<String> {
        log::info!("Executing command: {} {:?}", command, args);
        
        self.event_bus.emit(Event::Custom {
            event_type: "CommandExecuted".to_string(),
            data: serde_json::json!({
                "command": command,
                "args": args,
            }),
        }).await?;
        
        Ok("Command executed successfully".to_string())
    }
    
    // ============================================================================
    // Configuration
    // ============================================================================
    
    /// Get mod configuration value
    pub fn get_config(&self, key: &str) -> anyhow::Result<Value> {
        log::debug!("Getting config value for: {}", key);
        
        // Return placeholder - in full implementation, would read from config file
        Ok(Value::Null)
    }
    
    /// Set mod configuration value
    pub fn set_config(&self, key: &str, value: Value) -> anyhow::Result<()> {
        log::debug!("Setting config value for: {} = {:?}", key, value);
        
        self.event_bus.queue_event(Event::Custom {
            event_type: "ConfigUpdated".to_string(),
            data: serde_json::json!({
                "key": key,
                "value": value,
            }),
        });
        
        Ok(())
    }
    
    // ============================================================================
    // Data Persistence
    // ============================================================================
    
    /// Save mod data
    pub async fn save_data(&self, key: &str, data: Value) -> anyhow::Result<()> {
        log::debug!("Saving mod data for key: {}", key);
        
        self.event_bus.emit(Event::Custom {
            event_type: "ModDataSaved".to_string(),
            data: serde_json::json!({
                "key": key,
                "data": data,
            }),
        }).await?;
        
        Ok(())
    }
    
    /// Load mod data
    pub fn load_data(&self, key: &str) -> anyhow::Result<Value> {
        log::debug!("Loading mod data for key: {}", key);
        
        // Return placeholder - in full implementation, would read from persistent storage
        Ok(Value::Null)
    }
    
    // ============================================================================
    // World Queries
    // ============================================================================
    
    /// Get loaded worlds
    pub fn get_worlds(&self) -> Vec<String> {
        log::debug!("Getting loaded worlds");
        
        // Return placeholder - in full implementation, would query world manager
        vec![]
    }
    
    /// Check if world is loaded
    pub fn is_world_loaded(&self, world: &str) -> bool {
        log::debug!("Checking if world {} is loaded", world);
        
        // Return placeholder - in full implementation, would query world manager
        false
    }
    
    // ============================================================================
    // Player Queries
    // ============================================================================
    
    /// Get online players
    pub fn get_online_players(&self) -> Vec<String> {
        log::debug!("Getting online players");
        
        // Return placeholder - in full implementation, would query player manager
        vec![]
    }
    
    /// Send message to player
    pub async fn send_message(&self, player_id: &str, message: &str) -> anyhow::Result<()> {
        log::info!("Sending message to player {}: {}", player_id, message);
        
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
        log::info!("Broadcasting message: {}", message);
        
        self.event_bus.emit(Event::Custom {
            event_type: "BroadcastMessage".to_string(),
            data: serde_json::json!({
                "message": message,
            }),
        }).await?;
        
        Ok(())
    }
}
