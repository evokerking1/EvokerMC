//! Data pack system for loading game content

use crate::{BlockDefinition, ItemDefinition, EntityDefinition, RecipeDefinition};
use crate::{Registry, RegistryKey};
use std::path::{Path, PathBuf};
use tokio::fs;

/// Data pack structure
pub struct DataPack {
    /// Data pack name
    pub name: String,
    /// Data pack path
    pub path: PathBuf,
    /// Data pack description
    pub description: String,
    /// Pack format version
    pub pack_format: u32,
}

impl DataPack {
    /// Load a data pack from directory
    pub async fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        let meta_path = path.join("pack.mcmeta");
        
        if !meta_path.exists() {
            return Err(anyhow::anyhow!("Not a valid data pack: missing pack.mcmeta"));
        }
        
        let meta_content = fs::read_to_string(&meta_path).await?;
        let meta: PackMeta = serde_json::from_str(&meta_content)?;
        
        Ok(Self {
            name: path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string(),
            path: path.to_path_buf(),
            description: meta.pack.description,
            pack_format: meta.pack.pack_format,
        })
    }
    
    /// Load all blocks from this data pack
    pub async fn load_blocks(&self, registry: &Registry<BlockDefinition>) -> anyhow::Result<usize> {
        let blocks_dir = self.path.join("data").join(&self.name).join("blocks");
        self.load_json_definitions(&blocks_dir, registry).await
    }
    
    /// Load all items from this data pack
    pub async fn load_items(&self, registry: &Registry<ItemDefinition>) -> anyhow::Result<usize> {
        let items_dir = self.path.join("data").join(&self.name).join("items");
        self.load_json_definitions(&items_dir, registry).await
    }
    
    /// Load all entities from this data pack
    pub async fn load_entities(&self, registry: &Registry<EntityDefinition>) -> anyhow::Result<usize> {
        let entities_dir = self.path.join("data").join(&self.name).join("entities");
        self.load_json_definitions(&entities_dir, registry).await
    }
    
    /// Load all recipes from this data pack
    pub async fn load_recipes(&self, registry: &Registry<RecipeDefinition>) -> anyhow::Result<usize> {
        let recipes_dir = self.path.join("data").join(&self.name).join("recipes");
        self.load_json_definitions(&recipes_dir, registry).await
    }
    
    /// Generic loader for JSON definitions
    async fn load_json_definitions<T>(
        &self,
        dir: &Path,
        registry: &Registry<T>,
    ) -> anyhow::Result<usize>
    where
        T: Send + Sync + 'static + serde::de::DeserializeOwned + HasRegistryKey,
    {
        if !dir.exists() {
            return Ok(0);
        }
        
        let mut count = 0;
        let mut entries = fs::read_dir(dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                match self.load_definition::<T>(&path).await {
                    Ok(def) => {
                        let key = def.registry_key()?;
                        registry.register(key, def)?;
                        count += 1;
                    }
                    Err(e) => {
                        log::warn!("Failed to load definition from {:?}: {}", path, e);
                    }
                }
            }
        }
        
        Ok(count)
    }
    
    /// Load a single definition from JSON file
    async fn load_definition<T>(&self, path: &Path) -> anyhow::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let content = fs::read_to_string(path).await?;
        Ok(serde_json::from_str(&content)?)
    }
}

/// Trait for types that have a registry key
pub trait HasRegistryKey {
    fn registry_key(&self) -> anyhow::Result<RegistryKey>;
}

impl HasRegistryKey for BlockDefinition {
    fn registry_key(&self) -> anyhow::Result<RegistryKey> {
        self.key()
    }
}

impl HasRegistryKey for ItemDefinition {
    fn registry_key(&self) -> anyhow::Result<RegistryKey> {
        self.key()
    }
}

impl HasRegistryKey for EntityDefinition {
    fn registry_key(&self) -> anyhow::Result<RegistryKey> {
        self.key()
    }
}

impl HasRegistryKey for RecipeDefinition {
    fn registry_key(&self) -> anyhow::Result<RegistryKey> {
        self.key()
    }
}

/// pack.mcmeta structure
#[derive(Debug, serde::Deserialize)]
struct PackMeta {
    pack: PackInfo,
}

#[derive(Debug, serde::Deserialize)]
struct PackInfo {
    pack_format: u32,
    description: String,
}

/// Data pack loader
pub struct DataPackLoader {
    data_packs_dir: PathBuf,
}

impl DataPackLoader {
    /// Create a new data pack loader
    pub fn new(data_packs_dir: impl Into<PathBuf>) -> Self {
        Self {
            data_packs_dir: data_packs_dir.into(),
        }
    }
    
    /// Discover all data packs in the directory
    pub async fn discover_packs(&self) -> anyhow::Result<Vec<DataPack>> {
        let mut packs = Vec::new();
        
        if !self.data_packs_dir.exists() {
            fs::create_dir_all(&self.data_packs_dir).await?;
            return Ok(packs);
        }
        
        let mut entries = fs::read_dir(&self.data_packs_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                match DataPack::load(&path).await {
                    Ok(pack) => {
                        log::info!("Discovered data pack: {}", pack.name);
                        packs.push(pack);
                    }
                    Err(e) => {
                        log::warn!("Failed to load data pack from {:?}: {}", path, e);
                    }
                }
            }
        }
        
        Ok(packs)
    }
    
    /// Load all data packs into registries
    pub async fn load_all_packs(
        &self,
        blocks: &Registry<BlockDefinition>,
        items: &Registry<ItemDefinition>,
        entities: &Registry<EntityDefinition>,
        recipes: &Registry<RecipeDefinition>,
    ) -> anyhow::Result<()> {
        let packs = self.discover_packs().await?;
        
        for pack in packs {
            log::info!("Loading data pack: {}", pack.name);
            
            let block_count = pack.load_blocks(blocks).await?;
            log::info!("  Loaded {} blocks", block_count);
            
            let item_count = pack.load_items(items).await?;
            log::info!("  Loaded {} items", item_count);
            
            let entity_count = pack.load_entities(entities).await?;
            log::info!("  Loaded {} entities", entity_count);
            
            let recipe_count = pack.load_recipes(recipes).await?;
            log::info!("  Loaded {} recipes", recipe_count);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_datapack_loader() {
        let loader = DataPackLoader::new("data");
        // Test would require actual data pack files
    }
}
