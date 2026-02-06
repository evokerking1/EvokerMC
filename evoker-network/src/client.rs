//! Game client implementation

use crate::protocol::{Packet, PacketType};
use evoker_core::{Event, EventBus};
use parking_lot::RwLock;
use quinn::{Connection, Endpoint};
use std::sync::Arc;
use tokio::sync::mpsc;

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
    connection: RwLock<Option<Connection>>,
    endpoint: RwLock<Option<Endpoint>>,
    packet_tx: RwLock<Option<mpsc::UnboundedSender<Packet>>>,
}

impl Client {
    /// Create a new client
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self {
            server_address: RwLock::new(None),
            state: RwLock::new(ClientState::Disconnected),
            event_bus,
            player_id: RwLock::new(None),
            connection: RwLock::new(None),
            endpoint: RwLock::new(None),
            packet_tx: RwLock::new(None),
        }
    }
    
    /// Connect to a server
    pub async fn connect(&self, server_address: String) -> anyhow::Result<()> {
        log::info!("Connecting to server: {}", server_address);
        
        *self.state.write() = ClientState::Connecting;
        
        // Create client endpoint
        let endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
        
        // Parse server address
        let addr: std::net::SocketAddr = server_address.parse()?;
        
        // Connect to server
        let connection = endpoint.connect(addr, "localhost")?.await?;
        
        log::info!("Connected to server at {}", addr);
        
        *self.server_address.write() = Some(server_address.clone());
        *self.connection.write() = Some(connection.clone());
        *self.endpoint.write() = Some(endpoint);
        *self.state.write() = ClientState::Connected;
        
        self.event_bus.queue_event(Event::ClientConnected {
            server_address,
        });
        
        // Spawn packet handler
        let connection_tx = connection.clone();
        let (tx, mut rx) = mpsc::unbounded_channel();
        *self.packet_tx.write() = Some(tx);
        
        tokio::spawn(async move {
            while let Some(packet) = rx.recv().await {
                // Send packet through connection
                if let Ok((mut send, _recv)) = connection_tx.open_bi().await {
                    let data = packet.to_bytes().unwrap_or_default();
                    let _ = send.write_all(&data).await;
                    let _ = send.finish();
                }
            }
        });
        
        // Spawn receive handler
        let event_bus2 = self.event_bus.clone();
        let connection2 = connection.clone();
        tokio::spawn(async move {
            loop {
                match connection2.accept_bi().await {
                    Ok((_send, mut recv)) => {
                        if let Ok(buf) = recv.read_to_end(1024 * 1024).await {
                            // Process received packet
                            if let Ok(packet) = Packet::from_bytes(&buf) {
                                log::debug!("Received packet: {:?}", packet.packet_type);
                                // Handle packet based on type
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
        });
        
        Ok(())
    }
    
    /// Disconnect from server
    pub async fn disconnect(&self) -> anyhow::Result<()> {
        log::info!("Disconnecting from server");
        
        *self.state.write() = ClientState::Disconnecting;
        
        // Close connection
        if let Some(connection) = self.connection.write().take() {
            connection.close(0u32.into(), b"client disconnect");
        }
        
        // Close endpoint
        if let Some(endpoint) = self.endpoint.write().take() {
            endpoint.close(0u32.into(), b"client disconnect");
        }
        
        *self.server_address.write() = None;
        *self.state.write() = ClientState::Disconnected;
        *self.packet_tx.write() = None;
        
        self.event_bus.queue_event(Event::ClientDisconnected);
        
        log::info!("Disconnected from server");
        
        Ok(())
    }
    
    /// Send packet to server
    pub async fn send_packet(&self, packet: Packet) -> anyhow::Result<()> {
        if *self.state.read() != ClientState::Connected {
            return Err(anyhow::anyhow!("Not connected to server"));
        }
        
        if let Some(tx) = self.packet_tx.read().as_ref() {
            tx.send(packet)?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Packet sender not initialized"))
        }
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
    async fn test_client_state() {
        let event_bus = Arc::new(EventBus::new());
        let client = Client::new(event_bus);
        
        assert_eq!(client.state(), ClientState::Disconnected);
        assert!(!client.is_connected());
    }
}
