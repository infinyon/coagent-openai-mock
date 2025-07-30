//! # HTTP Server Implementation
//!
//! This module provides the HTTP server implementation using the Poem web framework.
//! It sets up routing, middleware, error handling, and CORS configuration for the
//! OpenAI mock API server.
//!
//! ## Features
//!
//! - **Routing**: Configures endpoints for completions and embeddings
//! - **Authentication**: Applies API key validation middleware
//! - **Error Handling**: Provides standardized error responses
//! - **CORS**: Enables cross-origin requests for browser clients
//! - **Graceful Shutdown**: Supports clean server shutdown
//!
//! ## Usage
//!
//! ```rust
//! use openai_mock::server::create_app;
//! use openai_mock::config::Config;
//!
//! // Create the application with default configuration
//! let app = create_app();
//!
//! // The app is now ready to be used with a server
//! println!("App created successfully");
//! ```

use poem::{
    EndpointExt, IntoResponse, Response, Result, Route,
    http::{Method, StatusCode, header},
    middleware::{Cors, Tracing},
    post,
};
use serde_json;

use crate::{
    auth::AuthMiddleware,
    config::Config,
    handlers::{create_chat_completion, create_completion, create_embedding},
};

/// Create the main application with default configuration
pub fn create_app() -> impl poem::Endpoint {
    create_app_with_default_key()
}

/// Create the application with default API key
pub fn create_app_with_default_key() -> impl poem::Endpoint {
    let protected_routes = Route::new()
        .at("/completions", post(create_completion))
        .at("/chat/completions", post(create_chat_completion))
        .at("/embeddings", post(create_embedding))
        .with(AuthMiddleware::with_default_key());

    let app = Route::new()
        .at("/health", poem::get(health_check))
        .at("/", poem::get(root_handler))
        .at("/v1/models", poem::get(list_models_handler))
        .nest("/v1", protected_routes);

    // Always apply both middleware for consistent type
    app.with(Tracing).with(create_cors_middleware())
}

/// Create the application with custom configuration
pub fn create_app_with_config(config: Config) -> impl poem::Endpoint {
    let protected_routes = Route::new()
        .at("/completions", post(create_completion))
        .at("/chat/completions", post(create_chat_completion))
        .at("/embeddings", post(create_embedding))
        .with(AuthMiddleware::new(&config.api_key));

    let app = Route::new()
        .at("/health", poem::get(health_check))
        .at("/", poem::get(root_handler))
        .at("/v1/models", poem::get(list_models_handler))
        .nest("/v1", protected_routes);

    // Always apply both middleware for consistent type
    app.with(Tracing).with(create_cors_middleware())
}

/// Create CORS middleware with OpenAI API compatible settings
fn create_cors_middleware() -> Cors {
    Cors::new()
        .allow_method(Method::GET)
        .allow_method(Method::POST)
        .allow_method(Method::OPTIONS)
        .allow_header(header::CONTENT_TYPE)
        .allow_header(header::AUTHORIZATION)
        .allow_header("x-api-key")
        .allow_origin("*")
        .max_age(86400) // 24 hours in seconds
}

/// Health check endpoint
#[poem::handler]
async fn health_check() -> Result<impl IntoResponse> {
    let health_response = serde_json::json!({
        "status": "healthy",
        "service": "openai-mock",
        "version": env!("CARGO_PKG_VERSION"),
        "endpoints": [
            "/v1/completions",
            "/v1/chat/completions",
            "/v1/embeddings"
        ]
    });

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(health_response.to_string()))
}

/// Root endpoint handler
#[poem::handler]
async fn root_handler() -> Result<impl IntoResponse> {
    let info_response = serde_json::json!({
        "message": "OpenAI Mock API Server",
        "version": env!("CARGO_PKG_VERSION"),
        "endpoints": {
            "completions": "/v1/completions",
            "chat_completions": "/v1/chat/completions",
            "embeddings": "/v1/embeddings",
            "health": "/health"
        },
        "authentication": {
            "type": "Bearer",
            "header": "Authorization",
            "api_key": "sk-mock-openai-api-key-12345"
        },
        "documentation": "https://platform.openai.com/docs/api-reference"
    });

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(info_response.to_string()))
}

