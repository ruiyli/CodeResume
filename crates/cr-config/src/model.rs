use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub ai: AiConfig,
    #[serde(default)]
    pub output: OutputConfig,
    #[serde(default)]
    pub ui: UiConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            ai: AiConfig::default(),
            output: OutputConfig::default(),
            ui: UiConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    /// "claude" | "openai"
    #[serde(default = "default_provider")]
    pub provider: String,
    /// Provider-specific model name
    pub model: Option<String>,
    /// API key (read from config, env var, or keyring)
    pub api_key: Option<String>,
    /// Custom base URL for proxies / local models
    pub base_url: Option<String>,
    /// Max tokens for AI responses
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    /// Temperature (0.0 - 1.0)
    #[serde(default = "default_temperature")]
    pub temperature: f32,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            provider: default_provider(),
            model: None,
            api_key: None,
            base_url: None,
            max_tokens: default_max_tokens(),
            temperature: default_temperature(),
        }
    }
}

fn default_provider() -> String {
    "claude".to_string()
}
fn default_max_tokens() -> u32 {
    4096
}
fn default_temperature() -> f32 {
    0.3
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Default template
    #[serde(default = "default_template")]
    pub template: String,
    /// Default language
    #[serde(default = "default_language")]
    pub language: String,
    /// Default output formats
    #[serde(default = "default_formats")]
    pub formats: Vec<String>,
    /// Path to typst binary
    pub typst_bin: Option<String>,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            template: default_template(),
            language: default_language(),
            formats: default_formats(),
            typst_bin: None,
        }
    }
}

fn default_template() -> String {
    "modern".to_string()
}
fn default_language() -> String {
    "en".to_string()
}
fn default_formats() -> Vec<String> {
    vec!["pdf".to_string(), "markdown".to_string()]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Show token usage and cost estimates after AI calls
    #[serde(default = "default_true")]
    pub show_cost: bool,
    /// Enable colored output
    #[serde(default = "default_true")]
    pub color: bool,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            show_cost: true,
            color: true,
        }
    }
}

fn default_true() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_sane_values() {
        let config = AppConfig::default();
        assert_eq!(config.ai.provider, "claude");
        assert_eq!(config.ai.max_tokens, 4096);
        assert_eq!(config.output.template, "modern");
        assert_eq!(config.output.language, "en");
        assert!(config.ui.show_cost);
    }

    #[test]
    fn config_roundtrip_toml() {
        let config = AppConfig::default();
        let toml_str = toml::to_string(&config).unwrap();
        let parsed: AppConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.ai.provider, config.ai.provider);
        assert_eq!(parsed.output.template, config.output.template);
    }
}
