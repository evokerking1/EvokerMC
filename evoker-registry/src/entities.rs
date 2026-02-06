//! Entity definitions loaded from JSON data files

use crate::RegistryKey;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Entity attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityAttributes {
    /// Maximum health
    pub max_health: f32,
    /// Movement speed
    pub movement_speed: f32,
    /// Follow range
    #[serde(default = "default_follow_range")]
    pub follow_range: f32,
    /// Attack damage
    #[serde(default)]
    pub attack_damage: f32,
    /// Custom attributes
    #[serde(flatten)]
    pub custom: HashMap<String, serde_json::Value>,
}

fn default_follow_range() -> f32 { 16.0 }

/// Entity AI behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityBehavior {
    /// Behavior type (e.g., "hostile", "passive", "neutral")
    pub behavior_type: String,
    /// AI goals
    #[serde(default)]
    pub goals: Vec<String>,
}

/// Entity definition loaded from data files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDefinition {
    /// Registry key (namespace:path)
    pub id: String,
    /// Display name
    pub name: String,
    /// Entity type (e.g., "mob", "projectile", "item")
    pub entity_type: String,
    /// Entity attributes
    pub attributes: EntityAttributes,
    /// Entity behavior/AI
    #[serde(default)]
    pub behavior: Option<EntityBehavior>,
    /// Model path
    pub model: String,
    /// Loot table
    #[serde(default)]
    pub loot_table: Option<String>,
}

impl EntityDefinition {
    /// Load entity definition from JSON
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
    fn test_entity_definition_parse() {
        let json = r#"{
            "id": "minecraft:zombie",
            "name": "Zombie",
            "entity_type": "mob",
            "attributes": {
                "max_health": 20.0,
                "movement_speed": 0.23,
                "attack_damage": 3.0
            },
            "model": "entities/zombie",
            "loot_table": "minecraft:entities/zombie"
        }"#;
        
        let entity = EntityDefinition::from_json(json).unwrap();
        assert_eq!(entity.name, "Zombie");
        assert_eq!(entity.attributes.max_health, 20.0);
    }
}
