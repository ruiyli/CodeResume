mod commands;
mod display;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "coderesume",
    version,
    about = "AI-powered resume generator for tech professionals"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Override config file path
    #[arg(long, global = true)]
    pub config: Option<std::path::PathBuf>,

    /// Override AI provider ("claude" | "openai")
    #[arg(long, global = true)]
    pub provider: Option<String>,

    /// Verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new resume interactively
    New(commands::new::NewArgs),
    /// Generate resume from a data file
    Generate(commands::generate::GenerateArgs),
    /// AI-optimize an existing resume
    Optimize(commands::optimize::OptimizeArgs),
    /// Tailor resume to a job description
    Tailor(commands::tailor::TailorArgs),
    /// Score and review a resume
    Review(commands::review::ReviewArgs),
    /// List available templates
    Templates,
    /// Configure API keys and preferences
    Config(commands::config::ConfigArgs),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Initialize tracing
    let filter = if cli.verbose { "debug" } else { "warn" };
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    // Load config
    let mut config = cr_config::load()?;
    if let Some(ref provider) = cli.provider {
        config.ai.provider = provider.clone();
    }

    match cli.command {
        Commands::Config(args) => commands::config::run(args, &config)?,
        Commands::Templates => commands::templates::run(&config)?,
        Commands::Generate(args) => commands::generate::run(args, &config).await?,
        Commands::Optimize(args) => commands::optimize::run(args, &config).await?,
        Commands::Tailor(args) => commands::tailor::run(args, &config).await?,
        Commands::Review(args) => commands::review::run(args, &config).await?,
        Commands::New(args) => commands::new::run(args, &config).await?,
    }

    Ok(())
}
