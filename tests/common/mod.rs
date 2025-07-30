//! Common test utilities for integration tests
//!
//! This module provides helper functions for setting up test servers
//! with free ports using the portpicker crate.

use openai_mock::{config::Config, server::create_app_with_config};
use poem::{Server, listener::TcpListener};
use std::time::Duration;
use tokio::time::timeout;

/// Creates a test server configuration with a free port
///
/// Uses portpicker to select an unused port to avoid conflicts
/// in test environments or when running tests in parallel.
pub fn create_test_config() -> (Config, u16) {
    let port = portpicker::pick_unused_port().expect("Failed to pick unused port");
    let config = Config::builder()
        .host("127.0.0.1")
        .port(port)
        .api_key("sk-test-integration-key")
        .build();

    (config, port)
}

/// Creates a test server with a free port and returns the config, base URL, and port
pub fn create_test_server() -> (Config, String, u16) {
    let (config, port) = create_test_config();
    let base_url = format!("http://127.0.0.1:{port}");

    (config, base_url, port)
}

/// Creates a test server with custom API key
pub fn create_test_server_with_key(api_key: &str) -> (Config, String, u16) {
    let port = portpicker::pick_unused_port().expect("Failed to pick unused port");
    let config = Config::builder()
        .host("127.0.0.1")
        .port(port)
        .api_key(api_key)
        .build();

    let base_url = format!("http://127.0.0.1:{port}");

    (config, base_url, port)
}

/// Helper function to wait for server to be ready
///
/// Polls the health endpoint until it responds successfully or times out.
/// Useful for ensuring the server is fully started before running tests.
pub async fn wait_for_server_ready(base_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let health_url = format!("{base_url}/health");

    // Try to connect for up to 5 seconds
    let result = timeout(Duration::from_secs(5), async {
        loop {
            match client.get(&health_url).send().await {
                Ok(response) if response.status().is_success() => break,
                _ => tokio::time::sleep(Duration::from_millis(50)).await,
            }
        }
    })
    .await;

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err("Server did not become ready in time".into()),
    }
}

/// Test server wrapper that automatically cleans up when dropped
pub struct TestServer {
    pub base_url: String,
    pub port: u16,
    server_handle: Option<tokio::task::JoinHandle<()>>,
}

impl TestServer {
    /// Creates and starts a new test server
    pub async fn start() -> Result<Self, Box<dyn std::error::Error>> {
        let (config, base_url, port) = create_test_server();
        let app = create_app_with_config(config.clone());
        let listener = TcpListener::bind(config.bind_address());

        let server_handle = tokio::spawn(async move {
            if let Err(e) = Server::new(listener).run(app).await {
                eprintln!("Test server error: {e}");
            }
        });

        wait_for_server_ready(&base_url).await?;

        Ok(TestServer {
            base_url,
            port,
            server_handle: Some(server_handle),
        })
    }

    /// Creates and starts a new test server with custom API key
    pub async fn start_with_key(api_key: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (config, base_url, port) = create_test_server_with_key(api_key);
        let app = create_app_with_config(config.clone());
        let listener = TcpListener::bind(config.bind_address());

        let server_handle = tokio::spawn(async move {
            if let Err(e) = Server::new(listener).run(app).await {
                eprintln!("Test server error: {e}");
            }
        });

        wait_for_server_ready(&base_url).await?;

        Ok(TestServer {
            base_url,
            port,
            server_handle: Some(server_handle),
        })
    }

    /// Returns the base URL of the server
    pub fn url(&self) -> &str {
        &self.base_url
    }

    /// Returns the port the server is running on
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Manually stop the server
    pub fn stop(&mut self) {
        if let Some(handle) = self.server_handle.take() {
            handle.abort();
        }
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Picks multiple unused ports for tests that need multiple servers
pub fn pick_multiple_ports(count: usize) -> Vec<u16> {
    let mut ports = Vec::with_capacity(count);
    for _ in 0..count {
        if let Some(port) = portpicker::pick_unused_port() {
            ports.push(port);
        } else {
            panic!("Failed to pick unused port");
        }
    }
    ports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pick_unused_port() {
        let port = portpicker::pick_unused_port().expect("Should pick a port");
        assert!(port > 1024, "Port should be above 1024");
    }

    #[test]
    fn test_pick_multiple_ports() {
        let ports = pick_multiple_ports(3);
        assert_eq!(ports.len(), 3);

        // All ports should be different
        for i in 0..ports.len() {
            for j in i + 1..ports.len() {
                assert_ne!(ports[i], ports[j], "Ports should be unique");
            }
        }
    }

    #[test]
    fn test_create_test_config() {
        let (config, port) = create_test_config();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, port);
        assert_eq!(config.api_key, "sk-test-integration-key");
        assert!(port > 1024);
    }

    #[test]
    fn test_create_test_server() {
        let (_config, base_url, port) = create_test_server();
        assert_eq!(base_url, format!("http://127.0.0.1:{port}"));
        assert!(port > 1024);
    }

    #[test]
    fn test_create_test_server_with_key() {
        let custom_key = "sk-custom-test-key";
        let (_config, base_url, port) = create_test_server_with_key(custom_key);
        assert_eq!(base_url, format!("http://127.0.0.1:{port}"));
        assert!(port > 1024);
    }

    #[tokio::test]
    async fn test_test_server_wrapper() {
        let server = TestServer::start().await.expect("Server should start");
        assert!(server.url().starts_with("http://127.0.0.1:"));
        assert!(server.port() > 1024);

        // Server should be ready
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/health", server.url()))
            .send()
            .await
            .expect("Health check should work");

        assert!(response.status().is_success());

        // Server will be stopped automatically when dropped
    }

    #[tokio::test]
    async fn test_test_server_with_custom_key() {
        let custom_key = "sk-my-custom-key";
        let server = TestServer::start_with_key(custom_key)
            .await
            .expect("Server should start");

        // Test that the server is using the custom key
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/v1/completions", server.url()))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {custom_key}"))
            .json(&serde_json::json!({
                "model": "text-davinci-003",
                "prompt": "Test"
            }))
            .send()
            .await
            .expect("Request should work");

        assert!(response.status().is_success());
    }
}
