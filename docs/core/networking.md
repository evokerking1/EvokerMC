# Networking

The `evoker-network` crate provides the multiplayer transport layer. It uses the **QUIC** protocol (via the `quinn` crate) for low-latency, encrypted, multiplexed connections.

## Why QUIC?

- **Encrypted by default** — TLS 1.3 is mandatory in QUIC.
- **Multiplexed streams** — multiple independent byte streams over one connection, no head-of-line blocking.
- **Connection migration** — connections survive IP address changes (e.g., switching Wi-Fi).
- **Low latency** — 0-RTT and 1-RTT handshakes, ideal for game traffic.

## Core Types

### `Server`

Listens for incoming player connections and manages active sessions.

```rust
pub struct Server {
    port: u16,
    max_players: usize,
    players: Arc<DashMap<String, Arc<PlayerConnection>>>,
    event_bus: Arc<EventBus>,
    // ...
}
```

### `Client`

Manages the outgoing connection to a server.

### `Packet`

The unit of communication. Each packet has a `PacketType` and a binary payload.

```rust
pub struct Packet {
    pub packet_type: PacketType,
    pub data: Vec<u8>,
}
```

### `PlayerConnection`

Server-side representation of a connected player.

```rust
pub struct PlayerConnection {
    pub id: String,
    pub name: String,
    pub address: SocketAddr,
    pub tx: mpsc::UnboundedSender<Packet>,
}
```

## Starting a Dedicated Server

```rust
use evoker_network::Server;
use std::sync::Arc;

let server = Server::new(
    25565,          // port
    100,            // max players
    event_bus.clone(),
);
server.start().await?;
```

### Sending Packets

```rust
// To a specific player
server.send_packet("player-uuid", packet).await?;

// To all connected players
server.broadcast_packet(packet).await?;
```

### Stopping the Server

```rust
server.stop().await?;
```

Closes the QUIC endpoint and disconnects all players.

## Connecting as a Client

```rust
use evoker_network::Client;

let client = Client::new(event_bus.clone());
client.connect("play.example.com:25565".to_string()).await?;
```

### Disconnecting

```rust
client.disconnect().await?;
```

## Integrated Server (Singleplayer)

For singleplayer, pass port `0` to let the OS choose a free loopback port:

```rust
let server = Server::new(0, 1, event_bus.clone());
server.start().await?;

let addr = server.local_addr().unwrap();  // e.g. "127.0.0.1:54321"
client.connect(addr.to_string()).await?;
```

See [Integrated Server](/architecture/integrated-server) for the full flow.

## Events

| Event | Fired by |
|-------|---------|
| `ServerStarted { port }` | `server.start()` |
| `ServerStopped` | `server.stop()` |
| `ClientConnected { server_address }` | `client.connect()` |
| `ClientDisconnected` | `client.disconnect()` |
| `PlayerJoined { player_id, player_name }` | New QUIC connection accepted |
| `PlayerLeft { player_id }` | Connection closed |

## TLS Certificates

The server generates a **self-signed certificate** at startup using the `rcgen` crate. Production deployments should replace this with a certificate from a trusted CA by configuring `ServerConfig` before calling `server.start()`.

## See Also

- [Server-Authoritative Design](/architecture/server-authoritative)
- [Integrated Server](/architecture/integrated-server)
