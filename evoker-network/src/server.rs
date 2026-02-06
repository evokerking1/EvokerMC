//! Game server implementation

use crate::protocol::{Packet, PacketType};
use dashmap::DashMap;
use evoker_core::{Event, EventBus};
use parking_lot::RwLock;
use std::net::SocketAddr;
use std::sync::Arc;

/// Player connection information
pub struct PlayerConnection {
    pub id: String,
    pub name: String,
    pub address: SocketAddr,
}

/// Game server
pub struct Server {
    port: u16,
    max_players: usize,
    players: Arc<DashMap<String, Arc<PlayerConnection>>>,
    event_bus: Arc<EventBus>,
    running: Arc<RwLock<bool>>,
}

impl Server {
    /// Create a new server
    pub fn new(port: u16, max_players: usize, event_bus: Arc<EventBus>) -> Self {
        Self {
            port,
            max_players,
            players: Arc::new(DashMap::new()),
            event_bus,
            running: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Start the server
    pub async fn start(&self) -> anyhow::Result<()> {
        log::info!("Starting server on port {}", self.port);
        
        *self.running.write() = true;
        
        self.event_bus.queue_event(Event::ServerStarted {
            port: self.port,
        });
        
        // In a real implementation, this would set up QUIC listeners
        // For now, it's a placeholder
        
        Ok(())
    }
    
    /// Stop the server
    pub async fn stop(&self) -> anyhow::Result<()> {
        log::info!("Stopping server");
        
        *self.running.write() = false;
        
        // Disconnect all players
        self.players.clear();
        
        self.event_bus.queue_event(Event::ServerStopped);
        
        Ok(())
    }
    
    /// Handle player connection
    pub async fn handle_player_join(
        &self,
        player_id: String,
        player_name: String,
        address: SocketAddr,
    ) -> anyhow::Result<()> {
        if self.players.len() >= self.max_players {
            return Err(anyhow::anyhow!("Server is full"));
        }
        
        let connection = Arc::new(PlayerConnection {
            id: player_id.clone(),
            name: player_name.clone(),
            address,
        });
        
        self.players.insert(player_id.clone(), connection);
        
        self.event_bus.queue_event(Event::PlayerJoined {
            player_id,
            player_name,
        });
        
        Ok(())
    }
    
    /// Handle player disconnection
    pub async fn handle_player_leave(&self, player_id: String) -> anyhow::Result<()> {
        if let Some((_, _)) = self.players.remove(&player_id) {
            self.event_bus.queue_event(Event::PlayerLeft {
                player_id,
            });
        }
        
        Ok(())
    }
    
    /// Broadcast packet to all players
    pub async fn broadcast_packet(&self, packet: Packet) -> anyhow::Result<()> {
        // In a real implementation, this would send the packet to all connected players
        log::debug!("Broadcasting packet: {:?}", packet.packet_type);
        Ok(())
    }
    
    /// Send packet to specific player
    pub async fn send_packet(&self, player_id: &str, packet: Packet) -> anyhow::Result<()> {
        if self.players.contains_key(player_id) {
            // In a real implementation, this would send the packet
            log::debug!("Sending packet to {}: {:?}", player_id, packet.packet_type);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Player not found: {}", player_id))
        }
    }
    
    /// Get player count
    pub fn player_count(&self) -> usize {
        self.players.len()
    }
    
    /// Check if server is running
    pub fn is_running(&self) -> bool {
        *self.running.read()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_server_lifecycle() {
        let event_bus = Arc::new(EventBus::new());
        let server = Server::new(25565, 100, event_bus);
        
        assert!(!server.is_running());
        
        server.start().await.unwrap();
        assert!(server.is_running());
        
        server.stop().await.unwrap();
        assert!(!server.is_running());
    }
}
