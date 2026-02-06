//! Network protocol definitions

use serde::{Deserialize, Serialize};

/// Packet types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PacketType {
    // Handshake
    Handshake,
    HandshakeResponse,
    
    // Status
    StatusRequest,
    StatusResponse,
    Ping,
    Pong,
    
    // Login
    LoginStart,
    LoginSuccess,
    
    // Play
    JoinGame,
    KeepAlive,
    ChatMessage,
    PlayerPosition,
    PlayerLook,
    PlayerPositionAndLook,
    ChunkData,
    BlockChange,
    MultiBlockChange,
    
    // Disconnect
    Disconnect,
}

/// Network packet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packet {
    pub packet_type: PacketType,
    pub data: Vec<u8>,
}

impl Packet {
    /// Create a new packet
    pub fn new(packet_type: PacketType, data: Vec<u8>) -> Self {
        Self {
            packet_type,
            data,
        }
    }
    
    /// Create a handshake packet
    pub fn handshake() -> Self {
        Self::new(PacketType::Handshake, Vec::new())
    }
    
    /// Create a ping packet
    pub fn ping() -> Self {
        Self::new(PacketType::Ping, Vec::new())
    }
    
    /// Create a disconnect packet
    pub fn disconnect(reason: &str) -> Self {
        Self::new(PacketType::Disconnect, reason.as_bytes().to_vec())
    }
    
    /// Serialize packet to bytes
    pub fn to_bytes(&self) -> anyhow::Result<Vec<u8>> {
        Ok(serde_json::to_vec(self)?)
    }
    
    /// Deserialize packet from bytes
    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        Ok(serde_json::from_slice(bytes)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_packet_serialization() {
        let packet = Packet::ping();
        let bytes = packet.to_bytes().unwrap();
        let deserialized = Packet::from_bytes(&bytes).unwrap();
        
        assert!(matches!(deserialized.packet_type, PacketType::Ping));
    }
}
