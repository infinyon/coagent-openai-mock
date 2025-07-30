# Request Models Documentation

This document describes the request models implemented for the OpenAI API mock server.

## Overview

The request models are designed to be fully compatible with the OpenAI API specification, supporting both the `/v1/completions` and `/v1/embeddings` endpoints. They include comprehensive validation and flexible input types.

## Completion Request Model

### `CreateCompletionRequest`

The completion request model supports all standard OpenAI completion parameters:

```rust
use openai_mock::models::CreateCompletionRequest;

let request = CreateCompletionRequest {
    model: "text-davinci-003".to_string(),
    prompt: PromptInput::String("Complete this sentence".to_string()),
    max_tokens: Some(100),
    temperature: Some(0.7),
    // ... other optional fields
};
```

#### Required Fields

- `model`: String - The model ID to use for completion
- `prompt`: PromptInput - The prompt text (string or array of strings)

#### Optional Fields

- `max_tokens`: Maximum tokens to generate (1-4096)
- `temperature`: Sampling temperature (0.0-2.0)
- `top_p`: Nucleus sampling parameter (0.0-1.0)
- `n`: Number of completions to generate (1-20)
- `stream`: Whether to stream responses
- `logprobs`: Number of log probabilities to return (0-5)
- `echo`: Whether to echo the prompt
- `stop`: Stop sequences (string or array)
- `presence_penalty`: Presence penalty (-2.0 to 2.0)
- `frequency_penalty`: Frequency penalty (-2.0 to 2.0)
- `best_of`: Generate best_of completions server-side
- `logit_bias`: Token bias modifications
- `user`: End-user identifier
- `suffix`: Completion suffix

#### Prompt Input Types

The `PromptInput` enum supports flexible prompt formats:

```rust
// Single string prompt
PromptInput::String("Hello world".to_string())

// Array of prompts
PromptInput::Array(vec![
    "Complete this:".to_string(),
    "Also complete this:".to_string()
])
```

#### Stop Sequences

Stop sequences can be specified as a single string or array:

```rust
// Single stop sequence
StopSequences::String("\n".to_string())

// Multiple stop sequences
StopSequences::Array(vec!["\n".to_string(), "END".to_string()])
```

## Embedding Request Model

### `CreateEmbeddingRequest`

The embedding request model supports OpenAI's text embedding parameters:

```rust
use openai_mock::models::CreateEmbeddingRequest;

let request = CreateEmbeddingRequest {
    model: "text-embedding-ada-002".to_string(),
    input: EmbeddingInput::String("Text to embed".to_string()),
    encoding_format: Some("float".to_string()),
    dimensions: Some(1536),
    user: Some("user-123".to_string()),
};
```

#### Required Fields

- `model`: String - The embedding model to use
- `input`: EmbeddingInput - Text input to embed

#### Optional Fields

- `encoding_format`: Format for embeddings ("float" or "base64")
- `dimensions`: Number of embedding dimensions (1-3072)
- `user`: End-user identifier

#### Input Types

The `EmbeddingInput` enum supports multiple input formats:

```rust
// Single string
EmbeddingInput::String("Hello world".to_string())

// Array of strings
EmbeddingInput::StringArray(vec![
    "First text".to_string(),
    "Second text".to_string()
])

// Token arrays (for pre-tokenized input)
EmbeddingInput::IntegerArray(vec![1, 2, 3, 4])
EmbeddingInput::IntegerArrayArray(vec![vec![1, 2], vec![3, 4]])
```

## JSON Examples

### Completion Request

```json
{
  "model": "text-davinci-003",
  "prompt": "Write a creative story about a robot:",
  "max_tokens": 150,
  "temperature": 0.8,
  "top_p": 0.9,
  "stop": ["\n\n", "THE END"],
  "presence_penalty": 0.1,
  "frequency_penalty": 0.2,
  "user": "creative-app-user-123"
}
```

### Embedding Request

```json
{
  "input": [
    "OpenAI provides cutting-edge AI models",
    "Machine learning is transforming industries"
  ],
  "model": "text-embedding-ada-002",
  "encoding_format": "float",
  "dimensions": 1536,
  "user": "semantic-search-app"
}
```

## Validation

Both request models include comprehensive validation:

### Completion Validation

- Model name cannot be empty
- Prompt cannot be empty
- `max_tokens` must be 1-4096
- `temperature` must be 0.0-2.0
- `top_p` must be 0.0-1.0
- `n` must be 1-20
- `logprobs` must be 0-5
- Penalties must be -2.0 to 2.0
- `best_of` must be >= `n` and <= 20

### Embedding Validation

- Model name cannot be empty
- Input cannot be empty
- `encoding_format` must be "float" or "base64"
- `dimensions` must be 1-3072

## Usage in Handlers

```rust
use openai_mock::models::CreateCompletionRequest;
use serde_json;

// Parse JSON request
let request: CreateCompletionRequest = serde_json::from_str(&json_body)?;

// Validate the request
request.validate()?;

// Use the request data
println!("Model: {}", request.model);
match request.prompt {
    PromptInput::String(text) => println!("Prompt: {}", text),
    PromptInput::Array(prompts) => println!("Prompts: {:?}", prompts),
}
```

## Testing

The models include comprehensive test coverage:

- Unit tests for validation logic
- Integration tests with real JSON payloads
- Serialization/deserialization round-trip tests
- Edge case validation
- OpenAI API compatibility tests

Run tests with:

```bash
cargo test models::requests
cargo test --test request_models_integration
```
