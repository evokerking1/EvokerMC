//! Lua scripting engine

use mlua::{Lua, Result as LuaResult};
use std::sync::Arc;

/// Lua script engine
pub struct LuaScriptEngine {
    lua: Arc<Lua>,
}

impl LuaScriptEngine {
    /// Create a new Lua engine
    pub fn new() -> anyhow::Result<Self> {
        let lua = Lua::new();
        Ok(Self {
            lua: Arc::new(lua),
        })
    }
    
    /// Execute Lua script
    pub async fn execute(&self, script: &str) -> anyhow::Result<()> {
        self.lua.load(script).exec_async().await?;
        Ok(())
    }
    
    /// Execute Lua file
    pub async fn execute_file(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let script = tokio::fs::read_to_string(path).await?;
        self.execute(&script).await
    }
    
    /// Register function
    pub fn register_function<F>(&self, name: &str, func: F) -> anyhow::Result<()>
    where
        F: mlua::Function<'static>,
    {
        self.lua.globals().set(name, func)?;
        Ok(())
    }
}

impl Default for LuaScriptEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create Lua engine")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_lua_execution() {
        let engine = LuaScriptEngine::new().unwrap();
        let result = engine.execute("return 1 + 1").await;
        assert!(result.is_ok());
    }
}
