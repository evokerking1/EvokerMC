//! Scripting API exposed to scripts

use evoker_core::{Event, EventBus};
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
    
    /// Log a message
    pub fn log(&self, message: &str) {
        log::info!("[Script] {}", message);
    }
    
    /// Emit an event
    pub async fn emit_event(&self, event_type: String, data: serde_json::Value) -> anyhow::Result<()> {
        self.event_bus.emit(Event::Custom { event_type, data }).await
    }
}
