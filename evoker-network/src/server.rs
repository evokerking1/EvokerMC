//! Game server implementation

use crate::protocol::{Packet, PacketType};
use dashmap::DashMap;
use evoker_core::{Event, EventBus};
use parking_lot::RwLock;
use quinn::{Endpoint, ServerConfig};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc;

/// Player connection information
pub struct PlayerConnection {
    pub id: String,
    pub name: String,
    pub address: SocketAddr,
    pub tx: mpsc::UnboundedSender<Packet>,
}

/// Game server
pub struct Server {
    port: u16,
    max_players: usize,
    players: Arc<DashMap<String, Arc<PlayerConnection>>>,
    event_bus: Arc<EventBus>,
    running: Arc<RwLock<bool>>,
    endpoint: Arc<RwLock<Option<Endpoint>>>,
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
            endpoint: Arc::new(RwLock::new(None)),
        }
    }
    
    /// Start the server
    pub async fn start(&self) -> anyhow::Result<()> {
        log::info!("Starting server on port {}", self.port);
        
        *self.running.write() = true;
        
        // Generate self-signed certificate for QUIC
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
        let key = rustls::pki_types::PrivatePkcs8KeyDer::from(cert.key_pair.serialize_der());
        let cert_der = rustls::pki_types::CertificateDer::from(cert.cert);
        
        let mut server_config = ServerConfig::with_single_cert(vec![cert_der], key.into())?;
        let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
        transport_config.max_concurrent_uni_streams(0_u8.into());
        
        // Bind to address
        let addr: SocketAddr = if self.port == 0 {
            "127.0.0.1:0".parse()?
        } else {
            format!("0.0.0.0:{}", self.port).parse()?
        };
        
        let endpoint = Endpoint::server(server_config, addr)?;
        let actual_addr = endpoint.local_addr()?;
        
        log::info!("Server listening on {}", actual_addr);
        
        *self.endpoint.write() = Some(endpoint.clone());
        
        self.event_bus.queue_event(Event::ServerStarted {
            port: actual_addr.port(),
        });
        
        // Spawn connection handler
        let running = self.running.clone();
        let players = self.players.clone();
        let event_bus = self.event_bus.clone();
        let max_players = self.max_players;
        
        tokio::spawn(async move {
            while *running.read() {
                if let Some(connecting) = endpoint.accept().await {
                    let players = players.clone();
                    let event_bus = event_bus.clone();
                    
                    tokio::spawn(async move {
                        match connecting.await {
                            Ok(connection) => {
                                if players.len() >= max_players {
                                    log::warn!("Server full, rejecting connection");
                                    return;
                                }
                                
                                let remote_addr = connection.remote_address();
                                log::info!("New connection from: {}", remote_addr);
                                
                                // Handle connection
                                let player_id = uuid::Uuid::new_v4().to_string();
                                let (tx, mut rx) = mpsc::unbounded_channel();
                                
                                let player_conn = Arc::new(PlayerConnection {
                                    id: player_id.clone(),
                                    name: format!("Player_{}", &player_id[..8]),
                                    address: remote_addr,
                                    tx,
                                });
                                
                                players.insert(player_id.clone(), player_conn.clone());
                                
                                event_bus.queue_event(Event::PlayerJoined {
                                    player_id: player_id.clone(),
                                    player_name: player_conn.name.clone(),
                                });
                                
                                // Handle packets from this connection
                                loop {
                                    tokio::select! {
                                        packet = rx.recv() => {
                                            if packet.is_none() {
                                                break;
                                            }
                                            // Send packet to client
                                        }
                                        result = connection.accept_bi() => {
                                            match result {
                                                Ok((_send, mut recv)) => {
                                                    // Read packet with size limit
                                                    if let Ok(buf) = recv.read_to_end(1024 * 1024).await {
                                                        // Process packet
                                                        if !buf.is_empty() {
                                                            log::debug!("Received {} bytes from client", buf.len());
                                                        }
                                                    }
                                                }
                                                Err(_) => break,
                                            }
                                        }
                                    }
                                }
                                
                                // Player disconnected
                                players.remove(&player_id);
                                event_bus.queue_event(Event::PlayerLeft { player_id });
                            }
                            Err(e) => {
                                log::error!("Connection failed: {}", e);
                            }
                        }
                    });
                }
            }
        });
        
        Ok(())
    }
    
    /// Stop the server
    pub async fn stop(&self) -> anyhow::Result<()> {
        log::info!("Stopping server");
        
        *self.running.write() = false;
        
        // Disconnect all players
        self.players.clear();
        
        // Close endpoint
        if let Some(endpoint) = self.endpoint.write().take() {
            endpoint.close(0u32.into(), b"server shutdown");
        }
        
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
        
        let (tx, _rx) = mpsc::unbounded_channel();
        
        let connection = Arc::new(PlayerConnection {
            id: player_id.clone(),
            name: player_name.clone(),
            address,
            tx,
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
        for player in self.players.iter() {
            if let Err(e) = player.tx.send(packet.clone()) {
                log::error!("Failed to send packet to player {}: {}", player.id, e);
            }
        }
        log::debug!("Broadcasting packet: {:?}", packet.packet_type);
        Ok(())
    }
    
    /// Send packet to specific player
    pub async fn send_packet(&self, player_id: &str, packet: Packet) -> anyhow::Result<()> {
        if let Some(player) = self.players.get(player_id) {
            let packet_type = packet.packet_type.clone();
            player.tx.send(packet)?;
            log::debug!("Sending packet to {}: {:?}", player_id, packet_type);
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
    
    /// Get local address
    pub fn local_addr(&self) -> Option<SocketAddr> {
        self.endpoint.read().as_ref().and_then(|e| e.local_addr().ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_server_lifecycle() {
        let event_bus = Arc::new(EventBus::new());
        let server = Server::new(0, 100, event_bus);
        
        assert!(!server.is_running());
        
        server.start().await.unwrap();
        assert!(server.is_running());
        assert!(server.local_addr().is_some());
        
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        server.stop().await.unwrap();
        assert!(!server.is_running());
    }
}
