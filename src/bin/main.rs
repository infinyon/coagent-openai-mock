use clap::Parser;
use poem::{Server, listener::TcpListener};
use std::env;
use tracing::info;

use openai_mock::{config::Config, server::create_app_with_config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    // Load configuration from command line arguments
    let config = Config::parse();

    // Validate configuration
    if let Err(e) = config.validate() {
        eprintln!("Configuration error: {e}");
        std::process::exit(1);
    }

    // Create the application with routes and middleware
    let app = create_app_with_config(config.clone());

    // Print startup information
    print_startup_info(&config);

    // Create TCP listener
    let bind_address = config.bind_address();
    let listener = TcpListener::bind(&bind_address);

    // Start the server
    info!("🚀 Starting OpenAI Mock Server...");
    info!("📡 Server listening on: http://{}", config.bind_address());

    Server::new(listener).run(app).await.map_err(|e| {
        tracing::error!("Server error: {}", e);
        Box::new(e) as Box<dyn std::error::Error>
    })
}

/// Print startup information to the console
fn print_startup_info(config: &Config) {
    println!();
    println!("🤖 OpenAI Mock Server");
    println!();
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("Address: {}", config.base_url());
    println!("API Key: {}", config.api_key);
    println!();
    println!("Available Endpoints:");
    println!("  GET  /                   - Server info");
    println!("  GET  /health             - Health check");
    println!("  POST /v1/completions     - Text completion");
    println!("  POST /v1/embeddings      - Text embeddings");
    println!();
    println!();
    println!("Press Ctrl+C to stop the server");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_startup_info_formatting() {
        let config = Config::builder().host("127.0.0.1").port(13673).build();

        // Test that print_startup_info doesn't panic
        print_startup_info(&config);

        // Test bind address formatting
        assert_eq!(config.bind_address(), "127.0.0.1:13673");
    }

    #[test]
    fn test_config_from_args() {
        // Test that we can create a config using the builder
        // (CLI args testing would require more complex setup)
        let config = Config::builder().host("127.0.0.1").port(13673).build();

        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 13673);
    }

    #[test]
    fn test_config_validation() {
        let config = Config::default();
        assert!(config.validate().is_ok());

        let invalid_config = Config::builder().port(0).build();
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_config_builder() {
        let config = Config::builder()
            .host("localhost")
            .port(13673)
            .api_key("sk-test-key")
            .build();

        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 13673);
        assert_eq!(config.api_key, "sk-test-key");
    }
}
