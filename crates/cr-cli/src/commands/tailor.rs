use clap::Args;
use cr_config::AppConfig;
use cr_core::jd::JobDescription;
use std::path::PathBuf;

#[derive(Args)]
pub struct TailorArgs {
    /// Resume data file
    pub input: PathBuf,
    /// Job description file path
    #[arg(short, long)]
    pub jd: String,
    /// Output file path
    #[arg(short, long, default_value = "resume-tailored")]
    pub output: String,
    /// Template to use
    #[arg(short, long)]
    pub template: Option<String>,
    /// Auto-apply suggestions without confirmation
    #[arg(long)]
    pub auto_apply: bool,
}

pub async fn run(args: TailorArgs, config: &AppConfig) -> anyhow::Result<()> {
    // Validate API configuration before attempting long operations
    cr_ai::provider::validate_api_config(&config.ai)?;

    let resume = cr_io::data_file::load(&args.input)?;
    println!("Loaded resume for: {}", resume.personal.name);

    // Load JD
    let jd_text = if PathBuf::from(&args.jd).exists() {
        std::fs::read_to_string(&args.jd)?
    } else {
        args.jd.clone()
    };

    let jd = JobDescription {
        raw_text: jd_text,
        company: None,
        title: None,
        parsed: None,
    };

    println!("Tailoring resume to job description...\n");

    let template_base = PathBuf::from("templates");
    let ctx =
        cr_service::context::ServiceContext::from_config(config.clone(), template_base).await?;
    let (match_result, tailored) = cr_service::tailor::run(&ctx, &resume, &jd).await?;

    // Display results
    println!("Match Score: {}/100\n", match_result.match_score);

    if !match_result.matched_keywords.is_empty() {
        println!("Matched Keywords:");
        for kw in &match_result.matched_keywords {
            println!("  + {}", kw);
        }
        println!();
    }

    if !match_result.missing_keywords.is_empty() {
        println!("Missing Keywords:");
        for kw in &match_result.missing_keywords {
            println!("  - {}", kw);
        }
        println!();
    }

    // Save tailored resume
    let output_path = PathBuf::from(&args.output).with_extension("yaml");
    cr_io::data_file::save(&tailored, &output_path)?;
    println!("Saved tailored resume to: {}", output_path.display());

    Ok(())
}
