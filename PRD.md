# Product Requirements Document — agileplus-plugin-core

## Overview

`agileplus-plugin-core` is the Rust crate that defines the core plugin traits, plugin registry, and extension contracts for the AgilePlus platform. It is the authoritative port interface layer that all AgilePlus plugins must implement. Plugin adapters (from external crates) depend on this crate; the AgilePlus host runtime also depends on it to load and dispatch plugins.

## Problem Statement

AgilePlus needs to be extensible without modifying its core. A well-defined plugin API allows third-party and first-party extensions to add features (new agents, custom workflows, data enrichers, UI panels) without forking the platform.

## Goals

- Define the Plugin trait that all AgilePlus plugins must implement.
- Provide a PluginRegistry for registering and resolving plugins by name/capability.
- Define plugin lifecycle hooks: on_load, on_unload, on_event.
- Define the PluginConfig and PluginMetadata types.
- Provide typed error types for plugin operations.
- Be async_trait-compatible for async plugin hooks.
- Publish to crates.io as agileplus-plugin-core.

## Non-Goals

- Does not implement any specific plugin.
- Does not implement plugin discovery or dynamic loading (that is the host runtime responsibility).
- Does not provide UI components.

## Epics and User Stories

### E1 — Plugin Trait
- E1.1: The Plugin trait defines fn id, fn metadata, async fn on_load, async fn on_unload.
- E1.2: The Plugin trait is Send + Sync + 'static so plugins can be stored in the registry across async boundaries.

### E2 — Event Handling
- E2.1: The EventHandler trait defines async fn on_event.
- E2.2: Plugins optionally implement EventHandler to receive platform events.
- E2.3: AgilePlusEvent is an enum covering: TaskCreated, TaskUpdated, SprintStarted, AgentCompleted, CustomEvent.

### E3 — Registry
- E3.1: PluginRegistry::register adds a plugin keyed by id.
- E3.2: PluginRegistry::get retrieves a registered plugin by id.
- E3.3: PluginRegistry::dispatch_event broadcasts to all registered event handlers.

### E4 — Configuration
- E4.1: PluginConfig is a serde_json::Value wrapper with typed accessor helpers.
- E4.2: PluginMetadata contains: id, version, author, capabilities.

### E5 — Error Handling
- E5.1: PluginError variants: LoadFailed, UnloadFailed, EventDispatchFailed, ConfigInvalid, NotFound.

### E6 — Testing
- E6.1: cargo test passes with zero failures.
- E6.2: Tests include a mock plugin implementation demonstrating trait usage.

## Acceptance Criteria

- cargo build and cargo test succeed.
- cargo clippy produces zero warnings.
- All public traits and types have doc comments.
- Rust edition 2024 as specified in Cargo.toml.
