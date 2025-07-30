//! Example demonstrating the completion generator functionality
//!
//! This example shows how to use the CompletionGenerator to create
//! fake OpenAI completion responses for testing purposes.

use openai_mock::generators::CompletionGenerator;
use openai_mock::models::requests::{CreateCompletionRequest, PromptInput};

fn main() {
    println!("OpenAI Mock - Completion Generator Example");
    println!("==========================================\n");

    // Example 1: Basic completion request
    let basic_request = CreateCompletionRequest {
        model: "text-davinci-003".to_string(),
        prompt: PromptInput::String("Hello, how are you?".to_string()),
        suffix: None,
        max_tokens: Some(50),
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

    let response = CompletionGenerator::generate_response(&basic_request);
    println!("Example 1: Basic Completion");
    println!("Request: Hello, how are you?");
    println!("Response ID: {}", response.id);
    println!("Model: {}", response.model);
    println!("Completion: {}", response.choices[0].text);
    println!("Finish Reason: {}", response.choices[0].finish_reason);
    println!(
        "Usage: {} prompt + {} completion = {} total tokens\n",
        response.usage.prompt_tokens, response.usage.completion_tokens, response.usage.total_tokens
    );

    // Example 2: Creative writing prompt
    let creative_request = CreateCompletionRequest {
        model: "text-davinci-003".to_string(),
        prompt: PromptInput::String("Write a story about a magical forest".to_string()),
        suffix: None,
        max_tokens: Some(100),
        temperature: Some(0.9),
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

    let response = CompletionGenerator::generate_response(&creative_request);
    println!("Example 2: Creative Writing");
    println!("Request: Write a story about a magical forest");
    println!("Response ID: {}", response.id);
    println!("Completion: {}", response.choices[0].text);
    println!("Usage: {} total tokens\n", response.usage.total_tokens);

    // Example 3: Code generation prompt
    let code_request = CreateCompletionRequest {
        model: "code-davinci-002".to_string(),
        prompt: PromptInput::String("Write a function that calculates fibonacci".to_string()),
        suffix: None,
        max_tokens: Some(150),
        temperature: Some(0.2),
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

    let response = CompletionGenerator::generate_response(&code_request);
    println!("Example 3: Code Generation");
    println!("Request: Write a function that calculates fibonacci");
    println!("Response ID: {}", response.id);
    println!("Completion: {}", response.choices[0].text);
    println!("Usage: {} total tokens\n", response.usage.total_tokens);

    // Example 4: Multiple choices
    let multi_request = CreateCompletionRequest {
        model: "text-davinci-003".to_string(),
        prompt: PromptInput::String("Explain quantum physics".to_string()),
        suffix: None,
        max_tokens: Some(75),
        temperature: Some(0.8),
        top_p: None,
        n: Some(3),
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

    let response = CompletionGenerator::generate_response(&multi_request);
    println!("Example 4: Multiple Choices");
    println!("Request: Explain quantum physics");
    println!("Response ID: {}", response.id);
    println!("Number of choices: {}", response.choices.len());
    for (i, choice) in response.choices.iter().enumerate() {
        println!("Choice {}: {}", i + 1, choice.text);
    }
    println!("Usage: {} total tokens\n", response.usage.total_tokens);

    // Example 5: With log probabilities
    let logprobs_request = CreateCompletionRequest {
        model: "text-davinci-003".to_string(),
        prompt: PromptInput::String("The weather today is".to_string()),
        suffix: None,
        max_tokens: Some(20),
        temperature: Some(0.5),
        top_p: None,
        n: Some(1),
        stream: None,
        logprobs: Some(3),
        echo: None,
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        best_of: None,
        logit_bias: None,
        user: None,
    };

    let response = CompletionGenerator::generate_response(&logprobs_request);
    println!("Example 5: With Log Probabilities");
    println!("Request: The weather today is");
    println!("Response ID: {}", response.id);
    println!("Completion: {}", response.choices[0].text);
    if let Some(logprobs) = &response.choices[0].logprobs {
        println!(
            "Log probabilities available for {} tokens:",
            logprobs.tokens.len()
        );
        for (token, logprob) in logprobs.tokens.iter().zip(logprobs.token_logprobs.iter()) {
            if let Some(prob) = logprob {
                println!("  '{token}': {prob:.3}");
            }
        }
    }
    println!("Usage: {} total tokens\n", response.usage.total_tokens);

    // Example 6: With echo
    let echo_request = CreateCompletionRequest {
        model: "text-davinci-003".to_string(),
        prompt: PromptInput::String("Complete this sentence: The best way to learn".to_string()),
        suffix: None,
        max_tokens: Some(30),
        temperature: Some(0.7),
        top_p: None,
        n: Some(1),
        stream: None,
        logprobs: None,
        echo: Some(true),
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        best_of: None,
        logit_bias: None,
        user: None,
    };

    let response = CompletionGenerator::generate_response(&echo_request);
    println!("Example 6: With Echo");
    println!("Request: Complete this sentence: The best way to learn");
    println!("Response ID: {}", response.id);
    println!("Response (with echo): {}", response.choices[0].text);
    println!("Usage: {} total tokens", response.usage.total_tokens);

    println!("\n==========================================");
    println!("All examples completed successfully!");
}
