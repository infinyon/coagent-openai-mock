use openai_mock::models::{
    CompletionChoice, CompletionUsage, CreateCompletionResponse, CreateEmbeddingResponse,
    EmbeddingData, EmbeddingUsage,
};

fn main() {
    // Test completion response
    let usage = CompletionUsage::new(10, 20);
    let choice = CompletionChoice::new("Hello, world!".to_string(), 0, "stop".to_string());
    let response = CreateCompletionResponse::new(
        "cmpl-123".to_string(),
        "text-davinci-003".to_string(),
        1677649420,
        vec![choice],
        usage,
    );

    println!("Completion Response JSON:");
    println!("{}", serde_json::to_string_pretty(&response).unwrap());

    // Test embedding response
    let usage = EmbeddingUsage::new(8);
    let embedding = EmbeddingData::new(vec![0.1, 0.2, 0.3, -0.1, -0.2], 0);
    let response =
        CreateEmbeddingResponse::new(vec![embedding], "text-embedding-ada-002".to_string(), usage);

    println!("\nEmbedding Response JSON:");
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
}
