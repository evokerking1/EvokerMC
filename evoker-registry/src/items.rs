//! Item definitions loaded from JSON data files

use crate::RegistryKey;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Item properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemProperties {
    /// Maximum stack size
    #[serde(default = "default_stack_size")]
    pub max_stack_size: u32,
    /// Maximum durability (0 = no durability)
    #[serde(default)]
    pub max_durability: u32,
    /// Is item fire resistant
    #[serde(default)]
    pub fire_resistant: bool,
    /// Rarity (common, uncommon, rare, epic)
    #[serde(default = "default_rarity")]
    pub rarity: String,
    /// Custom properties
    #[serde(flatten)]
    pub custom: HashMap<String, serde_json::Value>,
}

fn default_stack_size() -> u32 { 64 }
fn default_rarity() -> String { "common".to_string() }

impl Default for ItemProperties {
    fn default() -> Self {
        Self {
            max_stack_size: 64,
            max_durability: 0,
            fire_resistant: false,
            rarity: "common".to_string(),
            custom: HashMap::new(),
        }
    }
}

/// Item definition loaded from data files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDefinition {
    /// Registry key (namespace:path)
    pub id: String,
    /// Display name
    pub name: String,
    /// Description/lore
    #[serde(default)]
    pub description: Vec<String>,
    /// Item properties
    #[serde(default)]
    pub properties: ItemProperties,
    /// Texture path
    pub texture: String,
    /// Item model (optional)
    #[serde(default)]
    pub model: Option<String>,
    /// Corresponding block (if placeable)
    #[serde(default)]
    pub block: Option<String>,
}

impl ItemDefinition {
    /// Load item definition from JSON
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
    fn test_item_definition_parse() {
        let json = r#"{
            "id": "minecraft:diamond_sword",
            "name": "Diamond Sword",
            "description": ["A powerful weapon"],
            "properties": {
                "max_stack_size": 1,
                "max_durability": 1561,
                "rarity": "rare"
            },
            "texture": "items/diamond_sword"
        }"#;
        
        let item = ItemDefinition::from_json(json).unwrap();
        assert_eq!(item.name, "Diamond Sword");
        assert_eq!(item.properties.max_durability, 1561);
    }
}
