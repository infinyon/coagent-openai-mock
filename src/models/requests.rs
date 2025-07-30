use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request model for the Create Completion endpoint
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateCompletionRequest {
    /// ID of the model to use
    pub model: String,

    /// The prompt(s) to generate completions for
    pub prompt: PromptInput,

    /// The suffix that comes after a completion of inserted text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,

    /// The maximum number of tokens to generate in the completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    /// What sampling temperature to use, between 0 and 2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// How many completions to generate for each prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    /// Whether to stream back partial progress
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Include the log probabilities on the most likely tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<u32>,

    /// Echo back the prompt in addition to the completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<bool>,

    /// Up to 4 sequences where the API will stop generating further tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopSequences>,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,

    /// Generates best_of completions server-side and returns the "best"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<u32>,

    /// Modify the likelihood of specified tokens appearing in the completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, f32>>,

    /// A unique identifier representing your end-user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Prompt input can be a string or array of strings
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum PromptInput {
    String(String),
    Array(Vec<String>),
}

/// Stop sequences can be a string or array of strings
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum StopSequences {
    String(String),
    Array(Vec<String>),
}

/// Request model for the Create Embedding endpoint
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateEmbeddingRequest {
    /// ID of the model to use
    pub model: String,

    /// Input text to embed
    pub input: EmbeddingInput,

    /// The format to return the embeddings in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<String>,

    /// The number of dimensions the resulting output embeddings should have
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<u32>,

    /// A unique identifier representing your end-user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Embedding input can be a string, array of strings, or array of integers
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum EmbeddingInput {
    String(String),
    StringArray(Vec<String>),
    IntegerArray(Vec<i32>),
    IntegerArrayArray(Vec<Vec<i32>>),
}

/// Request model for the Create Chat Completion endpoint
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateChatCompletionRequest {
    /// ID of the model to use
    pub model: String,

    /// A list of messages comprising the conversation so far
    pub messages: Vec<ChatCompletionMessage>,

    /// What sampling temperature to use, between 0 and 2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// How many chat completion choices to generate for each input message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    /// Whether to stream back partial progress
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Up to 4 sequences where the API will stop generating further tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopSequences>,

    /// The maximum number of tokens to generate in the chat completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,

    /// Modify the likelihood of specified tokens appearing in the completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, f32>>,

    /// A unique identifier representing your end-user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// A list of tools the model may call
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ChatCompletionTool>>,

    /// Controls which (if any) tool is called by the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ChatCompletionToolChoice>,
}

/// A message in a chat completion conversation
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatCompletionMessage {
    /// The role of the messages author
    pub role: ChatCompletionRole,

    /// The contents of the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// The name of the author of this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The tool calls generated by the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ChatCompletionMessageToolCall>>,

    /// Tool call that this message is responding to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

/// The role of the message author
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ChatCompletionRole {
    System,
    User,
    Assistant,
    Tool,
}

/// A tool the model may call
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatCompletionTool {
    /// The type of the tool. Currently, only `function` is supported
    #[serde(rename = "type")]
    pub tool_type: String,

    /// The function definition
    pub function: ChatCompletionFunction,
}

/// Function definition for chat completion tools
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatCompletionFunction {
    /// The name of the function to be called
    pub name: String,

    /// A description of what the function does
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The parameters the functions accepts, described as a JSON Schema object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}

/// Tool choice for chat completion
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum ChatCompletionToolChoice {
    None(String),     // "none"
    Auto(String),     // "auto"
    Required(String), // "required"
    Function(ChatCompletionNamedToolChoice),
}

/// Named tool choice for forcing a specific function call
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatCompletionNamedToolChoice {
    /// The type of the tool. Currently, only `function` is supported
    #[serde(rename = "type")]
    pub tool_type: String,

    /// The function to call
    pub function: ChatCompletionFunctionChoice,
}

/// Function choice for named tool selection
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatCompletionFunctionChoice {
    /// The name of the function to call
    pub name: String,
}

/// Tool call made by the model
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatCompletionMessageToolCall {
    /// The ID of the tool call
    pub id: String,

    /// The type of the tool. Currently, only `function` is supported
    #[serde(rename = "type")]
    pub tool_type: String,

    /// The function that the model called
    pub function: ChatCompletionFunctionCall,
}

/// Function call made by the model
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatCompletionFunctionCall {
    /// The name of the function to call
    pub name: String,

    /// The arguments to call the function with, as generated by the model in JSON format
    pub arguments: String,
}

impl CreateCompletionRequest {
    /// Validate the completion request parameters
    pub fn validate(&self) -> Result<(), String> {
        // Validate model is not empty
        if self.model.trim().is_empty() {
            return Err("Model cannot be empty".to_string());
        }

        // Validate prompt is not empty
        match &self.prompt {
            PromptInput::String(s) if s.trim().is_empty() => {
                return Err("Prompt cannot be empty".to_string());
            }
            PromptInput::Array(arr) if arr.is_empty() => {
                return Err("Prompt array cannot be empty".to_string());
            }
            PromptInput::Array(arr) if arr.iter().any(|s| s.trim().is_empty()) => {
                return Err("Prompt array cannot contain empty strings".to_string());
            }
            _ => {}
        }

        // Validate max_tokens
        if let Some(max_tokens) = self.max_tokens {
            if max_tokens == 0 {
                return Err("max_tokens must be greater than 0".to_string());
            }
            if max_tokens > 4096 {
                return Err("max_tokens cannot exceed 4096".to_string());
            }
        }

        // Validate temperature
        if let Some(temperature) = self.temperature {
            if !(0.0..=2.0).contains(&temperature) {
                return Err("temperature must be between 0.0 and 2.0".to_string());
            }
        }

        // Validate top_p
        if let Some(top_p) = self.top_p {
            if !(0.0..=1.0).contains(&top_p) {
                return Err("top_p must be between 0.0 and 1.0".to_string());
            }
        }

        // Validate n
        if let Some(n) = self.n {
            if n == 0 || n > 20 {
                return Err("n must be between 1 and 20".to_string());
            }
        }

        // Validate logprobs
        if let Some(logprobs) = self.logprobs {
            if logprobs > 5 {
                return Err("logprobs cannot exceed 5".to_string());
            }
        }

        // Validate presence_penalty
        if let Some(penalty) = self.presence_penalty {
            if !(-2.0..=2.0).contains(&penalty) {
                return Err("presence_penalty must be between -2.0 and 2.0".to_string());
            }
        }

        // Validate frequency_penalty
        if let Some(penalty) = self.frequency_penalty {
            if !(-2.0..=2.0).contains(&penalty) {
                return Err("frequency_penalty must be between -2.0 and 2.0".to_string());
            }
        }

        // Validate best_of
        if let Some(best_of) = self.best_of {
            let n_value = self.n.unwrap_or(1);
            if best_of < n_value {
                return Err("best_of must be greater than or equal to n".to_string());
            }
            if best_of > 20 {
                return Err("best_of cannot exceed 20".to_string());
            }
        }

        Ok(())
    }
}

impl CreateChatCompletionRequest {
    /// Validate the chat completion request parameters
    pub fn validate(&self) -> Result<(), String> {
        // Validate model is not empty
        if self.model.trim().is_empty() {
            return Err("Model cannot be empty".to_string());
        }

        // Validate messages array is not empty
        if self.messages.is_empty() {
            return Err("Messages array cannot be empty".to_string());
        }

        // Validate temperature is within bounds
        if let Some(temp) = self.temperature {
            if !(0.0..=2.0).contains(&temp) {
                return Err("Temperature must be between 0 and 2".to_string());
            }
        }

        // Validate top_p is within bounds
        if let Some(top_p) = self.top_p {
            if !(0.0..=1.0).contains(&top_p) {
                return Err("Top_p must be between 0 and 1".to_string());
            }
        }

        // Validate n is positive
        if let Some(n) = self.n {
            if n == 0 {
                return Err("Number of completions (n) must be greater than 0".to_string());
            }
        }

        // Validate max_tokens is positive
        if let Some(max_tokens) = self.max_tokens {
            if max_tokens == 0 {
                return Err("Max tokens must be greater than 0".to_string());
            }
        }

        // Validate presence_penalty is within bounds
        if let Some(penalty) = self.presence_penalty {
            if !(-2.0..=2.0).contains(&penalty) {
                return Err("Presence penalty must be between -2.0 and 2.0".to_string());
            }
        }

        // Validate frequency_penalty is within bounds
        if let Some(penalty) = self.frequency_penalty {
            if !(-2.0..=2.0).contains(&penalty) {
                return Err("Frequency penalty must be between -2.0 and 2.0".to_string());
            }
        }

        // Validate messages
        for (i, message) in self.messages.iter().enumerate() {
            if let Err(err) = message.validate() {
                return Err(format!("Invalid message at index {i}: {err}"));
            }
        }

        Ok(())
    }
}

impl ChatCompletionMessage {
    /// Validate the chat completion message
    pub fn validate(&self) -> Result<(), String> {
        // For most roles, content should be present
        match self.role {
            ChatCompletionRole::System | ChatCompletionRole::User => {
                if self.content.is_none() || self.content.as_ref().unwrap().trim().is_empty() {
                    return Err("Content cannot be empty for system and user messages".to_string());
                }
            }
            ChatCompletionRole::Assistant => {
                // Assistant messages can have either content or tool_calls, but not neither
                if self.content.is_none() && self.tool_calls.is_none() {
                    return Err(
                        "Assistant messages must have either content or tool_calls".to_string()
                    );
                }
            }
            ChatCompletionRole::Tool => {
                // Tool messages must have content and tool_call_id
                if self.content.is_none() || self.content.as_ref().unwrap().trim().is_empty() {
                    return Err("Tool messages must have content".to_string());
                }
                if self.tool_call_id.is_none() {
                    return Err("Tool messages must have tool_call_id".to_string());
                }
            }
        }

        Ok(())
    }
}

impl CreateEmbeddingRequest {
    /// Validate the embedding request parameters
    pub fn validate(&self) -> Result<(), String> {
        // Validate model is not empty
        if self.model.trim().is_empty() {
            return Err("Model cannot be empty".to_string());
        }

        // Validate input is not empty
        match &self.input {
            EmbeddingInput::String(s) if s.trim().is_empty() => {
                return Err("Input cannot be empty".to_string());
            }
            EmbeddingInput::StringArray(arr) if arr.is_empty() => {
                return Err("Input array cannot be empty".to_string());
            }
            EmbeddingInput::StringArray(arr) if arr.iter().any(|s| s.trim().is_empty()) => {
                return Err("Input array cannot contain empty strings".to_string());
            }
            EmbeddingInput::IntegerArray(arr) if arr.is_empty() => {
                return Err("Input array cannot be empty".to_string());
            }
            EmbeddingInput::IntegerArrayArray(arr) if arr.is_empty() => {
                return Err("Input array cannot be empty".to_string());
            }
            EmbeddingInput::IntegerArrayArray(arr) if arr.iter().any(|inner| inner.is_empty()) => {
                return Err("Input array cannot contain empty arrays".to_string());
            }
            _ => {}
        }

        // Validate encoding_format
        if let Some(ref format) = self.encoding_format {
            match format.as_str() {
                "float" | "base64" => {}
                _ => return Err("encoding_format must be 'float' or 'base64'".to_string()),
            }
        }

        // Validate dimensions
        if let Some(dimensions) = self.dimensions {
            if dimensions == 0 {
                return Err("dimensions must be greater than 0".to_string());
            }
            // OpenAI models typically support up to 1536 or 3072 dimensions
            if dimensions > 3072 {
                return Err("dimensions cannot exceed 3072".to_string());
            }
        }

        Ok(())
    }

    /// Get the input as a vector of strings for processing
    pub fn get_input_strings(&self) -> Vec<String> {
        match &self.input {
            EmbeddingInput::String(s) => vec![s.clone()],
            EmbeddingInput::StringArray(arr) => arr.clone(),
            EmbeddingInput::IntegerArray(arr) => {
                vec![format!("{:?}", arr)]
            }
            EmbeddingInput::IntegerArrayArray(arr) => {
                arr.iter().map(|inner| format!("{inner:?}")).collect()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_request_validation() {
        // Valid request
        let valid_request = CreateCompletionRequest {
            model: "text-davinci-003".to_string(),
            prompt: PromptInput::String("Hello world".to_string()),
            max_tokens: Some(100),
            temperature: Some(0.7),
            top_p: Some(1.0),
            n: Some(1),
            stream: Some(false),
            logprobs: None,
            echo: None,
            stop: None,
            presence_penalty: Some(0.0),
            frequency_penalty: Some(0.0),
            best_of: None,
            logit_bias: None,
            user: None,
            suffix: None,
        };
        assert!(valid_request.validate().is_ok());

        // Invalid temperature
        let mut invalid_request = valid_request.clone();
        invalid_request.temperature = Some(3.0);
        assert!(invalid_request.validate().is_err());

        // Empty model
        let mut invalid_request = valid_request.clone();
        invalid_request.model = "".to_string();
        assert!(invalid_request.validate().is_err());

        // Empty prompt
        let mut invalid_request = valid_request.clone();
        invalid_request.prompt = PromptInput::String("".to_string());
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_embedding_request_validation() {
        // Valid request
        let valid_request = CreateEmbeddingRequest {
            model: "text-embedding-ada-002".to_string(),
            input: EmbeddingInput::String("Hello world".to_string()),
            encoding_format: Some("float".to_string()),
            dimensions: Some(1536),
            user: None,
        };
        assert!(valid_request.validate().is_ok());

        // Invalid encoding format
        let mut invalid_request = valid_request.clone();
        invalid_request.encoding_format = Some("invalid".to_string());
        assert!(invalid_request.validate().is_err());

        // Empty model
        let mut invalid_request = valid_request.clone();
        invalid_request.model = "".to_string();
        assert!(invalid_request.validate().is_err());

        // Empty input
        let mut invalid_request = valid_request.clone();
        invalid_request.input = EmbeddingInput::String("".to_string());
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_prompt_input_deserialization() {
        // String prompt
        let json = r#"{"model": "test", "prompt": "Hello"}"#;
        let req: CreateCompletionRequest = serde_json::from_str(json).unwrap();
        matches!(req.prompt, PromptInput::String(_));

        // Array prompt
        let json = r#"{"model": "test", "prompt": ["Hello", "World"]}"#;
        let req: CreateCompletionRequest = serde_json::from_str(json).unwrap();
        matches!(req.prompt, PromptInput::Array(_));
    }

    #[test]
    fn test_embedding_input_deserialization() {
        // String input
        let json = r#"{"model": "test", "input": "Hello"}"#;
        let req: CreateEmbeddingRequest = serde_json::from_str(json).unwrap();
        matches!(req.input, EmbeddingInput::String(_));

        // String array input
        let json = r#"{"model": "test", "input": ["Hello", "World"]}"#;
        let req: CreateEmbeddingRequest = serde_json::from_str(json).unwrap();
        matches!(req.input, EmbeddingInput::StringArray(_));

        // Integer array input
        let json = r#"{"model": "test", "input": [1, 2, 3]}"#;
        let req: CreateEmbeddingRequest = serde_json::from_str(json).unwrap();
        matches!(req.input, EmbeddingInput::IntegerArray(_));
    }

    #[test]
    fn test_get_input_strings() {
        let req = CreateEmbeddingRequest {
            model: "test".to_string(),
            input: EmbeddingInput::StringArray(vec!["Hello".to_string(), "World".to_string()]),
            encoding_format: None,
            dimensions: None,
            user: None,
        };

        let strings = req.get_input_strings();
        assert_eq!(strings, vec!["Hello", "World"]);
    }

    #[test]
    fn test_complete_json_deserialization() {
        // Test full completion request with all fields
        let json = r#"{
            "model": "text-davinci-003",
            "prompt": "Complete this sentence",
            "max_tokens": 100,
            "temperature": 0.7,
            "top_p": 1.0,
            "n": 1,
            "stream": false,
            "logprobs": 2,
            "echo": false,
            "stop": ["\n", "END"],
            "presence_penalty": 0.1,
            "frequency_penalty": 0.2,
            "best_of": 3,
            "logit_bias": {"50256": -100},
            "user": "user123",
            "suffix": " [END]"
        }"#;

        let req: Result<CreateCompletionRequest, _> = serde_json::from_str(json);
        assert!(req.is_ok());
        let req = req.unwrap();
        assert_eq!(req.model, "text-davinci-003");
        assert!(matches!(req.prompt, PromptInput::String(_)));
        assert_eq!(req.max_tokens, Some(100));
        assert_eq!(req.temperature, Some(0.7));
    }

    #[test]
    fn test_minimal_completion_request() {
        // Test with only required fields
        let json = r#"{
            "model": "text-davinci-003",
            "prompt": "Hello world"
        }"#;

        let req: Result<CreateCompletionRequest, _> = serde_json::from_str(json);
        assert!(req.is_ok());
        let req = req.unwrap();
        assert_eq!(req.model, "text-davinci-003");
        assert!(matches!(req.prompt, PromptInput::String(_)));
        assert_eq!(req.max_tokens, None);
        assert_eq!(req.temperature, None);
    }

    #[test]
    fn test_complete_embedding_request() {
        // Test full embedding request with all fields
        let json = r#"{
            "model": "text-embedding-ada-002",
            "input": ["Hello", "World"],
            "encoding_format": "float",
            "dimensions": 1536,
            "user": "user123"
        }"#;

        let req: Result<CreateEmbeddingRequest, _> = serde_json::from_str(json);
        assert!(req.is_ok());
        let req = req.unwrap();
        assert_eq!(req.model, "text-embedding-ada-002");
        assert!(matches!(req.input, EmbeddingInput::StringArray(_)));
        assert_eq!(req.encoding_format, Some("float".to_string()));
        assert_eq!(req.dimensions, Some(1536));
        assert_eq!(req.user, Some("user123".to_string()));
    }

    #[test]
    fn test_minimal_embedding_request() {
        // Test with only required fields
        let json = r#"{
            "model": "text-embedding-ada-002",
            "input": "Hello world"
        }"#;

        let req: Result<CreateEmbeddingRequest, _> = serde_json::from_str(json);
        assert!(req.is_ok());
        let req = req.unwrap();
        assert_eq!(req.model, "text-embedding-ada-002");
        assert!(matches!(req.input, EmbeddingInput::String(_)));
        assert_eq!(req.encoding_format, None);
        assert_eq!(req.dimensions, None);
    }

    #[test]
    fn test_invalid_json_requests() {
        // Missing required model field
        let json = r#"{"prompt": "Hello"}"#;
        let req: Result<CreateCompletionRequest, _> = serde_json::from_str(json);
        assert!(req.is_err());

        // Missing required input field
        let json = r#"{"model": "test"}"#;
        let req: Result<CreateEmbeddingRequest, _> = serde_json::from_str(json);
        assert!(req.is_err());

        // Invalid field type
        let json = r#"{"model": 123, "prompt": "Hello"}"#;
        let req: Result<CreateCompletionRequest, _> = serde_json::from_str(json);
        assert!(req.is_err());
    }

    #[test]
    fn test_request_validation_edge_cases() {
        // Test boundary values for completion request
        let mut request = CreateCompletionRequest {
            model: "test".to_string(),
            prompt: PromptInput::String("test".to_string()),
            max_tokens: Some(1),
            temperature: Some(0.0),
            top_p: Some(0.0),
            n: Some(1),
            stream: None,
            logprobs: Some(0),
            echo: None,
            stop: None,
            presence_penalty: Some(-2.0),
            frequency_penalty: Some(2.0),
            best_of: Some(1),
            logit_bias: None,
            user: None,
            suffix: None,
        };
        assert!(request.validate().is_ok());

        // Test maximum valid values
        request.max_tokens = Some(4096);
        request.temperature = Some(2.0);
        request.top_p = Some(1.0);
        request.n = Some(20);
        request.logprobs = Some(5);
        request.best_of = Some(20);
        assert!(request.validate().is_ok());

        // Test just over the boundary
        request.max_tokens = Some(4097);
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_embedding_request_validation_edge_cases() {
        // Test boundary values for embedding request
        let mut request = CreateEmbeddingRequest {
            model: "test".to_string(),
            input: EmbeddingInput::String("test".to_string()),
            encoding_format: Some("float".to_string()),
            dimensions: Some(1),
            user: None,
        };
        assert!(request.validate().is_ok());

        // Test maximum valid values
        request.dimensions = Some(3072);
        assert!(request.validate().is_ok());

        // Test just over the boundary
        request.dimensions = Some(3073);
        assert!(request.validate().is_err());

        // Test invalid encoding format
        request.dimensions = Some(1536);
        request.encoding_format = Some("invalid".to_string());
        assert!(request.validate().is_err());
    }
}
