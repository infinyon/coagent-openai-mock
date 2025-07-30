//! Example demonstrating the embedding generator functionality
//!
//! This example shows how to use the EmbeddingGenerator to create
//! fake OpenAI embedding responses for testing purposes.

use openai_mock::generators::EmbeddingGenerator;
use openai_mock::models::requests::{CreateEmbeddingRequest, EmbeddingInput};

fn main() {
    println!("OpenAI Mock - Embedding Generator Example");
    println!("==========================================\n");

    // Example 1: Basic embedding request with single string
    let basic_request = CreateEmbeddingRequest {
        model: "text-embedding-ada-002".to_string(),
        input: EmbeddingInput::String("Hello, world!".to_string()),
        encoding_format: None,
        dimensions: None,
        user: None,
    };

    let response = EmbeddingGenerator::generate_response(&basic_request);
    println!("Example 1: Basic Single String Embedding");
    println!("Input: Hello, world!");
    println!("Model: {}", response.model);
    println!("Number of embeddings: {}", response.data.len());
    println!("Embedding dimensions: {}", response.data[0].embedding.len());
    println!("First few values: {:?}", &response.data[0].embedding[0..5]);
    println!("Usage: {} tokens\n", response.usage.prompt_tokens);

    // Example 2: Multiple string inputs
    let multi_request = CreateEmbeddingRequest {
        model: "text-embedding-ada-002".to_string(),
        input: EmbeddingInput::StringArray(vec![
            "The cat sat on the mat".to_string(),
            "Machine learning is fascinating".to_string(),
            "OpenAI creates powerful AI models".to_string(),
        ]),
        encoding_format: None,
        dimensions: None,
        user: None,
    };

    let response = EmbeddingGenerator::generate_response(&multi_request);
    println!("Example 2: Multiple String Embeddings");
    println!("Number of inputs: 3");
    println!("Number of embeddings: {}", response.data.len());
    for (i, embedding) in response.data.iter().enumerate() {
        println!(
            "Embedding {}: {} dimensions, index: {}",
            i + 1,
            embedding.embedding.len(),
            embedding.index
        );
    }
    println!("Usage: {} tokens\n", response.usage.prompt_tokens);

    // Example 3: Different embedding model with different dimensions
    let large_model_request = CreateEmbeddingRequest {
        model: "text-embedding-3-large".to_string(),
        input: EmbeddingInput::String("This is a test with a larger model".to_string()),
        encoding_format: None,
        dimensions: None,
        user: None,
    };

    let response = EmbeddingGenerator::generate_response(&large_model_request);
    println!("Example 3: Large Model (3072 dimensions)");
    println!("Model: {}", response.model);
    println!("Embedding dimensions: {}", response.data[0].embedding.len());
    println!("Usage: {} tokens\n", response.usage.prompt_tokens);

    // Example 4: Custom dimensions
    let custom_dims_request = CreateEmbeddingRequest {
        model: "text-embedding-ada-002".to_string(),
        input: EmbeddingInput::String("Custom dimension example".to_string()),
        encoding_format: None,
        dimensions: Some(256),
        user: None,
    };

    let response = EmbeddingGenerator::generate_response(&custom_dims_request);
    println!("Example 4: Custom Dimensions");
    println!("Requested dimensions: 256");
    println!("Actual dimensions: {}", response.data[0].embedding.len());
    println!("Usage: {} tokens\n", response.usage.prompt_tokens);

    // Example 5: Integer array input
    let int_array_request = CreateEmbeddingRequest {
        model: "text-embedding-ada-002".to_string(),
        input: EmbeddingInput::IntegerArray(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
        encoding_format: None,
        dimensions: None,
        user: None,
    };

    let response = EmbeddingGenerator::generate_response(&int_array_request);
    println!("Example 5: Integer Array Input");
    println!("Input: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]");
    println!("Number of embeddings: {}", response.data.len());
    println!("Embedding dimensions: {}", response.data[0].embedding.len());
    println!("Usage: {} tokens\n", response.usage.prompt_tokens);

    // Example 6: Nested integer array input
    let nested_int_request = CreateEmbeddingRequest {
        model: "text-embedding-ada-002".to_string(),
        input: EmbeddingInput::IntegerArrayArray(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        encoding_format: None,
        dimensions: None,
        user: None,
    };

    let response = EmbeddingGenerator::generate_response(&nested_int_request);
    println!("Example 6: Nested Integer Array Input");
    println!("Input: [[1,2,3], [4,5,6], [7,8,9]]");
    println!("Number of embeddings: {}", response.data.len());
    for (i, embedding) in response.data.iter().enumerate() {
        println!(
            "Embedding {}: {} dimensions",
            i + 1,
            embedding.embedding.len()
        );
    }
    println!("Usage: {} tokens\n", response.usage.prompt_tokens);

    // Example 7: Consistency demonstration
    let consistency_request = CreateEmbeddingRequest {
        model: "text-embedding-ada-002".to_string(),
        input: EmbeddingInput::String("Consistency test".to_string()),
        encoding_format: None,
        dimensions: None,
        user: None,
    };

    let response1 = EmbeddingGenerator::generate_response(&consistency_request);
    let response2 = EmbeddingGenerator::generate_response(&consistency_request);

    println!("Example 7: Consistency Check");
    println!("Same input should produce identical embeddings:");
    println!(
        "First call - first 5 values: {:?}",
        &response1.data[0].embedding[0..5]
    );
    println!(
        "Second call - first 5 values: {:?}",
        &response2.data[0].embedding[0..5]
    );
    println!(
        "Embeddings are identical: {}",
        response1.data[0].embedding == response2.data[0].embedding
    );
    println!();

    // Example 8: Vector properties demonstration
    let vector_request = CreateEmbeddingRequest {
        model: "text-embedding-ada-002".to_string(),
        input: EmbeddingInput::String("Vector analysis example".to_string()),
        encoding_format: None,
        dimensions: None,
        user: None,
    };

    let response = EmbeddingGenerator::generate_response(&vector_request);
    let vector = &response.data[0].embedding;

    // Calculate vector magnitude (should be close to 1.0 since vectors are normalized)
    let magnitude: f64 = vector.iter().map(|x| x * x).sum::<f64>().sqrt();

    // Find min and max values
    let min_value = vector.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_value = vector.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    println!("Example 8: Vector Properties");
    println!("Vector magnitude: {:.10} (should be ~1.0)", magnitude);
    println!("Value range: [{:.6}, {:.6}]", min_value, max_value);
    println!(
        "All values in [-1, 1]: {}",
        vector.iter().all(|&x| x >= -1.0 && x <= 1.0)
    );

    println!("\n==========================================");
    println!("All examples completed successfully!");
    println!("The embedding generator creates consistent, normalized");
    println!("vectors that are suitable for testing OpenAI integrations.");
}
