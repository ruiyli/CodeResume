use super::*;
use reqwest::Client;
use std::time::Duration;
use serde_json::json;

pub struct ClaudeProvider {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl ClaudeProvider {
    pub fn new(config: &cr_config::AiConfig) -> anyhow::Result<Self> {
        let api_key = config
            .api_key
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Missing API key for Claude. Set ANTHROPIC_API_KEY or configure via `coderesume config set ai.api_key <KEY>`"))?;
        Ok(Self {
            client: Client::builder().timeout(Duration::from_secs(30)).build()?,
            api_key,
            model: config
                .model
                .clone()
                .unwrap_or_else(|| "claude-sonnet-4-20250514".to_string()),
            base_url: config
                .base_url
                .clone()
                .unwrap_or_else(|| "https://api.anthropic.com".to_string()),
        })
    }
}

#[async_trait::async_trait]
impl AiProvider for ClaudeProvider {
    fn model_id(&self) -> &str {
        &self.model
    }

    async fn chat(&self, request: ChatRequest) -> anyhow::Result<ChatResponse> {
        // Separate system message from conversation messages
        let mut system_content = String::new();
        let mut messages = Vec::new();

        for msg in &request.messages {
            match msg.role {
                Role::System => {
                    system_content = msg.content.clone();
                }
                Role::User => {
                    messages.push(json!({
                        "role": "user",
                        "content": msg.content
                    }));
                }
                Role::Assistant => {
                    messages.push(json!({
                        "role": "assistant",
                        "content": msg.content
                    }));
                }
            }
        }

        let mut body = json!({
            "model": self.model,
            "max_tokens": request.max_tokens,
            "temperature": request.temperature,
            "messages": messages,
        });

        if !system_content.is_empty() {
            body["system"] = json!(system_content);
        }

        let resp = self
            .client
            .post(format!("{}/v1/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        let resp_body: serde_json::Value = resp.json().await?;

        if !status.is_success() {
            let error_msg = resp_body["error"]["message"]
                .as_str()
                .unwrap_or("Unknown error");
            anyhow::bail!("Claude API error ({}): {}", status, error_msg);
        }

        let content = resp_body["content"][0]["text"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let usage = TokenUsage {
            input_tokens: resp_body["usage"]["input_tokens"].as_u64().unwrap_or(0) as u32,
            output_tokens: resp_body["usage"]["output_tokens"].as_u64().unwrap_or(0) as u32,
        };

        Ok(ChatResponse {
            content,
            usage,
            model: self.model.clone(),
        })
    }

    fn estimate_cost(&self, usage: &TokenUsage) -> f64 {
        // Claude Sonnet 4 pricing
        let input_cost = usage.input_tokens as f64 * 3.0 / 1_000_000.0;
        let output_cost = usage.output_tokens as f64 * 15.0 / 1_000_000.0;
        input_cost + output_cost
    }
}
