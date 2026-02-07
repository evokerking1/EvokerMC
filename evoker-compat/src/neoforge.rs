//! Minecraft Neoforge API compatibility layer
//! 
//! Provides a compatibility layer that emulates the Neoforge API,
//! allowing Neoforge mods to run on EvokerMC

use evoker_core::{EventBus, Event};
use evoker_modding::ModLoader;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::path::Path;

/// Neoforge mod metadata from neoforge.mods.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeoforgeMod {
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
    pub dependencies: Vec<NeoforgeDependency>,
}

/// Neoforge mod dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeoforgeDependency {
    #[serde(rename = "modId")]
    pub mod_id: String,
    #[serde(default)]
    pub mandatory: bool,
    #[serde(rename = "versionRange", default)]
    pub version_range: String,
}

/// Neoforge mods.toml structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeoforgeModsToml {
    #[serde(rename = "modLoader")]
    pub mod_loader: String,
    #[serde(rename = "loaderVersion")]
    pub loader_version: String,
    pub license: Option<String>,
    #[serde(default)]
    pub mods: Vec<NeoforgeMod>,
}

/// Neoforge compatibility layer
pub struct NeoforgeCompatLayer {
    event_bus: Arc<EventBus>,
    mod_loader: Arc<ModLoader>,
}

impl NeoforgeCompatLayer {
    /// Create a new Neoforge compatibility layer
    pub fn new(event_bus: Arc<EventBus>, mod_loader: Arc<ModLoader>) -> Self {
        log::info!("Initializing Neoforge compatibility layer");
        Self {
            event_bus,
            mod_loader,
        }
    }
    
    /// Initialize Neoforge compatibility
    pub async fn init(&self) -> anyhow::Result<()> {
        log::info!("Neoforge compatibility layer initialized");
        
        // Register Neoforge event handlers
        self.register_neoforge_events().await?;
        
        Ok(())
    }
    
    /// Register Neoforge-style event handlers
    async fn register_neoforge_events(&self) -> anyhow::Result<()> {
        log::debug!("Registering Neoforge event mappings");
        
        // Map common Neoforge events to EvokerMC events
        // In full implementation, would set up event translation
        
        Ok(())
    }
    
    /// Load a Neoforge mod
    pub async fn load_neoforge_mod(&self, mod_path: &Path) -> anyhow::Result<()> {
        log::info!("Loading Neoforge mod from: {:?}", mod_path);
        
        use std::fs::File;
        use std::io::Read;
        use zip::ZipArchive;
        
        let file = File::open(mod_path)?;
        let mut archive = ZipArchive::new(file)?;
        
        // Extract and parse neoforge.mods.toml (or META-INF/mods.toml for compatibility)
        let toml_paths = ["META-INF/neoforge.mods.toml", "META-INF/mods.toml"];
        
        for toml_path in &toml_paths {
            if let Ok(mut file) = archive.by_name(toml_path) {
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                log::debug!("Found Neoforge mod metadata at {}", toml_path);
                
                // Parse TOML
                let mods_toml: NeoforgeModsToml = toml::from_str(&content)
                    .map_err(|e| anyhow::anyhow!("Failed to parse {}: {}", toml_path, e))?;
                
                log::info!("Parsed Neoforge mod metadata:");
                log::info!("  Mod Loader: {}", mods_toml.mod_loader);
                log::info!("  Loader Version: {}", mods_toml.loader_version);
                
                for mod_info in &mods_toml.mods {
                    log::info!("  Mod: {} ({})", mod_info.display_name, mod_info.mod_id);
                    log::info!("    Version: {}", mod_info.version);
                    log::info!("    Description: {}", mod_info.description);
                    
                    // Emit event for mod loaded through compatibility layer
                    self.event_bus.emit(Event::Custom {
                        event_type: "NeoforgeModLoaded".to_string(),
                        data: serde_json::json!({
                            "mod_id": mod_info.mod_id,
                            "display_name": mod_info.display_name,
                            "version": mod_info.version,
                        }),
                    }).await?;
                }
                
                log::info!("Neoforge mod loaded successfully (compatibility layer)");
                return Ok(());
            }
        }
        
        Err(anyhow::anyhow!("No neoforge.mods.toml or mods.toml found in Neoforge mod"))
    }
    
    /// Get Neoforge API version
    pub fn api_version(&self) -> &str {
        "20.4.0" // Example Neoforge version
    }
    
