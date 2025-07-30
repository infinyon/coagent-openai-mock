//! # Chat Completions Generator
//!
//! This module provides functionality to generate fake responses for the OpenAI Chat Completions API.
//! It creates realistic-looking chat completion responses with appropriate message formats,
//! tool calls, and usage statistics.
//!
//! ## Features
//!
//! - **Response Generation**: Creates complete chat completion responses with messages
//! - **Multiple Choices**: Supports generating multiple completion choices
//! - **Tool Calls**: Generates realistic function/tool call responses
//! - **Usage Statistics**: Calculates token usage for prompt and completion
//! - **Content Variation**: Provides varied response content based on conversation context
//!
//! ## Usage
//!
//! ```rust
//! use openai_mock::generators::chat_completions::ChatCompletionGenerator;
//! use openai_mock::models::requests::{CreateChatCompletionRequest, ChatCompletionMessage, ChatCompletionRole};
//!
//! let request = CreateChatCompletionRequest {
//!     model: "gpt-3.5-turbo".to_string(),
//!     messages: vec![
//!         ChatCompletionMessage {
//!             role: ChatCompletionRole::User,
//!             content: Some("Hello!".to_string()),
//!             name: None,
//!             tool_calls: None,
//!             tool_call_id: None,
//!         }
//!     ],
//!     temperature: None,
//!     top_p: None,
//!     n: None,
//!     stream: None,
//!     stop: None,
//!     max_tokens: None,
//!     presence_penalty: None,
//!     frequency_penalty: None,
//!     logit_bias: None,
//!     user: None,
//!     tools: None,
//!     tool_choice: None,
//! };
//!
//! let response = ChatCompletionGenerator::generate_response(&request);
//! ```

use chrono;
use uuid::Uuid;

use crate::models::{
    requests::{ChatCompletionMessage, ChatCompletionRole, CreateChatCompletionRequest},
    responses::{
        ChatCompletionChoice, ChatCompletionMessageToolCall, ChatCompletionResponseMessage,
        CompletionUsage, CreateChatCompletionResponse,
    },
};

/// Generator for fake chat completion responses
///
/// This struct provides static methods to generate realistic chat completion responses
/// that conform to the OpenAI API specification. It creates varied content based on
/// the conversation context and handles different message types.
pub struct ChatCompletionGenerator;

impl ChatCompletionGenerator {
    /// Generate a fake chat completion response based on the request
    ///
    /// This method creates a complete chat completion response including:
    /// - Unique completion ID
    /// - Current timestamp
    /// - Generated message choices
    /// - Usage statistics
    ///
    /// # Arguments
    ///
    /// * `request` - The chat completion request to generate a response for
    ///
    /// # Returns
    ///
    /// A `CreateChatCompletionResponse` with generated content
    pub fn generate_response(
        request: &CreateChatCompletionRequest,
    ) -> CreateChatCompletionResponse {
        let id = Self::generate_id();
        let created = Self::generate_timestamp();
        let num_choices = request.n.unwrap_or(1);

        let mut choices = Vec::new();
        for i in 0..num_choices {
            let choice = Self::generate_choice(request, i);
            choices.push(choice);
        }

        let usage = Self::generate_usage(request, &choices);

        CreateChatCompletionResponse::new(id, request.model.clone(), created, choices, usage)
    }

    /// Generate a unique chat completion ID
    fn generate_id() -> String {
        let uuid = Uuid::new_v4();
        format!(
            "chatcmpl-{}",
            &uuid.to_string().replace("-", "")[..29]
        )
    }

    /// Generate current timestamp
    fn generate_timestamp() -> u64 {
        chrono::Utc::now().timestamp() as u64
    }

    /// Generate a single chat completion choice
    fn generate_choice(request: &CreateChatCompletionRequest, index: u32) -> ChatCompletionChoice {
        // Determine if we should generate tool calls based on the request
        let should_generate_tools = request.tools.is_some()
            && request.tool_choice.is_some()
            && Self::should_call_function(&request.messages);

        let message = if should_generate_tools {
            Self::generate_tool_call_message(request)
        } else {
            Self::generate_content_message(request)
        };

        let finish_reason = Self::determine_finish_reason(request, &message);

        ChatCompletionChoice::new(index, message, finish_reason)
    }

