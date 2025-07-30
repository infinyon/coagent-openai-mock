//! # Chat Completions Handler
//!
//! This module implements the HTTP handler for the OpenAI chat completions endpoint (`/v1/chat/completions`).
//! It handles POST requests, validates the request body, and generates fake chat completion responses
//! that conform to the OpenAI API specification.
//!
//! ## Features
//!
//! - **Request Validation**: Validates incoming chat completion requests against OpenAI schema
//! - **Error Handling**: Returns proper OpenAI-formatted error responses for invalid requests
//! - **Response Generation**: Uses the ChatCompletionGenerator to create realistic fake responses
//! - **Content Type Handling**: Properly handles JSON request/response content types
//! - **Tool Support**: Handles function/tool calling in chat completions
//!
//! ## Usage
//!
//! This handler is typically mounted at the `/v1/chat/completions` route with authentication middleware:
//!
//! ```rust
//! use poem::{Route, post, EndpointExt};
//! use openai_mock::handlers::chat_completions::create_chat_completion;
//! use openai_mock::auth::AuthMiddleware;
//!
//! let app = Route::new()
//!     .at("/v1/chat/completions", post(create_chat_completion))
//!     .with(AuthMiddleware::new("sk-mock-openai-api-key-12345"));
//! ```

use poem::{IntoResponse, Response, Result, handler, http::StatusCode, web::Json};
use serde_json;

use crate::generators::ChatCompletionGenerator;
use crate::models::{requests::CreateChatCompletionRequest, responses::ErrorResponse};

