//! WASM mod support

use std::path::Path;
use wasmtime::*;

/// WASM mod loader
pub struct WasmModLoader {
    engine: Engine,
}

impl WasmModLoader {
    /// Create a new WASM mod loader
    pub fn new() -> anyhow::Result<Self> {
        let engine = Engine::default();
        Ok(Self { engine })
    }
    
    /// Load WASM mod
    pub async fn load(&self, path: &Path) -> anyhow::Result<()> {
        log::info!("Loading WASM mod from: {:?}", path);
        
        let module = Module::from_file(&self.engine, path)?;
        let mut store = Store::new(&self.engine, ());
        let _instance = Instance::new(&mut store, &module, &[])?;
        
        log::info!("WASM mod loaded successfully");
        
        Ok(())
    }
}

impl Default for WasmModLoader {
    fn default() -> Self {
        Self::new().expect("Failed to create WASM loader")
    }
}