    /// Generate a content-based assistant message
    fn generate_content_message(
        request: &CreateChatCompletionRequest,
    ) -> ChatCompletionResponseMessage {
        let content = Self::generate_assistant_content(&request.messages, &request.model);
        ChatCompletionResponseMessage::assistant_message(content)
    }

    /// Generate an assistant message with tool calls
    fn generate_tool_call_message(
        request: &CreateChatCompletionRequest,
    ) -> ChatCompletionResponseMessage {
        if let Some(tools) = &request.tools {
            if !tools.is_empty() {
                let tool_call = Self::generate_tool_call(&tools[0]);
                return ChatCompletionResponseMessage::assistant_message_with_tools(vec![
                    tool_call,
                ]);
            }
        }

        // Fallback to content message if no tools available
        Self::generate_content_message(request)
    }

    /// Generate assistant content based on conversation context
    fn generate_assistant_content(messages: &[ChatCompletionMessage], model: &str) -> String {
        let last_user_message = messages
            .iter()
            .rev()
            .find(|msg| matches!(msg.role, ChatCompletionRole::User))
            .and_then(|msg| msg.content.as_ref());

        match last_user_message {
            Some(content) => Self::generate_contextual_response(content, model),
            None => Self::get_default_response(),
        }
    }

    /// Generate contextual response based on user input
    fn generate_contextual_response(user_content: &str, model: &str) -> String {
        let content_lower = user_content.to_lowercase();

        // Greeting responses
        if content_lower.contains("hello")
            || content_lower.contains("hi")
            || content_lower.contains("hey")
        {
            return Self::get_greeting_response();
        }

        // Question responses
        if content_lower.contains("?")
            || content_lower.contains("what")
            || content_lower.contains("how")
            || content_lower.contains("why")
        {
            return Self::get_question_response(&content_lower);
        }

        // Code-related responses
        if content_lower.contains("code")
            || content_lower.contains("programming")
            || content_lower.contains("function")
        {
            return Self::get_code_response();
        }

        // Math responses
        if content_lower.contains("calculate")
            || content_lower.contains("math")
            || content_lower.contains("number")
        {
            return Self::get_math_response();
        }

        // Creative responses
        if content_lower.contains("story")
            || content_lower.contains("creative")
            || content_lower.contains("write")
        {
            return Self::get_creative_response();
        }

        // Model-specific responses
        if model.contains("gpt-4") {
            Self::get_advanced_response()
        } else {
            Self::get_general_response()
        }
    }

    /// Generate a tool call for function calling
    fn generate_tool_call(
        tool: &crate::models::requests::ChatCompletionTool,
    ) -> ChatCompletionMessageToolCall {
        let call_id = format!(
            "call_{}",
            &Uuid::new_v4().to_string().replace("-", "")[..24]
        );
        let function_name = tool.function.name.clone();

        // Generate realistic function arguments based on the function name
        let arguments = Self::generate_function_arguments(&function_name);

        ChatCompletionMessageToolCall::new(call_id, function_name, arguments)
    }

    /// Generate realistic function arguments
    fn generate_function_arguments(function_name: &str) -> String {
        match function_name.to_lowercase().as_str() {
            name if name.contains("weather") => r#"{"location": "San Francisco, CA"}"#.to_string(),
            name if name.contains("search") => {
                r#"{"query": "artificial intelligence"}"#.to_string()
            }
            name if name.contains("calculate") => r#"{"expression": "2 + 2"}"#.to_string(),
            name if name.contains("time") => r#"{"timezone": "UTC"}"#.to_string(),
            _ => r#"{}"#.to_string(),
        }
    }

    /// Determine if we should call a function based on conversation context
    fn should_call_function(messages: &[ChatCompletionMessage]) -> bool {
        let last_user_message = messages
            .iter()
            .rev()
            .find(|msg| matches!(msg.role, ChatCompletionRole::User))
            .and_then(|msg| msg.content.as_ref());

        if let Some(content) = last_user_message {
            let content_lower = content.to_lowercase();
            return content_lower.contains("weather")
                || content_lower.contains("search")
                || content_lower.contains("calculate")
                || content_lower.contains("time");
        }

        false
    }

