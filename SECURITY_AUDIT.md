# Security Audit Report: agileplus-plugin-core

**Date:** 2026-04-02
**Auditor:** Polecat-43
**Scope:** Core plugin traits, registry, error types, and dependency surface

---

## Executive Summary

The `agileplus-plugin-core` crate is a small, well-structured Rust library (4 source files, ~360 lines) that defines plugin traits and a thread-safe registry. No critical vulnerabilities were found. Several medium and low-severity issues were identified, primarily around TOCTOU race conditions in the registry, missing Cargo.lock, and an overly broad `tokio` dependency.

**Risk Rating:** LOW-MEDIUM

---

## 1. Dependency Analysis

### 1.1 Cargo.toml Review

| Dependency | Version | Notes |
|---|---|---|
| serde | 1 | Stable, widely used. OK. |
| serde_json | 1 | Stable, widely used. OK. |
| thiserror | 2 | Stable, widely used. OK. |
| tokio | 1, features = ["full"] | **MEDIUM** — `full` pulls in all tokio features. |
| tracing | 0.1 | Stable, widely used. OK. |
| async-trait | 0.1 | Stable, widely used. OK. |

### 1.2 Findings

**[MEDIUM] Overly broad tokio dependency**
- `tokio` is declared with `features = ["full"]`, which enables all tokio feature flags including `process`, `fs`, `net`, `signal`, `sync`, `rt-multi-thread`, etc.
- The crate itself only needs `async-trait` for trait definitions and does not directly spawn tasks or use I/O.
- **Recommendation:** Remove `tokio` from `[dependencies]` entirely. The only async requirement is the `#[async_trait]` macro, which does not require tokio at runtime. If tokio is needed for tests, keep it in `[dev-dependencies]` only with minimal features (e.g., `["rt", "macros"]`).
- **Impact:** Larger binary, longer compile times, and a larger attack surface from unnecessary code.

**[LOW] No Cargo.lock committed**
- The repository does not contain a `Cargo.lock` file.
- For a library crate this is acceptable (Cargo.lock is typically not committed for libraries). However, it means reproducible builds depend on the consumer's lock file.
- **Recommendation:** Consider committing `Cargo.lock` for CI reproducibility, or document the expected behavior.

**[LOW] No minimum version constraints**
- All dependencies use semver `1` or `0.1` without patch-level floors (e.g., `>=1.0.200`).
- **Recommendation:** Pin to known-good minimum patch versions if any past versions had CVEs.

### 1.3 Cargo Audit

- `cargo audit` could not be executed in this environment (cargo not installed).
- **Recommendation:** Run `cargo audit` in CI. As of the dependency versions declared, no known CVEs are expected for these mature crates, but this should be verified.

---

## 2. Hardcoded Secrets / Credentials

**[PASS] No hardcoded secrets found.**

- Searched all `.rs` and `.toml` files for patterns: `password`, `secret`, `api_key`, `apikey`, `token`, `credential`, `private_key`, `ACCESS_KEY`, `SECRET_KEY`, `AWS_`, `GITHUB_`.
- No matches found.
- The `PluginConfig` struct uses `serde_json::Value` for adapter-specific config, which is appropriate — secrets should be injected at runtime, not baked in.

---

## 3. Race Condition Analysis

### 3.1 TOCTOU in `register_vcs` and `register_storage`

**[MEDIUM] Time-of-check-to-time-of-use (TOCTOU) race in plugin registration**

Location: `src/registry.rs:60-80` and `src/registry.rs:99-119`

```rust
pub fn register_vcs(&self, plugin: Box<dyn VcsPlugin>) -> PluginResult<()> {
    if self.is_finalized() {          // CHECK — acquires read lock, releases it
        return Err(...);
    }
    // <-- WINDOW: another thread can call finalize() here
    let name = plugin.name().to_string();
    let mut vcs = self.vcs.write()... // USE — acquires write lock
    ...
}
```

The `is_finalized()` check acquires and releases the `initialized` read lock. Between that release and the subsequent `self.vcs.write()` lock acquisition, another thread can call `finalize()` and set `initialized = true`. This means a plugin could be registered after the registry was intended to be finalized.

Similarly, the duplicate-check pattern has a window:
```rust
if self.is_finalized() { ... }        // check
let mut vcs = self.vcs.write()...     // lock
if vcs.contains_key(&name) { ... }    // check inside lock
```

The finalization check is outside the write lock. A concurrent `finalize()` call between the check and the write-lock acquisition could result in registration after finalization.

**Recommendation:** Acquire the write lock on `initialized` first, or combine the finalization check inside the same lock scope as the registration. For example:

```rust
pub fn register_vcs(&self, plugin: Box<dyn VcsPlugin>) -> PluginResult<()> {
    let name = plugin.name().to_string();
    let mut vcs = self.vcs.write().map_err(|_| ...)?;
    let initialized = self.initialized.read().map_err(|_| ...)?;
    if *initialized {
        return Err(PluginError::Initialization(
            "Registry is finalized, cannot register new plugins".to_string(),
        ));
    }
    if vcs.contains_key(&name) { ... }
    vcs.insert(name, Arc::from(plugin));
    Ok(())
}
```

