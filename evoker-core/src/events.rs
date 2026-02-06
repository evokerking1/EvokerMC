//! Event system for game engine
//! 
//! Provides a flexible event bus for communication between game components

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use dashmap::DashMap;
use parking_lot::RwLock;

/// Core event types in the game engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    /// Game state events
    GameStarted,
    GameStopped,
    GamePaused,
    GameResumed,
    
    /// World events
    WorldLoaded { world_name: String },
    WorldSaved { world_name: String },
    WorldUnloaded { world_name: String },
    
    /// Player events
    PlayerJoined { player_id: String, player_name: String },
    PlayerLeft { player_id: String },
    
    /// UI events
    MenuOpened { menu_type: String },
    MenuClosed { menu_type: String },
    
    /// Mod events
    ModLoaded { mod_id: String, mod_name: String },
    ModUnloaded { mod_id: String },
    
    /// Asset events
    AssetLoaded { asset_type: String, asset_id: String },
    AssetModified { asset_type: String, asset_id: String },
    
    /// Network events
    ServerStarted { port: u16 },
    ServerStopped,
    ClientConnected { server_address: String },
    ClientDisconnected,
    
    /// Custom event for mods to use
    Custom { event_type: String, data: serde_json::Value },
}

/// Event handler trait
#[async_trait]
pub trait EventHandler: Send + Sync {
    /// Handle an event
    async fn handle_event(&self, event: &Event) -> anyhow::Result<()>;
    
    /// Get the events this handler is interested in
    fn event_types(&self) -> Vec<String>;
}

/// Event bus for dispatching events
pub struct EventBus {
    handlers: Arc<DashMap<String, Vec<Arc<dyn EventHandler>>>>,
    event_queue: Arc<RwLock<Vec<Event>>>,
}

impl EventBus {
    /// Create a new event bus
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(DashMap::new()),
            event_queue: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Register an event handler
    pub fn register_handler(&self, handler: Arc<dyn EventHandler>) {
        for event_type in handler.event_types() {
            self.handlers
                .entry(event_type)
                .or_insert_with(Vec::new)
                .push(handler.clone());
        }
    }
    
    /// Emit an event
    pub async fn emit(&self, event: Event) -> anyhow::Result<()> {
        let event_type = format!("{:?}", event).split('(').next().unwrap_or("").to_string();
        
        if let Some(handlers) = self.handlers.get(&event_type) {
            for handler in handlers.iter() {
                handler.handle_event(&event).await?;
            }
        }
        
        // Also notify handlers registered for all events
        if let Some(handlers) = self.handlers.get("*") {
            for handler in handlers.iter() {
                handler.handle_event(&event).await?;
            }
        }
        
        Ok(())
    }
    
    /// Queue an event for later processing
    pub fn queue_event(&self, event: Event) {
        self.event_queue.write().push(event);
    }
    
    /// Process queued events
    pub async fn process_queue(&self) -> anyhow::Result<()> {
        let events: Vec<Event> = {
            let mut queue = self.event_queue.write();
            std::mem::take(&mut *queue)
        };
        
        for event in events {
            self.emit(event).await?;
        }
        
        Ok(())
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct TestHandler;
    
    #[async_trait]
    impl EventHandler for TestHandler {
        async fn handle_event(&self, _event: &Event) -> anyhow::Result<()> {
            Ok(())
        }
        
        fn event_types(&self) -> Vec<String> {
            vec!["GameStarted".to_string()]
        }
    }
    
    #[tokio::test]
    async fn test_event_bus() {
        let bus = EventBus::new();
        bus.register_handler(Arc::new(TestHandler));
        
        let result = bus.emit(Event::GameStarted).await;
        assert!(result.is_ok());
    }
}
