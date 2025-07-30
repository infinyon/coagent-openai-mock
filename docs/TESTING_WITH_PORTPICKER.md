# Testing with Portpicker

This document explains how to use the `portpicker` crate to select free ports for integration tests in the OpenAI Mock Server project.

## Overview

When running integration tests that start actual servers, port conflicts can occur if tests try to use the same port simultaneously. The `portpicker` crate solves this by automatically selecting unused ports for each test.

## Installation

The `portpicker` crate is already included in the `dev-dependencies` of this project:

```toml
[dev-dependencies]
portpicker = "0.1"
reqwest = { version = "0.12", features = ["json"] }
futures = "0.3"
```

## Basic Usage

### 1. Simple Port Selection

```rust
#[tokio::test]
async fn test_with_free_port() {
    // Pick a free port
    let port = portpicker::pick_unused_port().expect("Should pick a free port");
    
    // Use it in your configuration
    let config = Config::builder()
        .host("127.0.0.1")
        .port(port)
        .api_key("sk-test-key")
        .build();
    
    println!("Using port: {}", port);
    assert!(port > 1024); // Should be above well-known ports
}
```

### 2. Using the TestServer Wrapper (Recommended)

The project provides a `TestServer` wrapper that automatically handles port selection and server lifecycle:

```rust
mod common;
use common::TestServer;

#[tokio::test]
async fn test_with_test_server() {
    // Automatically picks a free port and starts the server
    let server = TestServer::start().await.expect("Server should start");
    
    println!("Server running on: {}", server.url());
    
    // Test your endpoints
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health", server.url()))
        .send()
        .await
        .expect("Health check should work");
    
    assert!(response.status().is_success());
    
    // Server automatically stops when `server` is dropped
}
```

### 3. Multiple Servers

```rust
#[tokio::test]
async fn test_multiple_servers() {
    // Each server gets its own free port
    let server1 = TestServer::start().await.expect("Server 1 should start");
    let server2 = TestServer::start().await.expect("Server 2 should start");
    
    // Verify they have different ports
    assert_ne!(server1.port(), server2.port());
    
    // Test both servers
    let client = reqwest::Client::new();
    
    let response1 = client.get(&format!("{}/health", server1.url())).send().await.unwrap();
    let response2 = client.get(&format!("{}/health", server2.url())).send().await.unwrap();
    
    assert!(response1.status().is_success());
    assert!(response2.status().is_success());
}
```

### 4. Pre-allocating Multiple Ports

Sometimes you need to know all ports upfront:

```rust
use common::pick_multiple_ports;

#[tokio::test]
async fn test_with_preallocated_ports() {
    let ports = pick_multiple_ports(3);
    println!("Pre-allocated ports: {:?}", ports);
    
    // Create configs with the pre-allocated ports
    let configs: Vec<_> = ports
        .iter()
        .enumerate()
        .map(|(i, &port)| {
            Config::builder()
                .host("127.0.0.1")
                .port(port)
                .api_key(&format!("sk-test-key-{}", i))
                .build()
        })
        .collect();
    
    // Use the configs to start servers...
}
```

## Common Patterns

### Parallel Test Execution

When running tests in parallel, each test gets its own port:

```rust
#[tokio::test]
async fn test_parallel_execution() {
    const NUM_PARALLEL_TESTS: usize = 5;
    let barrier = Arc::new(Barrier::new(NUM_PARALLEL_TESTS));
    
    let mut handles = Vec::new();
    for i in 0..NUM_PARALLEL_TESTS {
        let barrier = barrier.clone();
        let handle = tokio::spawn(async move {
            // Each task gets its own server with a unique port
            let server = TestServer::start().await.expect("Server should start");
            
            // Wait for all tasks to be ready
            barrier.wait().await;
            
            // Perform the test
            let client = reqwest::Client::new();
            let response = client
                .get(&format!("{}/health", server.url()))
                .send()
                .await
                .expect("Health check should work");
            
            assert!(response.status().is_success());
            server.port() // Return port for verification
        });
        handles.push(handle);
    }
    
    let results = futures::future::try_join_all(handles).await.unwrap();
    
    // Verify all tasks got different ports
    for i in 0..results.len() {
        for j in i + 1..results.len() {
            assert_ne!(results[i], results[j]);
        }
    }
}
```

### Error Handling

Handle cases where port allocation might fail:

```rust
#[tokio::test]
async fn test_with_retry_logic() {
    let max_retries = 3;
    let mut last_error = None;
    
    for attempt in 0..max_retries {
        match TestServer::start().await {
            Ok(server) => {
                println!("Server started on attempt {}", attempt + 1);
                
                // Test the server
                let client = reqwest::Client::new();
                let response = client
                    .get(&format!("{}/health", server.url()))
                    .send()
                    .await
                    .expect("Health check should work");
                
                assert!(response.status().is_success());
                return; // Success!
            }
            Err(e) => {
                println!("Attempt {} failed: {}", attempt + 1, e);
                last_error = Some(e);
                
                if attempt < max_retries - 1 {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }
        }
    }
    
    panic!("Failed to start server after {} attempts. Last error: {:?}", max_retries, last_error);
}
```

## Test Utilities

The project provides several utility functions in `tests/common/mod.rs`:

- `create_test_config()` - Creates a config with a free port
- `create_test_server()` - Creates a server config with a free port
- `create_test_server_with_key(api_key)` - Creates a server config with custom API key
- `wait_for_server_ready(base_url)` - Waits for server to be ready
- `pick_multiple_ports(count)` - Pre-allocates multiple ports
- `TestServer::start()` - Starts a managed test server
- `TestServer::start_with_key(api_key)` - Starts a server with custom API key

## Best Practices

1. **Use the TestServer wrapper** - It handles port selection and cleanup automatically
2. **Don't hardcode ports** - Always use portpicker for integration tests
3. **Test port uniqueness** - Verify different test instances get different ports
4. **Handle failures gracefully** - Port allocation can occasionally fail
5. **Clean up resources** - The TestServer wrapper handles this automatically
6. **Use appropriate timeouts** - Server startup can take time

## Running Tests

To run all integration tests:

```bash
cargo test --test server_integration
```

To run portpicker examples:

```bash
cargo test --test portpicker_examples
```

To run a specific test:

```bash
cargo test test_server_startup_and_health_check
```

## Troubleshooting

### Port Already in Use

If you get "port already in use" errors, it usually means:
1. Another test is still using the port
2. A previous test didn't clean up properly
3. The system is running low on available ports

Solutions:
- Use the TestServer wrapper for automatic cleanup
- Add retry logic with different ports
- Ensure tests properly clean up resources

### Tests Hanging

If tests hang, check:
1. Server startup is properly awaited
2. The `wait_for_server_ready` function times out appropriately
3. Network connectivity (firewalls, etc.)

### Port Range Issues

By default, portpicker selects from the ephemeral port range. If you need specific ranges, you may need to implement custom logic or use system configuration.

## Examples

See `tests/portpicker_examples.rs` for comprehensive examples of different usage patterns, including:
- Basic port selection
- Multiple servers
- Parallel execution
- Error handling
- Performance testing
- External service integration

These examples demonstrate real-world scenarios and best practices for using portpicker in your tests.