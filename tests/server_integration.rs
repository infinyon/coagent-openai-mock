//! Integration tests that start the actual server on a free port
//! Uses portpicker to avoid port conflicts in test environments

mod common;

use common::TestServer;
use serde_json::json;

#[tokio::test]
async fn test_server_startup_and_health_check() {
    let server = TestServer::start().await.expect("Server should start");

    // Test health endpoint
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health", server.url()))
        .send()
        .await
        .expect("Health check request failed");

    assert!(response.status().is_success());

    let body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse health check response");

    assert_eq!(body["status"], "healthy");
    assert_eq!(body["service"], "openai-mock");
    assert!(body["version"].is_string());
    assert!(body["endpoints"].is_array());

    // Server will be cleaned up automatically when dropped
}

#[tokio::test]
async fn test_completions_endpoint_integration() {
    let server = TestServer::start().await.expect("Server should start");

    // Test completions endpoint
    let client = reqwest::Client::new();
    let request_body = json!({
        "model": "text-davinci-003",
        "prompt": "Hello, world!",
        "max_tokens": 50,
        "temperature": 0.7
    });

    let response = client
        .post(&format!("{}/v1/completions", server.url()))
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer sk-test-integration-key")
        .json(&request_body)
        .send()
        .await
        .expect("Completions request failed");

    assert!(response.status().is_success());

    let body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse completions response");

    // Verify response structure
    assert!(body["id"].is_string());
    assert_eq!(body["object"], "text_completion");
    assert!(body["created"].is_number());
    assert_eq!(body["model"], "text-davinci-003");
    assert!(body["choices"].is_array());

    let choices = body["choices"].as_array().unwrap();
    assert!(!choices.is_empty());
    assert!(choices[0]["text"].is_string());
    assert!(choices[0]["index"].is_number());
    assert!(choices[0]["finish_reason"].is_string());
}

#[tokio::test]
async fn test_chat_completions_endpoint_integration() {
    let server = TestServer::start().await.expect("Server should start");

    // Test chat completions endpoint
    let client = reqwest::Client::new();
    let request_body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            {"role": "user", "content": "Hello!"}
        ],
        "max_tokens": 50,
        "temperature": 0.7
    });

    let response = client
        .post(&format!("{}/v1/chat/completions", server.url()))
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer sk-test-integration-key")
        .json(&request_body)
        .send()
        .await
        .expect("Chat completions request failed");

    assert!(response.status().is_success());

    let body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse chat completions response");

    // Verify response structure
    assert!(body["id"].is_string());
    assert_eq!(body["object"], "chat.completion");
    assert!(body["created"].is_number());
    assert_eq!(body["model"], "gpt-3.5-turbo");
    assert!(body["choices"].is_array());

    let choices = body["choices"].as_array().unwrap();
    assert!(!choices.is_empty());
    assert!(choices[0]["message"]["role"].is_string());
    assert!(choices[0]["message"]["content"].is_string());
    assert!(choices[0]["index"].is_number());
    assert!(choices[0]["finish_reason"].is_string());
}

#[tokio::test]
async fn test_embeddings_endpoint_integration() {
    let server = TestServer::start().await.expect("Server should start");

    // Test embeddings endpoint
    let client = reqwest::Client::new();
    let request_body = json!({
        "model": "text-embedding-ada-002",
        "input": "The quick brown fox jumps over the lazy dog",
        "encoding_format": "float"
    });

    let response = client
        .post(&format!("{}/v1/embeddings", server.url()))
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer sk-test-integration-key")
        .json(&request_body)
        .send()
        .await
        .expect("Embeddings request failed");

    assert!(response.status().is_success());

    let body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse embeddings response");

    // Verify response structure
    assert_eq!(body["object"], "list");
    assert!(body["data"].is_array());
    assert_eq!(body["model"], "text-embedding-ada-002");

    let data = body["data"].as_array().unwrap();
    assert!(!data.is_empty());
    assert_eq!(data[0]["object"], "embedding");
    assert!(data[0]["embedding"].is_array());
    assert!(data[0]["index"].is_number());
}

#[tokio::test]
async fn test_authentication_errors() {
    let server = TestServer::start().await.expect("Server should start");

    let client = reqwest::Client::new();

    // Test missing authorization
    let request_body = json!({
        "model": "text-davinci-003",
        "prompt": "Hello"
    });

    let response = client
        .post(&format!("{}/v1/completions", server.url()))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .expect("Request should complete");

    assert_eq!(response.status(), 401);

    // Test invalid authorization
    let response = client
        .post(&format!("{}/v1/completions", server.url()))
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer invalid-key")
        .json(&request_body)
        .send()
        .await
        .expect("Request should complete");

    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn test_concurrent_requests() {
    let server = TestServer::start().await.expect("Server should start");

    let client = reqwest::Client::new();

    // Create multiple concurrent requests
    let mut handles = Vec::new();
    for i in 0..5 {
        let client = client.clone();
        let base_url = server.url().to_string();
        let handle = tokio::spawn(async move {
            let request_body = json!({
                "model": "text-davinci-003",
                "prompt": format!("Request number {}", i),
                "max_tokens": 10
            });

            let response = client
                .post(&format!("{}/v1/completions", base_url))
                .header("Content-Type", "application/json")
                .header("Authorization", "Bearer sk-test-integration-key")
                .json(&request_body)
                .send()
                .await
                .expect("Request failed");

            assert!(response.status().is_success());
            response
                .json::<serde_json::Value>()
                .await
                .expect("Failed to parse response")
        });
        handles.push(handle);
    }

    // Wait for all requests to complete
    let results = futures::future::try_join_all(handles)
        .await
        .expect("All requests should succeed");

    // Verify all responses are valid
    for result in results {
        assert!(result["id"].is_string());
        assert_eq!(result["object"], "text_completion");
        assert!(result["choices"].is_array());
    }
}

/// Test helper to ensure multiple test runs don't conflict
#[tokio::test]
async fn test_multiple_servers_different_ports() {
    // Create two servers with different ports
    let server1 = TestServer::start().await.expect("Server 1 should start");
    let server2 = TestServer::start().await.expect("Server 2 should start");

    // Ensure they got different ports
    assert_ne!(server1.port(), server2.port());
    assert_ne!(server1.url(), server2.url());

    // Test both servers respond independently
    let client = reqwest::Client::new();

    let response1 = client
        .get(&format!("{}/health", server1.url()))
        .send()
        .await
        .expect("Health check 1 failed");
    let response2 = client
        .get(&format!("{}/health", server2.url()))
        .send()
        .await
        .expect("Health check 2 failed");

    assert!(response1.status().is_success());
    assert!(response2.status().is_success());

    // Servers will be cleaned up automatically when dropped
}
