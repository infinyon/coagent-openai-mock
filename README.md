# OpenAI Mock Server

A lightweight, fast HTTP server that mimics the OpenAI API for testing and development purposes. This server provides realistic fake responses for OpenAI's text generation and embedding endpoints while maintaining full compatibility with standard OpenAI client libraries.

## 🚀 Features

- **Complete API Compatibility**: Supports both legacy completions (`/v1/completions`) and chat completions (`/v1/chat/completions`) APIs
- **Embeddings API**: `/v1/embeddings` endpoint with consistent fake vectors
- **Tool/Function Calling**: Full support for OpenAI's function calling in chat completions
- **Authentication**: API key validation using Bearer token authentication
- **CORS Support**: Cross-origin requests for browser-based applications
- **Health Monitoring**: Built-in health check endpoint
- **Zero Dependencies**: No external API calls - completely offline
- **Realistic Responses**: Context-aware response generation

## 📦 Installation

### From Source

```bash
git clone <repository-url>
cd openai-mock
cargo build --release
```

### Running

```bash
cargo run
```

Or run the compiled binary:

```bash
./target/release/main
```

## 🛠️ Configuration

The server can be configured using command-line arguments:

| Argument | Default Value | Description |
|----------|---------------|-------------|
| `--host` | `0.0.0.0` | Server host address |
| `--port` | `13673` | Server port |
| `--api-key` | `sk-mock-openai-api-key-12345` | API key for authentication |
| `--request-timeout-secs` | `30` | Request timeout duration in seconds |
| `--enable-cors` | `true` | Enable CORS middleware |
| `--enable-logging` | `true` | Enable request logging middleware |
| `--log-level` | `info` | Logging level (`trace`, `debug`, `info`, `warn`, `error`) |

### Example with Custom Configuration

```bash
# Run with custom host and port
cargo run -- --host 127.0.0.1 --port 13673

# Run with custom API key and debug logging
cargo run -- --api-key sk-my-custom-key --log-level debug

# Production-like configuration
cargo run -- \
  --host 0.0.0.0 \
  --port 13673 \
  --api-key sk-production-key \
  --request-timeout-secs 60 \
  --enable-cors true \
  --log-level warn
```

### View Available Options

```bash
# View help
cargo run -- --help

# View version
cargo run -- --version
```

### Migration from Environment Variables

If you were previously using environment variables, here's the mapping to CLI arguments:

| Old Environment Variable | New CLI Argument |
|--------------------------|------------------|
| `HOST=127.0.0.1` | `--host 127.0.0.1` |
| `PORT=13673` | `--port 13673` |
| `OPENAI_MOCK_API_KEY=sk-key` | `--api-key sk-key` |
| `REQUEST_TIMEOUT=60` | `--request-timeout-secs 60` |
| `ENABLE_CORS=false` | `--enable-cors false` |
| `ENABLE_LOGGING=false` | `--enable-logging false` |
| `LOG_LEVEL=debug` | `--log-level debug` |

**Before (Environment Variables):**
```bash
export HOST=127.0.0.1
export PORT=13673
export OPENAI_MOCK_API_KEY=sk-test-key
export LOG_LEVEL=debug
cargo run
```

**After (CLI Arguments):**
```bash
cargo run -- --host 127.0.0.1 --port 13673 --api-key sk-test-key --log-level debug
```

## 🔑 Authentication

All API endpoints require authentication using an API key (configurable via `--api-key`):

```
Authorization: Bearer sk-mock-openai-api-key-12345
```

You can customize the API key when starting the server:

```bash
cargo run -- --api-key sk-your-custom-key

# Or with the compiled binary
./target/release/main --api-key sk-your-custom-key
```

### Common CLI Usage Patterns

```bash
# Development with custom port to avoid conflicts
cargo run -- --port 13674

# Testing with specific API key
cargo run -- --api-key sk-test-key-123

# Disable CORS for testing
cargo run -- --enable-cors false

# High timeout for slow operations
cargo run -- --request-timeout-secs 120

# Minimal logging for performance testing
cargo run -- --log-level error --enable-logging false

# Complete custom configuration
cargo run -- \
  --host 127.0.0.1 \
  --port 13673 \
  --api-key sk-dev-key \
  --request-timeout-secs 45 \
  --enable-cors true \
  --enable-logging true \
  --log-level debug
```

## 📡 Available Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/` | Server information |
| `GET` | `/health` | Health check |
| `POST` | `/v1/completions` | Legacy text completion |
| `POST` | `/v1/chat/completions` | Chat completions (recommended) |
| `POST` | `/v1/embeddings` | Text embeddings |

