//! JAR mod support
//! 
//! Loads Java-based mods from JAR files using JNI integration.

use crate::jvm::{JvmManager, JavaMod};
use std::path::Path;
use std::fs::File;
use std::io::Read;
use zip::ZipArchive;

/// JAR mod loader with JNI integration
pub struct JarModLoader {
    // Stores loaded JAR metadata
    loaded_jars: Vec<JarModMetadata>,
    // Store Java mod instances
    java_mods: Vec<JavaMod>,
}

/// Metadata extracted from JAR
#[derive(Debug, Clone)]
pub struct JarModMetadata {
    pub name: String,
    pub version: String,
    pub main_class: Option<String>,
    pub dependencies: Vec<String>,
}

impl JarModLoader {
    pub fn new() -> Self {
        Self {
            loaded_jars: Vec::new(),
            java_mods: Vec::new(),
        }
    }
    
    /// Load JAR mod with full JNI integration
    pub async fn load(&mut self, path: &Path) -> anyhow::Result<()> {
        log::info!("Loading JAR mod from: {:?}", path);
        
        // Open JAR file as ZIP
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;
        
        // Extract metadata from mod.json or META-INF/MANIFEST.MF
        let metadata = self.extract_metadata(&mut archive)?;
        
        log::info!("Loaded JAR mod metadata: {} v{}", metadata.name, metadata.version);
        
        // Try to initialize JVM and load the mod
        // If JVM is not available, just store the metadata
        if let Some(jvm) = JvmManager::get() {
            // Load the JAR into the JVM classpath
            jvm.load_jar(path)?;
            
            // If there's a main class, instantiate it
            if let Some(ref main_class) = metadata.main_class {
                log::info!("Instantiating main class: {}", main_class);
                
                // Convert Java class name format (com.example.Mod) to JNI format (com/example/Mod)
                let jni_class_name = main_class.replace('.', "/");
                
                match jvm.new_instance(&jni_class_name) {
                    Ok(instance) => {
                        let java_mod = JavaMod::new(main_class.clone(), instance);
                        
                        // Initialize the mod
                        java_mod.init()?;
                        
                        self.java_mods.push(java_mod);
                        log::info!("Java mod instantiated and initialized: {}", main_class);
                    }
                    Err(e) => {
                        log::error!("Failed to instantiate mod class {}: {}", main_class, e);
                        return Err(e);
                    }
                }
            } else {
                log::warn!("No main class specified in JAR metadata");
            }
        } else {
            log::warn!("JVM not initialized - JAR mod loaded but not instantiated");
            log::warn!("To use Java mods, ensure a JDK is installed and initialize the JVM");
        }
        
        self.loaded_jars.push(metadata);
        
        Ok(())
    }
    
    /// Extract metadata from JAR
    fn extract_metadata(&self, archive: &mut ZipArchive<File>) -> anyhow::Result<JarModMetadata> {
        // Try to find mod.json
        if let Ok(mut file) = archive.by_name("mod.json") {
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            
            let json: serde_json::Value = serde_json::from_str(&content)?;
            
            return Ok(JarModMetadata {
                name: json["name"].as_str().unwrap_or("Unknown").to_string(),
                version: json["version"].as_str().unwrap_or("0.0.0").to_string(),
                main_class: json["main"].as_str().map(|s| s.to_string()),
                dependencies: json["dependencies"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default(),
            });
        }
        
        // Fallback: extract from manifest
        if let Ok(mut file) = archive.by_name("META-INF/MANIFEST.MF") {
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            
            let mut name = "Unknown".to_string();
            let mut version = "0.0.0".to_string();
            let mut main_class = None;
            
            for line in content.lines() {
                if let Some(value) = line.strip_prefix("Implementation-Title: ") {
                    name = value.trim().to_string();
                } else if let Some(value) = line.strip_prefix("Implementation-Version: ") {
                    version = value.trim().to_string();
                } else if let Some(value) = line.strip_prefix("Main-Class: ") {
                    main_class = Some(value.trim().to_string());
                }
            }
            
            return Ok(JarModMetadata {
                name,
                version,
                main_class,
                dependencies: Vec::new(),
            });
        }
        
        // No metadata found
        Err(anyhow::anyhow!("No mod metadata found in JAR"))
    }
    
    /// Get list of loaded JAR mods
    pub fn loaded_mods(&self) -> &[JarModMetadata] {
        &self.loaded_jars
    }
    
    /// Get Java mod instances
    pub fn java_mods(&self) -> &[JavaMod] {
        &self.java_mods
    }
}

impl Default for JarModLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_jar_loader_creation() {
        let loader = JarModLoader::new();
        assert_eq!(loader.loaded_mods().len(), 0);
        assert_eq!(loader.java_mods().len(), 0);
    }
}
