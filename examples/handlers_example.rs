//! # OpenAI Mock Server Example
//!
//! This example demonstrates how to start the OpenAI mock server with custom configuration.
//! The server provides fake but realistic responses for OpenAI API endpoints.
//!
//! Run this example with:
//! ```bash
//! cargo run --example handlers_example
//! ```
//!
//! Then test the endpoints with curl:
//!
//! ## Test Completion Endpoint
//! ```bash
//! curl -X POST http://localhost:13673/v1/completions \
//!   -H "Content-Type: application/json" \
//!   -H "Authorization: Bearer sk-mock-openai-api-key-12345" \
//!   -d '{
//!     "model": "text-davinci-003",
//!     "prompt": "Hello, world!",
//!     "max_tokens": 10
//!   }'
//! ```
//!
//! ## Test Embedding Endpoint
//! ```bash
//! curl -X POST http://localhost:13673/v1/embeddings \
//!   -H "Content-Type: application/json" \
//!   -H "Authorization: Bearer sk-mock-openai-api-key-12345" \
//!   -d '{
//!     "model": "text-embedding-ada-002",
//!     "input": "Hello, world!"
//!   }'
//! ```
//!
//! ## Test Health Endpoint
//! ```bash
//! curl http://localhost:13673/health
//! ```

use openai_mock::{config::Config, server::create_app_with_config};
use poem::{Server, listener::TcpListener};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    println!("🚀 Starting OpenAI Mock Server Example...");
    println!();

    // Create custom server configuration
    let config = Config::builder()
        .host("127.0.0.1") // Localhost only for this example
        .port(13673) // Custom port
        .enable_cors(true) // Enable CORS
        .enable_logging(true) // Enable request logging
        .build();

    println!("📊 Server Configuration:");
    println!("  - Host: {}", config.bind_address());
    println!("  - CORS: {}", config.enable_cors);
    println!("  - Logging: {}", config.enable_logging);
    println!();

    // Create the application with custom configuration
    let app = create_app_with_config(config.clone());

    println!("🔑 API Key: {}", config.api_key);
    println!();
    println!("📡 Available endpoints:");
    println!("  GET  /                    - Server information");
    println!("  GET  /health              - Health check");
    println!("  POST /v1/completions      - Text completion (auth required)");
    println!("  POST /v1/embeddings       - Text embeddings (auth required)");
    println!();

    println!("💡 Example requests:");
    println!("  # Health check");
    println!("  curl http://{}/health", config.bind_address());
    println!();
    println!("  # Text completion");
    println!(
        "  curl -X POST http://{}/v1/completions \\",
        config.bind_address()
    );
    println!("    -H 'Content-Type: application/json' \\");
    println!("    -H 'Authorization: Bearer sk-mock-openai-api-key-12345' \\");
    println!("    -d '{{\"model\":\"text-davinci-003\",\"prompt\":\"Hello!\",\"max_tokens\":10}}'");
    println!();
    println!("  # Text embeddings");
    println!(
        "  curl -X POST http://{}/v1/embeddings \\",
        config.bind_address()
    );
    println!("    -H 'Content-Type: application/json' \\");
    println!("    -H 'Authorization: Bearer sk-mock-openai-api-key-12345' \\");
    println!("    -d '{{\"model\":\"text-embedding-ada-002\",\"input\":\"Hello!\"}}'");
    println!();

    // Create TCP listener
    let bind_address = config.bind_address();
    let listener = TcpListener::bind(&bind_address);

    info!("🚀 Example server starting on: http://{}", bind_address);
    println!("Press Ctrl+C to stop the server");
    println!();

    // Start the server
    Server::new(listener).run(app).await.map_err(|e| {
        tracing::error!("Server error: {}", e);
        Box::new(e) as Box<dyn std::error::Error>
    })
}
