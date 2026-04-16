use super::*;
use std::collections::VecDeque;
use std::sync::Mutex;

/// Mock provider that returns pre-configured responses in order.
pub struct MockProvider {
    responses: Mutex<VecDeque<String>>,
    default_response: String,
}

impl Default for MockProvider {
    fn default() -> Self {
        Self {
            responses: Mutex::new(VecDeque::new()),
            default_response: r#"{"overall_score":75,"dimensions":[],"suggestions":[],"strengths":["Good structure"]}"#.to_string(),
        }
    }
}

impl MockProvider {
    pub fn with_response(response: &str) -> Self {
        let mut q = VecDeque::new();
        q.push_back(response.to_string());
        Self {
            responses: Mutex::new(q),
            default_response: response.to_string(),
        }
    }

    pub fn with_responses(responses: Vec<String>) -> Self {
        Self {
            responses: Mutex::new(responses.into()),
            ..Default::default()
        }
    }
}

#[async_trait::async_trait]
impl AiProvider for MockProvider {
    fn model_id(&self) -> &str {
        "mock-model"
    }

    async fn chat(&self, _request: ChatRequest) -> anyhow::Result<ChatResponse> {
        let content = self
            .responses
            .lock()
            .unwrap()
            .pop_front()
            .unwrap_or_else(|| self.default_response.clone());

        Ok(ChatResponse {
            content,
            usage: TokenUsage {
                input_tokens: 100,
                output_tokens: 200,
            },
            model: "mock-model".to_string(),
        })
    }

    fn estimate_cost(&self, _usage: &TokenUsage) -> f64 {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mock_provider_returns_default() {
        let mock = MockProvider::default();
        let resp = mock
            .chat(ChatRequest {
                messages: vec![],
                max_tokens: 1000,
                temperature: 0.0,
                json_mode: true,
            })
            .await
            .unwrap();
        assert!(resp.content.contains("overall_score"));
    }

    #[tokio::test]
    async fn mock_provider_returns_queued_responses() {
        let mock =
            MockProvider::with_responses(vec!["response1".to_string(), "response2".to_string()]);
        let r1 = mock
            .chat(ChatRequest {
                messages: vec![],
                max_tokens: 100,
                temperature: 0.0,
                json_mode: false,
            })
            .await
            .unwrap();
        let r2 = mock
            .chat(ChatRequest {
                messages: vec![],
                max_tokens: 100,
                temperature: 0.0,
                json_mode: false,
            })
            .await
            .unwrap();
        assert_eq!(r1.content, "response1");
        assert_eq!(r2.content, "response2");
    }
}
