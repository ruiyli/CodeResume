use crate::model::AppConfig;
use anyhow::Context;
use std::path::PathBuf;

pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("coderesume")
}

pub fn config_path() -> PathBuf {
    config_dir().join("config.toml")
}

pub fn load() -> anyhow::Result<AppConfig> {
    let path = config_path();
    if !path.exists() {
        let mut config = AppConfig::default();
        resolve_env_api_key(&mut config);
        return Ok(config);
    }
    let contents = std::fs::read_to_string(&path)
        .context(format!("Failed to read config: {}", path.display()))?;
    let mut config: AppConfig = toml::from_str(&contents).context("Failed to parse config.toml")?;
    resolve_env_api_key(&mut config);

    // Validate config values
    if let Err(errors) = config.validate() {
        let msg = errors.join("\n  - ");
        anyhow::bail!("Invalid configuration in {}:\n  - {}", path.display(), msg);
    }

    Ok(config)
}

pub fn save(config: &AppConfig) -> anyhow::Result<()> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let contents = toml::to_string_pretty(config)?;
    std::fs::write(&path, contents)?;
    Ok(())
}

/// Override API key from env vars if not set in config
fn resolve_env_api_key(config: &mut AppConfig) {
    if config.ai.api_key.is_none() {
        config.ai.api_key = match config.ai.provider.as_str() {
            "claude" => std::env::var("ANTHROPIC_API_KEY").ok(),
            "openai" => std::env::var("OPENAI_API_KEY").ok(),
            _ => None,
        };
    }
}

/// Mask an API key for display: "sk-abc...xyz"
pub fn mask_api_key(key: &str) -> String {
    if key.len() <= 8 {
        return "****".to_string();
    }
    format!("{}...{}", &key[..4], &key[key.len() - 4..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_api_key_works() {
        assert_eq!(mask_api_key("sk-1234567890abcdef"), "sk-1...cdef");
        assert_eq!(mask_api_key("short"), "****");
    }

    #[test]
    fn load_returns_default_when_no_file() {
        // In test environment, config file likely doesn't exist
        let config = load();
        assert!(config.is_ok());
    }
}
