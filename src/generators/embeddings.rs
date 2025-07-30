//! # Embedding Response Generator
//!
//! This module provides functionality to generate realistic fake OpenAI embedding responses
//! for testing and development purposes. The generator creates syntactically valid responses
//! that match the official OpenAI API specification.
//!
//! ## Features
//!
//! - **Consistent Vector Generation**: Generates deterministic embedding vectors based on input hash
//! - **Multiple Input Types**: Supports string, string array, integer array, and nested integer array inputs
//! - **Proper Dimensionality**: Generates 1536-dimensional vectors for text-embedding-ada-002 (default)
//! - **Configurable Dimensions**: Supports custom dimensions when specified in request
//! - **Usage Statistics**: Generates realistic token usage counts
//! - **Proper Formatting**: All responses conform to OpenAI's embedding response format
//!
//! ## Example Usage
//!
//! ```rust
//! use openai_mock::generators::EmbeddingGenerator;
//! use openai_mock::models::requests::{CreateEmbeddingRequest, EmbeddingInput};
//!
//! let request = CreateEmbeddingRequest {
//!     model: "text-embedding-ada-002".to_string(),
//!     input: EmbeddingInput::String("Hello, world!".to_string()),
//!     encoding_format: None,
//!     dimensions: None,
//!     user: None,
//! };
//!
//! let response = EmbeddingGenerator::generate_response(&request);
//! println!("Generated {} embeddings", response.data.len());
//! ```
//!
//! ## Vector Generation Strategy
//!
//! The generator uses a deterministic approach to create embedding vectors:
//!
//! 1. **Hash-based seed**: Uses a simple hash of the input text as a seed
//! 2. **Pseudo-random generation**: Creates consistent but varied floating-point values
//! 3. **Normalization**: Ensures vectors have reasonable magnitude (typically [-1, 1] range)
//! 4. **Dimensionality**: Defaults to 1536 for text-embedding-ada-002, supports custom dimensions
//!
//! This approach ensures that the same input always produces the same embedding vector,
//! which is important for consistent testing behavior.

use crate::models::{
    requests::{CreateEmbeddingRequest, EmbeddingInput},
    responses::{CreateEmbeddingResponse, EmbeddingData, EmbeddingUsage},
};

/// Generator for creating fake embedding responses that conform to OpenAI API specifications
pub struct EmbeddingGenerator;

impl EmbeddingGenerator {
    /// Generate a fake embedding response based on the request
    pub fn generate_response(request: &CreateEmbeddingRequest) -> CreateEmbeddingResponse {
        let input_strings = Self::extract_input_strings(&request.input);
        let dimensions = Self::determine_dimensions(request);

        let mut data = Vec::new();
        for (index, input_text) in input_strings.iter().enumerate() {
            let embedding_vector = Self::generate_embedding_vector(input_text, dimensions);
            let embedding_data = EmbeddingData::new(embedding_vector, index as u32);
            data.push(embedding_data);
        }

        let usage = Self::generate_usage(&input_strings);

        CreateEmbeddingResponse::new(data, request.model.clone(), usage)
    }

    /// Extract input strings from the various input types
    fn extract_input_strings(input: &EmbeddingInput) -> Vec<String> {
        match input {
            EmbeddingInput::String(s) => vec![s.clone()],
            EmbeddingInput::StringArray(arr) => arr.clone(),
            EmbeddingInput::IntegerArray(arr) => {
                // Convert integer array to a string representation
                vec![format!("{:?}", arr)]
            }
            EmbeddingInput::IntegerArrayArray(arr) => {
                // Convert each integer array to a string representation
                arr.iter()
                    .map(|inner_arr| format!("{:?}", inner_arr))
                    .collect()
            }
        }
    }

    /// Determine the number of dimensions for the embedding vectors
    fn determine_dimensions(request: &CreateEmbeddingRequest) -> usize {
        // If dimensions are explicitly specified, use them
        if let Some(dims) = request.dimensions {
            return dims as usize;
        }

        // Default dimensions based on model
        match request.model.as_str() {
            "text-embedding-ada-002" => 1536,
            "text-embedding-3-small" => 1536,
            "text-embedding-3-large" => 3072,
            "text-similarity-ada-001" => 1024,
            "text-similarity-babbage-001" => 2048,
            "text-similarity-curie-001" => 4096,
            "text-similarity-davinci-001" => 12288,
            _ => 1536, // Default to ada-002 dimensions
        }
    }

