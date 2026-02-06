//! Python scripting engine

use pyo3::prelude::*;
use pyo3::types::PyModule;

/// Python script engine
pub struct PythonScriptEngine {
    // Python interpreter is global
}

impl PythonScriptEngine {
    /// Create a new Python engine
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {})
    }
    
    /// Execute Python script
    pub async fn execute(&self, script: &str) -> anyhow::Result<()> {
        Python::with_gil(|py| {
            py.run(script, None, None)?;
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
        Python::with_gil(|py| {
            let _module = PyModule::import(py, name)?;
            Ok::<(), PyErr>(())
        })?;
        Ok(())
    }
}

impl Default for PythonScriptEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create Python engine")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_python_execution() {
        let engine = PythonScriptEngine::new().unwrap();
        let result = engine.execute("x = 1 + 1").await;
        assert!(result.is_ok());
    }
}
