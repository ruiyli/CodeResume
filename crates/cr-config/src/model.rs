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

impl AppConfig {
    /// Validate configuration values are within acceptable ranges.
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // AI config validation
        if self.ai.temperature < 0.0 || self.ai.temperature > 2.0 {
            errors.push(format!(
                "ai.temperature must be between 0.0 and 2.0, got {}",
                self.ai.temperature
            ));
        }
        if self.ai.max_tokens == 0 || self.ai.max_tokens > 128_000 {
            errors.push(format!(
                "ai.max_tokens must be between 1 and 128000, got {}",
                self.ai.max_tokens
            ));
        }
        if !matches!(self.ai.provider.as_str(), "claude" | "openai" | "mock") {
            errors.push(format!(
                "ai.provider must be \"claude\", \"openai\", or \"mock\", got \"{}\"",
                self.ai.provider
            ));
        }

        // Output config validation
        if !matches!(self.output.language.as_str(), "en" | "zh") {
            errors.push(format!(
                "output.language must be \"en\" or \"zh\", got \"{}\"",
                self.output.language
            ));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
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

    #[test]
    fn validate_default_config_passes() {
        let config = AppConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn validate_bad_temperature() {
        let mut config = AppConfig::default();
        config.ai.temperature = 3.0;
        let errors = config.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("temperature")));
    }

    #[test]
    fn validate_negative_temperature() {
        let mut config = AppConfig::default();
        config.ai.temperature = -0.5;
        let errors = config.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("temperature")));
    }

    #[test]
    fn validate_bad_max_tokens() {
        let mut config = AppConfig::default();
        config.ai.max_tokens = 0;
        let errors = config.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("max_tokens")));
    }

    #[test]
    fn validate_bad_provider() {
        let mut config = AppConfig::default();
        config.ai.provider = "gemini".to_string();
        let errors = config.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("provider")));
    }

    #[test]
    fn validate_bad_language() {
        let mut config = AppConfig::default();
        config.output.language = "fr".to_string();
        let errors = config.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("language")));
    }
}
