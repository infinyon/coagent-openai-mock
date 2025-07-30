//! # Configuration Module
//!
//! This module provides configuration management for the OpenAI Mock Server.
//! It supports loading configuration from command line arguments with sensible
//! defaults for development and production use.
//!
//! ## Command Line Arguments
//!
//! - `--host`: Server host address (default: 0.0.0.0)
//! - `--port`: Server port (default: 13673)
//! - `--api-key`: API key for authentication (default: sk-mock-openai-api-key-12345)
//! - `--request-timeout-secs`: Request timeout in seconds (default: 30)
//! - `--enable-cors`: Enable CORS support (default: true)
//! - `--enable-logging`: Enable request logging (default: true)
//! - `--log-level`: Logging level (default: info)
//!
//! ## Usage
//!
//! ```rust
//! use openai_mock::config::Config;
//! use clap::Parser;
//!
//! // Load configuration from CLI arguments
//! let config = Config::parse();
//!

use clap::Parser;
use std::time::Duration;

const DEFAULT_HOST: &str = "0.0.0.0";
const DEFAULT_PORT: u16 = 13673;
const DEFAULT_API_KEY: &str = "sk-mock-openai-api-key-12345";
const DEFAULT_REQUEST_TIMEOUT_SECS: u64 = 30;
const DEFAULT_ENABLE_CORS: bool = true;
const DEFAULT_ENABLE_LOGGING: bool = true;
const DEFAULT_LOG_LEVEL: &str = "info";

/// Server configuration with all settings
#[derive(Debug, Clone, Parser)]
#[command(name = "openai-mock")]
#[command(about = "A mock OpenAI API server for testing and development")]
#[command(version)]
pub struct Config {
    /// Server host address
    #[arg(long, default_value = DEFAULT_HOST)]
    pub host: String,

    /// Server port
    #[arg(long, default_value_t = DEFAULT_PORT)]
    pub port: u16,

    /// API key for authentication
    #[arg(long, default_value = DEFAULT_API_KEY)]
    pub api_key: String,

    /// Request timeout duration in seconds
    #[arg(long, default_value_t = DEFAULT_REQUEST_TIMEOUT_SECS)]
    pub request_timeout_secs: u64,

    /// Enable CORS middleware
    #[arg(long, default_value_t = DEFAULT_ENABLE_CORS, action = clap::ArgAction::Set)]
    pub enable_cors: bool,

    /// Enable request logging middleware
    #[arg(long, default_value_t = DEFAULT_ENABLE_LOGGING, action = clap::ArgAction::Set)]
    pub enable_logging: bool,

    /// Logging level
    #[arg(long, default_value = DEFAULT_LOG_LEVEL)]
    pub log_level: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST.to_string(),
            port: DEFAULT_PORT,
            api_key: DEFAULT_API_KEY.to_string(),
            request_timeout_secs: DEFAULT_REQUEST_TIMEOUT_SECS,
            enable_cors: DEFAULT_ENABLE_CORS,
            enable_logging: DEFAULT_ENABLE_LOGGING,
            log_level: DEFAULT_LOG_LEVEL.to_string(),
        }
    }
}

/// Configuration builder for programmatic config creation
#[derive(Debug, Clone)]
pub struct ConfigBuilder {
    config: Config,
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigBuilder {
    /// Create a new ConfigBuilder with default values
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    /// Set the host address
    pub fn host<S: Into<String>>(mut self, host: S) -> Self {
        self.config.host = host.into();
        self
    }

    /// Set the port
    pub fn port(mut self, port: u16) -> Self {
        self.config.port = port;
        self
    }

    /// Set the API key
    pub fn api_key<S: Into<String>>(mut self, api_key: S) -> Self {
        self.config.api_key = api_key.into();
        self
    }

    /// Set the request timeout in seconds
    pub fn request_timeout_secs(mut self, secs: u64) -> Self {
        self.config.request_timeout_secs = secs;
        self
    }

    /// Set CORS enablement
    pub fn enable_cors(mut self, enable: bool) -> Self {
        self.config.enable_cors = enable;
        self
    }

    /// Set logging enablement
    pub fn enable_logging(mut self, enable: bool) -> Self {
        self.config.enable_logging = enable;
        self
    }

    /// Set log level
    pub fn log_level<S: Into<String>>(mut self, level: S) -> Self {
        self.config.log_level = level.into();
        self
    }

    /// Build the configuration
    pub fn build(self) -> Config {
        self.config
    }

    /// Build and validate the configuration
    pub fn build_validated(self) -> Result<Config, ConfigError> {
        let config = self.config;
        config.validate()?;
        Ok(config)
    }
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder {
            config: Config::default(),
        }
    }

    /// Get the request timeout as Duration
    pub fn request_timeout(&self) -> Duration {
        Duration::from_secs(self.request_timeout_secs)
    }

