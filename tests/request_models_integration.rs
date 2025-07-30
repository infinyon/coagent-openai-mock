use openai_mock::models::{
    CreateCompletionRequest, CreateEmbeddingRequest, EmbeddingInput, PromptInput, StopSequences,
};

#[test]
fn test_completion_request_like_openai_docs() {
    // Test case similar to OpenAI API documentation examples
    let json = r#"{
        "model": "text-davinci-003",
        "prompt": "Say this is a test",
        "max_tokens": 7,
        "temperature": 0
    }"#;

    let request: CreateCompletionRequest =
        serde_json::from_str(json).expect("Should parse completion request from JSON");

    assert_eq!(request.model, "text-davinci-003");
    assert!(matches!(request.prompt, PromptInput::String(ref s) if s == "Say this is a test"));
    assert_eq!(request.max_tokens, Some(7));
    assert_eq!(request.temperature, Some(0.0));

    // Validate the request
    assert!(request.validate().is_ok());
}

#[test]
fn test_completion_request_with_array_prompt() {
    // Test with array prompt as supported by OpenAI API
    let json = r#"{
        "model": "text-davinci-003",
        "prompt": ["Complete this:", "Also complete this:"],
        "max_tokens": 50,
        "temperature": 0.7,
        "n": 2
    }"#;

    let request: CreateCompletionRequest =
        serde_json::from_str(json).expect("Should parse completion request with array prompt");

    assert_eq!(request.model, "text-davinci-003");
    assert!(matches!(request.prompt, PromptInput::Array(ref arr) if arr.len() == 2));
    assert_eq!(request.max_tokens, Some(50));
    assert_eq!(request.temperature, Some(0.7));
    assert_eq!(request.n, Some(2));

    assert!(request.validate().is_ok());
}

#[test]
fn test_embedding_request_like_openai_docs() {
    // Test case similar to OpenAI API documentation examples
    let json = r#"{
        "input": "The food was delicious and the waiter...",
        "model": "text-embedding-ada-002",
        "encoding_format": "float"
    }"#;

    let request: CreateEmbeddingRequest =
        serde_json::from_str(json).expect("Should parse embedding request from JSON");

    assert_eq!(request.model, "text-embedding-ada-002");
    assert!(matches!(request.input, EmbeddingInput::String(ref s) if s.contains("delicious")));
    assert_eq!(request.encoding_format, Some("float".to_string()));

    // Validate the request
    assert!(request.validate().is_ok());
}

#[test]
fn test_embedding_request_with_array_input() {
    // Test with array input as supported by OpenAI API
    let json = r#"{
        "input": ["The quick brown fox", "jumped over the lazy dog"],
        "model": "text-embedding-ada-002",
        "encoding_format": "float"
    }"#;

    let request: CreateEmbeddingRequest =
        serde_json::from_str(json).expect("Should parse embedding request with array input");

    assert_eq!(request.model, "text-embedding-ada-002");
    assert!(matches!(request.input, EmbeddingInput::StringArray(ref arr) if arr.len() == 2));
    assert_eq!(request.encoding_format, Some("float".to_string()));

    assert!(request.validate().is_ok());

    // Test the utility method
    let input_strings = request.get_input_strings();
    assert_eq!(input_strings.len(), 2);
    assert!(input_strings[0].contains("fox"));
    assert!(input_strings[1].contains("dog"));
}

#[test]
fn test_completion_request_with_stop_sequences() {
    // Test with stop sequences (string and array variants)
    let json_string_stop = r#"{
        "model": "text-davinci-003",
        "prompt": "List three colors:",
        "stop": "\n"
    }"#;

    let request: CreateCompletionRequest = serde_json::from_str(json_string_stop)
        .expect("Should parse completion request with string stop");
    assert!(request.validate().is_ok());

    let json_array_stop = r#"{
        "model": "text-davinci-003",
        "prompt": "List three colors:",
        "stop": ["\n", ".", "END"]
    }"#;

    let request: CreateCompletionRequest = serde_json::from_str(json_array_stop)
        .expect("Should parse completion request with array stop");
    assert!(request.validate().is_ok());
}

#[test]
fn test_real_world_completion_request() {
    // Complex real-world example with many parameters
    let json = r#"{
        "model": "text-davinci-003",
        "prompt": "Write a creative story about a robot learning to paint:",
        "max_tokens": 150,
        "temperature": 0.8,
        "top_p": 0.9,
        "n": 1,
        "stream": false,
        "logprobs": 2,
        "echo": false,
        "stop": ["\n\n", "THE END"],
        "presence_penalty": 0.1,
        "frequency_penalty": 0.2,
        "user": "creative-writing-app-user-123"
    }"#;

    let request: CreateCompletionRequest =
        serde_json::from_str(json).expect("Should parse complex completion request");

    assert_eq!(request.model, "text-davinci-003");
    assert_eq!(request.max_tokens, Some(150));
    assert_eq!(request.temperature, Some(0.8));
    assert_eq!(request.top_p, Some(0.9));
    assert_eq!(request.n, Some(1));
    assert_eq!(request.stream, Some(false));
    assert_eq!(request.logprobs, Some(2));
    assert_eq!(request.echo, Some(false));
    assert_eq!(request.presence_penalty, Some(0.1));
    assert_eq!(request.frequency_penalty, Some(0.2));
    assert_eq!(
        request.user,
        Some("creative-writing-app-user-123".to_string())
    );

    assert!(request.validate().is_ok());
}

