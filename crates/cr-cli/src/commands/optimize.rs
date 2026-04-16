use clap::Args;
use cr_config::AppConfig;
use std::path::PathBuf;

#[derive(Args)]
pub struct OptimizeArgs {
    /// Resume data file to optimize
    pub input: PathBuf,
    /// Write optimized data to this file (default: overwrite input)
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    /// Also regenerate PDF after optimization
    #[arg(long)]
    pub render: bool,
    /// Template for rendering
    #[arg(short, long)]
    pub template: Option<String>,
}

pub async fn run(args: OptimizeArgs, config: &AppConfig) -> anyhow::Result<()> {
    // Validate API configuration before attempting long operations
    cr_ai::provider::validate_api_config(&config.ai)?;

    let resume = cr_io::data_file::load(&args.input)?;
    println!("Loaded resume for: {}", resume.personal.name);
    println!("Optimizing with AI ({})...\n", config.ai.provider);

    let template_base = PathBuf::from("templates");
    let ctx =
        cr_service::context::ServiceContext::from_config(config.clone(), template_base).await?;
    let optimized = cr_service::optimize::run(&ctx, &resume).await?;

    // Save
    let output_path = args.output.as_ref().unwrap_or(&args.input);
    cr_io::data_file::save(&optimized, output_path)?;
    println!("Saved optimized resume to: {}", output_path.display());

    Ok(())
}
