//! JAR mod support
//! 
//! Loads Java-based mods from JAR files. Uses metadata extraction
//! for mod discovery. Full JVM integration would require JNI.

use std::path::Path;
use std::fs::File;
use std::io::Read;
use zip::ZipArchive;

/// JAR mod loader
pub struct JarModLoader {
    // Stores loaded JAR metadata
    loaded_jars: Vec<JarModMetadata>,
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
        }
    }
    
    /// Load JAR mod
    pub async fn load(&mut self, path: &Path) -> anyhow::Result<()> {
        log::info!("Loading JAR mod from: {:?}", path);
        
        // Open JAR file as ZIP
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;
        
        // Extract metadata from mod.json or META-INF/MANIFEST.MF
        let metadata = self.extract_metadata(&mut archive)?;
        
        log::info!("Loaded JAR mod: {} v{}", metadata.name, metadata.version);
        
        self.loaded_jars.push(metadata);
        
        // Note: Full JVM integration would require:
        // 1. Initialize JVM using jni crate
        // 2. Load JAR classes into JVM
        // 3. Instantiate mod main class
        // 4. Call mod initialization methods
        // This is a foundation for future JNI integration
        
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
    }
}
