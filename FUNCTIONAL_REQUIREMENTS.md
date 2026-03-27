# Functional Requirements — agileplus-plugin-core

## FR-TRAIT-001
The crate SHALL export a Plugin trait with methods: id, metadata, on_load, on_unload.

## FR-TRAIT-002
Plugin trait SHALL have bounds Send + Sync + 'static.

## FR-TRAIT-003
Plugin trait SHALL use async_trait for async method compatibility.

## FR-EVENT-001
The crate SHALL export an EventHandler trait with method on_event.

## FR-EVENT-002
AgilePlusEvent SHALL be a non-exhaustive enum with variants: TaskCreated, TaskUpdated, SprintStarted, AgentCompleted, Custom.

## FR-REG-001
PluginRegistry::register SHALL add the plugin keyed by plugin.id().

## FR-REG-002
PluginRegistry::get SHALL return a reference to a registered plugin by id.

## FR-REG-003
PluginRegistry::dispatch_event SHALL call on_event on all registered plugins that implement EventHandler.

## FR-META-001
PluginMetadata SHALL have fields: id, version, author, capabilities.

## FR-CONFIG-001
PluginConfig SHALL be a newtype wrapper over serde_json::Value with a typed get accessor.

## FR-ERR-001
PluginError SHALL have variants: LoadFailed, UnloadFailed, EventDispatchFailed, ConfigInvalid, NotFound.

## FR-ERR-002
PluginError SHALL implement thiserror::Error.

## FR-BUILD-001
Cargo.toml SHALL specify edition = "2024" and rust-version = "1.86".

## FR-TEST-001
cargo test SHALL pass.

## FR-TEST-002
Tests SHALL include a MockPlugin struct that implements the Plugin trait.

## FR-LINT-001
cargo clippy SHALL exit 0 with zero warnings.
