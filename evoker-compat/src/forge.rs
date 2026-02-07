//! Minecraft Forge API compatibility layer
//! 
//! Provides a compatibility layer that emulates the Forge API,
//! allowing Forge mods to run on EvokerMC

use evoker_core::{EventBus, Event};
use evoker_modding::ModLoader;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::path::Path;

/// Forge mod metadata from mods.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeMod {
    #[serde(rename = "modId")]
    pub mod_id: String,
    pub version: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub authors: String,
    #[serde(default)]
    pub dependencies: Vec<ForgeDependency>,
}

/// Forge mod dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeDependency {
    #[serde(rename = "modId")]
    pub mod_id: String,
    #[serde(default)]
    pub mandatory: bool,
    #[serde(rename = "versionRange", default)]
    pub version_range: String,
}

/// Forge mods.toml structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeModsToml {
    #[serde(rename = "modLoader")]
    pub mod_loader: String,
    #[serde(rename = "loaderVersion")]
    pub loader_version: String,
    pub license: Option<String>,
    #[serde(default)]
    pub mods: Vec<ForgeMod>,
}

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
        log::debug!("Registering Forge event mappings");
        
        // Map common Forge events to EvokerMC events
        // In full implementation, would set up event translation
        
        Ok(())
    }
    
    /// Load a Forge mod
    pub async fn load_forge_mod(&self, mod_path: &Path) -> anyhow::Result<()> {
        log::info!("Loading Forge mod from: {:?}", mod_path);
        
        use std::fs::File;
        use std::io::Read;
        use zip::ZipArchive;
        
        let file = File::open(mod_path)?;
        let mut archive = ZipArchive::new(file)?;
        
        // Extract and parse mods.toml
        if let Ok(mut file) = archive.by_name("META-INF/mods.toml") {
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            log::debug!("Found Forge mod metadata");
            
            // Parse TOML
            let mods_toml: ForgeModsToml = toml::from_str(&content)
                .map_err(|e| anyhow::anyhow!("Failed to parse mods.toml: {}", e))?;
            
            log::info!("Parsed Forge mod metadata:");
            log::info!("  Mod Loader: {}", mods_toml.mod_loader);
            log::info!("  Loader Version: {}", mods_toml.loader_version);
            
            for mod_info in &mods_toml.mods {
                log::info!("  Mod: {} ({})", mod_info.display_name, mod_info.mod_id);
                log::info!("    Version: {}", mod_info.version);
                log::info!("    Description: {}", mod_info.description);
                
                // Emit event for mod loaded through compatibility layer
                self.event_bus.emit(Event::Custom {
                    event_type: "ForgeModLoaded".to_string(),
                    data: serde_json::json!({
                        "mod_id": mod_info.mod_id,
                        "display_name": mod_info.display_name,
                        "version": mod_info.version,
                    }),
                }).await?;
            }
            
            log::info!("Forge mod loaded successfully (compatibility layer)");
        } else {
            return Err(anyhow::anyhow!("No mods.toml found in Forge mod"));
        }
        
        Ok(())
    }
    
    /// Get Forge API version
    pub fn api_version(&self) -> &str {
        "1.20.1-47.2.0" // Example Forge version
    }
    
    /// Register a block through Forge API
    pub fn register_block(&self, registry_name: &str, properties: serde_json::Value) -> anyhow::Result<()> {
        log::info!("Registering Forge block: {}", registry_name);
        
        self.event_bus.queue_event(Event::Custom {
            event_type: "ForgeBlockRegistered".to_string(),
            data: serde_json::json!({
                "registry_name": registry_name,
                "properties": properties,
            }),
        });
        
        Ok(())
    }
    
    /// Register an item through Forge API
    pub fn register_item(&self, registry_name: &str, properties: serde_json::Value) -> anyhow::Result<()> {
        log::info!("Registering Forge item: {}", registry_name);
        
        self.event_bus.queue_event(Event::Custom {
            event_type: "ForgeItemRegistered".to_string(),
            data: serde_json::json!({
                "registry_name": registry_name,
                "properties": properties,
            }),
        });
        
        Ok(())
    }
    
    /// Register an entity through Forge API
    pub fn register_entity(&self, registry_name: &str, properties: serde_json::Value) -> anyhow::Result<()> {
        log::info!("Registering Forge entity: {}", registry_name);
        
        self.event_bus.queue_event(Event::Custom {
            event_type: "ForgeEntityRegistered".to_string(),
            data: serde_json::json!({
                "registry_name": registry_name,
                "properties": properties,
            }),
        });
        
        Ok(())
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
        log::debug!("Posting Forge event: {}", event_type);
        
        // Map Forge event types to EvokerMC events
        let evoker_event = match event_type {
            // Server lifecycle events
            "ServerStartingEvent" => Event::ServerStarted { port: 25565 },
            "ServerStartedEvent" => Event::ServerStarted { port: 25565 },
            "ServerStoppingEvent" => Event::ServerStopped,
            "ServerStoppedEvent" => Event::ServerStopped,
            
            // Common setup events
            "FMLCommonSetupEvent" => Event::Custom {
                event_type: "ForgeCommonSetup".to_string(),
                data: serde_json::json!({}),
            },
            "FMLClientSetupEvent" => Event::Custom {
                event_type: "ForgeClientSetup".to_string(),
                data: serde_json::json!({}),
            },
            
            // Registry events
            "RegisterEvent" => Event::Custom {
                event_type: "ForgeRegister".to_string(),
                data: serde_json::json!({}),
            },
            
            // Default case
            _ => Event::Custom {
                event_type: format!("Forge:{}", event_type),
                data: serde_json::json!({}),
            },
        };
        
        self.evoker_bus.emit(evoker_event).await
    }
    
    /// Register an event listener
    pub fn register(&self, event_type: &str, handler: Arc<dyn evoker_core::EventHandler>) {
        log::debug!("Registering Forge event listener for: {}", event_type);
        self.evoker_bus.register_handler(handler);
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
    
    #[test]
    fn test_parse_forge_toml() {
        let toml_content = r#"
modLoader="javafml"
loaderVersion="[47,)"
license="MIT"

[[mods]]
modId="examplemod"
version="1.0.0"
displayName="Example Mod"
description="An example Forge mod"
authors="TestAuthor"
"#;
        
        let result: Result<ForgeModsToml, _> = toml::from_str(toml_content);
        assert!(result.is_ok());
        
        let mods_toml = result.unwrap();
        assert_eq!(mods_toml.mod_loader, "javafml");
        assert_eq!(mods_toml.mods.len(), 1);
        assert_eq!(mods_toml.mods[0].mod_id, "examplemod");
    }
}
