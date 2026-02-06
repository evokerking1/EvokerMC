//! Game state management and main game loop

use crate::{Event, EventBus};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use parking_lot::RwLock;
use tokio::time::{Duration, interval};

/// Game state enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameState {
    /// Game is in main menu
    MainMenu,
    /// Game is loading
    Loading,
    /// Game is running
    Running,
    /// Game is paused
    Paused,
    /// Game is saving
    Saving,
    /// Game is quitting
    Quitting,
}

/// Main game controller
pub struct Game {
    state: Arc<RwLock<GameState>>,
    event_bus: Arc<EventBus>,
    tick_rate: Duration,
    running: Arc<RwLock<bool>>,
}

impl Game {
    /// Create a new game instance
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self {
            state: Arc::new(RwLock::new(GameState::MainMenu)),
            event_bus,
            tick_rate: Duration::from_millis(50), // 20 TPS (ticks per second)
            running: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Get current game state
    pub fn state(&self) -> GameState {
        *self.state.read()
    }
    
    /// Set game state
    pub fn set_state(&self, new_state: GameState) {
        let old_state = *self.state.read();
        *self.state.write() = new_state;
        
        log::info!("Game state changed: {:?} -> {:?}", old_state, new_state);
        
        // Emit appropriate events
        match new_state {
            GameState::Running => {
                self.event_bus.queue_event(Event::GameStarted);
            }
            GameState::Paused => {
                self.event_bus.queue_event(Event::GamePaused);
            }
            GameState::Quitting => {
                self.event_bus.queue_event(Event::GameStopped);
            }
            _ => {}
        }
    }
    
    /// Start the game loop
    pub async fn start(&self) -> anyhow::Result<()> {
        *self.running.write() = true;
        self.set_state(GameState::Running);
        
        let mut ticker = interval(self.tick_rate);
        
        while *self.running.read() {
            ticker.tick().await;
            
            // Process queued events
            self.event_bus.process_queue().await?;
            
            // Main game tick
            self.tick().await?;
            
            // Check if we should quit
            if self.state() == GameState::Quitting {
                break;
            }
        }
        
        Ok(())
    }
    
    /// Stop the game
    pub fn stop(&self) {
        log::info!("Stopping game...");
        self.set_state(GameState::Quitting);
        *self.running.write() = false;
    }
    
    /// Single game tick
    async fn tick(&self) -> anyhow::Result<()> {
        // This is where the game logic would run each tick
        // For now, it's just a placeholder
        Ok(())
    }
    
    /// Pause the game
    pub fn pause(&self) {
        if self.state() == GameState::Running {
            self.set_state(GameState::Paused);
        }
    }
    
    /// Resume the game
    pub fn resume(&self) {
        if self.state() == GameState::Paused {
            self.set_state(GameState::Running);
            self.event_bus.queue_event(Event::GameResumed);
        }
    }
    
    /// Check if game is running
    pub fn is_running(&self) -> bool {
        *self.running.read()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_game_state() {
        let event_bus = Arc::new(EventBus::new());
        let game = Game::new(event_bus);
        
        assert_eq!(game.state(), GameState::MainMenu);
        
        game.set_state(GameState::Running);
        assert_eq!(game.state(), GameState::Running);
        
        game.pause();
        assert_eq!(game.state(), GameState::Paused);
        
        game.resume();
        assert_eq!(game.state(), GameState::Running);
    }
}
