//! # Embeddings Handler
//!
//! This module implements the HTTP handler for the OpenAI embeddings endpoint (`/v1/embeddings`).
//! It handles POST requests, validates the request body, and generates fake embedding responses
//! that conform to the OpenAI API specification.
//!
//! ## Features
//!
//! - **Request Validation**: Validates incoming embedding requests against OpenAI schema
//! - **Error Handling**: Returns proper OpenAI-formatted error responses for invalid requests
//! - **Response Generation**: Uses the EmbeddingGenerator to create realistic fake responses
//! - **Content Type Handling**: Properly handles JSON request/response content types
//! - **Multiple Input Types**: Supports string, string array, integer array, and nested integer array inputs
//!
//! ## Usage
//!
//! This handler is typically mounted at the `/v1/embeddings` route with authentication middleware:
//!
//! ```rust
//! use poem::{Route, post, EndpointExt};
//! use openai_mock::handlers::embeddings::create_embedding;
//! use openai_mock::auth::AuthMiddleware;
//!
//! let app = Route::new()
//!     .at("/v1/embeddings", post(create_embedding))
//!     .with(AuthMiddleware::new("sk-mock-openai-api-key-12345"));
//! ```

use poem::{IntoResponse, Response, Result, handler, http::StatusCode, web::Json};
use serde_json;

use crate::generators::EmbeddingGenerator;
use crate::models::{requests::CreateEmbeddingRequest, responses::ErrorResponse};

/// Handler for POST /v1/embeddings
///
/// This handler processes embedding requests and returns fake embedding responses.
/// The request body must be valid JSON that deserializes to a CreateEmbeddingRequest.
///
/// ## Request Format
///
/// The request must include:
/// - `model`: The model to use for embeddings
/// - `input`: The input text(s) to embed (string, string array, or integer arrays)
/// - Other optional parameters like `dimensions`, `encoding_format`, etc.
///
/// ## Response Format
///
/// Returns a JSON response with:
/// - `object`: Always "list"
/// - `data`: Array of embedding objects
/// - `model`: The model used
/// - `usage`: Token usage statistics
///
/// Each embedding object contains:
/// - `object`: Always "embedding"
/// - `embedding`: Array of floating-point values (the embedding vector)
/// - `index`: Index of this embedding in the list
///
/// ## Error Handling
///
/// Returns appropriate HTTP status codes and OpenAI-formatted error responses for:
/// - Invalid JSON (400 Bad Request)
/// - Missing required fields (400 Bad Request)
/// - Invalid parameter values (400 Bad Request)
///
/// # Examples
///
/// ## Successful Request
///
/// ```json
/// POST /v1/embeddings
/// Content-Type: application/json
/// Authorization: Bearer sk-mock-openai-api-key-12345
///
/// {
///   "model": "text-embedding-ada-002",
///   "input": "Hello, world!"
/// }
/// ```
///
/// ## Successful Response
///
/// ```json
/// {
///   "object": "list",
///   "data": [{
///     "object": "embedding",
///     "embedding": [0.1, -0.2, 0.3, ...],
///     "index": 0
///   }],
///   "model": "text-embedding-ada-002",
///   "usage": {
///     "prompt_tokens": 4,
///     "total_tokens": 4
///   }
/// }
/// ```
///
/// ## Error Response
///
/// ```json
/// {
///   "error": {
///     "message": "You must provide a model parameter",
///     "type": "invalid_request_error",
///     "param": "model",
///     "code": null
///   }
/// }
/// ```
#[handler]
pub async fn create_embedding(body: Json<CreateEmbeddingRequest>) -> Result<impl IntoResponse> {
    // Extract the request from the JSON wrapper
    let request = body.0;

    // Validate the request
    if let Err(validation_error) = request.validate() {
        let error_response = ErrorResponse::invalid_request_error(validation_error);
        let json_body = match serde_json::to_string(&error_response) {
            Ok(json) => json,
            Err(_) => {
                return Ok(create_internal_error_response());
            }
        };

        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("content-type", "application/json")
            .body(json_body));
    }

    // Generate the embedding response
    let embedding_response = EmbeddingGenerator::generate_response(&request);

    // Serialize the response to JSON
    let json_body = match serde_json::to_string(&embedding_response) {
        Ok(json) => json,
        Err(_) => {
            return Ok(create_internal_error_response());
        }
    };

    // Return the successful response
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(json_body))
}

/// Create a generic internal server error response
fn create_internal_error_response() -> Response {
    let error_response = ErrorResponse::new(
        "Internal server error occurred while processing the request".to_string(),
        "server_error".to_string(),
    );

    let json_body = match serde_json::to_string(&error_response) {
        Ok(json) => json,
        Err(_) => {
            r#"{"error":{"message":"Internal server error","type":"server_error"}}"#.to_string()
        }
    };

    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header("content-type", "application/json")
        .body(json_body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::requests::{CreateEmbeddingRequest, EmbeddingInput};

    fn create_test_request() -> CreateEmbeddingRequest {
        CreateEmbeddingRequest {
            model: "text-embedding-ada-002".to_string(),
            input: EmbeddingInput::String("Hello, world!".to_string()),
            encoding_format: None,
            dimensions: None,
            user: None,
        }
    }

    #[test]
    fn test_create_embedding_request_validation() {
        let valid_request = create_test_request();
        assert!(valid_request.validate().is_ok());

        let mut invalid_request = create_test_request();
        invalid_request.model = "".to_string();
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_internal_error_response() {
        let response = create_internal_error_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_embedding_generator_integration() {
        let request = create_test_request();
        let response = EmbeddingGenerator::generate_response(&request);

        assert_eq!(response.object, "list");
        assert_eq!(response.model, "text-embedding-ada-002");
        assert!(!response.data.is_empty());
        assert!(response.usage.total_tokens > 0);
    }

    #[test]
    fn test_embedding_multiple_inputs() {
        let mut request = create_test_request();
        request.input =
            EmbeddingInput::StringArray(vec!["First text".to_string(), "Second text".to_string()]);

        let response = EmbeddingGenerator::generate_response(&request);
        assert_eq!(response.data.len(), 2);
    }

    #[test]
    fn test_embedding_custom_dimensions() {
        let mut request = create_test_request();
        request.dimensions = Some(512);

        let response = EmbeddingGenerator::generate_response(&request);
        assert_eq!(response.data[0].embedding.len(), 512);
    }
}
