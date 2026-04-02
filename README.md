# agileplus-plugin-core

Core plugin traits and registry for AgilePlus extensibility.

## Overview

`agileplus-plugin-core` is the Rust crate that defines the core plugin traits, plugin registry, and extension contracts for the AgilePlus platform. It is the authoritative port interface layer that all AgilePlus plugins must implement. Plugin adapters (from external crates) depend on this crate; the AgilePlus host runtime also depends on it to load and dispatch plugins.

## Features

- **Plugin Trait**: Core interface for all AgilePlus plugins
- **Event Handling**: Async event dispatch to registered plugins
- **Plugin Registry**: Registration and resolution by ID/capability
- **Lifecycle Hooks**: on_load, on_unload, on_event
- **Configuration**: Typed plugin configuration with serde_json::Value
- **Error Handling**: Comprehensive error types for plugin operations

## Installation

```bash
# Add to Cargo.toml
[dependencies]
agileplus-plugin-core = { git = "https://github.com/KooshaPari/agileplus-plugin-core" }
```

## Quick Start

```rust
use agileplus_plugin_core::{
    Plugin, PluginMetadata, PluginConfig, PluginRegistry, PluginError,
    EventHandler, AgilePlusEvent,
};
use async_trait::async_trait;

// Implement a plugin
pub struct MyPlugin;

#[async_trait]
impl Plugin for MyPlugin {
    fn id(&self) -> &'static str {
        "my-plugin"
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: "my-plugin".to_string(),
            version: "1.0.0".to_string(),
            author: "Your Name".to_string(),
            capabilities: vec!["sync".to_string()],
        }
    }

    async fn on_load(&self, config: PluginConfig) -> Result<(), PluginError> {
        println!("Plugin loaded with config: {:?}", config);
        Ok(())
    }

    async fn on_unload(&self) -> Result<(), PluginError> {
        println!("Plugin unloading");
        Ok(())
    }
}

#[async_trait]
impl EventHandler for MyPlugin {
    async fn on_event(&self, event: AgilePlusEvent) -> Result<(), PluginError> {
        match event {
            AgilePlusEvent::TaskCreated { .. } => println!("Task created!"),
            _ => {}
        }
        Ok(())
    }
}

// Register and use
let mut registry = PluginRegistry::new();
registry.register(Box::new(MyPlugin))?;

let plugin = registry.get("my-plugin").unwrap();
registry.dispatch_event(AgilePlusEvent::TaskCreated { id: 1 }).await?;
```

## API Overview

### Plugin Trait

```rust
#[async_trait]
pub trait Plugin: Send + Sync + 'static {
    fn id(&self) -> &'static str;
    fn metadata(&self) -> PluginMetadata;
    async fn on_load(&self, config: PluginConfig) -> Result<(), PluginError>;
    async fn on_unload(&self) -> Result<(), PluginError>;
}
```

### EventHandler Trait

```rust
#[async_trait]
pub trait EventHandler: Send + Sync + 'static {
    async fn on_event(&self, event: AgilePlusEvent) -> Result<(), PluginError>;
}
```

### PluginRegistry

| Method | Description |
|--------|-------------|
| `PluginRegistry::new()` | Create empty registry |
| `registry.register(plugin)` | Add plugin to registry |
| `registry.get(id)` | Retrieve plugin by ID |
| `registry.dispatch_event(event)` | Broadcast to all event handlers |

### AgilePlusEvent

```rust
pub enum AgilePlusEvent {
    TaskCreated { id: i64, title: String },
    TaskUpdated { id: i64, changes: Vec<String> },
    SprintStarted { sprint_id: i64, name: String },
    AgentCompleted { agent_id: String, result: String },
    Custom { name: String, payload: serde_json::Value },
}
```

### PluginMetadata

```rust
pub struct PluginMetadata {
    pub id: String,
    pub version: String,
    pub author: String,
    pub capabilities: Vec<String>,
}
```

### PluginConfig

```rust
pub struct PluginConfig(pub serde_json::Value);

impl PluginConfig {
    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        // Typed accessor
    }
}
```

### PluginError

```rust
pub enum PluginError {
    LoadFailed(String),
    UnloadFailed(String),
    EventDispatchFailed(String),
    ConfigInvalid(String),
    NotFound(String),
}
```

## Architecture

Following Hexagonal Architecture:
- **Port**: `Plugin` and `EventHandler` traits
- **Adapters**: External crates implement these traits

```
agileplus-plugin-core/
├── src/
│   ├── lib.rs           # Crate root, re-exports
│   ├── traits.rs         # Plugin and EventHandler traits
│   ├── registry.rs       # PluginRegistry implementation
│   ├── error.rs          # PluginError types
│   ├── lifecycle.rs      # Plugin lifecycle management
│   └── versioning.rs     # Version compatibility
├── tests/                # Mock plugin tests
└── Cargo.toml
```

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Check lints
cargo clippy -- -D warnings

# Format code
cargo fmt
```

## Related Plugins

| Plugin | Description |
|--------|-------------|
| [agileplus-plugin-git](https://github.com/KooshaPari/agileplus-plugin-git) | Git VCS operations |
| [agileplus-plugin-sqlite](https://github.com/KooshaPari/agileplus-plugin-sqlite) | SQLite storage |

## Links

- **Docs**: [https://docs.rs/agileplus-plugin-core](https://docs.rs/agileplus-plugin-core) (pending publication)
- **Repository**: https://github.com/KooshaPari/agileplus-plugin-core
- **AgilePlus**: https://github.com/KooshaPari/AgilePlus
- **Issues**: https://github.com/KooshaPari/agileplus-plugin-core/issues

## Contributing

1. This crate defines the contract - changes require careful consideration
2. All trait changes must be backward compatible or follow deprecation cycle
3. Ensure all tests pass: `cargo test`
4. Ensure clippy clean: `cargo clippy -- -D warnings`
5. Format code: `cargo fmt`
6. All public items must have doc comments

## License

MIT