    /// Determine the finish reason for the completion
    fn determine_finish_reason(
        request: &CreateChatCompletionRequest,
        message: &ChatCompletionResponseMessage,
    ) -> String {
        // If there are tool calls, finish reason is tool_calls
        if message.tool_calls.is_some() {
            return "tool_calls".to_string();
        }

        // Check if we hit max tokens (simplified logic)
        if let Some(max_tokens) = request.max_tokens {
            if let Some(content) = &message.content {
                let estimated_tokens = Self::estimate_tokens(content);
                if estimated_tokens >= max_tokens {
                    return "length".to_string();
                }
            }
        }

        // Default to stop
        "stop".to_string()
    }

    /// Generate usage statistics
    fn generate_usage(
        request: &CreateChatCompletionRequest,
        choices: &[ChatCompletionChoice],
    ) -> CompletionUsage {
        let prompt_tokens = Self::estimate_prompt_tokens(&request.messages);
        let completion_tokens = Self::estimate_completion_tokens(choices);

        CompletionUsage::new(prompt_tokens, completion_tokens)
    }

    /// Estimate tokens in the prompt messages
    fn estimate_prompt_tokens(messages: &[ChatCompletionMessage]) -> u32 {
        let total_chars: usize = messages
            .iter()
            .map(|msg| {
                msg.content.as_ref().map(|c| c.len()).unwrap_or(0) + msg.role.to_string().len() + 20 // overhead for message structure
            })
            .sum();

        Self::estimate_tokens_from_chars(total_chars)
    }

    /// Estimate tokens in completion choices
    fn estimate_completion_tokens(choices: &[ChatCompletionChoice]) -> u32 {
        let total_chars: usize = choices
            .iter()
            .map(|choice| {
                choice
                    .message
                    .content
                    .as_ref()
                    .map(|c| c.len())
                    .unwrap_or(0)
                    + choice
                        .message
                        .tool_calls
                        .as_ref()
                        .map(|calls| {
                            calls
                                .iter()
                                .map(|call| {
                                    call.function.arguments.len() + call.function.name.len()
                                })
                                .sum::<usize>()
                        })
                        .unwrap_or(0)
            })
            .sum();

        Self::estimate_tokens_from_chars(total_chars)
    }

    /// Simple token estimation (roughly 4 characters per token)
    fn estimate_tokens_from_chars(chars: usize) -> u32 {
        ((chars as f64) / 4.0).ceil() as u32
    }

    /// Estimate tokens in a string
    fn estimate_tokens(text: &str) -> u32 {
        Self::estimate_tokens_from_chars(text.len())
    }

    // Response generators for different types of content
    fn get_greeting_response() -> String {
        let responses = [
            "Hello! How can I assist you today?",
            "Hi there! What can I help you with?",
            "Hey! I'm here to help. What do you need?",
            "Hello! I'm ready to assist you with any questions or tasks you have.",
        ];
        responses[Self::simple_hash("greeting") % responses.len()].to_string()
    }

    fn get_question_response(content: &str) -> String {
        if content.contains("what") {
            "That's an interesting question. Let me provide you with a comprehensive answer based on my knowledge."
        } else if content.contains("how") {
            "Here's how you can approach this: I'll break it down into clear, actionable steps."
        } else if content.contains("why") {
            "There are several reasons for this. Let me explain the key factors involved."
        } else {
            "I'd be happy to help answer your question. Let me provide you with detailed information."
        }.to_string()
    }

    fn get_code_response() -> String {
        "I can help you with programming! Here's a solution:\n\n```python\ndef example_function():\n    return \"Hello, World!\"\n```\n\nThis code demonstrates a basic function that returns a greeting."
            .to_string()
    }

    fn get_math_response() -> String {
        "I can help with mathematical calculations. For example, if you're looking to solve an equation or perform calculations, I can guide you through the process step by step."
            .to_string()
    }

    fn get_creative_response() -> String {
        "I'd be delighted to help with creative writing! Here's a short example:\n\nOnce upon a time, in a world where artificial intelligence and human creativity merged seamlessly, there lived a helpful assistant who loved to tell stories..."
            .to_string()
    }

    fn get_advanced_response() -> String {
        "As an advanced AI model, I can provide detailed, nuanced responses to complex questions. I'll analyze your request from multiple angles and provide comprehensive insights."
            .to_string()
    }

