//! EvokerMC - A highly moddable 3D block game engine
//! 
//! # Architecture
//! 
//! EvokerMC uses a server-authoritative architecture where:
//! - **Singleplayer**: Uses an integrated server running locally
//! - **Multiplayer**: Connects to a dedicated server
//! - **Server controls everything**: The server is the source of truth for all game state
//! 
//! # Features
//! 
//! - World management (save/load/quit)
//! - Main menu system
//! - Multiplayer networking
//! - Mod loading (JAR and WASM)
//! - Scripting (Lua and Python)
//! - Forge/Neoforge compatibility
//! - In-game asset editor
//! 
//! # Modules
//! 
//! - `evoker-core`: Core game engine and event system
//! - `evoker-world`: World management and chunk system
//! - `evoker-network`: Multiplayer networking
//! - `evoker-modding`: Mod loading system
//! - `evoker-scripting`: Lua and Python scripting
//! - `evoker-ui`: User interface and menus
//! - `evoker-assets`: Asset management and editor
//! - `evoker-compat`: Forge/Neoforge compatibility

use evoker_core::{Config, EventBus, Game, GameState};
use evoker_network::{Client, Server};
use evoker_world::World;
use evoker_modding::ModLoader;
use evoker_ui::MainMenu;
use evoker_assets::AssetManager;
use evoker_compat::{ForgeCompatLayer, NeoforgeCompatLayer};
use std::sync::Arc;
use std::path::PathBuf;

/// Application state
pub struct App {
    config: Config,
    event_bus: Arc<EventBus>,
    game: Arc<Game>,
    client: Option<Arc<Client>>,
    integrated_server: Option<Arc<Server>>,
    world: Option<Arc<World>>,
    mod_loader: Arc<ModLoader>,
    asset_manager: Arc<AssetManager>,
    main_menu: Option<MainMenu>,
}

impl App {
    /// Create a new application
    pub async fn new() -> anyhow::Result<Self> {
        // Initialize core
        evoker_core::init().await?;
        
        // Load configuration
        let config = Config::default();
        
        // Create event bus
        let event_bus: Arc<EventBus> = Arc::new(EventBus::new());
        
        // Create game instance
        let game: Arc<Game> = Arc::new(Game::new(event_bus.clone()));
        
        // Initialize mod loader
        let mod_loader: Arc<ModLoader> = Arc::new(ModLoader::new(
            config.mods.mods_dir.clone(),
            event_bus.clone(),
        )?);
        
        // Initialize asset manager
        let asset_manager: Arc<AssetManager> = Arc::new(AssetManager::new(
            PathBuf::from("assets"),
            event_bus.clone(),
        )?);
        
        // Create main menu
        let main_menu = Some(MainMenu::new(event_bus.clone()));
        
        Ok(Self {
            config,
            event_bus,
            game,
            client: None,
            integrated_server: None,
            world: None,
            mod_loader,
            asset_manager,
            main_menu,
        })
    }
    
    /// Start singleplayer with integrated server
    pub async fn start_singleplayer(&mut self, world_name: String) -> anyhow::Result<()> {
        log::info!("Starting singleplayer with integrated server");
        
        // Start integrated server on random port
        let server = Arc::new(Server::new(
            0, // Random port for integrated server
            1, // Single player
            self.event_bus.clone(),
        ));
        server.start().await?;
        
        // Get the actual address the server is listening on
        let server_addr = server.local_addr()
            .ok_or_else(|| anyhow::anyhow!("Server did not bind to an address"))?;
        
        log::info!("Integrated server started on {}", server_addr);
        
        self.integrated_server = Some(server.clone());
        
        // Create client and connect to integrated server
        let client = Arc::new(Client::new(self.event_bus.clone()));
        client.connect(server_addr.to_string()).await?;
        self.client = Some(client);
        
        // Load or create world
        let world_path = PathBuf::from("worlds");
        let world = Arc::new(World::load(
            world_name.clone(),
            world_path.clone(),
            self.event_bus.clone(),
        ).await.or_else(|_| {
            // Create new world if load fails
            World::new(
                world_name,
                12345, // Default seed
                world_path,
                self.event_bus.clone(),
            )
        })?);
        self.world = Some(world);
        
        // Hide main menu
        self.main_menu = None;
        
        // Start game
        self.game.set_state(GameState::Running);
        
        log::info!("Singleplayer started with integrated server");
        
        Ok(())
    }
    
    /// Connect to multiplayer server
    pub async fn connect_multiplayer(&mut self, server_address: String) -> anyhow::Result<()> {
        log::info!("Connecting to multiplayer server: {}", server_address);
        
        // Create client and connect
        let client = Arc::new(Client::new(self.event_bus.clone()));
        client.connect(server_address).await?;
        self.client = Some(client);
        
        // Hide main menu
        self.main_menu = None;
        
        // Start game
        self.game.set_state(GameState::Running);
        
        log::info!("Connected to multiplayer server");
        
        Ok(())
    }
    
    /// Stop and return to main menu
    pub async fn quit_to_menu(&mut self) -> anyhow::Result<()> {
        log::info!("Quitting to main menu");
        
        // Save and unload world
        if let Some(world) = &self.world {
            world.save().await?;
            world.unload().await?;
        }
        self.world = None;
        
        // Disconnect client
        if let Some(client) = &self.client {
            client.disconnect().await?;
        }
        self.client = None;
        
        // Stop integrated server if running
        if let Some(server) = &self.integrated_server {
            server.stop().await?;
        }
        self.integrated_server = None;
        
        // Show main menu
        self.main_menu = Some(MainMenu::new(self.event_bus.clone()));
        self.game.set_state(GameState::MainMenu);
        
        log::info!("Returned to main menu");
        
        Ok(())
    }
    
    /// Initialize compatibility layers
    pub async fn init_compat_layers(&self) -> anyhow::Result<()> {
        log::info!("Initializing compatibility layers");
        
        // Initialize Forge compatibility
        let forge = ForgeCompatLayer::new(
            self.event_bus.clone(),
            self.mod_loader.clone(),
        );
        forge.init().await?;
        
        // Initialize Neoforge compatibility
        let neoforge = NeoforgeCompatLayer::new(
            self.event_bus.clone(),
            self.mod_loader.clone(),
        );
        neoforge.init().await?;
        
        log::info!("Compatibility layers initialized");
        
        Ok(())
    }
    
    /// Load all mods
    pub async fn load_mods(&self) -> anyhow::Result<()> {
        if self.config.mods.enable_mods {
            log::info!("Loading mods");
            self.mod_loader.load_all_mods().await?;
            log::info!("Mods loaded successfully");
        }
        Ok(())
    }
    
    /// Run the application
    pub async fn run(&mut self) -> anyhow::Result<()> {
        log::info!("Starting EvokerMC");
        
        // Initialize compatibility layers
        self.init_compat_layers().await?;
        
        // Load mods
        self.load_mods().await?;
        
        // Main application loop
        // In a full implementation, this would:
        // - Initialize window and graphics
        // - Run event loop
        // - Render frames
        // - Process input
        // - Update game state

        println!("EvokerMc Initialized and ready");
        log::info!("EvokerMC initialized and ready");
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging, disabled due to being problematic
    // env_logger::init();
    
    // Create and run application
    let mut app = App::new().await?;
    app.run().await?;
    
    Ok(())
}