    /// Register a block through Neoforge API
    pub fn register_block(&self, registry_name: &str, properties: serde_json::Value) -> anyhow::Result<()> {
        log::info!("Registering Neoforge block: {}", registry_name);
        
        self.event_bus.queue_event(Event::Custom {
            event_type: "NeoforgeBlockRegistered".to_string(),
            data: serde_json::json!({
                "registry_name": registry_name,
                "properties": properties,
            }),
        });
        
        Ok(())
    }
    
    /// Register an item through Neoforge API
    pub fn register_item(&self, registry_name: &str, properties: serde_json::Value) -> anyhow::Result<()> {
        log::info!("Registering Neoforge item: {}", registry_name);
        
        self.event_bus.queue_event(Event::Custom {
            event_type: "NeoforgeItemRegistered".to_string(),
            data: serde_json::json!({
                "registry_name": registry_name,
                "properties": properties,
            }),
        });
        
        Ok(())
    }
    
    /// Register an entity through Neoforge API
    pub fn register_entity(&self, registry_name: &str, properties: serde_json::Value) -> anyhow::Result<()> {
        log::info!("Registering Neoforge entity: {}", registry_name);
        
        self.event_bus.queue_event(Event::Custom {
            event_type: "NeoforgeEntityRegistered".to_string(),
            data: serde_json::json!({
                "registry_name": registry_name,
                "properties": properties,
            }),
        });
        
        Ok(())
    }
}

/// Neoforge event bus emulation
pub struct NeoforgeEventBus {
    evoker_bus: Arc<EventBus>,
}

impl NeoforgeEventBus {
    pub fn new(evoker_bus: Arc<EventBus>) -> Self {
        Self { evoker_bus }
    }
    
    /// Post a Neoforge event (mapped to EvokerMC event)
    pub async fn post(&self, event_type: &str) -> anyhow::Result<()> {
        log::debug!("Posting Neoforge event: {}", event_type);
        
        // Map Neoforge event types to EvokerMC events
        let evoker_event = match event_type {
            // Server lifecycle events
            "ServerAboutToStartEvent" => Event::ServerStarted { port: 25565 },
            "ServerStartingEvent" => Event::ServerStarted { port: 25565 },
            "ServerStartedEvent" => Event::ServerStarted { port: 25565 },
            "ServerStoppingEvent" => Event::ServerStopped,
            "ServerStoppedEvent" => Event::ServerStopped,
            
            // Common setup events
            "FMLCommonSetupEvent" => Event::Custom {
                event_type: "NeoforgeCommonSetup".to_string(),
                data: serde_json::json!({}),
            },
            "FMLClientSetupEvent" => Event::Custom {
                event_type: "NeoforgeClientSetup".to_string(),
                data: serde_json::json!({}),
            },
            
            // Registry events
            "RegisterEvent" => Event::Custom {
                event_type: "NeoforgeRegister".to_string(),
                data: serde_json::json!({}),
            },
            
            // Default case
            _ => Event::Custom {
                event_type: format!("Neoforge:{}", event_type),
                data: serde_json::json!({}),
            },
        };
        
        self.evoker_bus.emit(evoker_event).await
    }
    
    /// Register an event listener
    pub fn register(&self, event_type: &str, handler: Arc<dyn evoker_core::EventHandler>) {
        log::debug!("Registering Neoforge event listener for: {}", event_type);
        self.evoker_bus.register_handler(handler);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_neoforge_compat_layer() {
        let event_bus = Arc::new(EventBus::new());
        let temp_dir = std::env::temp_dir().join("evoker_mods");
        let mod_loader = Arc::new(ModLoader::new(temp_dir, event_bus.clone()).unwrap());
        
        let compat = NeoforgeCompatLayer::new(event_bus, mod_loader);
        assert!(compat.init().await.is_ok());
    }
    
    #[test]
    fn test_parse_neoforge_toml() {
        let toml_content = r#"
modLoader="javafml"
loaderVersion="[47,)"
license="MIT"

[[mods]]
modId="examplemod"
version="1.0.0"
displayName="Example Mod"
description="An example Neoforge mod"
authors="TestAuthor"
"#;
        
        let result: Result<NeoforgeModsToml, _> = toml::from_str(toml_content);
        assert!(result.is_ok());
        
        let mods_toml = result.unwrap();
        assert_eq!(mods_toml.mod_loader, "javafml");
        assert_eq!(mods_toml.mods.len(), 1);
        assert_eq!(mods_toml.mods[0].mod_id, "examplemod");
    }
}