    /// Generate a consistent fake embedding vector for the given input text
    fn generate_embedding_vector(input_text: &str, dimensions: usize) -> Vec<f64> {
        let seed = Self::hash_string(input_text);
        let mut vector = Vec::with_capacity(dimensions);

        // Use a simple linear congruential generator for deterministic pseudo-random numbers
        let mut rng_state = seed;

        for i in 0..dimensions {
            // Generate pseudo-random value based on seed and position
            rng_state = (rng_state
                .wrapping_mul(1103515245)
                .wrapping_add(12345)
                .wrapping_add(i as u64))
                % (1u64 << 31);

            // Convert to float in range [-1, 1] with some bias toward smaller values
            let raw_value = (rng_state as f64) / ((1u64 << 31) as f64);
            let normalized_value = (raw_value - 0.5) * 2.0; // Range [-1, 1]

            // Apply some smoothing to make values more realistic (closer to 0)
            let smoothed_value = normalized_value * (0.3 + 0.7 * (1.0 - raw_value.abs()));

            vector.push(smoothed_value);
        }

        // Normalize the vector to have unit length (common for embeddings)
        Self::normalize_vector(&mut vector);

        vector
    }

    /// Normalize a vector to unit length
    fn normalize_vector(vector: &mut Vec<f64>) {
        let magnitude: f64 = vector.iter().map(|x| x * x).sum::<f64>().sqrt();

        if magnitude > 0.0 {
            for value in vector.iter_mut() {
                *value /= magnitude;
            }
        }
    }

    /// Generate a simple hash for consistent seed generation
    fn hash_string(input: &str) -> u64 {
        let mut hash = 5381u64;
        for byte in input.bytes() {
            hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
        }
        hash
    }

    /// Generate usage statistics based on the input
    fn generate_usage(input_strings: &[String]) -> EmbeddingUsage {
        let total_text = input_strings.join(" ");
        let prompt_tokens = Self::estimate_tokens(&total_text);
        EmbeddingUsage::new(prompt_tokens)
    }