## 🧪 API Usage Examples

### Chat Completions (Recommended)

The modern chat completions API supports multi-turn conversations and function calling:

```bash
curl -X POST http://localhost:13673/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer sk-mock-openai-api-key-12345" \
  -d '{
    "model": "gpt-3.5-turbo",
    "messages": [
      {"role": "system", "content": "You are a helpful assistant."},
      {"role": "user", "content": "Explain quantum computing in simple terms"}
    ],
    "max_tokens": 150,
    "temperature": 0.7
  }'
```

**Response:**
```json
{
  "id": "chatcmpl-abc123",
  "object": "chat.completion",
  "created": 1677649420,
  "model": "gpt-3.5-turbo",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": "Quantum computing is a revolutionary approach to computation that harnesses the principles of quantum mechanics..."
      },
      "finish_reason": "stop"
    }
  ],
  "usage": {
    "prompt_tokens": 25,
    "completion_tokens": 87,
    "total_tokens": 112
  }
}
```

### Function Calling

The chat completions API supports OpenAI's function calling feature:

```bash
curl -X POST http://localhost:13673/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer sk-mock-openai-api-key-12345" \
  -d '{
    "model": "gpt-3.5-turbo",
    "messages": [
      {"role": "user", "content": "What is the weather like in San Francisco?"}
    ],
    "tools": [
      {
        "type": "function",
        "function": {
          "name": "get_weather",
          "description": "Get the current weather for a location",
          "parameters": {
            "type": "object",
            "properties": {
              "location": {
                "type": "string",
                "description": "The city and state, e.g. San Francisco, CA"
              }
            },
            "required": ["location"]
          }
        }
      }
    ],
    "tool_choice": "auto"
  }'
```

### Legacy Completions

The legacy completions API for single-turn text completion:

```bash
curl -X POST http://localhost:13673/v1/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer sk-mock-openai-api-key-12345" \
  -d '{
    "model": "text-davinci-003",
    "prompt": "Write a haiku about programming:",
    "max_tokens": 60,
    "temperature": 0.8
  }'
```

**Response:**
```json
{
  "id": "cmpl-abc123",
  "object": "text_completion",
  "created": 1677649420,
  "model": "text-davinci-003",
  "choices": [
    {
      "text": "\n\nCode flows like water,\nBugs dance in morning sunlight,\nSolutions take form.",
      "index": 0,
      "finish_reason": "stop"
    }
  ],
  "usage": {
    "prompt_tokens": 6,
    "completion_tokens": 16,
    "total_tokens": 22
  }
}
```

### Embeddings

Generate text embeddings for semantic search and similarity tasks:

```bash
curl -X POST http://localhost:13673/v1/embeddings \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer sk-mock-openai-api-key-12345" \
  -d '{
    "model": "text-embedding-ada-002",
    "input": "The quick brown fox jumps over the lazy dog"
  }'
```

**Response:**
```json
{
  "object": "list",
  "data": [
    {
      "object": "embedding",
      "index": 0,
      "embedding": [0.123, -0.456, 0.789, ...]
    }
  ],
  "model": "text-embedding-ada-002",
  "usage": {
    "prompt_tokens": 9,
    "total_tokens": 9
  }
}
```

### Health Check

Monitor server health:

```bash
curl http://localhost:13673/health
```

## 🔧 Supported Models

### Chat Completions
- `gpt-3.5-turbo`
- `gpt-4`
- `gpt-4o`
- `gpt-4-turbo`

### Legacy Completions
- `text-davinci-003`
- `text-davinci-002`
- `gpt-3.5-turbo-instruct`

### Embeddings
- `text-embedding-ada-002`
- `text-embedding-3-small`
- `text-embedding-3-large`

## 📝 Common Parameters

### Chat Completions Parameters
| Parameter | Type | Description | Default |
|-----------|------|-------------|---------|
| `model` | string | Model to use | Required |
| `messages` | array | Conversation messages | Required |
| `max_tokens` | integer | Maximum tokens to generate | No limit |
| `temperature` | number | Sampling temperature (0-2) | 1.0 |
| `top_p` | number | Nucleus sampling parameter | 1.0 |
| `n` | integer | Number of completions | 1 |
| `stream` | boolean | Stream the response | false |
| `stop` | string/array | Stop sequences | null |
| `presence_penalty` | number | Penalty for new tokens (-2.0 to 2.0) | 0 |
| `frequency_penalty` | number | Penalty for repeated tokens (-2.0 to 2.0) | 0 |
| `tools` | array | Available functions/tools | null |
| `tool_choice` | string/object | Tool selection strategy | "auto" |

