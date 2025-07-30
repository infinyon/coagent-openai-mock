//! Examples demonstrating different ways to use portpicker for tests
//!
//! This file shows various patterns for using the portpicker crate
//! to select free ports for integration testing.

mod common;

use common::{TestServer, pick_multiple_ports};
use openai_mock::config::Config;
use std::sync::Arc;
use tokio::sync::Barrier;

/// Example 1: Basic usage - single test server with free port
#[tokio::test]
async fn example_basic_portpicker() {
    // Pick a free port manually
    let port = portpicker::pick_unused_port().expect("Should pick a free port");
    println!("Selected port: {port}");

    // Use it in your test configuration
    let config = Config::builder()
        .host("127.0.0.1")
        .port(port)
        .api_key("sk-test-key")
        .build();

    assert_eq!(config.port, port);
    assert!(port > 1024); // Should be above well-known ports
}

/// Example 2: Using the TestServer wrapper (recommended approach)
#[tokio::test]
async fn example_test_server_wrapper() {
    // The TestServer automatically picks a free port and manages lifecycle
    let server = TestServer::start().await.expect("Server should start");

    println!("Server running on: {}", server.url());
    println!("Port: {}", server.port());

    // Test your endpoints
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health", server.url()))
        .send()
        .await
        .expect("Health check should work");

    assert!(response.status().is_success());

    // Server automatically stops when `server` is dropped
}

/// Example 3: Multiple servers with different ports
#[tokio::test]
async fn example_multiple_servers() {
    // Each server gets its own free port
    let server1 = TestServer::start().await.expect("Server 1 should start");
    let server2 = TestServer::start().await.expect("Server 2 should start");
    let server3 = TestServer::start().await.expect("Server 3 should start");

    // Verify they all have different ports
    let ports = vec![server1.port(), server2.port(), server3.port()];
    println!("Allocated ports: {ports:?}");

    // All ports should be unique
    for i in 0..ports.len() {
        for j in i + 1..ports.len() {
            assert_ne!(ports[i], ports[j], "Ports should be unique");
        }
    }

    // All servers should be responding
    let client = reqwest::Client::new();
    for server in [&server1, &server2, &server3] {
        let response = client
            .get(format!("{}/health", server.url()))
            .send()
            .await
            .expect("Health check should work");
        assert!(response.status().is_success());
    }
}

/// Example 4: Pre-allocating multiple ports
#[tokio::test]
async fn example_preallocate_ports() {
    // Sometimes you need to know all ports upfront
    let ports = pick_multiple_ports(3);
    println!("Pre-allocated ports: {ports:?}");

    // Create configs with the pre-allocated ports
    let configs: Vec<_> = ports
        .iter()
        .enumerate()
        .map(|(i, &port)| {
            Config::builder()
                .host("127.0.0.1")
                .port(port)
                .api_key(format!("sk-test-key-{i}"))
                .build()
        })
        .collect();

    // Verify each config has the expected port
    for (i, config) in configs.iter().enumerate() {
        assert_eq!(config.port, ports[i]);
        println!("Config {}: {}:{}", i, config.host, config.port);
    }
}

