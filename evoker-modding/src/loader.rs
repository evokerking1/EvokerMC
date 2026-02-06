//! Mod loader implementation

use dashmap::DashMap;
use evoker_core::{Event, EventBus};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Mod type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModType {
    /// JAR-based mod (Java)
    Jar,
    /// WASM-based mod
    Wasm,
}

/// Mod metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModInfo {
    /// Unique mod identifier
    pub id: String,
    /// Mod display name
    pub name: String,
    /// Mod version
    pub version: String,
    /// Mod description
    pub description: String,
    /// Mod authors
    pub authors: Vec<String>,
    /// Mod dependencies
    pub dependencies: Vec<String>,
    /// Mod type
    pub mod_type: ModType,
    /// Path to mod file
    pub path: PathBuf,
}

/// Loaded mod instance
pub struct LoadedMod {
    pub info: ModInfo,
    pub enabled: RwLock<bool>,
}

/// Mod loader
pub struct ModLoader {
    mods_dir: PathBuf,
    loaded_mods: Arc<DashMap<String, Arc<LoadedMod>>>,
    event_bus: Arc<EventBus>,
}

impl ModLoader {
    /// Create a new mod loader
    pub fn new(mods_dir: PathBuf, event_bus: Arc<EventBus>) -> anyhow::Result<Self> {
        std::fs::create_dir_all(&mods_dir)?;
        
        Ok(Self {
            mods_dir,
            loaded_mods: Arc::new(DashMap::new()),
            event_bus,
        })
    }
    
    /// Discover mods in the mods directory
    pub async fn discover_mods(&self) -> anyhow::Result<Vec<ModInfo>> {
        let mut mods = Vec::new();
        
        let entries = std::fs::read_dir(&self.mods_dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(mod_info) = self.read_mod_info(&path).await? {
                    mods.push(mod_info);
                }
            }
        }
        
        Ok(mods)
    }
    
    /// Read mod info from file
    async fn read_mod_info(&self, path: &Path) -> anyhow::Result<Option<ModInfo>> {
        let extension = path.extension().and_then(|e| e.to_str());
        
        match extension {
            Some("jar") => {
                // Read JAR mod info
                self.read_jar_mod_info(path).await
            }
            Some("wasm") => {
                // Read WASM mod info
                self.read_wasm_mod_info(path).await
            }
            _ => Ok(None),
        }
    }
    
    /// Read JAR mod info
    async fn read_jar_mod_info(&self, path: &Path) -> anyhow::Result<Option<ModInfo>> {
        use zip::ZipArchive;
        use std::fs::File;
        
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;
        
        // Look for mod.json or META-INF/mod.json
        for name in &["mod.json", "META-INF/mod.json"] {
            if let Ok(mut file) = archive.by_name(name) {
                let mut content = String::new();
                std::io::Read::read_to_string(&mut file, &mut content)?;
                
                let mut info: ModInfo = serde_json::from_str(&content)?;
                info.path = path.to_path_buf();
                info.mod_type = ModType::Jar;
                
                return Ok(Some(info));
            }
        }
        
        Ok(None)
    }
    
    /// Read WASM mod info
    async fn read_wasm_mod_info(&self, path: &Path) -> anyhow::Result<Option<ModInfo>> {
        // Look for accompanying .json file
        let json_path = path.with_extension("wasm.json");
        if json_path.exists() {
            let content = tokio::fs::read_to_string(&json_path).await?;
            let mut info: ModInfo = serde_json::from_str(&content)?;
            info.path = path.to_path_buf();
            info.mod_type = ModType::Wasm;
            return Ok(Some(info));
        }
        
        Ok(None)
    }
    
    /// Load a mod
    pub async fn load_mod(&self, mod_info: ModInfo) -> anyhow::Result<()> {
        log::info!("Loading mod: {} v{}", mod_info.name, mod_info.version);
        
        // Check dependencies
        for dep in &mod_info.dependencies {
            if !self.loaded_mods.contains_key(dep) {
                return Err(anyhow::anyhow!(
                    "Missing dependency: {} for mod {}",
                    dep,
                    mod_info.id
                ));
            }
        }
        
        let loaded_mod = Arc::new(LoadedMod {
            info: mod_info.clone(),
            enabled: RwLock::new(true),
        });
        
        self.loaded_mods.insert(mod_info.id.clone(), loaded_mod);
        
        self.event_bus.queue_event(Event::ModLoaded {
            mod_id: mod_info.id.clone(),
            mod_name: mod_info.name.clone(),
        });
        
        log::info!("Mod loaded: {}", mod_info.name);
        
        Ok(())
    }
    
    /// Unload a mod
    pub async fn unload_mod(&self, mod_id: &str) -> anyhow::Result<()> {
        if let Some((_, mod_)) = self.loaded_mods.remove(mod_id) {
            log::info!("Unloading mod: {}", mod_.info.name);
            
            self.event_bus.queue_event(Event::ModUnloaded {
                mod_id: mod_id.to_string(),
            });
        }
        
        Ok(())
    }
    
    /// Load all discovered mods
    pub async fn load_all_mods(&self) -> anyhow::Result<()> {
        let mods = self.discover_mods().await?;
        
        // Sort by dependencies
        let sorted_mods = self.sort_by_dependencies(mods)?;
        
        for mod_info in sorted_mods {
            if let Err(e) = self.load_mod(mod_info).await {
                log::error!("Failed to load mod: {}", e);
            }
        }
        
        Ok(())
    }
    
    /// Sort mods by dependencies
    fn sort_by_dependencies(&self, mods: Vec<ModInfo>) -> anyhow::Result<Vec<ModInfo>> {
        // Simple topological sort
        let mut sorted = Vec::new();
        let mut remaining = mods;
        
        while !remaining.is_empty() {
            let mut progress = false;
            
            remaining.retain(|mod_info| {
                let deps_satisfied = mod_info.dependencies.iter().all(|dep| {
                    sorted.iter().any(|m: &ModInfo| &m.id == dep)
                });
                
                if deps_satisfied {
                    sorted.push(mod_info.clone());
                    progress = true;
                    false
                } else {
                    true
                }
            });
            
            if !progress && !remaining.is_empty() {
                return Err(anyhow::anyhow!("Circular dependency detected"));
            }
        }
        
        Ok(sorted)
    }
    
    /// Get loaded mod
    pub fn get_mod(&self, mod_id: &str) -> Option<Arc<LoadedMod>> {
        self.loaded_mods.get(mod_id).map(|m| m.clone())
    }
    
    /// Get all loaded mods
    pub fn get_all_mods(&self) -> Vec<Arc<LoadedMod>> {
        self.loaded_mods.iter().map(|m| m.value().clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_mod_loader() {
        let event_bus = Arc::new(EventBus::new());
        let temp_dir = std::env::temp_dir().join("evoker_mods_test");
        
        let loader = ModLoader::new(temp_dir.clone(), event_bus).unwrap();
        
        // Cleanup
        let _ = std::fs::remove_dir_all(temp_dir);
    }
}
