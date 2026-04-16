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

/// Validate that API configuration is available before attempting to use it
pub fn validate_api_config(ai_config: &cr_config::AiConfig) -> anyhow::Result<()> {
    match ai_config.provider.as_str() {
        "mock" => {
            // Mock provider doesn't need real credentials
            Ok(())
        }
        "claude" => {
            if ai_config.api_key.is_none() || ai_config.api_key.as_ref().map(|k| k.is_empty()).unwrap_or(true) {
                anyhow::bail!(
                    "Missing Claude API key.\n\
                    Set via environment variable: export ANTHROPIC_API_KEY=sk-ant-...\n\
                    Or configure: coderesume config set ai.api_key <KEY>"
                );
            }
            Ok(())
        }
        "openai" => {
            if ai_config.api_key.is_none() || ai_config.api_key.as_ref().map(|k| k.is_empty()).unwrap_or(true) {
                anyhow::bail!(
                    "Missing OpenAI API key.\n\
                    Set via environment variable: export OPENAI_API_KEY=sk-...\n\
                    Or configure: coderesume config set ai.api_key <KEY>"
                );
            }
            Ok(())
        }
        other => anyhow::bail!("Unknown AI provider: {other}"),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_api_config_mock() {
        let config = cr_config::AiConfig {
            provider: "mock".to_string(),
            api_key: None,
            model: None,
            base_url: None,
            ..Default::default()
        };

        // Mock provider should not require API key
        assert!(validate_api_config(&config).is_ok());
    }

    #[test]
    fn test_validate_api_config_claude_missing_key() {
        let config = cr_config::AiConfig {
            provider: "claude".to_string(),
            api_key: None,
            model: None,
            base_url: None,
            ..Default::default()
        };

        // Claude provider should fail without API key
        assert!(validate_api_config(&config).is_err());
    }

    #[test]
    fn test_validate_api_config_claude_empty_key() {
        let config = cr_config::AiConfig {
            provider: "claude".to_string(),
            api_key: Some(String::new()),
            model: None,
            base_url: None,
            ..Default::default()
        };

        // Claude provider should fail with empty API key
        assert!(validate_api_config(&config).is_err());
    }

    #[test]
    fn test_validate_api_config_claude_valid_key() {
        let config = cr_config::AiConfig {
            provider: "claude".to_string(),
            api_key: Some("sk-ant-valid-key".to_string()),
            model: None,
            base_url: None,
            ..Default::default()
        };

        // Claude provider should pass with valid API key
        assert!(validate_api_config(&config).is_ok());
    }

    #[test]
    fn test_validate_api_config_openai_missing_key() {
        let config = cr_config::AiConfig {
            provider: "openai".to_string(),
            api_key: None,
            model: None,
            base_url: None,
            ..Default::default()
        };

        // OpenAI provider should fail without API key
        assert!(validate_api_config(&config).is_err());
    }

    #[test]
    fn test_validate_api_config_openai_valid_key() {
        let config = cr_config::AiConfig {
            provider: "openai".to_string(),
            api_key: Some("sk-valid-openai-key".to_string()),
            model: None,
            base_url: None,
            ..Default::default()
        };

        // OpenAI provider should pass with valid API key
        assert!(validate_api_config(&config).is_ok());
    }

    #[test]
    fn test_validate_api_config_unknown_provider() {
        let config = cr_config::AiConfig {
            provider: "unknown-provider".to_string(),
            api_key: Some("some-key".to_string()),
            model: None,
            base_url: None,
            ..Default::default()
        };

        // Unknown provider should fail
        assert!(validate_api_config(&config).is_err());
    }

    #[test]
    fn test_build_provider_mock() {
        let config = cr_config::AiConfig {
            provider: "mock".to_string(),
            api_key: None,
            model: None,
            base_url: None,
            ..Default::default()
        };

        let provider = build_provider(&config);
        assert!(provider.is_ok());
    }

    #[test]
    fn test_chat_message_construction() {
        let msg = ChatMessage {
            role: Role::User,
            content: "Hello".to_string(),
        };
        
        assert_eq!(msg.content, "Hello");
    }

    #[test]
    fn test_chat_request_construction() {
        let req = ChatRequest {
            messages: vec![],
            max_tokens: 2048,
            temperature: 0.7,
            json_mode: false,
        };
        
        assert_eq!(req.max_tokens, 2048);
        assert!(!req.json_mode);
    }

    #[test]
    fn test_token_usage_default() {
        let usage = TokenUsage::default();
        assert_eq!(usage.input_tokens, 0);
        assert_eq!(usage.output_tokens, 0);
    }
}
