//! Lua scripting engine

use mlua::{Lua, Function, UserData, UserDataMethods};
use std::sync::Arc;
use crate::api::ScriptApi;

/// Lua script engine
pub struct LuaScriptEngine {
    lua: Arc<Lua>,
    script_api: Arc<ScriptApi>,
}

impl LuaScriptEngine {
    /// Create a new Lua engine
    pub fn new(script_api: Arc<ScriptApi>) -> anyhow::Result<Self> {
        let lua = Lua::new();
        
        let engine = Self {
            lua: Arc::new(lua),
            script_api,
        };
        
        // Register the script API
        engine.register_api()?;
        
        Ok(engine)
    }
    
    /// Register script API with Lua
    fn register_api(&self) -> anyhow::Result<()> {
        let api = self.script_api.clone();
        
        // Create a global 'game' table with API methods
        self.lua.globals().set("game", LuaGameApi { api })?;
        
        Ok(())
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
    
    /// Register Rust function callable from Lua
    pub fn register_function<'a>(&self, name: &str, func: Function<'a>) -> anyhow::Result<()> {
        self.lua.globals().set(name, func)?;
        Ok(())
    }
}

/// Lua wrapper for ScriptApi
struct LuaGameApi {
    api: Arc<ScriptApi>,
}

impl UserData for LuaGameApi {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        // Logging methods
        methods.add_method("log", |_, this, msg: String| {
            this.api.log(&msg);
            Ok(())
        });
        
        methods.add_method("debug", |_, this, msg: String| {
            this.api.debug(&msg);
            Ok(())
        });
        
        methods.add_method("warn", |_, this, msg: String| {
            this.api.warn(&msg);
            Ok(())
        });
        
        methods.add_method("error", |_, this, msg: String| {
            this.api.error(&msg);
            Ok(())
        });
        
        // Block methods
        methods.add_async_method("setBlock", |_, this, (world, x, y, z, block_id): (String, i32, i32, i32, String)| async move {
            this.api.set_block(&world, x, y, z, &block_id).await
                .map_err(|e| mlua::Error::external(e))
        });
        
        methods.add_method("getBlock", |_, this, (world, x, y, z): (String, i32, i32, i32)| {
            this.api.get_block(&world, x, y, z)
                .map_err(|e| mlua::Error::external(e))
        });
        
        methods.add_async_method("fillBlocks", |_, this, 
            (world, x1, y1, z1, x2, y2, z2, block_id): (String, i32, i32, i32, i32, i32, i32, String)| async move {
            this.api.fill_blocks(&world, x1, y1, z1, x2, y2, z2, &block_id).await
                .map_err(|e| mlua::Error::external(e))
        });
        
        // Entity methods
        methods.add_async_method("spawnEntity", |_, this, (world, entity_id, x, y, z): (String, String, f64, f64, f64)| async move {
            this.api.spawn_entity(&world, &entity_id, x, y, z).await
                .map_err(|e| mlua::Error::external(e))
        });
        
        methods.add_async_method("removeEntity", |_, this, entity_uuid: String| async move {
            this.api.remove_entity(&entity_uuid).await
                .map_err(|e| mlua::Error::external(e))
        });
        
        // Player methods
        methods.add_method("getPlayers", |_, this, ()| {
            Ok(this.api.get_players())
        });
        
        methods.add_async_method("sendMessage", |_, this, (player_id, message): (String, String)| async move {
            this.api.send_message(&player_id, &message).await
                .map_err(|e| mlua::Error::external(e))
        });
        
        methods.add_async_method("broadcast", |_, this, message: String| async move {
            this.api.broadcast(&message).await
                .map_err(|e| mlua::Error::external(e))
        });
        
        methods.add_async_method("teleport", |_, this, (player_id, world, x, y, z): (String, String, f64, f64, f64)| async move {
            this.api.teleport(&player_id, &world, x, y, z).await
                .map_err(|e| mlua::Error::external(e))
        });
        
        // Item methods
        methods.add_async_method("giveItem", |_, this, (player_id, item_id, count): (String, String, u32)| async move {
            this.api.give_item(&player_id, &item_id, count).await
                .map_err(|e| mlua::Error::external(e))
        });
        
        methods.add_async_method("removeItem", |_, this, (player_id, item_id, count): (String, String, u32)| async move {
            this.api.remove_item(&player_id, &item_id, count).await
                .map_err(|e| mlua::Error::external(e))
        });
        
        // World methods
        methods.add_method("getWorlds", |_, this, ()| {
            Ok(this.api.get_worlds())
        });
        
        methods.add_method("getTime", |_, this, world: String| {
            Ok(this.api.get_time(&world))
        });
        
        methods.add_async_method("setTime", |_, this, (world, time): (String, i64)| async move {
            this.api.set_time(&world, time).await
                .map_err(|e| mlua::Error::external(e))
        });
        
        methods.add_method("getWeather", |_, this, world: String| {
            Ok(this.api.get_weather(&world))
        });
        
        methods.add_async_method("setWeather", |_, this, (world, weather): (String, String)| async move {
            this.api.set_weather(&world, &weather).await
                .map_err(|e| mlua::Error::external(e))
        });
        
        // Command execution
        methods.add_async_method("executeCommand", |_, this, command: String| async move {
            this.api.execute_command(&command).await
                .map_err(|e| mlua::Error::external(e))
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use evoker_core::EventBus;
    
    #[tokio::test]
    async fn test_lua_execution() {
        let event_bus = Arc::new(EventBus::new());
        let script_api = Arc::new(ScriptApi::new(event_bus));
        let engine = LuaScriptEngine::new(script_api).unwrap();
        let result = engine.execute("game:log('Hello from Lua!')").await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_lua_api_access() {
        let event_bus = Arc::new(EventBus::new());
        let script_api = Arc::new(ScriptApi::new(event_bus));
        let engine = LuaScriptEngine::new(script_api).unwrap();
        let script = r#"
            game:log("Testing Lua API")
            local worlds = game:getWorlds()
        "#;
        let result = engine.execute(script).await;
        assert!(result.is_ok());
    }
}
