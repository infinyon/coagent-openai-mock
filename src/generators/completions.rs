//! # Completion Response Generator
//!
//! This module provides functionality to generate realistic fake OpenAI completion responses
//! for testing and development purposes. The generator creates syntactically valid responses
//! that match the official OpenAI API specification.
//!
//! ## Features
//!
//! - **Realistic Text Generation**: Generates contextually appropriate completion text based on prompt patterns
//! - **Multiple Response Types**: Supports greeting, creative writing, explanatory, code, and general completions
//! - **Proper Formatting**: All responses conform to OpenAI's completion response format
//! - **Usage Statistics**: Generates realistic token usage counts
//! - **Log Probabilities**: Optional generation of fake but structured log probability data
//! - **Multiple Choices**: Support for generating multiple completion alternatives
//! - **Echo Support**: Can echo the original prompt in the response when requested
//!
//! ## Example Usage
//!
//! ```rust
//! use openai_mock::generators::completions::CompletionGenerator;
//! use openai_mock::models::requests::{CreateCompletionRequest, PromptInput};
//!
//! let request = CreateCompletionRequest {
//!     model: "text-davinci-003".to_string(),
//!     prompt: PromptInput::String("Hello, world!".to_string()),
//!     max_tokens: Some(50),
//!     temperature: Some(0.7),
//!     n: Some(1),
//!     suffix: None,
//!     top_p: None,
//!     stream: None,
//!     logprobs: None,
//!     echo: None,
//!     stop: None,
//!     presence_penalty: None,
//!     frequency_penalty: None,
//!     best_of: None,
//!     logit_bias: None,
//!     user: None,
//! };
//!
//! let response = CompletionGenerator::generate_response(&request);
//! println!("Generated completion: {}", response.choices[0].text);
//! ```
//!
//! ## Response Generation Strategy
//!
//! The generator uses pattern matching on the input prompt to determine the most appropriate
//! type of fake response:
//!
//! - **Greeting prompts** (containing "hello") → Friendly greeting responses
//! - **Creative prompts** (containing "write", "create") → Story-like completions
//! - **Explanatory prompts** (containing "explain", "what") → Educational responses
//! - **Code prompts** (containing "code", "function") → Code block responses
//! - **General prompts** → Generic but contextually appropriate responses
//!
//! All responses are deterministic based on prompt content to ensure consistent behavior
//! during testing while still providing variety.

use crate::models::{
    requests::CreateCompletionRequest,
    responses::{CompletionChoice, CompletionLogprobs, CompletionUsage, CreateCompletionResponse},
};
use chrono::{Datelike, Timelike};
use std::collections::HashMap;
use uuid::Uuid;

/// Generator for creating fake completion responses that conform to OpenAI API specifications
pub struct CompletionGenerator;

impl CompletionGenerator {
    /// Generate a fake completion response based on the request
    pub fn generate_response(request: &CreateCompletionRequest) -> CreateCompletionResponse {
        let id = Self::generate_id();
        let created = Self::generate_timestamp();
        let num_choices = request.n.unwrap_or(1);

        let mut choices = Vec::new();
        for i in 0..num_choices {
            let choice = Self::generate_choice(request, i);
            choices.push(choice);
        }

        let usage = Self::generate_usage(request, &choices);

        CreateCompletionResponse::new(id, request.model.clone(), created, choices, usage)
    }

    /// Generate a unique completion ID
    fn generate_id() -> String {
        let uuid = Uuid::new_v4();
        format!(
            "cmpl-{}",
            uuid.to_string().replace("-", "")[..24].to_string()
        )
    }

    /// Generate current timestamp
    fn generate_timestamp() -> u64 {
        chrono::Utc::now().timestamp() as u64
    }

    /// Generate a single completion choice
    fn generate_choice(request: &CreateCompletionRequest, index: u32) -> CompletionChoice {
        let text = Self::generate_completion_text(request);
        let finish_reason = Self::determine_finish_reason(request, &text);

        let mut choice = CompletionChoice::new(text, index, finish_reason);

        // Add logprobs if requested
        if let Some(logprobs_count) = request.logprobs {
            if logprobs_count > 0 {
                let logprobs = Self::generate_logprobs(&choice.text, logprobs_count);
                choice = choice.with_logprobs(logprobs);
            }
        }

        choice
    }