    /// Estimate token count for a given text
    fn estimate_tokens(text: &str) -> u32 {
        // Rough approximation: 1 token ≈ 4 characters for English text
        // Add some variation based on text characteristics
        let base_estimate = (text.len() / 4).max(1) as u32;
        let word_count = text.split_whitespace().count() as u32;

        // Embedding tokens are typically more efficient than completion tokens
        // Use a slightly lower estimate
        ((base_estimate + word_count) as f64 * 0.8) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_request() -> CreateEmbeddingRequest {
        CreateEmbeddingRequest {
            model: "text-embedding-ada-002".to_string(),
            input: EmbeddingInput::String("Hello world".to_string()),
            encoding_format: None,
            dimensions: None,
            user: None,
        }
    }

    #[test]
    fn test_generate_response_basic() {
        let request = create_test_request();
        let response = EmbeddingGenerator::generate_response(&request);

        assert_eq!(response.object, "list");
        assert_eq!(response.model, "text-embedding-ada-002");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].object, "embedding");
        assert_eq!(response.data[0].index, 0);
        assert_eq!(response.data[0].embedding.len(), 1536); // Default ada-002 dimensions
        assert!(response.usage.prompt_tokens > 0);
        assert_eq!(response.usage.total_tokens, response.usage.prompt_tokens);
    }

    #[test]
    fn test_generate_response_multiple_inputs() {
        let mut request = create_test_request();
        request.input = EmbeddingInput::StringArray(vec![
            "First text".to_string(),
            "Second text".to_string(),
            "Third text".to_string(),
        ]);

        let response = EmbeddingGenerator::generate_response(&request);

        assert_eq!(response.data.len(), 3);
        for (i, embedding) in response.data.iter().enumerate() {
            assert_eq!(embedding.index, i as u32);
            assert_eq!(embedding.embedding.len(), 1536);
            assert_eq!(embedding.object, "embedding");
        }
    }

    #[test]
    fn test_custom_dimensions() {
        let mut request = create_test_request();
        request.dimensions = Some(512);

        let response = EmbeddingGenerator::generate_response(&request);

        assert_eq!(response.data[0].embedding.len(), 512);
    }

    #[test]
    fn test_different_models() {
        let models_and_dimensions = vec![
            ("text-embedding-ada-002", 1536),
            ("text-embedding-3-large", 3072),
            ("text-similarity-babbage-001", 2048),
            ("unknown-model", 1536), // Should default to 1536
        ];

        for (model, expected_dims) in models_and_dimensions {
            let mut request = create_test_request();
            request.model = model.to_string();

            let response = EmbeddingGenerator::generate_response(&request);
            assert_eq!(
                response.data[0].embedding.len(),
                expected_dims,
                "Model {} should have {} dimensions",
                model,
                expected_dims
            );
        }
    }

    #[test]
    fn test_consistency() {
        let request = create_test_request();
        let response1 = EmbeddingGenerator::generate_response(&request);
        let response2 = EmbeddingGenerator::generate_response(&request);

        // Same input should produce identical embeddings
        assert_eq!(response1.data[0].embedding, response2.data[0].embedding);
    }

    #[test]
    fn test_vector_normalization() {
        let request = create_test_request();
        let response = EmbeddingGenerator::generate_response(&request);

        let vector = &response.data[0].embedding;
        let magnitude: f64 = vector.iter().map(|x| x * x).sum::<f64>().sqrt();

        // Vector should be approximately unit length (allowing for floating point precision)
        assert!(
            (magnitude - 1.0).abs() < 1e-10,
            "Vector magnitude should be close to 1.0, got {}",
            magnitude
        );
    }

    #[test]
    fn test_different_inputs_produce_different_embeddings() {
        let mut request1 = create_test_request();
        request1.input = EmbeddingInput::String("Hello world".to_string());

        let mut request2 = create_test_request();
        request2.input = EmbeddingInput::String("Goodbye world".to_string());

        let response1 = EmbeddingGenerator::generate_response(&request1);
        let response2 = EmbeddingGenerator::generate_response(&request2);

        // Different inputs should produce different embeddings
        assert_ne!(response1.data[0].embedding, response2.data[0].embedding);
    }

    #[test]
    fn test_integer_array_input() {
        let mut request = create_test_request();
        request.input = EmbeddingInput::IntegerArray(vec![1, 2, 3, 4, 5]);

        let response = EmbeddingGenerator::generate_response(&request);

        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].embedding.len(), 1536);
    }

    #[test]
    fn test_nested_integer_array_input() {
        let mut request = create_test_request();
        request.input = EmbeddingInput::IntegerArrayArray(vec![vec![1, 2, 3], vec![4, 5, 6]]);

        let response = EmbeddingGenerator::generate_response(&request);

        assert_eq!(response.data.len(), 2);
        for embedding in &response.data {
            assert_eq!(embedding.embedding.len(), 1536);
        }
    }

    #[test]
    fn test_hash_consistency() {
        let hash1 = EmbeddingGenerator::hash_string("test");
        let hash2 = EmbeddingGenerator::hash_string("test");
        let hash3 = EmbeddingGenerator::hash_string("different");

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_usage_calculation() {
        let request = create_test_request();
        let response = EmbeddingGenerator::generate_response(&request);

        assert!(response.usage.prompt_tokens > 0);
        assert_eq!(response.usage.total_tokens, response.usage.prompt_tokens);
    }

    #[test]
    fn test_token_estimation() {
        assert!(EmbeddingGenerator::estimate_tokens("hello") > 0);
        assert!(
            EmbeddingGenerator::estimate_tokens("hello world test")
                > EmbeddingGenerator::estimate_tokens("hello")
        );
    }

    #[test]
    fn test_extract_input_strings() {
        // Test string input
        let input1 = EmbeddingInput::String("test".to_string());
        let result1 = EmbeddingGenerator::extract_input_strings(&input1);
        assert_eq!(result1, vec!["test"]);

        // Test string array input
        let input2 = EmbeddingInput::StringArray(vec!["test1".to_string(), "test2".to_string()]);
        let result2 = EmbeddingGenerator::extract_input_strings(&input2);
        assert_eq!(result2, vec!["test1", "test2"]);

        // Test integer array input
        let input3 = EmbeddingInput::IntegerArray(vec![1, 2, 3]);
        let result3 = EmbeddingGenerator::extract_input_strings(&input3);
        assert_eq!(result3.len(), 1);
        assert!(result3[0].contains("1"));

        // Test nested integer array input
        let input4 = EmbeddingInput::IntegerArrayArray(vec![vec![1, 2], vec![3, 4]]);
        let result4 = EmbeddingGenerator::extract_input_strings(&input4);
        assert_eq!(result4.len(), 2);
    }

    #[test]
    fn test_vector_values_in_reasonable_range() {
        let request = create_test_request();
        let response = EmbeddingGenerator::generate_response(&request);

        let vector = &response.data[0].embedding;

        // All values should be in a reasonable range (since we normalize, they should be between -1 and 1)
        for &value in vector {
            assert!(
                value >= -1.0 && value <= 1.0,
                "Vector value {} is out of range [-1, 1]",
                value
            );
        }
    }
}
