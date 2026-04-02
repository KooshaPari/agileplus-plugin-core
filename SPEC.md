# agileplus-plugin-core Specification

## Architecture
```
┌─────────────────────────────────────────────────────┐
│           Plugin Core (Rust)                       │
├─────────────────────────────────────────────────────┤
│  ┌──────────┐    ┌──────────────────────────┐ │
│  │  Plugin  │◀───│    Plugin Trait          │ │
│  │ Traits   │    │  + execute()              │ │
│  │          │    │  + capabilities()        │ │
│  └──────────┘    └──────────────────────────┘ │
│        │                                          │
│        ▼                                          │
│  ┌──────────────┐    ┌──────────────────────┐  │
│  │  Registry   │◀───│  load/unload/list     │  │
│  └──────────────┘    └──────────────────────┘  │
└─────────────────────────────────────────────────┘
```

## Components

| Component | Responsibility | Public API |
|-----------|----------------|-----------|
| PluginTrait | Core plugin interface | `execute()`, `capabilities()` |
| Registry | Track loaded plugins | `load()`, `unload()`, `get()` |
| Capability | Access control enum | `FileSystem`, `Network`, `Gitness`, `Sqlite` |

## Data Models

```rust
trait Plugin {
    fn execute(&self, ctx: &PluginContext) -> Result<PluginResult>;
    fn capabilities(&self) -> Vec<Capability>;
}

struct PluginContext {
    org_id: String,
    workspace_id: String,
    user_id: String,
    capabilities: Vec<Capability>,
}

enum Capability {
    FileSystem(PathBuf),
    Network,
    Gitness,
    Sqlite,
}
```

## Performance Targets

| Metric | Target |
|--------|--------|
| Plugin load | <50ms |
| Plugin execute | <100ms |
| Registry lookup | <1ms |
| Concurrent plugins | 20 max |