    /// Generate realistic fake completion text
    fn generate_completion_text(request: &CreateCompletionRequest) -> String {
        // Extract the first prompt for text generation
        let prompt = match &request.prompt {
            crate::models::requests::PromptInput::String(s) => s.clone(),
            crate::models::requests::PromptInput::Array(arr) => {
                arr.first().unwrap_or(&String::new()).clone()
            }
        };

        // Determine max tokens for the response
        let max_tokens = request.max_tokens.unwrap_or(16);

        // Generate contextually appropriate response based on prompt patterns
        let completion = if prompt.to_lowercase().contains("hello") {
            Self::get_greeting_completion(&prompt, max_tokens)
        } else if prompt.to_lowercase().contains("write")
            || prompt.to_lowercase().contains("create")
        {
            Self::get_creative_completion(&prompt, max_tokens)
        } else if prompt.to_lowercase().contains("explain")
            || prompt.to_lowercase().contains("what")
        {
            Self::get_explanatory_completion(&prompt, max_tokens)
        } else if prompt.to_lowercase().contains("code")
            || prompt.to_lowercase().contains("function")
        {
            Self::get_code_completion(&prompt, max_tokens)
        } else {
            Self::get_general_completion(&prompt, max_tokens)
        };

        // Add echo if requested
        if request.echo.unwrap_or(false) {
            format!("{}{}", prompt, completion)
        } else {
            completion
        }
    }

    /// Generate greeting-style completions
    fn get_greeting_completion(prompt: &str, max_tokens: u32) -> String {
        let responses = vec![
            " Hello! How can I help you today?",
            " Hi there! Nice to meet you.",
            " Hello! I'm here to assist you.",
            " Hi! What would you like to know?",
            " Hello! Feel free to ask me anything.",
        ];

        let response = responses[prompt.len() % responses.len()];
        Self::truncate_to_tokens(response, max_tokens)
    }

    /// Generate creative writing completions
    fn get_creative_completion(_prompt: &str, max_tokens: u32) -> String {
        let responses = vec![
            " Once upon a time, in a land far away, there lived a curious explorer who discovered amazing secrets hidden in ancient ruins.",
            " The story begins on a rainy Tuesday morning when everything seemed ordinary, but little did anyone know that extraordinary events were about to unfold.",
            " In the bustling city, among the towering skyscrapers and busy streets, a small coffee shop held the key to an incredible adventure.",
            " The old library contained more than just books - it held mysteries that had been waiting centuries to be uncovered by the right person.",
        ];

        let response = responses[chrono::Utc::now().timestamp() as usize % responses.len()];
        Self::truncate_to_tokens(response, max_tokens)
    }

    /// Generate explanatory completions
    fn get_explanatory_completion(_prompt: &str, max_tokens: u32) -> String {
        let responses = vec![
            " This is a complex topic that involves multiple interconnected concepts. Let me break it down into simpler parts for better understanding.",
            " The fundamental principle behind this is based on well-established scientific theories that have been validated through extensive research.",
            " To understand this properly, we need to consider the historical context and how various factors have influenced its development over time.",
            " This concept can be explained through a practical example that demonstrates its real-world applications and benefits.",
        ];

        let response = responses[chrono::Utc::now().day() as usize % responses.len()];
        Self::truncate_to_tokens(response, max_tokens)
    }

    /// Generate code-related completions
    fn get_code_completion(_prompt: &str, max_tokens: u32) -> String {
        let responses = vec![
            "\n```rust\nfn example() {\n    println!(\"Hello, world!\");\n}\n```",
            "\n```python\ndef example():\n    print(\"Hello, world!\")\n    return True\n```",
            "\n```javascript\nfunction example() {\n    console.log(\"Hello, world!\");\n    return true;\n}\n```",
            "\n```java\npublic void example() {\n    System.out.println(\"Hello, world!\");\n}\n```",
        ];

        let response = responses[chrono::Utc::now().hour() as usize % responses.len()];
        Self::truncate_to_tokens(response, max_tokens)
    }