    fn get_general_response() -> String {
        "I'm here to help! Please feel free to ask me anything, and I'll do my best to provide you with accurate and helpful information."
            .to_string()
    }

    fn get_default_response() -> String {
        "Hello! I'm an AI assistant. How can I help you today?".to_string()
    }

    /// Simple hash function for deterministic randomness
    fn simple_hash(input: &str) -> usize {
        input.chars().map(|c| c as usize).sum()
    }
}

impl std::fmt::Display for ChatCompletionRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatCompletionRole::System => write!(f, "system"),
            ChatCompletionRole::User => write!(f, "user"),
            ChatCompletionRole::Assistant => write!(f, "assistant"),
            ChatCompletionRole::Tool => write!(f, "tool"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::requests::{ChatCompletionMessage, ChatCompletionRole};

    fn create_test_request() -> CreateChatCompletionRequest {
        CreateChatCompletionRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![ChatCompletionMessage {
                role: ChatCompletionRole::User,
                content: Some("Hello, how are you?".to_string()),
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
    fn test_generate_response_basic() {
        let request = create_test_request();
        let response = ChatCompletionGenerator::generate_response(&request);

        assert_eq!(response.object, "chat.completion");
        assert_eq!(response.model, "gpt-3.5-turbo");
        assert!(!response.choices.is_empty());
        assert!(response.usage.total_tokens > 0);
    }

    #[test]
    fn test_generate_multiple_choices() {
        let mut request = create_test_request();
        request.n = Some(3);

        let response = ChatCompletionGenerator::generate_response(&request);
        assert_eq!(response.choices.len(), 3);

        for (i, choice) in response.choices.iter().enumerate() {
            assert_eq!(choice.index, i as u32);
        }
    }

    #[test]
    fn test_contextual_responses() {
        let test_cases = vec![
            ("Hello there!", "greeting"),
            ("What is AI?", "question"),
            ("Write some code for me", "code"),
            ("Tell me a story", "creative"),
        ];

        for (input, expected_type) in test_cases {
            let mut request = create_test_request();
            request.messages[0].content = Some(input.to_string());

            let response = ChatCompletionGenerator::generate_response(&request);
            let content = response.choices[0].message.content.as_ref().unwrap();

            match expected_type {
                "greeting" => assert!(
                    content.to_lowercase().contains("hello")
                        || content.to_lowercase().contains("hi")
                ),
                "question" => assert!(content.len() > 20), // Should be a substantial response
                "code" => {
                    assert!(content.contains("```") || content.to_lowercase().contains("code"))
                }
                "creative" => assert!(content.len() > 50), // Should be a longer creative response
                _ => {}
            }
        }
    }

    #[test]
    fn test_usage_calculation() {
        let request = create_test_request();
        let response = ChatCompletionGenerator::generate_response(&request);

        assert!(response.usage.prompt_tokens > 0);
        assert!(response.usage.completion_tokens > 0);
        assert_eq!(
            response.usage.total_tokens,
            response.usage.prompt_tokens + response.usage.completion_tokens
        );
    }

    #[test]
    fn test_finish_reason() {
        let request = create_test_request();
        let response = ChatCompletionGenerator::generate_response(&request);

        let finish_reason = &response.choices[0].finish_reason;
        assert!(
            finish_reason == "stop" || finish_reason == "length" || finish_reason == "tool_calls"
        );
    }

    #[test]
    fn test_id_generation() {
        let request = create_test_request();
        let response1 = ChatCompletionGenerator::generate_response(&request);
        let response2 = ChatCompletionGenerator::generate_response(&request);

        assert!(response1.id.starts_with("chatcmpl-"));
        assert!(response2.id.starts_with("chatcmpl-"));
        assert_ne!(response1.id, response2.id);
    }

    #[test]
    fn test_token_estimation() {
        let text = "Hello, world!";
        let tokens = ChatCompletionGenerator::estimate_tokens(text);
        assert!(tokens > 0);
        assert!(tokens <= 10); // Should be reasonable for short text
    }

    #[test]
    fn test_message_validation_integration() {
        let request = create_test_request();

        // Should not panic and should produce valid response
        let response = ChatCompletionGenerator::generate_response(&request);
        assert!(!response.choices.is_empty());
        assert_eq!(response.choices[0].message.role, "assistant");
    }
}
