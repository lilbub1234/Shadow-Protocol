// Shroud Framework - Plugin SDK
// Copyright (c) 2025 Shroud Protocol Contributors
// Licensed under MIT License

//! Plugin system for extending Shroud Framework

use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

/// Plugin trait that all plugins must implement
pub trait Plugin: Send + Sync {
    /// Plugin name
    fn name(&self) -> &str;

    /// Plugin version
    fn version(&self) -> Version;

    /// Initialize the plugin
    fn initialize(&mut self, context: &PluginContext) -> Result<(), PluginError>;

    /// Shutdown the plugin
    fn shutdown(&mut self) -> Result<(), PluginError>;

    /// Get plugin capabilities
    fn capabilities(&self) -> Vec<Capability>;

    /// Get plugin metadata
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: self.name().to_string(),
            version: self.version(),
            capabilities: self.capabilities(),
            author: None,
            description: None,
            homepage: None,
        }
    }
}

/// Plugin version
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }

    pub fn is_compatible(&self, required: &Version) -> bool {
        // Same major version, minor >= required
        self.major == required.major && self.minor >= required.minor
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Plugin capability
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Capability {
    /// Provides a proof system backend
    ProofSystem(String),

    /// Provides circuit gadgets
    Gadgets(Vec<String>),

    /// Provides blockchain adapter
    BlockchainAdapter(String),

    /// Provides UI theme
    UITheme(String),

    /// Provides code generator
    CodeGenerator(String),

    /// Custom capability
    Custom(String),
}

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: Version,
    pub capabilities: Vec<Capability>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub homepage: Option<String>,
}

/// Plugin context provided during initialization
pub struct PluginContext {
    /// Shroud Framework version
    pub framework_version: Version,

    /// Configuration
    pub config: HashMap<String, String>,

    /// Shared data directory
    pub data_dir: std::path::PathBuf,

    /// Logger
    pub logger: Arc<dyn PluginLogger>,
}

impl PluginContext {
    pub fn new(framework_version: Version) -> Self {
        Self {
            framework_version,
            config: HashMap::new(),
            data_dir: std::path::PathBuf::from(".shroud/plugins"),
            logger: Arc::new(DefaultLogger),
        }
    }

    pub fn set_config(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.config.insert(key.into(), value.into());
    }

    pub fn get_config(&self, key: &str) -> Option<&String> {
        self.config.get(key)
    }
}

/// Plugin logger trait
pub trait PluginLogger: Send + Sync {
    fn log(&self, level: LogLevel, message: &str);

    fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }

    fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }

    fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

struct DefaultLogger;

impl PluginLogger for DefaultLogger {
    fn log(&self, level: LogLevel, message: &str) {
        let prefix = match level {
            LogLevel::Debug => "[DEBUG]",
            LogLevel::Info => "[INFO]",
            LogLevel::Warn => "[WARN]",
            LogLevel::Error => "[ERROR]",
        };
        println!("{} {}", prefix, message);
    }
}

/// Plugin errors
#[derive(Debug)]
pub enum PluginError {
    InitializationFailed(String),
    InvalidConfig(String),
    IncompatibleVersion { required: Version, found: Version },
    MissingCapability(String),
    AlreadyLoaded(String),
    NotFound(String),
    LoadFailed(String),
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginError::InitializationFailed(msg) => write!(f, "Initialization failed: {}", msg),
            PluginError::InvalidConfig(msg) => write!(f, "Invalid config: {}", msg),
            PluginError::IncompatibleVersion { required, found } => {
                write!(f, "Incompatible version: required {}, found {}", required, found)
            }
            PluginError::MissingCapability(cap) => write!(f, "Missing capability: {}", cap),
            PluginError::AlreadyLoaded(name) => write!(f, "Plugin already loaded: {}", name),
            PluginError::NotFound(name) => write!(f, "Plugin not found: {}", name),
            PluginError::LoadFailed(msg) => write!(f, "Load failed: {}", msg),
        }
    }
}

impl std::error::Error for PluginError {}

/// Plugin registry
pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
    framework_version: Version,
}

impl PluginRegistry {
    pub fn new(framework_version: Version) -> Self {
        Self {
            plugins: HashMap::new(),
            framework_version,
        }
    }

    /// Register a plugin
    pub fn register(&mut self, mut plugin: Box<dyn Plugin>) -> Result<(), PluginError> {
        let name = plugin.name().to_string();

        // Check if already loaded
        if self.plugins.contains_key(&name) {
            return Err(PluginError::AlreadyLoaded(name));
        }

        // Check version compatibility
        let plugin_version = plugin.version();
        if !plugin_version.is_compatible(&self.framework_version) {
            return Err(PluginError::IncompatibleVersion {
                required: self.framework_version,
                found: plugin_version,
            });
        }

        // Initialize plugin
        let context = PluginContext::new(self.framework_version);
        plugin.initialize(&context)?;

        // Register
        println!("Registered plugin: {} v{}", name, plugin_version);
        self.plugins.insert(name, plugin);

        Ok(())
    }

