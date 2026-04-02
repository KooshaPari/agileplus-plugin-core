# AGENTS.md — agileplus-plugin-core

Extends shelf-level agent rules. See `AgilePlus/AGENTS.md` for canonical definitions.

## Project Identity

- **Name**: agileplus-plugin-core
- **Type**: Rust library (AgilePlus plugin system core)
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/agileplus-plugin-core`

## AgilePlus Integration

All work MUST be tracked in AgilePlus:
- Reference: `.agileplus/` directory
- CLI: `agileplus <command>` (from project root)
- Specs: `.agileplus/specs/<feature-id>/`

## Quick Commands

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --check
```
