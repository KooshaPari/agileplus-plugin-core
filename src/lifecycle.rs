//! Plugin lifecycle state machine.
//!
//! Defines the states a plugin transitions through during its lifetime:
//! `Unloaded` -> `Loading` -> `Loaded` -> `Starting` -> `Running` -> `Stopping` -> `Stopped`
//!
//! Invalid transitions return an error.

use std::fmt;

use crate::error::{PluginError, PluginResult};

/// Lifecycle states for a plugin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginState {
    /// Plugin is not loaded.
    Unloaded,
    /// Plugin is being loaded (reading manifest, validating).
    Loading,
    /// Plugin is loaded but not started.
    Loaded,
    /// Plugin is starting (initializing resources).
    Starting,
    /// Plugin is running and accepting requests.
    Running,
    /// Plugin is stopping (cleaning up resources).
    Stopping,
    /// Plugin has stopped cleanly.
    Stopped,
    /// Plugin encountered an error.
    Error,
}

impl fmt::Display for PluginState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unloaded => write!(f, "unloaded"),
            Self::Loading => write!(f, "loading"),
            Self::Loaded => write!(f, "loaded"),
            Self::Starting => write!(f, "starting"),
            Self::Running => write!(f, "running"),
            Self::Stopping => write!(f, "stopping"),
            Self::Stopped => write!(f, "stopped"),
            Self::Error => write!(f, "error"),
        }
    }
}

impl PluginState {
    /// Transition to the next state.
    ///
    /// Returns an error if the transition is invalid.
    pub fn transition(&self, next: PluginState) -> PluginResult<PluginState> {
        let valid = matches!(
            (self, next),
            (Self::Unloaded, Self::Loading)
                | (Self::Loading, Self::Loaded)
                | (Self::Loading, Self::Error)
                | (Self::Loaded, Self::Starting)
                | (Self::Loaded, Self::Unloaded)
                | (Self::Starting, Self::Running)
                | (Self::Starting, Self::Error)
                | (Self::Running, Self::Stopping)
                | (Self::Running, Self::Error)
                | (Self::Stopping, Self::Stopped)
                | (Self::Stopping, Self::Error)
                | (Self::Stopped, Self::Unloaded)
                | (Self::Error, Self::Unloaded)
        );

        if valid {
            Ok(next)
        } else {
            Err(PluginError::Lifecycle(format!(
                "invalid transition: {} -> {}",
                self, next
            )))
        }
    }

    /// Check if this state indicates the plugin is active.
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Running | Self::Starting | Self::Stopping)
    }

    /// Check if this state indicates the plugin is healthy.
    pub fn is_healthy(&self) -> bool {
        matches!(self, Self::Running | Self::Loaded)
    }
}

/// Manages the lifecycle state of a single plugin.
#[derive(Debug)]
pub struct LifecycleManager {
    state: PluginState,
}

impl LifecycleManager {
    /// Create a new lifecycle manager in the unloaded state.
    pub fn new() -> Self {
        Self {
            state: PluginState::Unloaded,
        }
    }

    /// Get the current state.
    pub fn state(&self) -> PluginState {
        self.state
    }

    /// Attempt to transition to a new state.
    pub fn transition(&mut self, next: PluginState) -> PluginResult<()> {
        self.state = self.state.transition(next)?;
        Ok(())
    }

    /// Check if the plugin is active.
    pub fn is_active(&self) -> bool {
        self.state.is_active()
    }

    /// Check if the plugin is healthy.
    pub fn is_healthy(&self) -> bool {
        self.state.is_healthy()
    }
}

impl Default for LifecycleManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transitions() {
        let state = PluginState::Unloaded;
        assert_eq!(
            state.transition(PluginState::Loading).unwrap(),
            PluginState::Loading
        );

        let state = PluginState::Loading;
        assert_eq!(
            state.transition(PluginState::Loaded).unwrap(),
            PluginState::Loaded
        );

        let state = PluginState::Loaded;
        assert_eq!(
            state.transition(PluginState::Starting).unwrap(),
            PluginState::Starting
        );

        let state = PluginState::Starting;
        assert_eq!(
            state.transition(PluginState::Running).unwrap(),
            PluginState::Running
        );

        let state = PluginState::Running;
        assert_eq!(
            state.transition(PluginState::Stopping).unwrap(),
            PluginState::Stopping
        );

        let state = PluginState::Stopping;
        assert_eq!(
            state.transition(PluginState::Stopped).unwrap(),
            PluginState::Stopped
        );
    }

    #[test]
    fn test_error_transitions() {
        let state = PluginState::Loading;
        assert_eq!(
            state.transition(PluginState::Error).unwrap(),
            PluginState::Error
        );

        let state = PluginState::Error;
        assert_eq!(
            state.transition(PluginState::Unloaded).unwrap(),
            PluginState::Unloaded
        );
    }

    #[test]
    fn test_invalid_transitions() {
        let state = PluginState::Unloaded;
        assert!(state.transition(PluginState::Running).is_err());

        let state = PluginState::Running;
        assert!(state.transition(PluginState::Loaded).is_err());

        let state = PluginState::Stopped;
        assert!(state.transition(PluginState::Running).is_err());
    }

    #[test]
    fn test_lifecycle_manager() {
        let mut manager = LifecycleManager::new();
        assert_eq!(manager.state(), PluginState::Unloaded);
        assert!(!manager.is_active());
        assert!(!manager.is_healthy());

        manager.transition(PluginState::Loading).unwrap();
        manager.transition(PluginState::Loaded).unwrap();
        assert!(manager.is_healthy());
        assert!(!manager.is_active());

        manager.transition(PluginState::Starting).unwrap();
        manager.transition(PluginState::Running).unwrap();
        assert!(manager.is_active());
        assert!(manager.is_healthy());

        manager.transition(PluginState::Stopping).unwrap();
        assert!(manager.is_active());
        assert!(!manager.is_healthy());

        manager.transition(PluginState::Stopped).unwrap();
        assert!(!manager.is_active());
        assert!(!manager.is_healthy());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", PluginState::Unloaded), "unloaded");
        assert_eq!(format!("{}", PluginState::Running), "running");
        assert_eq!(format!("{}", PluginState::Error), "error");
    }
}
