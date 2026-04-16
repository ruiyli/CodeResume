use super::*;
use reqwest::Client;
use serde_json::json;

pub struct OpenAiProvider {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl OpenAiProvider {
    pub fn new(config: &cr_config::AiConfig) -> anyhow::Result<Self> {
        let api_key = config
            .api_key
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Missing API key for OpenAI. Set OPENAI_API_KEY or configure via `coderesume config set ai.api_key <KEY>`"))?;

        Ok(Self {
            client: Client::new(),
            api_key,
            model: config.model.clone().unwrap_or_else(|| "gpt-4o".to_string()),
            base_url: config
                .base_url
                .clone()
                .unwrap_or_else(|| "https://api.openai.com".to_string()),
        })
    }
}

#[async_trait::async_trait]
impl AiProvider for OpenAiProvider {
    fn model_id(&self) -> &str {
        &self.model
    }

    async fn chat(&self, request: ChatRequest) -> anyhow::Result<ChatResponse> {
        let messages: Vec<serde_json::Value> = request
            .messages
            .iter()
            .map(|msg| {
                let role = match msg.role {
                    Role::System => "system",
                    Role::User => "user",
                    Role::Assistant => "assistant",
                };
                json!({
                    "role": role,
                    "content": msg.content
                })
            })
            .collect();

        let mut body = json!({
            "model": self.model,
            "messages": messages,
            "max_tokens": request.max_tokens,
            "temperature": request.temperature,
        });

        if request.json_mode {
            body["response_format"] = json!({"type": "json_object"});
        }

        let resp = self
            .client
            .post(format!("{}/v1/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        let resp_body: serde_json::Value = resp.json().await?;

        if !status.is_success() {
            let error_msg = resp_body["error"]["message"]
                .as_str()
                .unwrap_or("Unknown error");
            anyhow::bail!("OpenAI API error ({}): {}", status, error_msg);
        }

        let content = resp_body["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let usage = TokenUsage {
            input_tokens: resp_body["usage"]["prompt_tokens"].as_u64().unwrap_or(0) as u32,
            output_tokens: resp_body["usage"]["completion_tokens"]
                .as_u64()
                .unwrap_or(0) as u32,
        };

        Ok(ChatResponse {
            content,
            usage,
            model: self.model.clone(),
        })
    }

    fn estimate_cost(&self, usage: &TokenUsage) -> f64 {
        // GPT-4o pricing
        let input_cost = usage.input_tokens as f64 * 2.5 / 1_000_000.0;
        let output_cost = usage.output_tokens as f64 * 10.0 / 1_000_000.0;
        input_cost + output_cost
    }
}