    /// Unregister a plugin
    pub fn unregister(&mut self, name: &str) -> Result<(), PluginError> {
        if let Some(mut plugin) = self.plugins.remove(name) {
            plugin.shutdown()?;
            println!("Unregistered plugin: {}", name);
            Ok(())
        } else {
            Err(PluginError::NotFound(name.to_string()))
        }
    }

    /// Get a plugin by name
    pub fn get(&self, name: &str) -> Option<&dyn Plugin> {
        self.plugins.get(name).map(|p| p.as_ref())
    }

    /// Get all plugins
    pub fn list(&self) -> Vec<&dyn Plugin> {
        self.plugins.values().map(|p| p.as_ref()).collect()
    }

    /// Get plugins by capability
    pub fn find_by_capability(&self, capability: &Capability) -> Vec<&dyn Plugin> {
        self.plugins
            .values()
            .filter(|p| p.capabilities().contains(capability))
            .map(|p| p.as_ref())
            .collect()
    }

    /// Shutdown all plugins
    pub fn shutdown_all(&mut self) -> Result<(), PluginError> {
        for (name, mut plugin) in self.plugins.drain() {
            plugin.shutdown()?;
            println!("Shutdown plugin: {}", name);
        }
        Ok(())
    }
}

/// Example plugin: Custom hash function
pub struct CustomHashPlugin {
    name: String,
    version: Version,
}

impl CustomHashPlugin {
    pub fn new() -> Self {
        Self {
            name: "custom-hash".to_string(),
            version: Version::new(0, 1, 0),
        }
    }
}

impl Plugin for CustomHashPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> Version {
        self.version
    }

    fn initialize(&mut self, context: &PluginContext) -> Result<(), PluginError> {
        context.logger.info(&format!(
            "Initializing {} v{}",
            self.name, self.version
        ));

        // Initialization logic here
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), PluginError> {
        println!("Shutting down {} v{}", self.name, self.version);
        Ok(())
    }

    fn capabilities(&self) -> Vec<Capability> {
        vec![Capability::Gadgets(vec!["custom_hash".to_string()])]
    }
}

/// Example plugin: Custom proof system
pub struct CustomProofSystemPlugin {
    name: String,
    version: Version,
}

impl CustomProofSystemPlugin {
    pub fn new() -> Self {
        Self {
            name: "custom-proof-system".to_string(),
            version: Version::new(0, 1, 0),
        }
    }
}

impl Plugin for CustomProofSystemPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> Version {
        self.version
    }

    fn initialize(&mut self, context: &PluginContext) -> Result<(), PluginError> {
        context.logger.info(&format!(
            "Initializing {} v{}",
            self.name, self.version
        ));

        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), PluginError> {
        println!("Shutting down {} v{}", self.name, self.version);
        Ok(())
    }

    fn capabilities(&self) -> Vec<Capability> {
        vec![Capability::ProofSystem("custom".to_string())]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_compatibility() {
        let v1 = Version::new(0, 1, 0);
        let v2 = Version::new(0, 1, 5);
        let v3 = Version::new(0, 2, 0);
        let v4 = Version::new(1, 0, 0);

        assert!(v2.is_compatible(&v1));
        assert!(v3.is_compatible(&v1));
        assert!(!v1.is_compatible(&v3));
        assert!(!v4.is_compatible(&v1));
    }

    #[test]
    fn test_plugin_registry() {
        let mut registry = PluginRegistry::new(Version::new(0, 1, 0));

        let plugin = Box::new(CustomHashPlugin::new());
        assert!(registry.register(plugin).is_ok());

        assert!(registry.get("custom-hash").is_some());
        assert_eq!(registry.list().len(), 1);
    }

    #[test]
    fn test_plugin_capabilities() {
        let mut registry = PluginRegistry::new(Version::new(0, 1, 0));

        registry.register(Box::new(CustomHashPlugin::new())).unwrap();
        registry.register(Box::new(CustomProofSystemPlugin::new())).unwrap();

        let hash_plugins = registry.find_by_capability(
            &Capability::Gadgets(vec!["custom_hash".to_string()])
        );
        assert_eq!(hash_plugins.len(), 1);

        let proof_plugins = registry.find_by_capability(
            &Capability::ProofSystem("custom".to_string())
        );
        assert_eq!(proof_plugins.len(), 1);
    }

    #[test]
    fn test_plugin_lifecycle() {
        let mut registry = PluginRegistry::new(Version::new(0, 1, 0));

        let plugin = Box::new(CustomHashPlugin::new());
        registry.register(plugin).unwrap();

        assert!(registry.unregister("custom-hash").is_ok());
        assert!(registry.get("custom-hash").is_none());
    }
}
