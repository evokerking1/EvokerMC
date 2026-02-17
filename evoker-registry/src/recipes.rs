//! Recipe definitions loaded from JSON data files

use crate::RegistryKey;
use serde::{Deserialize, Serialize};

/// Recipe ingredient
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Ingredient {
    /// Single item
    Item { item: String },
    /// Tag (group of items)
    Tag { tag: String },
    /// Multiple possible items
    Items { items: Vec<String> },
}

/// Recipe result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeResult {
    /// Item ID
    pub item: String,
    /// Count
    #[serde(default = "default_count")]
    pub count: u32,
}

fn default_count() -> u32 { 1 }

/// Recipe type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RecipeType {
    /// Crafting recipe (shaped or shapeless)
    #[serde(rename = "minecraft:crafting_shaped")]
    CraftingShaped {
        pattern: Vec<String>,
        key: std::collections::HashMap<char, Ingredient>,
        result: RecipeResult,
    },
    #[serde(rename = "minecraft:crafting_shapeless")]
    CraftingShapeless {
        ingredients: Vec<Ingredient>,
        result: RecipeResult,
    },
    /// Smelting recipe
    #[serde(rename = "minecraft:smelting")]
    Smelting {
        ingredient: Ingredient,
        result: String,
        experience: f32,
        #[serde(rename = "cookingtime")]
        cooking_time: u32,
    },
    /// Smoking recipe
    #[serde(rename = "minecraft:smoking")]
    Smoking {
        ingredient: Ingredient,
        result: String,
        experience: f32,
        #[serde(rename = "cookingtime")]
        cooking_time: u32,
    },
    /// Blasting recipe
    #[serde(rename = "minecraft:blasting")]
    Blasting {
        ingredient: Ingredient,
        result: String,
        experience: f32,
        #[serde(rename = "cookingtime")]
        cooking_time: u32,
    },
}

/// Recipe definition loaded from data files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeDefinition {
    /// Recipe ID
    pub id: String,
    /// Recipe data
    #[serde(flatten)]
    pub recipe: RecipeType,
}

impl RecipeDefinition {
    /// Load recipe definition from JSON
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
    fn test_shaped_recipe_parse() {
        // Use format! with r## to avoid Rust 2021 prefix syntax issues in rustdoc
        let json = format!(
            r##"{{
    "id": "minecraft{}stick",
    "type": "minecraft{}crafting_shaped",
    "pattern": [
        "#",
        "#"
    ],
    "key": {{
        "#": {{ "item": "minecraft{}oak_planks" }}
    }},
    "result": {{
        "item": "minecraft{}stick",
        "count": 4
    }}
}}"##,
            ":", ":", ":", ":"
        );
        
        let recipe = RecipeDefinition::from_json(&json).unwrap();
        assert_eq!(recipe.id, "minecraft:stick");
    }
}
