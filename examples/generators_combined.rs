//! Combined example demonstrating both completion and embedding generators
//!
//! This example shows how both the CompletionGenerator and EmbeddingGenerator
//! work together to provide comprehensive OpenAI API mocking capabilities.

use openai_mock::generators::{CompletionGenerator, EmbeddingGenerator};
use openai_mock::models::requests::{
    CreateCompletionRequest, CreateEmbeddingRequest, EmbeddingInput, PromptInput,
};

fn main() {
    println!("OpenAI Mock - Combined Generators Example");
    println!("=========================================\n");

    // Scenario: Building a simple chatbot that uses both completions and embeddings
    let user_query = "What is machine learning?";
    let knowledge_base = vec![
        "Machine learning is a subset of artificial intelligence".to_string(),
        "Deep learning uses neural networks with multiple layers".to_string(),
        "Supervised learning requires labeled training data".to_string(),
        "Unsupervised learning finds patterns in unlabeled data".to_string(),
    ];

    println!("🤖 Chatbot Scenario: User asks '{}'", user_query);
    println!("📚 Knowledge base has {} entries\n", knowledge_base.len());

    // Step 1: Generate embeddings for the user query
    println!("Step 1: Generating embedding for user query...");
    let query_embedding_request = CreateEmbeddingRequest {
        model: "text-embedding-ada-002".to_string(),
        input: EmbeddingInput::String(user_query.to_string()),
        encoding_format: None,
        dimensions: None,
        user: None,
    };

    let query_embedding_response = EmbeddingGenerator::generate_response(&query_embedding_request);
    println!("✅ Query embedding generated:");
    println!(
        "   - Dimensions: {}",
        query_embedding_response.data[0].embedding.len()
    );
    println!(
        "   - Tokens used: {}",
        query_embedding_response.usage.prompt_tokens
    );

    // Show first few values as a preview
    let query_vector = &query_embedding_response.data[0].embedding;
    println!("   - First 5 values: {:?}", &query_vector[0..5]);

    // Step 2: Generate embeddings for knowledge base entries
    println!("\nStep 2: Generating embeddings for knowledge base...");
    let kb_embedding_request = CreateEmbeddingRequest {
        model: "text-embedding-ada-002".to_string(),
        input: EmbeddingInput::StringArray(knowledge_base.clone()),
        encoding_format: None,
        dimensions: None,
        user: None,
    };

    let kb_embedding_response = EmbeddingGenerator::generate_response(&kb_embedding_request);
    println!("✅ Knowledge base embeddings generated:");
    println!(
        "   - Number of embeddings: {}",
        kb_embedding_response.data.len()
    );
    println!(
        "   - Total tokens used: {}",
        kb_embedding_response.usage.prompt_tokens
    );

    // Step 3: Simulate similarity search (in real scenario, we'd compute cosine similarity)
    println!("\nStep 3: Simulating similarity search...");
    let most_relevant_idx = 0; // In reality, this would be the highest similarity score
    let most_relevant_context = &knowledge_base[most_relevant_idx];
    println!("✅ Most relevant context found:");
    println!("   - Index: {}", most_relevant_idx);
    println!("   - Text: '{}'", most_relevant_context);

    // Step 4: Generate completion using the relevant context
    println!("\nStep 4: Generating completion with context...");
    let completion_prompt = format!(
        "Context: {}\n\nQuestion: {}\n\nAnswer:",
        most_relevant_context, user_query
    );

    let completion_request = CreateCompletionRequest {
        model: "text-davinci-003".to_string(),
        prompt: PromptInput::String(completion_prompt.clone()),
        suffix: None,
        max_tokens: Some(150),
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
    };

    let completion_response = CompletionGenerator::generate_response(&completion_request);
    println!("✅ Completion generated:");
    println!("   - Response ID: {}", completion_response.id);
    println!("   - Model: {}", completion_response.model);
    println!(
        "   - Finish reason: {}",
        completion_response.choices[0].finish_reason
    );
    println!(
        "   - Tokens: {} prompt + {} completion = {} total",
        completion_response.usage.prompt_tokens,
        completion_response.usage.completion_tokens,
        completion_response.usage.total_tokens
    );

    println!("\n💬 Final Answer:");
    println!("{}", completion_response.choices[0].text);

    // Demonstrate consistency across multiple calls
    println!("\n{}", "=".repeat(50));
    println!("🔄 Consistency Demonstration");
    println!("{}", "=".repeat(50));

    println!("\nTesting that identical inputs produce identical outputs...");

    // Test completion consistency
    let completion_response_2 = CompletionGenerator::generate_response(&completion_request);
    let completions_identical =
        completion_response.choices[0].text == completion_response_2.choices[0].text;
    println!("✅ Completion consistency: {}", completions_identical);

    // Test embedding consistency
    let query_embedding_response_2 =
        EmbeddingGenerator::generate_response(&query_embedding_request);
    let embeddings_identical =
        query_embedding_response.data[0].embedding == query_embedding_response_2.data[0].embedding;
    println!("✅ Embedding consistency: {}", embeddings_identical);

    // Performance metrics summary
    println!("\n{}", "=".repeat(50));
    println!("📊 Performance Summary");
    println!("{}", "=".repeat(50));

    let total_embedding_tokens =
        query_embedding_response.usage.prompt_tokens + kb_embedding_response.usage.prompt_tokens;
    let total_completion_tokens = completion_response.usage.total_tokens;
    let total_tokens = total_embedding_tokens + total_completion_tokens;

    println!("Embedding operations:");
    println!(
        "  - Query embedding: {} tokens",
        query_embedding_response.usage.prompt_tokens
    );
    println!(
        "  - Knowledge base embeddings: {} tokens",
        kb_embedding_response.usage.prompt_tokens
    );
    println!("  - Total embedding tokens: {}", total_embedding_tokens);

    println!("\nCompletion operations:");
    println!("  - Completion tokens: {}", total_completion_tokens);

    println!("\nOverall:");
    println!("  - Total API tokens: {}", total_tokens);
    println!("  - Number of API calls: 3 (1 query embedding + 1 KB embedding + 1 completion)");
    println!(
        "  - Knowledge base entries processed: {}",
        knowledge_base.len()
    );

    // Show different model capabilities
    println!("\n{}", "=".repeat(50));
    println!("🎯 Model Capabilities Demo");
    println!("{}", "=".repeat(50));

    // Different embedding models
    let models_to_test = vec![
        ("text-embedding-ada-002", 1536),
        ("text-embedding-3-large", 3072),
        ("text-similarity-babbage-001", 2048),
    ];

    for (model, _expected_dims) in models_to_test {
        let test_request = CreateEmbeddingRequest {
            model: model.to_string(),
            input: EmbeddingInput::String("Test input".to_string()),
            encoding_format: None,
            dimensions: None,
            user: None,
        };

        let test_response = EmbeddingGenerator::generate_response(&test_request);
        println!(
            "✅ {}: {} dimensions",
            model,
            test_response.data[0].embedding.len()
        );
    }

    // Different completion patterns
    let completion_patterns = vec![
        ("Hello, how are you?", "greeting"),
        ("Write a story about space", "creative"),
        ("Explain quantum physics", "explanatory"),
        ("Write a Python function", "code"),
    ];

    println!("\nCompletion patterns:");
    for (prompt, pattern_type) in completion_patterns {
        let test_request = CreateCompletionRequest {
            model: "text-davinci-003".to_string(),
            prompt: PromptInput::String(prompt.to_string()),
            suffix: None,
            max_tokens: Some(20),
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
        };

        let test_response = CompletionGenerator::generate_response(&test_request);
        println!(
            "✅ {} pattern: '{}' → '{}'",
            pattern_type,
            prompt,
            test_response.choices[0].text.trim()
        );
    }

    println!("\n{}", "=".repeat(50));
    println!("🎉 All examples completed successfully!");
    println!("The OpenAI Mock generators provide:");
    println!("  ✓ Consistent, deterministic responses");
    println!("  ✓ Realistic token usage calculations");
    println!("  ✓ Proper OpenAI API format compliance");
    println!("  ✓ Multiple model support");
    println!("  ✓ Various input type handling");
    println!("  ✓ Suitable for comprehensive testing");
    println!("{}", "=".repeat(50));
}
