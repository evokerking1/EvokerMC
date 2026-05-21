# Integrated Server

EvokerMC's singleplayer experience is powered by an **integrated server** — a full game server that runs locally inside the same process as the client.

## Why an Integrated Server?

Traditional singleplayer games run all game logic directly in the client process. EvokerMC deliberately avoids this by using the same server code for both singleplayer and multiplayer. The benefits are:

- **Identical code paths**: bugs found in singleplayer are the same bugs that exist in multiplayer.
- **Mod compatibility**: mods written for a dedicated server work unchanged in singleplayer.
- **No split logic**: there is no "client-only game mode" to maintain separately.

## Lifecycle

### 1. Player Clicks "Create / Play World"

```rust
app.start_singleplayer("my_world".to_string()).await?;
```

### 2. Engine Starts the Integrated Server

```rust
let server = Server::new(
    0,          // port 0 → OS picks a free loopback port
    1,          // max players (singleplayer = 1)
    event_bus.clone(),
);
server.start().await?;
let server_addr = server.local_addr()?;
```

Port `0` causes the OS to assign an ephemeral port on the loopback interface (`127.0.0.1`). The port is never exposed to the network.

### 3. Client Connects to the Integrated Server

```rust
let client = Client::new(event_bus.clone());
client.connect(server_addr.to_string()).await?;
```

From this point on the game runs exactly like a multiplayer session, except both endpoints are in the same process communicating over loopback.

### 4. World is Loaded

The world is loaded from disk (or created fresh) and attached to the server:

```rust
let world = World::load("my_world", worlds_path, event_bus.clone())
    .await
    .or_else(|_| World::new("my_world", seed, worlds_path, event_bus.clone()))?;
```

### 5. Quitting to Menu

When the player returns to the main menu:

1. The world is saved and unloaded (`world.save().await?`).
2. The client disconnects (`client.disconnect().await?`).
3. The integrated server is stopped (`server.stop().await?`).

## Configuration

The integrated server uses the same `Server` struct as a dedicated server. The key differences are:

| Setting | Integrated | Dedicated |
|---------|-----------|----------|
| Port | `0` (loopback, OS-assigned) | Configured (default `25565`) |
| Max players | `1` | Configured |
| Accessible from LAN/WAN | ❌ | ✅ |

## Events

The integrated server emits the same events as a dedicated server:

- `ServerStarted { port }` — fired when the QUIC endpoint is bound.
- `PlayerJoined { player_id, player_name }` — fired when the local client connects.
- `PlayerLeft { player_id }` — fired on disconnect.
- `ServerStopped` — fired after `stop()` completes.

## See Also

- [Server-Authoritative Design](/architecture/server-authoritative)
- [Networking](/core/networking)