#[test]
fn test_real_world_embedding_request() {
    // Real-world embedding example with dimensions
    let json = r#"{
        "input": [
            "OpenAI provides cutting-edge AI models",
            "Machine learning is transforming industries",
            "Natural language processing enables human-computer interaction"
        ],
        "model": "text-embedding-ada-002",
        "encoding_format": "float",
        "dimensions": 1536,
        "user": "semantic-search-app-user-456"
    }"#;

    let request: CreateEmbeddingRequest =
        serde_json::from_str(json).expect("Should parse complex embedding request");

    assert_eq!(request.model, "text-embedding-ada-002");
    assert!(matches!(request.input, EmbeddingInput::StringArray(ref arr) if arr.len() == 3));
    assert_eq!(request.encoding_format, Some("float".to_string()));
    assert_eq!(request.dimensions, Some(1536));
    assert_eq!(
        request.user,
        Some("semantic-search-app-user-456".to_string())
    );

    assert!(request.validate().is_ok());
}

#[test]
fn test_validation_errors() {
    // Test various validation errors

    // Empty model
    let invalid_request = CreateCompletionRequest {
        model: "".to_string(),
        prompt: PromptInput::String("test".to_string()),
        max_tokens: None,
        temperature: None,
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
        suffix: None,
    };
    assert!(invalid_request.validate().is_err());

    // Temperature out of range
    let mut invalid_request = invalid_request.clone();
    invalid_request.model = "valid-model".to_string();
    invalid_request.temperature = Some(3.0);
    assert!(invalid_request.validate().is_err());

    // Invalid dimensions for embedding
    let invalid_embedding = CreateEmbeddingRequest {
        model: "text-embedding-ada-002".to_string(),
        input: EmbeddingInput::String("test".to_string()),
        encoding_format: None,
        dimensions: Some(0),
        user: None,
    };
    assert!(invalid_embedding.validate().is_err());
}

#[test]
fn test_serialization_roundtrip() {
    // Test that we can serialize and deserialize without data loss
    let original = CreateCompletionRequest {
        model: "test-model".to_string(),
        prompt: PromptInput::Array(vec!["prompt1".to_string(), "prompt2".to_string()]),
        max_tokens: Some(100),
        temperature: Some(0.7),
        top_p: Some(0.9),
        n: Some(2),
        stream: Some(false),
        logprobs: Some(2),
        echo: Some(true),
        stop: Some(StopSequences::Array(vec![
            "\n".to_string(),
            "END".to_string(),
        ])),
        presence_penalty: Some(0.1),
        frequency_penalty: Some(0.2),
        best_of: Some(3),
        logit_bias: None,
        user: Some("test-user".to_string()),
        suffix: Some(" [SUFFIX]".to_string()),
    };

    let json = serde_json::to_string(&original).expect("Should serialize");
    let deserialized: CreateCompletionRequest =
        serde_json::from_str(&json).expect("Should deserialize");

    assert_eq!(original.model, deserialized.model);
    assert_eq!(original.max_tokens, deserialized.max_tokens);
    assert_eq!(original.temperature, deserialized.temperature);
    assert_eq!(original.user, deserialized.user);
    assert_eq!(original.suffix, deserialized.suffix);
}

#[test]
fn test_openai_api_compatibility() {
    // These are actual examples from OpenAI API documentation
    // Testing that our models can parse them correctly

    // Example from OpenAI docs for completions
    let openai_completion_example = r#"{
        "model": "text-davinci-003",
        "prompt": "Say this is a test",
        "max_tokens": 7,
        "temperature": 0
    }"#;

    let request: CreateCompletionRequest = serde_json::from_str(openai_completion_example)
        .expect("Should parse OpenAI completion example");
    assert!(request.validate().is_ok());
    assert_eq!(request.model, "text-davinci-003");
    assert_eq!(request.max_tokens, Some(7));

    // Example from OpenAI docs for embeddings
    let openai_embedding_example = r#"{
        "input": "The food was delicious and the waiter...",
        "model": "text-embedding-ada-002",
        "encoding_format": "float"
    }"#;

    let request: CreateEmbeddingRequest = serde_json::from_str(openai_embedding_example)
        .expect("Should parse OpenAI embedding example");
    assert!(request.validate().is_ok());
    assert_eq!(request.model, "text-embedding-ada-002");
    assert_eq!(request.encoding_format, Some("float".to_string()));

    // Streaming completion example
    let streaming_example = r#"{
        "model": "text-davinci-003",
        "prompt": "Tell me a story",
        "max_tokens": 100,
        "stream": true
    }"#;

    let request: CreateCompletionRequest =
        serde_json::from_str(streaming_example).expect("Should parse streaming completion example");
    assert!(request.validate().is_ok());
    assert_eq!(request.stream, Some(true));
}