Or better: hold the `initialized` write lock during the entire registration to prevent concurrent finalization.

### 3.2 Poisoned Lock Handling

**[LOW] Poisoned lock errors are swallowed in read paths**

Location: `src/registry.rs:84-88`, `src/registry.rs:92-93`

```rust
pub fn vcs(&self, name: &str) -> Option<Arc<dyn VcsPlugin>> {
    self.vcs.read().ok()  // PoisonError silently swallowed
        .and_then(|g| g.get(name).cloned())
}
```

If a thread panics while holding the `RwLock`, the lock becomes poisoned. The read methods silently return `None`/empty on poison, which could mask bugs and lead to confusing behavior (plugins appear missing).

**Recommendation:** Consider logging the poison event via `tracing` before returning `None`, or propagate the error. At minimum, document this behavior.

### 3.3 No Deadlock Risk

The registry uses separate `RwLock`s for `vcs`, `storage`, and `initialized`. These are never held simultaneously in a nested fashion, so there is no deadlock risk from lock ordering.

---

## 4. Error Handling Analysis

### 4.1 Error Types

**[PASS] Comprehensive error types.**

The `PluginError` enum in `src/error.rs` covers all major failure modes:
- `Initialization` — plugin startup failures
- `NotFound` — missing plugin lookup
- `AlreadyRegistered` — duplicate registration
- `AlreadyExists` — entity collision
- `Operation` — generic operation failure
- `Config` — configuration errors
- `Io` — std::io::Error conversion via `#[from]`
- `Serialization` — serde_json::Error conversion via `#[from]`
- `Execution` — runtime execution failures
- `Validation` — input validation failures

All variants have descriptive `#[error(...)]` messages via `thiserror`.

### 4.2 Findings

**[LOW] No `Display` on data types**

The data types (`WorktreeInfo`, `MergeResult`, `ConflictInfo`, `FeatureArtifacts`, `PluginConfig`, `RegistryStats`) derive `Debug` but not `Display`. This is acceptable for internal types but could make user-facing error messages less readable if these types appear in errors.

**[LOW] Missing error context for serde_json**

The `Serialization` variant uses `#[from] serde_json::Error` which loses the context of what was being serialized/deserialized. Consider wrapping with context:

```rust
#[error("Serialization error during {operation}: {source}")]
Serialization { operation: &'static str, source: serde_json::Error },
```

**[INFO] No tests for error types**

There are no dedicated tests for `PluginError` variants or `PluginResult`. The error types are straightforward, but a test ensuring `Display` output is correct would be good practice.

---

## 5. Additional Observations

### 5.1 Duplicate Doc Comment

`src/registry.rs:12-13` has a duplicated doc comment line:
```rust
/// Thread-safe plugin registry.
/// Thread-safe plugin registry.
```
Minor cosmetic issue.

### 5.2 Missing `StoragePlugin` Mock in Tests

The test module only includes `MockVcsPlugin`. There is no `MockStoragePlugin` to test storage registration paths. This means `register_storage`, `storage()`, and `storage_adapters()` are not covered by unit tests.

### 5.3 No Tests for `health_check`

The `health_check` method on `PluginRegistry` is not tested. A test should verify it iterates all plugins and propagates errors.

### 5.4 `async-trait` Deprecation Consideration

The `async-trait` crate (0.1.x) is a workaround for pre-1.75 Rust. Since the MSRV is `1.86`, native async trait support is available. Consider migrating to native `async fn` in traits to remove the `async-trait` dependency.

---

## Summary of Findings

| ID | Severity | Category | Description |
|---|---|---|---|
| SEC-001 | MEDIUM | Dependency | `tokio` with `features = ["full"]` pulls in unnecessary code |
| SEC-002 | MEDIUM | Concurrency | TOCTOU race between `is_finalized()` check and plugin registration |
| SEC-003 | LOW | Concurrency | Poisoned lock errors silently swallowed in read paths |
| SEC-004 | LOW | Dependencies | No `Cargo.lock` committed for reproducible builds |
| SEC-005 | LOW | Error Handling | `serde_json::Error` loses serialization context |
| SEC-006 | LOW | Testing | Missing `MockStoragePlugin` and `health_check` tests |
| SEC-007 | INFO | Maintenance | `async-trait` can be replaced with native async traits (MSRV 1.86) |
| SEC-008 | INFO | Code Quality | Duplicate doc comment in `registry.rs` |

---

## Recommendations Priority

1. **Fix SEC-002** — Eliminate TOCTOU race by holding locks across check-and-act sequences.
2. **Fix SEC-001** — Remove or minimize tokio features in `[dependencies]`.
3. **Fix SEC-003** — Log or propagate poisoned lock errors instead of silently returning defaults.
4. **Address SEC-006** — Add missing test coverage for storage plugin registration and health checks.
5. **Consider SEC-007** — Migrate to native async traits when convenient.
