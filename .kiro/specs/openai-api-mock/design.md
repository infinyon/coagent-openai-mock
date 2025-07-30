# Design Document

## Overview

The OpenAI API Mock is a Rust crate that provides a lightweight HTTP server implementation mimicking the OpenAI API. The server uses the Poem web framework to handle HTTP requests and responses, implementing the `/v1/completions` and `/v1/embeddings` endpoints with proper authentication and response formatting.

The design prioritizes modularity and extensibility, allowing easy addition of new endpoints while maintaining compliance with the official OpenAI OpenAPI specification. The server returns syntactically valid fake responses that can be consumed by standard OpenAI client libraries without modification.

## Architecture

The application follows a layered architecture pattern:

```
┌─────────────────────────────────────┐
│           HTTP Layer (Poem)         │
│  ┌─────────────────────────────────┐ │
│  │        Route Handlers           │ │
│  └─────────────────────────────────┘ │
└─────────────────────────────────────┘
┌─────────────────────────────────────┐
│         Business Logic Layer        │
│  ┌─────────────┐ ┌─────────────────┐ │
│  │ Auth Service│ │ Response Builder│ │
│  └─────────────┘ └─────────────────┘ │
└─────────────────────────────────────┘
┌─────────────────────────────────────┐
│           Data Models               │
│  ┌─────────────┐ ┌─────────────────┐ │
│  │ Request DTOs│ │ Response DTOs   │ │
│  └─────────────┘ └─────────────────┘ │
└─────────────────────────────────────┘
```

### Key Architectural Decisions

1. **Poem Framework**: Chosen for its modern async/await support, built-in middleware system, and clean routing API
2. **Modular Structure**: Separate modules for handlers, models, authentication, and response generation
3. **Static Response Generation**: Fake responses are generated using predefined templates with some randomization
4. **Middleware-based Authentication**: API key validation implemented as Poem middleware for reusability

## Components and Interfaces

### 1. HTTP Server (`src/server.rs`)
- Initializes Poem application with routes and middleware
- Configures CORS, logging, and error handling
- Exposes configuration options (port, host)

### 2. Authentication Middleware (`src/auth.rs`)
- Validates Bearer token format
- Checks against hardcoded API key: `sk-mock-openai-api-key-12345`
- Returns appropriate 401 responses for invalid authentication
- Implements `Endpoint` trait for Poem middleware integration

### 3. Route Handlers (`src/handlers/`)

#### Completions Handler (`src/handlers/completions.rs`)
- Handles POST `/v1/completions`
- Validates request body against OpenAI schema
- Generates fake completion responses with realistic structure
- Supports basic parameters: model, prompt, max_tokens, temperature

#### Embeddings Handler (`src/handlers/embeddings.rs`)
- Handles POST `/v1/embeddings`
- Validates request body for input text and model
- Generates fake embedding vectors (1536 dimensions for text-embedding-ada-002)
- Returns properly formatted embedding response

### 4. Data Models (`src/models/`)

#### Request Models (`src/models/requests.rs`)
```rust
#[derive(Deserialize)]
pub struct CreateCompletionRequest {
    pub model: String,
    pub prompt: String,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    // ... other OpenAI parameters
}

#[derive(Deserialize)]
pub struct CreateEmbeddingRequest {
    pub model: String,
    pub input: String,
    // ... other parameters
}
```

#### Response Models (`src/models/responses.rs`)
```rust
#[derive(Serialize)]
pub struct CreateCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<CompletionChoice>,
    pub usage: Usage,
}

#[derive(Serialize)]
pub struct CreateEmbeddingResponse {
    pub object: String,
    pub data: Vec<EmbeddingData>,
    pub model: String,
    pub usage: EmbeddingUsage,
}
```

### 5. Response Generators (`src/generators/`)
- `CompletionGenerator`: Creates realistic fake completion responses
- `EmbeddingGenerator`: Generates random but consistent embedding vectors
- Utilities for generating IDs, timestamps, and usage statistics

### 6. Configuration (`src/config.rs`)
- Server configuration (port, host, API key)
- Environment variable support
- Default values for development

## Data Models

### Authentication
- API Key: Hardcoded string `sk-mock-openai-api-key-12345`
- Header format: `Authorization: Bearer {api_key}`
- Error responses follow OpenAI format with appropriate status codes

### Request/Response Flow
1. Client sends HTTP request with Authorization header
2. Authentication middleware validates API key
3. Route handler deserializes request body
4. Response generator creates fake but valid response
5. JSON response returned with appropriate headers

### Fake Data Generation
- **Completion Text**: Pool of realistic completion responses
- **Embedding Vectors**: Deterministic random vectors based on input hash
- **IDs**: Generated using format `cmpl-{random}` or `emb-{random}`
- **Timestamps**: Current Unix timestamp
- **Usage Stats**: Calculated based on input length

## Error Handling

### Authentication Errors
- Missing Authorization header → 401 with OpenAI-formatted error
- Invalid Bearer token format → 401 with appropriate message
- Wrong API key → 401 with "Invalid API key" message

### Request Validation Errors
- Invalid JSON → 400 with parsing error details
- Missing required fields → 400 with field validation errors
- Invalid parameter values → 400 with parameter-specific messages

### Server Errors
- Internal errors → 500 with generic error message
- Proper error logging for debugging

## Testing Strategy

### Unit Tests
- Authentication middleware validation
- Request/response model serialization
- Response generator logic
- Error handling scenarios

### Integration Tests
- Full HTTP request/response cycles
- Authentication flow testing
- Response format validation

### End-to-End Tests (Hurl)
- Real HTTP requests against running server
- Authentication success/failure scenarios
- Response format validation
- Client library compatibility testing

#### Hurl Test Structure
```hurl
# Test successful completion
POST http://localhost:13673/v1/completions
Authorization: Bearer sk-mock-openai-api-key-12345
Content-Type: application/json
{
  "model": "text-davinci-003",
  "prompt": "Hello world",
  "max_tokens": 10
}

HTTP/1.1 200
[Asserts]
jsonpath "$.object" == "text_completion"
jsonpath "$.choices" count == 1
```

### Performance Considerations
- Async/await throughout for non-blocking I/O
- Minimal memory allocation for fake responses
- Connection pooling handled by Poem
- Graceful shutdown support

### Security Considerations
- API key validation on all protected endpoints
- Input sanitization for request parameters
- Rate limiting can be added as middleware
- CORS configuration for browser clients

This design provides a solid foundation for the OpenAI API mock while maintaining extensibility for future endpoint additions.
