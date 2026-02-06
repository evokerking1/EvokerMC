//! Networking module for multiplayer functionality
//! 
//! Provides server and client implementations using QUIC protocol

pub mod server;
pub mod client;
pub mod protocol;

pub use server::Server;
pub use client::Client;
pub use protocol::{Packet, PacketType};
