//! # OpenAI Mock Server
//!
//! A lightweight HTTP server that mimics the OpenAI API for testing and development purposes.
//! This server provides fake but realistic responses for completion and embedding endpoints
//! while maintaining compatibility with standard OpenAI client libraries.
//!
//! ## Features
//!
//! - **Completion API**: `/v1/completions` endpoint with realistic fake responses
//! - **Embedding API**: `/v1/embeddings` endpoint with consistent fake vectors
//! - **Authentication**: API key validation using Bearer token authentication
//! - **CORS Support**: Cross-origin requests for browser-based applications
//! - **Health Checks**: Built-in health monitoring endpoint
//!
//! ## Usage
//!
//! Start the server with default settings:
//! ```bash
//! cargo run
//! ```
//!
//! The server will be available at `http://localhost:3000` with the following endpoints:
//! - `GET /` - Server information
//! - `GET /health` - Health check
//! - `POST /v1/completions` - Text completion (requires authentication)
//! - `POST /v1/embeddings` - Text embeddings (requires authentication)
//!
//! ## Authentication
//!
//! All API endpoints require authentication using the hardcoded API key:
//! ```text
//! Authorization: Bearer sk-mock-openai-api-key-12345
//! ```
//!
//! ## Environment Variables
//!
//! - `HOST`: Server host address (default: 0.0.0.0)
//! - `PORT`: Server port (default: 3000)
//! - `RUST_LOG`: Logging level (default: info)

pub mod auth;
pub mod config;
pub mod generators;
pub mod handlers;
pub mod models;
pub mod server;
