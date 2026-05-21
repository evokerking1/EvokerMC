# Server-Authoritative Architecture

EvokerMC uses a **server-authoritative architecture** where the server is the single source of truth for all game state.

## How It Works

```
┌─────────────┐           ┌─────────────┐
│   CLIENT    │  Network  │   SERVER    │
│             │◄─────────►│             │
│ - Rendering │           │ - Game Logic│
│ - Input     │  Input ──►│ - Validation│
│ - Prediction│           │ - Physics   │
│             │◄── State  │ - AI        │
└─────────────┘           └─────────────┘
```

The client is intentionally "dumb":

- It captures player input (movement, mouse look, clicks) and sends it to the server.
- The server validates input, runs the simulation, and sends authoritative state updates back.
- The client renders whatever state the server tells it to.

This model is applied **identically** for both singleplayer and multiplayer. In singleplayer an [Integrated Server](/architecture/integrated-server) runs locally so no networking port is exposed to the outside world, but the same code path is used.

## Benefits

1. **Fair Multiplayer**: The server validates every action, so clients cannot send fabricated packets to gain advantages (speed hacks, fly hacks, etc.).
2. **Consistent State**: There is one authoritative source of truth. All clients see the same world.
3. **Simpler Development**: Game logic lives in one place. Singleplayer and multiplayer exercise the exact same server code, making bugs easy to reproduce and fix.
4. **Mod Safety**: Mods run on the server. They cannot access or modify the rendering pipeline or input handling on other players' clients.

## State Flow

1. Player presses a movement key → client sends `MoveInput` packet.
2. Server receives the packet, validates the move (collision detection, physics).
3. Server updates its canonical position for the player.
4. Server broadcasts the new position to all clients in range.
5. Clients update their local representation.

Any packet the server considers invalid is silently dropped; the server's state is never compromised.

## Limitations

Because the client relies on server confirmation, there is inherent latency between input and visible effect. A future client-side prediction layer will mask this by applying tentative state locally while waiting for server confirmation — a standard technique used by games such as Quake and Overwatch.

## See Also

- [Integrated Server](/architecture/integrated-server)
- [Networking](/core/networking)
- [Architecture Overview](/architecture/overview)