    /// Get the server bind address
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /// Get the base URL for the server
    pub fn base_url(&self) -> String {
        let host = if self.host == "0.0.0.0" {
            "localhost"
        } else {
            &self.host
        };
        format!("http://{}:{}", host, self.port)
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate port range
        if self.port == 0 {
            return Err(ConfigError::InvalidPort("Port cannot be 0".to_string()));
        }

        // Validate API key format
        if self.api_key.is_empty() {
            return Err(ConfigError::InvalidApiKey(
                "API key cannot be empty".to_string(),
            ));
        }

        if !self.api_key.starts_with("sk-") {
            return Err(ConfigError::InvalidApiKey(
                "API key should start with 'sk-'".to_string(),
            ));
        }

        // Validate timeout
        if self.request_timeout_secs == 0 {
            return Err(ConfigError::InvalidTimeout(
                "Timeout cannot be 0".to_string(),
            ));
        }

        // Validate log level
        match self.log_level.to_lowercase().as_str() {
            "trace" | "debug" | "info" | "warn" | "error" => {}
            _ => {
                return Err(ConfigError::InvalidLogLevel(format!(
                    "Invalid log level: '{}'. Valid levels: trace, debug, info, warn, error",
                    self.log_level
                )));
            }
        }

        Ok(())
    }

    /// Print configuration summary
    pub fn print_summary(&self) {
        println!("Configuration Summary:");
        println!("  Host: {}", self.host);
        println!("  Port: {}", self.port);
        println!("  Bind Address: {}", self.bind_address());
        println!("  Base URL: {}", self.base_url());
        println!(
            "  API Key: {}***",
            &self.api_key[..self.api_key.len().min(10)]
        );
        println!("  Request Timeout: {}s", self.request_timeout_secs);
        println!("  CORS Enabled: {}", self.enable_cors);
        println!("  Logging Enabled: {}", self.enable_logging);
        println!("  Log Level: {}", self.log_level);
    }
}

/// Configuration error types
#[derive(Debug, Clone)]
pub enum ConfigError {
    InvalidPort(String),
    InvalidApiKey(String),
    InvalidTimeout(String),
    InvalidLogLevel(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::InvalidPort(msg) => write!(f, "Invalid port: {msg}"),
            ConfigError::InvalidApiKey(msg) => write!(f, "Invalid API key: {msg}"),
            ConfigError::InvalidTimeout(msg) => write!(f, "Invalid timeout: {msg}"),
            ConfigError::InvalidLogLevel(msg) => write!(f, "Invalid log level: {msg}"),
        }
    }
}

impl std::error::Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.host, DEFAULT_HOST);
        assert_eq!(config.port, DEFAULT_PORT);
        assert_eq!(config.api_key, DEFAULT_API_KEY);
        assert_eq!(config.request_timeout_secs, DEFAULT_REQUEST_TIMEOUT_SECS);
        assert_eq!(config.enable_cors, DEFAULT_ENABLE_CORS);
        assert_eq!(config.enable_logging, DEFAULT_ENABLE_LOGGING);
        assert_eq!(config.log_level, DEFAULT_LOG_LEVEL);
    }

    #[test]
    fn test_bind_address() {
        let config = Config::builder().host("127.0.0.1").port(13673).build();
        assert_eq!(config.bind_address(), "127.0.0.1:13673");
    }

    #[test]
    fn test_base_url() {
        let config = Config::builder().host("127.0.0.1").port(13673).build();
        assert_eq!(config.base_url(), "http://127.0.0.1:13673");

        let config = Config::builder().host("0.0.0.0").port(13673).build();
        assert_eq!(config.base_url(), "http://localhost:13673");
    }

    #[test]
    fn test_config_validation() {
        // Valid config
        let config = Config::default();
        assert!(config.validate().is_ok());

        // Invalid port
        let config = Config::builder().port(0).build();
        assert!(matches!(
            config.validate(),
            Err(ConfigError::InvalidPort(_))
        ));

        // Invalid API key (empty)
        let config = Config::builder().api_key("").build();
        assert!(matches!(
            config.validate(),
            Err(ConfigError::InvalidApiKey(_))
        ));

        // Invalid API key (wrong format)
        let config = Config::builder().api_key("invalid-key").build();
        assert!(matches!(
            config.validate(),
            Err(ConfigError::InvalidApiKey(_))
        ));

        // Invalid timeout
        let config = Config::builder().request_timeout_secs(0).build();
        assert!(matches!(
            config.validate(),
            Err(ConfigError::InvalidTimeout(_))
        ));

        // Invalid log level
        let config = Config::builder().log_level("invalid").build();
        assert!(matches!(
            config.validate(),
            Err(ConfigError::InvalidLogLevel(_))
        ));
    }

    #[test]
    fn test_request_timeout() {
        let config = Config::builder().request_timeout_secs(60).build();
        assert_eq!(config.request_timeout_secs, 60);
        assert_eq!(config.request_timeout(), Duration::from_secs(60));
    }

    #[test]
    fn test_build_validated() {
        // Valid config should build successfully
        let result = Config::builder()
            .host("127.0.0.1")
            .port(13673)
            .api_key("sk-valid-key")
            .build_validated();
        assert!(result.is_ok());

        // Invalid config should fail
        let result = Config::builder().port(0).build_validated();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_print_summary() {
        let config = Config::builder()
            .host("127.0.0.1")
            .port(13673)
            .api_key("sk-test-key-123456789")
            .build();

        // This test just ensures print_summary doesn't panic
        config.print_summary();
    }
}
