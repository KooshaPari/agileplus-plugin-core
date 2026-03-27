# Architecture Decision Records — agileplus-plugin-core

## ADR-001: Trait-Based Plugin System

**Status:** Accepted

**Context:** Plugin systems can use in-process traits, WASM via Extism, or IPC. Each has different isolation and performance characteristics.

**Decision:** Use in-process Rust traits for the initial plugin system. WASM/Extism support is a future ADR candidate.

**Rationale:** Trait-based plugins have zero serialisation overhead, full type safety, and are trivially testable. The AgilePlus plugin ecosystem is currently first-party; strong isolation is not yet required.

**Alternatives Considered:**
- Extism/WASM: strong isolation but adds significant latency per call; not justified for first-party plugins.
- IPC/gRPC: too heavy for in-process extensibility.

**Consequences:** Plugins run in the same process as AgilePlus. A misbehaving plugin can crash the host. Mitigation: plugins are loaded from trusted sources only.

---

## ADR-002: Non-Exhaustive AgilePlusEvent Enum

**Status:** Accepted

**Context:** The event enum must grow over time as AgilePlus adds features. Plugins compiled against older versions must not break when new event variants are added.

**Decision:** AgilePlusEvent is marked non_exhaustive. All match expressions must include a wildcard arm.

**Rationale:** Prevents semver-breaking additions to the event enum. Plugins handle unknown events via the wildcard arm.

**Consequences:** Plugins cannot exhaustively match all events without a wildcard. This is intentional.

---

## ADR-003: async-trait for Plugin Hooks

**Status:** Accepted

**Context:** AgilePlus is async-first using Tokio. Plugin hooks must be async to avoid blocking the runtime.

**Decision:** Use the async-trait crate for Plugin and EventHandler trait methods.

**Rationale:** async-trait is the pragmatic choice for dyn-compatible async traits. Stable async trait support for dyn objects requires careful use of return position impl trait which is still complex.

**Consequences:** Small per-call allocation for async dispatch. Acceptable for plugin hook frequency.
