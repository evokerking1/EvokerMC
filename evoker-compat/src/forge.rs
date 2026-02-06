//! Minecraft Forge API compatibility layer
//! 
//! Provides a compatibility layer that emulates the Forge API,
//! allowing Forge mods to run on EvokerMC

use evoker_core::{EventBus, Event};
use evoker_modding::ModLoader;
use std::sync::Arc;

/// Forge compatibility layer
pub struct ForgeCompatLayer {
    event_bus: Arc<EventBus>,
    mod_loader: Arc<ModLoader>,
}

impl ForgeCompatLayer {
    /// Create a new Forge compatibility layer
    pub fn new(event_bus: Arc<EventBus>, mod_loader: Arc<ModLoader>) -> Self {
        log::info!("Initializing Forge compatibility layer");
        Self {
            event_bus,
            mod_loader,
        }
    }
    
    /// Initialize Forge compatibility
    pub async fn init(&self) -> anyhow::Result<()> {
        log::info!("Forge compatibility layer initialized");
        
        // Register Forge event handlers
        self.register_forge_events().await?;
        
        Ok(())
    }
    
    /// Register Forge-style event handlers
    async fn register_forge_events(&self) -> anyhow::Result<()> {
        // Map Forge events to EvokerMC events
        log::debug!("Registering Forge event mappings");
        Ok(())
    }
    
    /// Load a Forge mod
    pub async fn load_forge_mod(&self, mod_path: &std::path::Path) -> anyhow::Result<()> {
        log::info!("Loading Forge mod from: {:?}", mod_path);
        
        // In a real implementation, this would:
        // 1. Parse the Forge mod metadata (mods.toml)
        // 2. Load the mod JAR
        // 3. Map Forge API calls to EvokerMC equivalents
        // 4. Initialize the mod
        
        Ok(())
    }
    
    /// Get Forge API version
    pub fn api_version(&self) -> &str {
        "1.20.1-47.2.0" // Example Forge version
    }
}

/// Forge event bus emulation
pub struct ForgeEventBus {
    evoker_bus: Arc<EventBus>,
}

impl ForgeEventBus {
    pub fn new(evoker_bus: Arc<EventBus>) -> Self {
        Self { evoker_bus }
    }
    
    /// Post a Forge event (mapped to EvokerMC event)
    pub async fn post(&self, event_type: &str) -> anyhow::Result<()> {
        // Map Forge event types to EvokerMC events
        let evoker_event = match event_type {
            "ServerStartingEvent" => Event::ServerStarted { port: 25565 },
            "ServerStoppedEvent" => Event::ServerStopped,
            _ => Event::Custom {
                event_type: event_type.to_string(),
                data: serde_json::json!({}),
            },
        };
        
        self.evoker_bus.emit(evoker_event).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_forge_compat_layer() {
        let event_bus = Arc::new(EventBus::new());
        let temp_dir = std::env::temp_dir().join("evoker_mods");
        let mod_loader = Arc::new(ModLoader::new(temp_dir, event_bus.clone()).unwrap());
        
        let compat = ForgeCompatLayer::new(event_bus, mod_loader);
        assert!(compat.init().await.is_ok());
    }
}
