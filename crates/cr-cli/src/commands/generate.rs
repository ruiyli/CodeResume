use clap::Args;
use cr_config::AppConfig;
use cr_core::template::{OutputFormat, RenderRequest, TemplateId};
use cr_render::engine::RenderEngine;
use std::path::PathBuf;

#[derive(Args)]
pub struct GenerateArgs {
    /// Path to resume data file (YAML or JSON)
    pub input: PathBuf,
    /// Output file path (without extension)
    #[arg(short, long, default_value = "resume")]
    pub output: String,
    /// Template to use
    #[arg(short, long)]
    pub template: Option<String>,
    /// Output formats: pdf, markdown
    #[arg(short, long)]
    pub format: Vec<String>,
}

pub async fn run(args: GenerateArgs, config: &AppConfig) -> anyhow::Result<()> {
    // Load resume data
    let resume = cr_io::data_file::load(&args.input)?;
    println!("Loaded resume for: {}", resume.personal.name);

    // Resolve template
    let template_str = args.template.as_deref().unwrap_or(&config.output.template);
    let template: TemplateId = template_str
        .parse()
        .map_err(|e: String| anyhow::anyhow!(e))?;

    // Resolve formats
    let formats: Vec<OutputFormat> = if args.format.is_empty() {
        config
            .output
            .formats
            .iter()
            .filter_map(|f| match f.as_str() {
                "pdf" => Some(OutputFormat::Pdf),
                "markdown" => Some(OutputFormat::Markdown),
                _ => None,
            })
            .collect()
    } else {
        args.format
            .iter()
            .filter_map(|f| match f.as_str() {
                "pdf" => Some(OutputFormat::Pdf),
                "markdown" | "md" => Some(OutputFormat::Markdown),
                _ => None,
            })
            .collect()
    };

    let output_path = PathBuf::from(&args.output);
    let template_base = PathBuf::from("templates");

    // Generate directly without AI — just render
    let engine = RenderEngine::new(config.output.typst_bin.clone(), template_base);
    let request = RenderRequest {
        template,
        language: resume.language,
        output_path,
        formats,
    };
    let outputs = engine.render(&resume, &request)?;

    println!("\nGenerated:");
    for path in outputs {
        println!("  {}", path.display());
    }

    Ok(())
}
