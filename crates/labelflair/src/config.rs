//! Configuration for Labelflair

use self::v1::ConfigV1;

pub mod v1;

/// Configuration for Labelflair
///
/// This enum represents the configuration for Labelflair. The configuration is versioned to allow
/// for future extensions and changes without breaking existing configurations.
pub enum Configuration {
    /// Version 1
    V1(ConfigV1),
}