/// Example 5: Parallel test execution with port isolation
#[tokio::test]
async fn example_parallel_test_isolation() {
    const NUM_PARALLEL_TESTS: usize = 5;

    // Create a barrier to synchronize test start
    let barrier = Arc::new(Barrier::new(NUM_PARALLEL_TESTS));

    // Spawn multiple parallel tasks, each with its own server
    let mut handles = Vec::new();
    for i in 0..NUM_PARALLEL_TESTS {
        let barrier = barrier.clone();
        let handle = tokio::spawn(async move {
            // Each task gets its own server with a unique port
            let server = TestServer::start()
                .await
                .unwrap_or_else(|_| panic!("Server {i} should start"));

            println!("Task {} got port {}", i, server.port());

            // Wait for all tasks to be ready
            barrier.wait().await;

            // Perform the actual test
            let client = reqwest::Client::new();
            let response = client
                .get(format!("{}/health", server.url()))
                .send()
                .await
                .unwrap_or_else(|_| panic!("Health check {i} should work"));

            assert!(response.status().is_success());

            // Return the port for verification
            server.port()
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete and collect results
    let results = futures::future::try_join_all(handles)
        .await
        .expect("All tasks should complete successfully");

    // Verify all tasks got different ports
    println!("All task ports: {results:?}");
    for i in 0..results.len() {
        for j in i + 1..results.len() {
            assert_ne!(
                results[i], results[j],
                "Tasks {i} and {j} should have different ports"
            );
        }
    }
}

/// Example 6: Testing port exhaustion handling
#[tokio::test]
async fn example_port_exhaustion_handling() {
    // In a real scenario, you might want to handle the case where
    // no free ports are available (though this is rare)

    let mut successful_allocations = 0;
    let max_attempts = 10;

    for i in 0..max_attempts {
        match portpicker::pick_unused_port() {
            Some(port) => {
                println!("Attempt {i}: Got port {port}");
                successful_allocations += 1;
            }
            None => {
                println!("Attempt {i}: No free port available");
                break;
            }
        }
    }

    // Should be able to allocate at least some ports
    assert!(
        successful_allocations > 0,
        "Should be able to allocate at least one port"
    );
    println!("Successfully allocated {successful_allocations} ports");
}

/// Example 7: Custom port range (if your system supports it)
#[tokio::test]
async fn example_port_selection_patterns() {
    // portpicker doesn't support custom ranges directly, but you can
    // implement your own logic if needed

    let mut high_ports = Vec::new();
    let mut attempts = 0;
    const MAX_ATTEMPTS: usize = 20;

    // Try to get ports in the higher range (useful for avoiding conflicts)
    while high_ports.len() < 3 && attempts < MAX_ATTEMPTS {
        if let Some(port) = portpicker::pick_unused_port() {
            if port > 40000 {
                high_ports.push(port);
                println!("Found high port: {port}");
            }
        }
        attempts += 1;
    }

    if !high_ports.is_empty() {
        println!("High ports found: {high_ports:?}");

        // Use one of the high ports for testing
        let server = TestServer::start().await.expect("Server should start");
        println!("Server started on port: {}", server.port());
    } else {
        println!("No high ports found in {MAX_ATTEMPTS} attempts");
    }
}

/// Example 8: Integration with external services
#[tokio::test]
async fn example_external_service_integration() {
    // When testing integration with external services, you might need
    // multiple servers or want to verify port allocation patterns

    let main_server = TestServer::start().await.expect("Main server should start");

    // Simulate starting a mock external service
    let mock_external_port =
        portpicker::pick_unused_port().expect("Should pick port for mock external service");

    println!("Main server: {}", main_server.url());
    println!("Mock external service port: {mock_external_port}");

    // In a real test, you might start an actual mock service here
    // For this example, we'll just verify the ports are different
    assert_ne!(
        main_server.port(),
        mock_external_port,
        "Services should use different ports"
    );

    // Test the main server
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health", main_server.url()))
        .send()
        .await
        .expect("Health check should work");

    assert!(response.status().is_success());
}

/// Example 9: Performance testing with port management
#[tokio::test]
async fn example_performance_testing() {
    use std::time::Instant;

    let start = Instant::now();

    // Measure port allocation performance
    let mut ports = Vec::new();
    for _ in 0..10 {
        if let Some(port) = portpicker::pick_unused_port() {
            ports.push(port);
        }
    }

    let allocation_time = start.elapsed();
    println!("Allocated {} ports in {:?}", ports.len(), allocation_time);

    // Start a server and measure startup time
    let start = Instant::now();
    let server = TestServer::start().await.expect("Server should start");
    let startup_time = start.elapsed();

    println!("Server startup took: {startup_time:?}");
    println!("Server running on port: {}", server.port());

    // Verify the server is responsive
    let client = reqwest::Client::new();
    let start = Instant::now();
    let response = client
        .get(format!("{}/health", server.url()))
        .send()
        .await
        .expect("Health check should work");
    let response_time = start.elapsed();

    assert!(response.status().is_success());
    println!("Health check response time: {response_time:?}");

    // Assert reasonable performance (adjust thresholds as needed)
    assert!(
        allocation_time.as_millis() < 1000,
        "Port allocation should be fast"
    );
    assert!(
        startup_time.as_secs() < 10,
        "Server startup should be reasonable"
    );
    assert!(
        response_time.as_millis() < 1000,
        "Health check should be fast"
    );
}

/// Example 10: Error handling and retry logic
#[tokio::test]
async fn example_error_handling() {
    // Implement retry logic for server startup
    let max_retries = 3;
    let mut last_error = None;

    for attempt in 0..max_retries {
        match TestServer::start().await {
            Ok(server) => {
                println!("Server started successfully on attempt {}", attempt + 1);
                println!("Server URL: {}", server.url());

                // Test the server
                let client = reqwest::Client::new();
                let response = client
                    .get(format!("{}/health", server.url()))
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
                    // Wait a bit before retrying
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                }
            }
        }
    }

    // If we get here, all attempts failed
    panic!(
        "Failed to start server after {max_retries} attempts. Last error: {last_error:?}"
    );
}
