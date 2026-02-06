//! Block definitions loaded from JSON data files

use crate::RegistryKey;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Block properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockProperties {
    /// Block hardness (mining speed)
    pub hardness: f32,
    /// Explosion resistance
    pub resistance: f32,
    /// Is block solid
    pub solid: bool,
    /// Is block opaque (blocks light)
    pub opaque: bool,
    /// Can be replaced (like grass, water)
    pub replaceable: bool,
    /// Light emission level (0-15)
    #[serde(default)]
    pub light_level: u8,
    /// Custom properties
    #[serde(flatten)]
    pub custom: HashMap<String, serde_json::Value>,
}

impl Default for BlockProperties {
    fn default() -> Self {
        Self {
            hardness: 1.0,
            resistance: 1.0,
            solid: true,
            opaque: true,
            replaceable: false,
            light_level: 0,
            custom: HashMap::new(),
        }
    }
}

/// Block textures
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlockTextures {
    /// Single texture for all faces
    All { all: String },
    /// Different textures for each face
    Faces {
        top: String,
        bottom: String,
        north: String,
        south: String,
        east: String,
        west: String,
    },
}

/// Block definition loaded from data files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDefinition {
    /// Registry key (namespace:path)
    pub id: String,
    /// Display name
    pub name: String,
    /// Block properties
    #[serde(default)]
    pub properties: BlockProperties,
    /// Block textures
    pub textures: BlockTextures,
    /// Loot table
    #[serde(default)]
    pub loot_table: Option<String>,
    /// Block model (optional, for custom models)
    #[serde(default)]
    pub model: Option<String>,
}

impl BlockDefinition {
    /// Load block definition from JSON
    pub fn from_json(json: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(json)?)
    }
    
    /// Get registry key
    pub fn key(&self) -> anyhow::Result<RegistryKey> {
        RegistryKey::parse(&self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_block_definition_parse() {
        let json = r#"{
            "id": "minecraft:stone",
            "name": "Stone",
            "properties": {
                "hardness": 1.5,
                "resistance": 6.0,
                "solid": true,
                "opaque": true
            },
            "textures": {
                "all": "blocks/stone"
            }
        }"#;
        
        let block = BlockDefinition::from_json(json).unwrap();
        assert_eq!(block.name, "Stone");
        assert_eq!(block.properties.hardness, 1.5);
    }
}