/// Handler for GET /v1/models
#[poem::handler]
async fn list_models_handler() -> Result<impl IntoResponse> {
    // Example static models, can be expanded as needed
    let models = serde_json::json!({
        "object": "list",
        "data": [
            {
                "id": "text-davinci-003",
                "object": "model",
                "created": 1677610602,
                "owned_by": "openai",
                "permission": [],
                "root": "text-davinci-003",
                "parent": null
            },
            {
                "id": "gpt-3.5-turbo",
                "object": "model",
                "created": 1677610602,
                "owned_by": "openai",
                "permission": [],
                "root": "gpt-3.5-turbo",
                "parent": null
            },
            {
                "id": "gpt-4",
                "object": "model",
                "created": 1677610602,
                "owned_by": "openai",
                "permission": [],
                "root": "gpt-4",
                "parent": null
            },
            {
                "id": "text-embedding-ada-002",
                "object": "model",
                "created": 1677610602,
                "owned_by": "openai",
                "permission": [],
                "root": "text-embedding-ada-002",
                "parent": null
            }
        ]
    });

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(models.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use poem::test::TestClient;

    #[tokio::test]
    async fn test_health_check() {
        let app = create_app();
        let cli = TestClient::new(app);

        let resp = cli.get("/health").send().await;
        resp.assert_status_is_ok();
        resp.assert_header("content-type", "application/json");

        let body = resp.0.into_body().into_string().await.unwrap();
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();

        assert_eq!(json["status"], "healthy");
        assert_eq!(json["service"], "openai-mock");
        assert!(json["endpoints"].is_array());
    }

    #[tokio::test]
    async fn test_root_handler() {
        let app = create_app();
        let cli = TestClient::new(app);

        let resp = cli.get("/").send().await;
        resp.assert_status_is_ok();
        resp.assert_header("content-type", "application/json");

        let body = resp.0.into_body().into_string().await.unwrap();
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();

        assert_eq!(json["message"], "OpenAI Mock API Server");
        assert!(json["endpoints"].is_object());
        assert!(json["authentication"].is_object());
    }

    #[tokio::test]
    async fn test_authentication_required() {
        let app = create_app();
        let cli = TestClient::new(app);

        let resp = cli
            .post("/v1/completions")
            .header("content-type", "application/json")
            .body(r#"{"model":"test","prompt":"test"}"#)
            .send()
            .await;

        resp.assert_status(StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_with_valid_authentication() {
        let app = create_app();
        let cli = TestClient::new(app);

        let resp = cli
            .post("/v1/completions")
            .header("content-type", "application/json")
            .header("authorization", "Bearer sk-mock-openai-api-key-12345")
            .body(r#"{"model":"text-davinci-003","prompt":"Hello"}"#)
            .send()
            .await;

        resp.assert_status_is_ok();
    }

    #[tokio::test]
    async fn test_chat_completions_with_valid_authentication() {
        let app = create_app();
        let cli = TestClient::new(app);

        let resp = cli
            .post("/v1/chat/completions")
            .header("content-type", "application/json")
            .header("authorization", "Bearer sk-mock-openai-api-key-12345")
            .body(r#"{"model":"gpt-3.5-turbo","messages":[{"role":"user","content":"Hello"}]}"#)
            .send()
            .await;

        resp.assert_status_is_ok();
    }

    #[tokio::test]
    async fn test_server_config() {
        let config = Config::builder()
            .host("127.0.0.1")
            .port(13673)
            .request_timeout_secs(60)
            .enable_cors(false)
            .enable_logging(false)
            .build();

        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 13673);
        assert_eq!(config.request_timeout_secs, 60);
        assert!(!config.enable_cors);
        assert!(!config.enable_logging);
        assert_eq!(config.bind_address(), "127.0.0.1:13673");
    }

    #[tokio::test]
    async fn test_app_with_custom_config() {
        let config = Config::builder().api_key("sk-test-key").build();
        let app = create_app_with_config(config);
        let cli = TestClient::new(app);

        let resp = cli.get("/health").send().await;
        resp.assert_status_is_ok();
    }

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 13673);
        assert!(config.enable_cors);
        assert!(config.enable_logging);
    }
}