/// Handler for POST /v1/chat/completions
///
/// This handler processes chat completion requests and returns fake chat completion responses.
/// The request body must be valid JSON that deserializes to a CreateChatCompletionRequest.
///
/// ## Request Format
///
/// The request must include:
/// - `model`: The model to use for chat completion
/// - `messages`: Array of message objects with role and content
/// - Other optional parameters like `max_tokens`, `temperature`, `tools`, etc.
///
/// ## Response Format
///
/// Returns a JSON response with:
/// - `id`: Unique completion ID
/// - `object`: Always "chat.completion"
/// - `created`: Unix timestamp
/// - `model`: The model used
/// - `choices`: Array of completion choices with messages
/// - `usage`: Token usage statistics
///
/// ## Error Handling
///
/// Returns appropriate HTTP status codes and OpenAI-formatted error responses for:
/// - Invalid JSON (400 Bad Request)
/// - Missing required fields (400 Bad Request)
/// - Invalid parameter values (400 Bad Request)
/// - Empty messages array (400 Bad Request)
///
/// # Examples
///
/// ## Successful Request
///
/// ```json
/// POST /v1/chat/completions
/// Content-Type: application/json
/// Authorization: Bearer sk-mock-openai-api-key-12345
///
/// {
///   "model": "gpt-3.5-turbo",
///   "messages": [
///     {"role": "user", "content": "Hello, how are you?"}
///   ],
///   "max_tokens": 50
/// }
/// ```
///
/// ## Successful Response
///
/// ```json
/// {
///   "id": "chatcmpl-abc123",
///   "object": "chat.completion",
///   "created": 1677649420,
///   "model": "gpt-3.5-turbo",
///   "choices": [{
///     "index": 0,
///     "message": {
///       "role": "assistant",
///       "content": "Hello! I'm doing well, thank you for asking. How can I assist you today?"
///     },
///     "finish_reason": "stop"
///   }],
///   "usage": {
///     "prompt_tokens": 13,
///     "completion_tokens": 17,
///     "total_tokens": 30
///   }
/// }
/// ```
///
/// ## Tool Call Request
///
/// ```json
/// POST /v1/chat/completions
/// Content-Type: application/json
/// Authorization: Bearer sk-mock-openai-api-key-12345
///
/// {
///   "model": "gpt-3.5-turbo",
///   "messages": [
///     {"role": "user", "content": "What's the weather like in San Francisco?"}
///   ],
///   "tools": [{
///     "type": "function",
///     "function": {
///       "name": "get_weather",
///       "description": "Get the current weather",
///       "parameters": {
///         "type": "object",
///         "properties": {
///           "location": {"type": "string"}
///         }
///       }
///     }
///   }],
///   "tool_choice": "auto"
/// }
/// ```
///
/// ## Tool Call Response
///
/// ```json
/// {
///   "id": "chatcmpl-def456",
///   "object": "chat.completion",
///   "created": 1677649420,
///   "model": "gpt-3.5-turbo",
///   "choices": [{
///     "index": 0,
///     "message": {
///       "role": "assistant",
///       "content": null,
///       "tool_calls": [{
///         "id": "call_abc123",
///         "type": "function",
///         "function": {
///           "name": "get_weather",
///           "arguments": "{\"location\": \"San Francisco, CA\"}"
///         }
///       }]
///     },
///     "finish_reason": "tool_calls"
///   }],
///   "usage": {
///     "prompt_tokens": 20,
///     "completion_tokens": 15,
///     "total_tokens": 35
///   }
/// }
/// ```
///
/// ## Error Response
///
/// ```json
/// {
///   "error": {
///     "message": "Messages array cannot be empty",
///     "type": "invalid_request_error",
///     "param": "messages",
///     "code": null
///   }
/// }
/// ```
#[handler]
pub async fn create_chat_completion(
    body: Json<CreateChatCompletionRequest>,
) -> Result<impl IntoResponse> {
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

    // Generate the chat completion response
    let chat_completion_response = ChatCompletionGenerator::generate_response(&request);

    // Serialize the response to JSON
    let json_body = match serde_json::to_string(&chat_completion_response) {
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
    use crate::models::requests::{
        ChatCompletionMessage, ChatCompletionRole, CreateChatCompletionRequest,
    };

    fn create_test_request() -> CreateChatCompletionRequest {
        CreateChatCompletionRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![ChatCompletionMessage {
                role: ChatCompletionRole::User,
                content: Some("Hello, world!".to_string()),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            }],
            temperature: Some(0.7),
            top_p: None,
            n: None,
            stream: None,
            stop: None,
            max_tokens: Some(100),
            presence_penalty: None,
            frequency_penalty: None,
            logit_bias: None,
            user: None,
            tools: None,
            tool_choice: None,
        }
    }

    #[test]
    fn test_create_chat_completion_request_validation() {
        let valid_request = create_test_request();
        assert!(valid_request.validate().is_ok());

        let mut invalid_request = create_test_request();
        invalid_request.model = "".to_string();
        assert!(invalid_request.validate().is_err());

        let mut empty_messages_request = create_test_request();
        empty_messages_request.messages = vec![];
        assert!(empty_messages_request.validate().is_err());
    }

    #[test]
    fn test_internal_error_response() {
        let response = create_internal_error_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_chat_completion_generator_integration() {
        let request = create_test_request();
        let response = ChatCompletionGenerator::generate_response(&request);

        assert_eq!(response.object, "chat.completion");
        assert_eq!(response.model, "gpt-3.5-turbo");
        assert!(!response.choices.is_empty());
        assert_eq!(response.choices[0].message.role, "assistant");
        assert!(response.usage.total_tokens > 0);
    }

    #[test]
    fn test_system_message_handling() {
        let mut request = create_test_request();
        request.messages.insert(
            0,
            ChatCompletionMessage {
                role: ChatCompletionRole::System,
                content: Some("You are a helpful assistant.".to_string()),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
        );

        let response = ChatCompletionGenerator::generate_response(&request);
        assert!(!response.choices.is_empty());
        assert_eq!(response.choices[0].message.role, "assistant");
    }

    #[test]
    fn test_multiple_messages_conversation() {
        let mut request = create_test_request();
        request.messages = vec![
            ChatCompletionMessage {
                role: ChatCompletionRole::System,
                content: Some("You are a helpful assistant.".to_string()),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
            ChatCompletionMessage {
                role: ChatCompletionRole::User,
                content: Some("Hello!".to_string()),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
            ChatCompletionMessage {
                role: ChatCompletionRole::Assistant,
                content: Some("Hi there! How can I help you?".to_string()),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
            ChatCompletionMessage {
                role: ChatCompletionRole::User,
                content: Some("What's the weather like?".to_string()),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
        ];

        let response = ChatCompletionGenerator::generate_response(&request);
        assert!(!response.choices.is_empty());
        assert!(response.usage.prompt_tokens > 0);
    }

    #[test]
    fn test_temperature_and_parameters() {
        let mut request = create_test_request();
        request.temperature = Some(0.9);
        request.max_tokens = Some(50);
        request.n = Some(2);

        let response = ChatCompletionGenerator::generate_response(&request);
        assert_eq!(response.choices.len(), 2);
        assert_eq!(response.choices[0].index, 0);
        assert_eq!(response.choices[1].index, 1);
    }

    #[test]
    fn test_different_models() {
        let models = vec!["gpt-3.5-turbo", "gpt-4", "gpt-4o"];

        for model in models {
            let mut request = create_test_request();
            request.model = model.to_string();

            let response = ChatCompletionGenerator::generate_response(&request);
            assert_eq!(response.model, model);
            assert!(!response.choices.is_empty());
        }
    }

    #[test]
    fn test_message_validation_edge_cases() {
        // Test empty content for user message
        let mut request = create_test_request();
        request.messages[0].content = Some("".to_string());
        assert!(request.validate().is_err());

        // Test missing content for user message
        request.messages[0].content = None;
        assert!(request.validate().is_err());

        // Test assistant message with neither content nor tool_calls
        let mut assistant_request = create_test_request();
        assistant_request.messages = vec![ChatCompletionMessage {
            role: ChatCompletionRole::Assistant,
            content: None,
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }];
        assert!(assistant_request.validate().is_err());
    }
}
