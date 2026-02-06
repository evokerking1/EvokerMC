//! Scripting engine supporting Lua and Python
//! 
//! Allows mods to use scripting languages for easier mod development

pub mod lua;
pub mod python;
pub mod api;

pub use lua::LuaScriptEngine;
pub use python::PythonScriptEngine;
pub use api::ScriptApi;

/// Script engine type
#[derive(Debug, Clone, Copy)]
pub enum ScriptEngineType {
    Lua,
    Python,
}