### Legacy Completions Parameters
| Parameter | Type | Description | Default |
|-----------|------|-------------|---------|
| `model` | string | Model to use | Required |
| `prompt` | string/array | Text prompt to complete | Required |
| `max_tokens` | integer | Maximum tokens to generate | 16 |
| `temperature` | number | Sampling temperature (0-2) | 1.0 |
| `top_p` | number | Nucleus sampling parameter | 1.0 |
| `n` | integer | Number of completions | 1 |
| `stream` | boolean | Stream the response | false |
| `stop` | string/array | Stop sequences | null |
| `presence_penalty` | number | Penalty for new tokens (-2.0 to 2.0) | 0 |
| `frequency_penalty` | number | Penalty for repeated tokens (-2.0 to 2.0) | 0 |

## 🚦 Error Handling

The server returns OpenAI-compatible error responses:

```json
{
  "error": {
    "message": "Invalid request: missing required field 'model'",
    "type": "invalid_request_error",
    "param": "model",
    "code": null
  }
}
```

Common error types:
- `invalid_request_error`: Malformed request
- `invalid_api_key`: Invalid or missing API key
- `rate_limit_exceeded`: Rate limiting (not implemented in mock)
- `server_error`: Internal server error

## 🧪 Testing and Development

### Running Tests

```bash
# Run all tests
cargo test

# Run only integration tests
cargo test --test server_integration

# Run portpicker examples
cargo test --test portpicker_examples
```

### Testing with Portpicker

This project uses the `portpicker` crate to automatically select free ports for integration tests, preventing port conflicts when running tests in parallel or in CI environments.

#### Basic Test Server Usage

```rust
use common::TestServer;

#[tokio::test]
async fn test_my_feature() {
    // Automatically picks a free port and starts server
    let server = TestServer::start().await.expect("Server should start");
    
    // Test your endpoints
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health", server.url()))
        .send()
        .await
        .expect("Health check should work");
    
    assert!(response.status().is_success());
    // Server automatically stops when dropped
}
```

#### Manual Port Selection

```rust
#[tokio::test]
async fn test_with_custom_port() {
    let port = portpicker::pick_unused_port().expect("Should pick a free port");
    let config = Config::builder()
        .host("127.0.0.1")
        .port(port)
        .api_key("sk-test-key")
        .build();
    
    // Use config to start server...
}
```

For detailed testing patterns and examples, see [`docs/TESTING_WITH_PORTPICKER.md`](docs/TESTING_WITH_PORTPICKER.md).

### Running with Debug Logging

```bash
cargo run -- --log-level debug
```

## 🎯 Use Cases

- **Development**: Test AI features without API costs
- **CI/CD**: Automated testing of AI-powered applications
- **Prototyping**: Rapid development without external dependencies
- **Offline Development**: Work without internet connectivity
- **Rate Limit Testing**: Test application behavior under various conditions
- **Cost Control**: Avoid unexpected API charges during development

## 🔧 Building and Deployment

### Docker (Optional)

Create a `Dockerfile`:

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/main /usr/local/bin/openai-mock
EXPOSE 13673
ENTRYPOINT ["openai-mock"]
CMD ["--host", "0.0.0.0", "--port", "13673"]
```

Build and run:

```bash
docker build -t openai-mock .

# Run with default configuration
docker run -p 13673:13673 openai-mock

# Run with custom configuration
docker run -p 8080:8080 openai-mock --host 0.0.0.0 --port 8080 --api-key sk-custom-key

# Run with custom API key and debug logging
docker run -p 13673:13673 openai-mock --api-key sk-production-key --log-level debug

# Run with CORS disabled
docker run -p 13673:13673 openai-mock --enable-cors false --log-level warn
```

## 📚 API Compatibility

This mock server implements the OpenAI API specification and is compatible with:

- OpenAI Python client (`openai` package)
- OpenAI Node.js client
- LangChain
- LlamaIndex
- Any application using OpenAI's REST API

## ⚠️ Limitations

- Responses are generated locally and may not match real OpenAI model behavior
- No actual AI processing - responses are rule-based and deterministic
- Some advanced features like streaming are simulated
- Rate limiting is not enforced (all requests are accepted)

## 🆘 Support

For issues and questions:

1. Check existing GitHub issues
2. Create a new issue with detailed information
3. Include request/response examples when reporting bugs

---

**Note**: This is a mock server for development and testing purposes only. For production applications, use the official OpenAI API.