    /// Generate general purpose completions
    fn get_general_completion(_prompt: &str, max_tokens: u32) -> String {
        let responses = vec![
            " This is an interesting topic that deserves careful consideration and thoughtful analysis.",
            " There are several important factors to consider when approaching this subject matter.",
            " The key to understanding this lies in examining the underlying principles and their practical applications.",
            " This represents a fascinating area of study with many opportunities for further exploration.",
            " The complexity of this subject requires a multifaceted approach to fully appreciate its nuances.",
        ];

        let hash = Self::simple_hash(_prompt);
        let response = responses[hash % responses.len()];
        Self::truncate_to_tokens(response, max_tokens)
    }

    /// Simple hash function for consistent but varied responses
    fn simple_hash(input: &str) -> usize {
        input.chars().map(|c| c as usize).sum()
    }

    /// Truncate response to approximate token count
    fn truncate_to_tokens(text: &str, max_tokens: u32) -> String {
        // Rough approximation: 1 token ≈ 4 characters for English text
        let max_chars = (max_tokens * 4) as usize;

        if text.len() <= max_chars {
            text.to_string()
        } else {
            // Try to break at word boundaries
            let truncated = &text[..max_chars];
            if let Some(last_space) = truncated.rfind(' ') {
                truncated[..last_space].to_string()
            } else {
                truncated.to_string()
            }
        }
    }

    /// Determine the finish reason for the completion
    fn determine_finish_reason(request: &CreateCompletionRequest, text: &str) -> String {
        // Check if we hit a stop sequence
        if let Some(stop_sequences) = &request.stop {
            let stops = match stop_sequences {
                crate::models::requests::StopSequences::String(s) => vec![s.clone()],
                crate::models::requests::StopSequences::Array(arr) => arr.clone(),
            };

            for stop in stops {
                if text.contains(&stop) {
                    return "stop".to_string();
                }
            }
        }

        // Check if we likely hit max_tokens (rough approximation)
        let estimated_tokens = text.len() / 4;
        let max_tokens = request.max_tokens.unwrap_or(16) as usize;

        if estimated_tokens >= max_tokens {
            "length".to_string()
        } else {
            "stop".to_string()
        }
    }

    /// Generate fake log probabilities
    fn generate_logprobs(text: &str, logprobs_count: u32) -> CompletionLogprobs {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut tokens = Vec::new();
        let mut token_logprobs = Vec::new();
        let mut text_offset = Vec::new();
        let mut top_logprobs = Vec::new();

        let mut current_offset = 0u32;

        for word in words.iter().take(logprobs_count as usize) {
            tokens.push(word.to_string());

            // Generate fake but realistic log probabilities (typically negative)
            let logprob = -0.1 - (word.len() as f64 * 0.05);
            token_logprobs.push(Some(logprob));

            text_offset.push(current_offset);

            // Generate top logprobs alternatives
            let mut top_map = HashMap::new();
            top_map.insert(word.to_string(), logprob);

            // Add some fake alternatives
            if word.len() > 3 {
                top_map.insert(format!("{}s", word), logprob - 0.5);
                top_map.insert(format!("un{}", word), logprob - 1.0);
            }

            top_logprobs.push(Some(top_map));
            current_offset += word.len() as u32 + 1; // +1 for space
        }

        CompletionLogprobs {
            tokens,
            token_logprobs,
            top_logprobs: Some(top_logprobs),
            text_offset,
        }
    }

    /// Generate usage statistics based on the request and response
    fn generate_usage(
        request: &CreateCompletionRequest,
        choices: &[CompletionChoice],
    ) -> CompletionUsage {
        // Estimate prompt tokens based on prompt length
        let prompt_text = match &request.prompt {
            crate::models::requests::PromptInput::String(s) => s.clone(),
            crate::models::requests::PromptInput::Array(arr) => arr.join(" "),
        };

        let prompt_tokens = Self::estimate_tokens(&prompt_text);

        // Estimate completion tokens based on generated text
        let completion_tokens = choices
            .iter()
            .map(|choice| Self::estimate_tokens(&choice.text))
            .sum();

        CompletionUsage::new(prompt_tokens, completion_tokens)
    }

