use clap::Args;
use cr_config::AppConfig;
use std::path::PathBuf;

#[derive(Args)]
pub struct NewArgs {
    /// Output file path (without extension)
    #[arg(short, long, default_value = "resume")]
    pub output: String,
    /// Template to use
    #[arg(short, long)]
    pub template: Option<String>,
    /// Language (en/zh)
    #[arg(short, long)]
    pub lang: Option<String>,
    /// Import from existing PDF
    #[arg(long)]
    pub from_pdf: Option<PathBuf>,
}

pub async fn run(args: NewArgs, _config: &AppConfig) -> anyhow::Result<()> {
    println!("Welcome to CodeResume!\n");
    println!("Let's create your resume step by step.\n");

    // TODO: Implement full interactive wizard
    // For now, provide a minimal implementation

    // Language selection
    let lang = if let Some(ref l) = args.lang {
        l.clone()
    } else {
        inquire::Select::new("Resume language:", vec!["English", "中文"])
            .prompt()
            .map(|s| if s == "中文" { "zh" } else { "en" })?
            .to_string()
    };

    let language = match lang.as_str() {
        "zh" => cr_core::resume::Language::Zh,
        _ => cr_core::resume::Language::En,
    };

    // Personal info
    let name = inquire::Text::new("Your name:").prompt()?;
    let email = inquire::Text::new("Email:").prompt()?;
    let phone = inquire::Text::new("Phone (optional, press Enter to skip):")
        .prompt_skippable()?
        .filter(|s| !s.is_empty());
    let title = inquire::Text::new("Job title (e.g., Senior Software Engineer):")
        .prompt_skippable()?
        .filter(|s| !s.is_empty());
    let github = inquire::Text::new("GitHub URL (optional):")
        .prompt_skippable()?
        .filter(|s| !s.is_empty());
    let linkedin = inquire::Text::new("LinkedIn URL (optional):")
        .prompt_skippable()?
        .filter(|s| !s.is_empty());

    let personal = cr_core::resume::PersonalInfo {
        name,
        name_alt: None,
        email,
        phone,
        location: None,
        title,
        photo: None,
        website: None,
        linkedin,
        github,
        extra_links: Default::default(),
    };

    let resume = cr_core::resume::Resume {
        version: "1.0".to_string(),
        language,
        personal,
        summary: None,
        experience: vec![],
        education: vec![],
        skills: Default::default(),
        projects: vec![],
        certifications: vec![],
        publications: vec![],
        open_source: vec![],
        custom_sections: vec![],
        meta: Default::default(),
    };

    // Save
    let output_path = PathBuf::from(&args.output).with_extension("yaml");
    cr_io::data_file::save(&resume, &output_path)?;
    println!("\nResume data saved to: {}", output_path.display());
    println!("Edit the YAML file to add experience, education, skills, etc.");
    println!("Then run: coderesume generate {}", output_path.display());

    Ok(())
}
