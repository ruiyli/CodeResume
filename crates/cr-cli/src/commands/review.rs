use clap::Args;
use cr_config::AppConfig;
use cr_core::scoring::Severity;
use std::path::PathBuf;

#[derive(Args)]
pub struct ReviewArgs {
    /// Resume data file to review
    pub input: PathBuf,
    /// Optional JD for context-aware scoring
    #[arg(short, long)]
    pub jd: Option<String>,
    /// Output report to file
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

pub async fn run(args: ReviewArgs, config: &AppConfig) -> anyhow::Result<()> {
    // Validate API configuration before attempting long operations
    cr_ai::provider::validate_api_config(&config.ai)?;

    let resume = cr_io::data_file::load(&args.input)?;
    println!("Reviewing resume for: {}\n", resume.personal.name);

    let template_base = PathBuf::from("templates");
    let ctx =
        cr_service::context::ServiceContext::from_config(config.clone(), template_base).await?;
    let report = cr_service::review::run(&ctx, &resume).await?;

    // Display score report
    println!("Overall Score: {}/100", report.overall_score);
    println!("{}\n", score_bar(report.overall_score));

    if !report.dimensions.is_empty() {
        println!("Dimensions:");
        for dim in &report.dimensions {
            println!(
                "  {:<25} {:>3}/100  {}",
                dim.name,
                dim.score,
                score_bar(dim.score)
            );
            println!("  {}", dim.feedback);
            println!();
        }
    }

    if !report.strengths.is_empty() {
        println!("Strengths:");
        for s in &report.strengths {
            println!("  + {}", s);
        }
        println!();
    }

    if !report.suggestions.is_empty() {
        println!("Suggestions:");
        for s in &report.suggestions {
            let icon = match s.severity {
                Severity::Critical => "!!",
                Severity::Warning => "! ",
                Severity::Info => "i ",
            };
            println!("  [{}] {}: {}", icon, s.section, s.message);
            if let Some(ref fix) = s.example_fix {
                println!("       -> {}", fix);
            }
        }
    }

    // Save report if requested
    if let Some(ref output_path) = args.output {
        let json = serde_json::to_string_pretty(&report)?;
        std::fs::write(output_path, json)?;
        println!("\nReport saved to: {}", output_path.display());
    }

    Ok(())
}

fn score_bar(score: u8) -> String {
    let filled = (score as usize) / 10;
    let empty = 10 - filled;
    format!("[{}{}]", "#".repeat(filled), "-".repeat(empty))
}