    /// Estimate token count for a given text
    fn estimate_tokens(text: &str) -> u32 {
        // Rough approximation: 1 token ≈ 4 characters for English text
        // Add some randomness to make it more realistic
        let base_estimate = (text.len() / 4).max(1) as u32;
        let variation = (text.len() % 3) as u32;
        base_estimate + variation
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::requests::PromptInput;

    fn create_test_request() -> CreateCompletionRequest {
        CreateCompletionRequest {
            model: "text-davinci-003".to_string(),
            prompt: PromptInput::String("Hello world".to_string()),
            suffix: None,
            max_tokens: Some(10),
            temperature: Some(0.7),
            top_p: None,
            n: Some(1),
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
    fn test_generate_response_basic() {
        let request = create_test_request();
        let response = CompletionGenerator::generate_response(&request);

        assert_eq!(response.object, "text_completion");
        assert_eq!(response.model, "text-davinci-003");
        assert_eq!(response.choices.len(), 1);
        assert!(response.id.starts_with("cmpl-"));
        assert!(response.created > 0);
    }

    #[test]
    fn test_generate_multiple_choices() {
        let mut request = create_test_request();
        request.n = Some(3);

        let response = CompletionGenerator::generate_response(&request);

        assert_eq!(response.choices.len(), 3);
        for (i, choice) in response.choices.iter().enumerate() {
            assert_eq!(choice.index, i as u32);
        }
    }

    #[test]
    fn test_generate_with_logprobs() {
        let mut request = create_test_request();
        request.logprobs = Some(5);

        let response = CompletionGenerator::generate_response(&request);

        assert_eq!(response.choices.len(), 1);
        assert!(response.choices[0].logprobs.is_some());

        let logprobs = response.choices[0].logprobs.as_ref().unwrap();
        assert!(!logprobs.tokens.is_empty());
        assert_eq!(logprobs.tokens.len(), logprobs.token_logprobs.len());
        assert_eq!(logprobs.tokens.len(), logprobs.text_offset.len());
    }

    #[test]
    fn test_generate_with_echo() {
        let mut request = create_test_request();
        request.echo = Some(true);

        let response = CompletionGenerator::generate_response(&request);

        // The response should contain the original prompt
        assert!(response.choices[0].text.contains("Hello world"));
    }

    #[test]
    fn test_different_prompt_types() {
        // Test greeting prompt
        let mut request = create_test_request();
        request.prompt = PromptInput::String("Hello there".to_string());
        let response = CompletionGenerator::generate_response(&request);
        assert!(!response.choices[0].text.is_empty());

        // Test creative prompt
        request.prompt = PromptInput::String("Write a story".to_string());
        let response = CompletionGenerator::generate_response(&request);
        assert!(!response.choices[0].text.is_empty());

        // Test code prompt
        request.prompt = PromptInput::String("Write a function".to_string());
        let response = CompletionGenerator::generate_response(&request);
        assert!(!response.choices[0].text.is_empty());
    }

    #[test]
    fn test_usage_calculation() {
        let request = create_test_request();
        let response = CompletionGenerator::generate_response(&request);

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
        let response = CompletionGenerator::generate_response(&request);

        // Should be either "stop" or "length"
        assert!(
            response.choices[0].finish_reason == "stop"
                || response.choices[0].finish_reason == "length"
        );
    }

    #[test]
    fn test_id_generation() {
        let id1 = CompletionGenerator::generate_id();
        let id2 = CompletionGenerator::generate_id();

        assert!(id1.starts_with("cmpl-"));
        assert!(id2.starts_with("cmpl-"));
        assert_ne!(id1, id2); // Should be unique
        assert_eq!(id1.len(), 29); // "cmpl-" + 24 chars
    }

    #[test]
    fn test_token_estimation() {
        assert!(CompletionGenerator::estimate_tokens("hello") > 0);
        assert!(
            CompletionGenerator::estimate_tokens("hello world test")
                > CompletionGenerator::estimate_tokens("hello")
        );
    }

    #[test]
    fn test_text_truncation() {
        let long_text = "This is a very long text that should be truncated when max tokens is low";
        let truncated = CompletionGenerator::truncate_to_tokens(long_text, 5);

        assert!(truncated.len() < long_text.len());
        assert!(truncated.len() <= 20); // 5 tokens * 4 chars approximation
    }
}
