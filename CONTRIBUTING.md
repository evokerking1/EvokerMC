# Contributing to EvokerMC

Thank you for your interest in contributing to EvokerMC!

## Development Setup

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Git

### Building

```bash
git clone https://github.com/evokerking1/EvokerMC
cd EvokerMC
cargo build
```

### Running Tests

```bash
cargo test
```

### Code Style

We follow standard Rust conventions:

```bash
# Format code
cargo fmt

# Run linter
cargo clippy
```

## Project Structure

- `evoker-core/` - Core engine and event system
- `evoker-world/` - World management
- `evoker-network/` - Networking
- `evoker-modding/` - Mod system
- `evoker-scripting/` - Scripting engines
- `evoker-ui/` - User interface
- `evoker-assets/` - Asset management
- `evoker-compat/` - API compatibility
- `evoker-registry/` - Data-driven system
- `data/` - Example data packs
- `docs/` - Documentation

## Making Changes

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Write/update tests
5. Ensure code compiles and tests pass
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to your branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## Code Guidelines

- Write clear, self-documenting code
- Add doc comments for public APIs
- Write tests for new functionality
- Follow Rust idioms and best practices
- Keep functions small and focused
- Use meaningful variable names

## Documentation

- Update rustdoc comments for code changes
- Update markdown docs in `docs/` for user-facing changes
- Keep ARCHITECTURE.md updated for architectural changes

## Areas to Contribute

- **Rendering**: 3D graphics with wgpu
- **Physics**: Collision detection and entity physics
- **World Generation**: Noise-based terrain generation
- **Entity AI**: Behavior trees and pathfinding
- **Gameplay**: Combat, inventory, crafting systems
- **Mods**: Example mods and tutorials
- **Documentation**: Improve guides and API docs
- **Testing**: Increase test coverage

## Questions?

Open an issue for discussion or clarification.

## License

By contributing, you agree that your contributions will be licensed under MIT OR Apache-2.0.
