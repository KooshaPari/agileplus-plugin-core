# agileplus-plugin-core

Core plugin traits and registry for AgilePlus. Defines the plugin contract, capability interfaces, and the runtime registry that manages loaded plugins.

## Stack
- Language: Rust
- Key deps: Cargo, serde, possibly Extism/WASM for sandboxed plugins

## Structure
- `src/`: Rust library
  - Plugin trait definitions
  - Plugin registry (load, unload, lookup)
  - Capability interfaces (what plugins can do/access)

## Key Patterns
- Trait-based plugin contract: all plugins implement core traits
- Registry is the single source of truth for loaded plugins
- Capability system limits what plugins can access (principle of least privilege)
- No global mutable state — registry passed via dependency injection

## Adding New Functionality
- New plugin capabilities: add trait methods + capability enum variants
- New registry operations: extend `src/registry.rs`
- Run `cargo test` to verify
