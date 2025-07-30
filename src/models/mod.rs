pub mod requests;
pub mod responses;

// Re-export commonly used types
pub use requests::{
    ChatCompletionFunction, ChatCompletionFunctionCall, ChatCompletionMessage,
    ChatCompletionMessageToolCall, ChatCompletionRole, ChatCompletionTool,
    ChatCompletionToolChoice, CreateChatCompletionRequest, CreateCompletionRequest,
    CreateEmbeddingRequest, EmbeddingInput, PromptInput, StopSequences,
};
pub use responses::{
    ApiError, ChatCompletionChoice, ChatCompletionFunctionCall as ResponseFunctionCall,
    ChatCompletionLogprobs, ChatCompletionMessageToolCall as ResponseMessageToolCall,
    ChatCompletionResponseMessage, ChatCompletionTokenLogprob, ChatCompletionTopLogprob,
    CompletionChoice, CompletionLogprobs, CompletionUsage, CreateChatCompletionResponse,
    CreateCompletionResponse, CreateEmbeddingResponse, EmbeddingData, EmbeddingUsage,
    ErrorResponse,
};
