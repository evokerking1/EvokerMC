//! Core registry system for data-driven game objects

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::marker::PhantomData;

/// Registry key for identifying registered objects
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RegistryKey {
    /// Namespace (e.g., "minecraft", "mymod")
    pub namespace: String,
    /// Path (e.g., "stone", "iron_sword")
    pub path: String,
}

impl RegistryKey {
    /// Create a new registry key
    pub fn new(namespace: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            path: path.into(),
        }
    }
    
    /// Parse from string (namespace:path)
    pub fn parse(s: &str) -> anyhow::Result<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid registry key format: {}", s));
        }
        Ok(Self::new(parts[0], parts[1]))
    }
    
    /// Convert to string representation
    pub fn to_string(&self) -> String {
        format!("{}:{}", self.namespace, self.path)
    }
}

impl std::fmt::Display for RegistryKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

/// Generic registry for game objects
pub struct Registry<T> {
    name: String,
    entries: Arc<DashMap<RegistryKey, Arc<T>>>,
    _phantom: PhantomData<T>,
}

impl<T> Registry<T> 
where
    T: Send + Sync + 'static,
{
    /// Create a new registry
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            entries: Arc::new(DashMap::new()),
            _phantom: PhantomData,
        }
    }
    
    /// Register an object
    pub fn register(&self, key: RegistryKey, value: T) -> anyhow::Result<()> {
        if self.entries.contains_key(&key) {
            return Err(anyhow::anyhow!(
                "Registry '{}': Key '{}' already registered",
                self.name,
                key
            ));
        }
        
        log::debug!("Registering {} in registry '{}'", key, self.name);
        self.entries.insert(key, Arc::new(value));
        Ok(())
    }
    
    /// Get a registered object
    pub fn get(&self, key: &RegistryKey) -> Option<Arc<T>> {
        self.entries.get(key).map(|e| e.clone())
    }
    
    /// Check if a key is registered
    pub fn contains(&self, key: &RegistryKey) -> bool {
        self.entries.contains_key(key)
    }
    
    /// Get all registered keys
    pub fn keys(&self) -> Vec<RegistryKey> {
        self.entries.iter().map(|e| e.key().clone()).collect()
    }
    
    /// Get number of registered entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    
    /// Check if registry is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    
    /// Clear all entries
    pub fn clear(&self) {
        self.entries.clear();
    }
    
    /// Get registry name
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[derive(Debug, Clone)]
    struct TestItem {
        name: String,
    }
    
    #[test]
    fn test_registry() {
        let registry = Registry::<TestItem>::new("test");
        let key = RegistryKey::new("test", "item1");
        let item = TestItem {
            name: "Test Item".to_string(),
        };
        
        assert!(registry.register(key.clone(), item).is_ok());
        assert!(registry.contains(&key));
        assert_eq!(registry.len(), 1);
    }
    
    #[test]
    fn test_registry_key_parse() {
        let key = RegistryKey::parse("minecraft:stone").unwrap();
        assert_eq!(key.namespace, "minecraft");
        assert_eq!(key.path, "stone");
    }
}
