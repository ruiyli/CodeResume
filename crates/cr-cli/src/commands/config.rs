use clap::{Args, Subcommand};
use cr_config::{mask_api_key, AppConfig};

#[derive(Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub action: ConfigAction,
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Show current configuration
    Show,
    /// Set a configuration value
    Set {
        /// Config key (e.g., ai.provider, ai.api_key, output.template)
        key: String,
        /// Config value
        value: String,
    },
    /// Initialize config interactively
    Init,
}

pub fn run(args: ConfigArgs, config: &AppConfig) -> anyhow::Result<()> {
    match args.action {
        ConfigAction::Show => show_config(config),
        ConfigAction::Set { key, value } => set_config(&key, &value),
        ConfigAction::Init => init_config(),
    }
}

fn show_config(config: &AppConfig) -> anyhow::Result<()> {
    println!("CodeResume Configuration");
    println!("========================\n");
    println!("Config file: {}", cr_config::config_path().display());
    println!();

    println!("[ai]");
    println!("  provider    = {}", config.ai.provider);
    if let Some(ref model) = config.ai.model {
        println!("  model       = {}", model);
    }
    if let Some(ref key) = config.ai.api_key {
        println!("  api_key     = {}", mask_api_key(key));
    } else {
        println!("  api_key     = (not set)");
    }
    println!("  max_tokens  = {}", config.ai.max_tokens);
    println!("  temperature = {}", config.ai.temperature);
    println!();

    println!("[output]");
    println!("  template    = {}", config.output.template);
    println!("  language    = {}", config.output.language);
    println!("  formats     = {:?}", config.output.formats);
    println!();

    println!("[ui]");
    println!("  show_cost   = {}", config.ui.show_cost);
    println!("  color       = {}", config.ui.color);
    println!();

    // Environment variable status
    println!("Environment:");
    println!(
        "  ANTHROPIC_API_KEY = {}",
        if std::env::var("ANTHROPIC_API_KEY").is_ok() {
            "set"
        } else {
            "not set"
        }
    );
    println!(
        "  OPENAI_API_KEY    = {}",
        if std::env::var("OPENAI_API_KEY").is_ok() {
            "set"
        } else {
            "not set"
        }
    );

    Ok(())
}

fn set_config(key: &str, value: &str) -> anyhow::Result<()> {
    let mut config = cr_config::load()?;

    match key {
        "ai.provider" => config.ai.provider = value.to_string(),
        "ai.model" => config.ai.model = Some(value.to_string()),
        "ai.api_key" => config.ai.api_key = Some(value.to_string()),
        "ai.base_url" => config.ai.base_url = Some(value.to_string()),
        "ai.max_tokens" => {
            config.ai.max_tokens = value
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid value for max_tokens: {value}"))?;
        }
        "ai.temperature" => {
            config.ai.temperature = value
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid value for temperature: {value}"))?;
        }
        "output.template" => config.output.template = value.to_string(),
        "output.language" => config.output.language = value.to_string(),
        _ => anyhow::bail!("Unknown config key: {key}"),
    }

    cr_config::save(&config)?;
    println!(
        "Set {} = {}",
        key,
        if key.contains("api_key") {
            "****"
        } else {
            value
        }
    );
    Ok(())
}

fn init_config() -> anyhow::Result<()> {
    let provider =
        inquire::Select::new("Select AI provider:", vec!["claude", "openai"]).prompt()?;

    let api_key = inquire::Password::new("Enter API key:")
        .with_display_mode(inquire::PasswordDisplayMode::Masked)
        .prompt()?;

    let template = inquire::Select::new(
        "Default template:",
        vec!["modern", "classic", "minimal", "two-column", "academic"],
    )
    .prompt()?;

    let language = inquire::Select::new("Default language:", vec!["en", "zh"]).prompt()?;

    let mut config = AppConfig::default();
    config.ai.provider = provider.to_string();
    config.ai.api_key = Some(api_key);
    config.output.template = template.to_string();
    config.output.language = language.to_string();

    cr_config::save(&config)?;
    println!(
        "\nConfiguration saved to {}",
        cr_config::config_path().display()
    );
    println!("You can now use `coderesume new` to create your first resume!");
    Ok(())
}
