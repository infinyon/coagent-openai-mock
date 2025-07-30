pub mod chat_completions;
pub mod completions;
pub mod embeddings;

// Re-export handlers for easier access
pub use chat_completions::create_chat_completion;
pub use completions::create_completion;
pub use embeddings::create_embedding;
