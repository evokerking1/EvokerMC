//! Mod API for interacting with the game engine

use evoker_core::{Event, EventBus};
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
}
