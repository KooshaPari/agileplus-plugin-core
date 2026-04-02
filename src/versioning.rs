//! Plugin API versioning and compatibility checking.
//!
//! Provides versioning guarantees for the plugin API so that plugins can declare
//! which API versions they support, and the registry can verify compatibility
//! before loading them.

use std::fmt;

use crate::error::{PluginError, PluginResult};

/// The current plugin API version.
///
/// This follows semver: MAJOR.MINOR.PATCH
/// - MAJOR: incompatible API changes
/// - MINOR: backwards-compatible additions
/// - PATCH: backwards-compatible bug fixes
pub const CURRENT_API_VERSION: PluginApiVersion = PluginApiVersion {
    major: 0,
    minor: 1,
    patch: 0,
};

/// A plugin API version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PluginApiVersion {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl PluginApiVersion {
    /// Create a new version.
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Check if this version is compatible with another.
    ///
    /// Two versions are compatible if they share the same major version
    /// and this version's minor is >= the other's minor.
    pub fn is_compatible_with(&self, other: &PluginApiVersion) -> bool {
        self.major == other.major && self.minor >= other.minor
    }
}

impl fmt::Display for PluginApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Result of a version compatibility check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionCompatibility {
    /// Versions are compatible.
    Compatible,
    /// Plugin requires a newer API than the host provides.
    PluginRequiresNewer,
    /// Plugin requires an older major API version.
    IncompatibleMajor,
}

impl VersionCompatibility {
    /// Check if the plugin can be loaded.
    pub fn can_load(&self) -> bool {
        matches!(self, Self::Compatible)
    }
}

/// Check compatibility between the host API version and a plugin's required version.
pub fn check_compatibility(
    host_version: &PluginApiVersion,
    plugin_version: &PluginApiVersion,
) -> VersionCompatibility {
    if host_version.major == plugin_version.major {
        if host_version.minor >= plugin_version.minor {
            VersionCompatibility::Compatible
        } else {
            VersionCompatibility::PluginRequiresNewer
        }
    } else {
        VersionCompatibility::IncompatibleMajor
    }
}

/// Validate that a plugin's API version is compatible with the current host API.
pub fn validate_plugin_version(plugin_version: &PluginApiVersion) -> PluginResult<()> {
    match check_compatibility(&CURRENT_API_VERSION, plugin_version) {
        VersionCompatibility::Compatible => Ok(()),
        VersionCompatibility::PluginRequiresNewer => Err(PluginError::Version(format!(
            "plugin requires API {} but host provides {}",
            plugin_version, CURRENT_API_VERSION
        ))),
        VersionCompatibility::IncompatibleMajor => Err(PluginError::Version(format!(
            "plugin API major version {} incompatible with host {}",
            plugin_version.major, CURRENT_API_VERSION.major
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_display() {
        let v = PluginApiVersion::new(1, 2, 3);
        assert_eq!(format!("{}", v), "1.2.3");
    }

    #[test]
    fn test_compatible_versions() {
        let host = PluginApiVersion::new(1, 5, 0);
        let plugin = PluginApiVersion::new(1, 3, 0);
        assert_eq!(
            check_compatibility(&host, &plugin),
            VersionCompatibility::Compatible
        );

        let plugin = PluginApiVersion::new(1, 5, 0);
        assert_eq!(
            check_compatibility(&host, &plugin),
            VersionCompatibility::Compatible
        );
    }

    #[test]
    fn test_plugin_requires_newer() {
        let host = PluginApiVersion::new(1, 2, 0);
        let plugin = PluginApiVersion::new(1, 5, 0);
        assert_eq!(
            check_compatibility(&host, &plugin),
            VersionCompatibility::PluginRequiresNewer
        );
    }

    #[test]
    fn test_incompatible_major() {
        let host = PluginApiVersion::new(2, 0, 0);
        let plugin = PluginApiVersion::new(1, 0, 0);
        assert_eq!(
            check_compatibility(&host, &plugin),
            VersionCompatibility::IncompatibleMajor
        );
    }

    #[test]
    fn test_validate_plugin_version() {
        let compatible = PluginApiVersion::new(0, 1, 0);
        assert!(validate_plugin_version(&compatible).is_ok());

        let incompatible = PluginApiVersion::new(1, 0, 0);
        assert!(validate_plugin_version(&incompatible).is_err());
    }
}
