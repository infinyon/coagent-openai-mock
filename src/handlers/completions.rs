//! # Completions Handler
//!
//! This module implements the HTTP handler for the OpenAI completions endpoint (`/v1/completions`).
//! It handles POST requests, validates the request body, and generates fake completion responses
//! that conform to the OpenAI API specification.
//!
//! ## Features
//!
//! - **Request Validation**: Validates incoming completion requests against OpenAI schema
//! - **Error Handling**: Returns proper OpenAI-formatted error responses for invalid requests
//! - **Response Generation**: Uses the CompletionGenerator to create realistic fake responses
//! - **Content Type Handling**: Properly handles JSON request/response content types
//!
//! ## Usage
//!
//! This handler is typically mounted at the `/v1/completions` route with authentication middleware:
//!
//! ```rust
//! use poem::{Route, post, EndpointExt};
//! use openai_mock::handlers::completions::create_completion;
//! use openai_mock::auth::AuthMiddleware;
//!
//! let app = Route::new()
//!     .at("/v1/completions", post(create_completion))
//!     .with(AuthMiddleware::new("sk-mock-openai-api-key-12345"));
//! ```

use poem::{IntoResponse, Response, Result, handler, http::StatusCode, web::Json};
use serde_json;

use crate::generators::CompletionGenerator;
use crate::models::{requests::CreateCompletionRequest, responses::ErrorResponse};

/// Handler for POST /v1/completions
///
/// This handler processes completion requests and returns fake completion responses.
/// The request body must be valid JSON that deserializes to a CreateCompletionRequest.
///
/// ## Request Format
///
/// The request must include:
/// - `model`: The model to use for completion
/// - `prompt`: The prompt to complete
/// - Other optional parameters like `max_tokens`, `temperature`, etc.
///
/// ## Response Format
///
/// Returns a JSON response with:
/// - `id`: Unique completion ID
/// - `object`: Always "text_completion"
/// - `created`: Unix timestamp
/// - `model`: The model used
/// - `choices`: Array of completion choices
/// - `usage`: Token usage statistics
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
/// POST /v1/completions
/// Content-Type: application/json
/// Authorization: Bearer sk-mock-openai-api-key-12345
///
/// {
///   "model": "text-davinci-003",
///   "prompt": "Hello, world!",
///   "max_tokens": 10
/// }
/// ```
///
/// ## Successful Response
///
/// ```json
/// {
///   "id": "cmpl-abc123",
///   "object": "text_completion",
///   "created": 1677649420,
///   "model": "text-davinci-003",
///   "choices": [{
///     "text": " How can I help you today?",
///     "index": 0,
///     "finish_reason": "stop"
///   }],
///   "usage": {
///     "prompt_tokens": 4,
///     "completion_tokens": 7,
///     "total_tokens": 11
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
pub async fn create_completion(body: Json<CreateCompletionRequest>) -> Result<impl IntoResponse> {
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

    // Generate the completion response
    let completion_response = CompletionGenerator::generate_response(&request);

    // Serialize the response to JSON
    let json_body = match serde_json::to_string(&completion_response) {
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
    use crate::models::requests::{CreateCompletionRequest, PromptInput};

    fn create_test_request() -> CreateCompletionRequest {
        CreateCompletionRequest {
            model: "text-davinci-003".to_string(),
            prompt: PromptInput::String("Hello, world!".to_string()),
            suffix: None,
            max_tokens: Some(10),
            temperature: Some(0.7),
            top_p: None,
            n: None,
            stream: None,
            logprobs: None,
            echo: None,
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            best_of: None,
            logit_bias: None,
            user: None,
        }
    }

    #[test]
    fn test_create_completion_request_validation() {
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
    fn test_completion_generator_integration() {
        let request = create_test_request();
        let response = CompletionGenerator::generate_response(&request);

        assert_eq!(response.object, "text_completion");
        assert_eq!(response.model, "text-davinci-003");
        assert!(!response.choices.is_empty());
        assert!(response.usage.total_tokens > 0);
    }
}
