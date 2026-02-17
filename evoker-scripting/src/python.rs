//! Python scripting engine

use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::sync::Arc;
use std::ffi::CString;
use crate::api::ScriptApi;

/// Python script engine
pub struct PythonScriptEngine {
    script_api: Arc<ScriptApi>,
}

impl PythonScriptEngine {
    /// Create a new Python engine
    pub fn new(script_api: Arc<ScriptApi>) -> anyhow::Result<Self> {
        Ok(Self { script_api })
    }
    
    /// Execute Python script
    pub async fn execute(&self, script: &str) -> anyhow::Result<()> {
        Python::attach(|py| {
            // Create a game module with API bindings
            let game_module = PyModule::new(py, "game")?;
            
            // Add logging functions as module functions
            game_module.add_function(wrap_pyfunction!(py_log, &game_module)?)?;
            game_module.add_function(wrap_pyfunction!(py_debug, &game_module)?)?;
            game_module.add_function(wrap_pyfunction!(py_warn, &game_module)?)?;
            game_module.add_function(wrap_pyfunction!(py_error, &game_module)?)?;
            
            // Add to sys.modules so it can be imported
            let sys = PyModule::import(py, "sys")?;
            let sys_modules = sys.getattr("modules")?;
            sys_modules.set_item("game", game_module)?;
            
            // Execute the script
            // Convert the script to a CString for the new PyO3 API
            let code_cstr = CString::new(script).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Script contains null byte: {}", e)))?;
            py.run(code_cstr.as_c_str(), None, None)?;
            Ok::<(), PyErr>(())
        })?;
        
        Ok(())
    }
    
    /// Execute Python file
    pub async fn execute_file(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let script = tokio::fs::read_to_string(path).await?;
        self.execute(&script).await
    }
    
    /// Import and execute Python module
    pub fn import_module(&self, name: &str) -> anyhow::Result<()> {
        Python::attach(|py| {
            let _module = PyModule::import(py, name)?;
            Ok::<(), PyErr>(())
        })?;
        Ok(())
    }
}

// Python wrapper functions for the ScriptApi
// Note: These are simplified - in a full implementation, they would
// properly access the ScriptApi through thread-local storage or similar mechanism

#[pyfunction]
fn py_log(msg: String) {
    log::info!("[Script] {}", msg);
}

#[pyfunction]
fn py_debug(msg: String) {
    log::debug!("[Script] {}", msg);
}

#[pyfunction]
fn py_warn(msg: String) {
    log::warn!("[Script] {}", msg);
}

#[pyfunction]
fn py_error(msg: String) {
    log::error!("[Script] {}", msg);
}

#[cfg(test)]
mod tests {
    use super::*;
    use evoker_core::EventBus;
    
    #[tokio::test]
    async fn test_python_execution() {
        let event_bus = Arc::new(EventBus::new());
        let script_api = Arc::new(ScriptApi::new(event_bus));
        let engine = PythonScriptEngine::new(script_api).unwrap();
        let result = engine.execute("x = 1 + 1").await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_python_api_access() {
        let event_bus = Arc::new(EventBus::new());
        let script_api = Arc::new(ScriptApi::new(event_bus));
        let engine = PythonScriptEngine::new(script_api).unwrap();
        let script = r#"
import game
game.py_log("Hello from Python!")
"#;
        let result = engine.execute(script).await;
        assert!(result.is_ok());
    }
}
