pub mod claude;
pub mod mock;
pub mod openai;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Clone, Copy)]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone)]
pub struct ChatRequest {
    pub messages: Vec<ChatMessage>,
    pub max_tokens: u32,
    pub temperature: f32,
    /// Hint: expect JSON output
    pub json_mode: bool,
}

#[derive(Debug, Clone)]
pub struct ChatResponse {
    pub content: String,
    pub usage: TokenUsage,
    pub model: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TokenUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[async_trait]
pub trait AiProvider: Send + Sync {
    /// Unique model identifier
    fn model_id(&self) -> &str;

    /// Send a chat completion request
    async fn chat(&self, request: ChatRequest) -> anyhow::Result<ChatResponse>;

    /// Estimated cost in USD for the given usage
    fn estimate_cost(&self, usage: &TokenUsage) -> f64;
}

/// Factory: config → boxed provider
pub fn build_provider(ai_config: &cr_config::AiConfig) -> anyhow::Result<Box<dyn AiProvider>> {
    match ai_config.provider.as_str() {
        "claude" => Ok(Box::new(claude::ClaudeProvider::new(ai_config)?)),
        "openai" => Ok(Box::new(openai::OpenAiProvider::new(ai_config)?)),
        "mock" => Ok(Box::new(mock::MockProvider::default())),
        other => anyhow::bail!("Unknown AI provider: {other}"),
    }
}
