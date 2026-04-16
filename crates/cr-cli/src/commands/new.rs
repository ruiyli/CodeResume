use clap::Args;
use cr_config::AppConfig;
use cr_core::resume::{Certification, DateValue, Education, Experience, Language, SkillGroup};
use cr_service::create::{CreateInput, create};
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
        "zh" => Language::Zh,
        _ => Language::En,
    };

    // Personal info section
    println!("\n📋 Personal Information");
    let name = inquire::Text::new("Your name:").prompt()?;
    let email = inquire::Text::new("Email:").prompt()?;
    let phone = inquire::Text::new("Phone (optional, press Enter to skip):")
        .prompt_skippable()?
        .filter(|s| !s.is_empty());
    let title = inquire::Text::new("Job title (e.g., Senior Software Engineer):")
        .prompt_skippable()?
        .filter(|s| !s.is_empty());
    let location = inquire::Text::new("Location (optional):")
        .prompt_skippable()?
        .filter(|s| !s.is_empty());
    let website = inquire::Text::new("Personal website (optional):")
        .prompt_skippable()?
        .filter(|s| !s.is_empty());
    let linkedin = inquire::Text::new("LinkedIn URL (optional):")
        .prompt_skippable()?
        .filter(|s| !s.is_empty());
    let github = inquire::Text::new("GitHub URL (optional):")
        .prompt_skippable()?
        .filter(|s| !s.is_empty());

    let summary = inquire::Text::new("Professional summary (optional):")
        .prompt_skippable()?
        .filter(|s| !s.is_empty());

    // Create initial resume using service layer
    let create_input = CreateInput {
        language,
        name,
        email,
        phone,
        title,
        location,
        website,
        linkedin,
        github,
        summary,
    };

    let mut resume = create(create_input)?;

    // Experience section
    println!("\n💼 Experience");
    loop {
        let add_experience = inquire::Confirm::new("Add an experience entry?")
            .with_default(resume.experience.is_empty())
            .prompt()?;

        if !add_experience {
            break;
        }

        let company = inquire::Text::new("Company name:").prompt()?;
        let job_title = inquire::Text::new("Job title:").prompt()?;
        let location = inquire::Text::new("Location (optional):")
            .prompt_skippable()?
            .filter(|s| !s.is_empty());

        let (start_year, start_month) = prompt_date("Start date")?;
        let start_date = if let Some(month) = start_month {
            DateValue::YearMonth {
                year: start_year,
                month,
            }
        } else {
            DateValue::Year(start_year)
        };

        let is_current = inquire::Confirm::new("Currently working here?")
            .with_default(true)
            .prompt()?;

        let end_date = if !is_current {
            let (end_year, end_month) = prompt_date("End date")?;
            Some(if let Some(month) = end_month {
                DateValue::YearMonth {
                    year: end_year,
                    month,
                }
            } else {
                DateValue::Year(end_year)
            })
        } else {
            None
        };

        let highlights_input = inquire::Text::new("Key achievements (comma-separated):")
            .prompt_skippable()?
            .unwrap_or_default();
        let highlights: Vec<String> = highlights_input
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let technologies_input = inquire::Text::new("Technologies used (comma-separated):")
            .prompt_skippable()?
            .unwrap_or_default();
        let technologies: Vec<String> = technologies_input
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        resume.experience.push(Experience {
            company,
            title: job_title,
            location,
            start_date,
            end_date,
            highlights,
            technologies,
            raw_notes: None,
        });

        println!("✓ Experience entry added");
    }

    // Education section
    println!("\n🎓 Education");
    loop {
        let add_education = inquire::Confirm::new("Add an education entry?")
            .with_default(resume.education.is_empty())
            .prompt()?;

        if !add_education {
            break;
        }

        let institution = inquire::Text::new("School/University name:").prompt()?;
        let degree = inquire::Text::new("Degree (e.g., Bachelor of Science):").prompt()?;
        let _location = inquire::Text::new("Location (optional):")
            .prompt_skippable()?
            .filter(|s| !s.is_empty());

        let (start_year, start_month) = prompt_date("Start date")?;
        let start_date = if let Some(month) = start_month {
            DateValue::YearMonth {
                year: start_year,
                month,
            }
        } else {
            DateValue::Year(start_year)
        };

        let (grad_year, grad_month) = prompt_date("Graduation date")?;
        let end_date = if let Some(month) = grad_month {
            Some(DateValue::YearMonth {
                year: grad_year,
                month,
            })
        } else {
            Some(DateValue::Year(grad_year))
        };

        let gpa = inquire::Text::new("GPA (optional):")
            .prompt_skippable()?
            .filter(|s| !s.is_empty());

        resume.education.push(Education {
            institution,
            degree,
            start_date,
            end_date,
            gpa,
            highlights: vec![],
        });

        println!("✓ Education entry added");
    }

    // Skills section
    println!("\n🛠️  Skills");
    loop {
        let add_skill_category = inquire::Confirm::new("Add a skill category?")
            .with_default(resume.skills.groups.is_empty())
            .prompt()?;

        if !add_skill_category {
            break;
        }

        let category = inquire::Text::new("Skill category (e.g., Programming Languages):").prompt()?;
        let skills_input = inquire::Text::new("Skills in this category (comma-separated):")
            .prompt()?;
        let skills: Vec<String> = skills_input
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if !skills.is_empty() {
            resume.skills.groups.push(SkillGroup { category, skills });
            println!("✓ Skill category added");
        }
    }

    // Projects section
    println!("\n📦 Projects");
    loop {
        let add_project = inquire::Confirm::new("Add a project?")
            .with_default(false)
            .prompt()?;

        if !add_project {
            break;
        }

        let name = inquire::Text::new("Project name:").prompt()?;
        let description = inquire::Text::new("Description:").prompt()?;
        let url = inquire::Text::new("Project URL/GitHub (optional):")
            .prompt_skippable()?
            .filter(|s| !s.is_empty());

        let highlights_input = inquire::Text::new("Key points (comma-separated):")
            .prompt_skippable()?
            .unwrap_or_default();
        let highlights: Vec<String> = highlights_input
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let technologies_input = inquire::Text::new("Technologies used (comma-separated):")
            .prompt_skippable()?
            .unwrap_or_default();
        let technologies: Vec<String> = technologies_input
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        resume.projects.push(cr_core::resume::Project {
            name,
            description,
            url,
            highlights,
            technologies,
            role: None,
        });

        println!("✓ Project added");
    }

    // Certifications section (optional)
    println!("\n📜 Certifications (optional)");
    loop {
        let add_cert = inquire::Confirm::new("Add a certification?")
            .with_default(false)
            .prompt()?;

        if !add_cert {
            break;
        }

        let name = inquire::Text::new("Certification name:").prompt()?;
        let issuer = inquire::Text::new("Issuer/Organization:").prompt()?;
        let date_str = inquire::Text::new("Issue date (optional, YYYY or YYYY-MM):")
            .prompt_skippable()?;
        
        let date = date_str.and_then(|d| {
            if d.is_empty() {
                None
            } else {
                let parts: Vec<&str> = d.split('-').collect();
                if let Ok(year) = parts[0].parse::<u16>() {
                    let month = if parts.len() > 1 {
                        parts[1].parse::<u8>().ok()
                    } else {
                        None
                    };
                    Some(if let Some(m) = month {
                        DateValue::YearMonth { year, month: m }
                    } else {
                        DateValue::Year(year)
                    })
                } else {
                    None
                }
            }
        });

        resume.certifications.push(Certification {
            name,
            issuer,
            date,
            url: None,
        });

        println!("✓ Certification added");
    }

    // Template selection
    println!("\n🎨 Template");
    let template = if let Some(ref t) = args.template {
        t.clone()
    } else {
        let templates = vec!["Classic", "Modern Tech", "Minimal", "Two Column", "Academic", "ATS-Simple", "Brilliant"];
        inquire::Select::new("Choose a template:", templates)
            .prompt()
            .map(|s| s.to_lowercase())?
    };

    // Save resume
    let output_path = PathBuf::from(&args.output).with_extension("yaml");
    cr_io::data_file::save(&resume, &output_path)?;
    println!("\n✅ Resume data saved to: {}", output_path.display());

    // Generate output
    println!("\n🚀 Generating resume...");
    let _render_request = cr_core::template::RenderRequest {
        template: template.parse().map_err(|e: String| anyhow::anyhow!(e))?,
        language: resume.language,
        output_path: PathBuf::from(&args.output),
        formats: vec![cr_core::template::OutputFormat::Pdf, cr_core::template::OutputFormat::Markdown],
    };

    println!("✅ Resume created successfully!");
    println!("Next: Run `coderesume generate {} -t {}` to generate PDF", output_path.display(), template);

    Ok(())
}

fn prompt_date(label: &str) -> anyhow::Result<(u16, Option<u8>)> {
    let date_str = inquire::Text::new(&format!("{} (YYYY or YYYY-MM):", label))
        .prompt()?;

    let parts: Vec<&str> = date_str.split('-').collect();

    let year: u16 = parts[0].parse()
        .map_err(|_| anyhow::anyhow!("Invalid year format"))?;

    let month = if parts.len() > 1 {
        let m: u8 = parts[1].parse()
            .map_err(|_| anyhow::anyhow!("Invalid month format"))?;
        if m < 1 || m > 12 {
            anyhow::bail!("Month must be between 1 and 12");
        }
        Some(m)
    } else {
        None
    };

    Ok((year, month))
}
