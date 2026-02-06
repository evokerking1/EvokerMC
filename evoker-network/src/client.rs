//! Game client implementation

use crate::protocol::{Packet, PacketType};
use evoker_core::{Event, EventBus};
use parking_lot::RwLock;
use std::sync::Arc;

/// Game client state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientState {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting,
}

/// Game client
pub struct Client {
    server_address: RwLock<Option<String>>,
    state: RwLock<ClientState>,
    event_bus: Arc<EventBus>,
    player_id: RwLock<Option<String>>,
}

impl Client {
    /// Create a new client
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self {
            server_address: RwLock::new(None),
            state: RwLock::new(ClientState::Disconnected),
            event_bus,
            player_id: RwLock::new(None),
        }
    }
    
    /// Connect to a server
    pub async fn connect(&self, server_address: String) -> anyhow::Result<()> {
        log::info!("Connecting to server: {}", server_address);
        
        *self.state.write() = ClientState::Connecting;
        
        // In a real implementation, this would establish a QUIC connection
        // For now, simulate a successful connection
        
        *self.server_address.write() = Some(server_address.clone());
        *self.state.write() = ClientState::Connected;
        
        self.event_bus.queue_event(Event::ClientConnected {
            server_address,
        });
        
        log::info!("Connected to server");
        
        Ok(())
    }
    
    /// Disconnect from server
    pub async fn disconnect(&self) -> anyhow::Result<()> {
        log::info!("Disconnecting from server");
        
        *self.state.write() = ClientState::Disconnecting;
        
        // In a real implementation, this would close the connection
        
        *self.server_address.write() = None;
        *self.state.write() = ClientState::Disconnected;
        
        self.event_bus.queue_event(Event::ClientDisconnected);
        
        log::info!("Disconnected from server");
        
        Ok(())
    }
    
    /// Send packet to server
    pub async fn send_packet(&self, packet: Packet) -> anyhow::Result<()> {
        if *self.state.read() != ClientState::Connected {
            return Err(anyhow::anyhow!("Not connected to server"));
        }
        
        // In a real implementation, this would send the packet
        log::debug!("Sending packet: {:?}", packet.packet_type);
        
        Ok(())
    }
    
    /// Get current state
    pub fn state(&self) -> ClientState {
        *self.state.read()
    }
    
    /// Check if connected
    pub fn is_connected(&self) -> bool {
        *self.state.read() == ClientState::Connected
    }
    
    /// Get server address
    pub fn server_address(&self) -> Option<String> {
        self.server_address.read().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_client_connection() {
        let event_bus = Arc::new(EventBus::new());
        let client = Client::new(event_bus);
        
        assert_eq!(client.state(), ClientState::Disconnected);
        
        client.connect("localhost:25565".to_string()).await.unwrap();
        assert_eq!(client.state(), ClientState::Connected);
        
        client.disconnect().await.unwrap();
        assert_eq!(client.state(), ClientState::Disconnected);
    }
}
