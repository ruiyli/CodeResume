use clap::Args;
use cr_config::AppConfig;
use cr_core::template::TemplateId;
use cr_render::engine::RenderEngine;
use std::path::PathBuf;

#[derive(Args)]
pub struct ValidateArgs {
    /// Path to resume data file (YAML or JSON)
    pub input: PathBuf,
    /// Check ATS compatibility (extracts text and analyzes)
    #[arg(long, default_value_t = false)]
    pub check_ats: bool,
    /// Template to validate (validates all if not specified)
    #[arg(short, long)]
    pub template: Option<String>,
    /// Output directory for generated PDFs during validation
    #[arg(short, long, default_value = "./validate-output")]
    pub output_dir: String,
}

pub async fn run(args: ValidateArgs, config: &AppConfig) -> anyhow::Result<()> {
    // Load resume data
    let resume = cr_io::data_file::load(&args.input)?;
    println!("Validating resume: {}", resume.personal.name);

    // Create output directory
    std::fs::create_dir_all(&args.output_dir)?;

    // Determine which templates to validate
    let templates_to_check: Vec<TemplateId> = if let Some(ref template_name) = args.template {
        vec![template_name
            .parse()
            .map_err(|e: String| anyhow::anyhow!(e))?]
    } else {
        // Check all templates
        vec![
            TemplateId::Modern,
            TemplateId::Classic,
            TemplateId::Minimal,
            TemplateId::Academic,
            TemplateId::TwoColumn,
        ]
    };

    let template_base = PathBuf::from("templates");
    let engine = RenderEngine::new(config.output.typst_bin.clone(), template_base);

    println!("\n📋 Template Validation Report\n");
    println!("{:-<60}", "");

    let mut all_compatible = true;

    for template in templates_to_check {
        print!("Checking {} template... ", template.dir_name());

        // Generate PDF for this template
        let output_file = format!("{}/validate-{}", args.output_dir, template.dir_name());
        let output_path = PathBuf::from(&output_file);

        let request = cr_core::template::RenderRequest {
            template: template.clone(),
            language: resume.language,
            output_path: output_path.clone(),
            formats: vec![cr_core::template::OutputFormat::Pdf],
        };

        match engine.render(&resume, &request) {
            Ok(outputs) => {
                let pdf_path = outputs.first().ok_or_else(|| anyhow::anyhow!("No PDF generated"))?;
                println!("✓");

                if args.check_ats {
                    // Extract and analyze text
                    match cr_io::pdf_parse::extract_text(pdf_path) {
                        Ok(text) => {
                            let analysis = cr_io::pdf_parse::analyze_ats_compatibility(&text);
                            
                            if analysis.is_compatible {
                                println!("  └─ ATS Compatibility: ✓ PASS ({}  chars extracted)", analysis.text_length);
                            } else {
                                all_compatible = false;
                                println!("  └─ ATS Compatibility: ✗ FAIL ({}  chars extracted)", analysis.text_length);
                                for issue in &analysis.issues {
                                    println!("     · {}", issue);
                                }
                            }

                            if !analysis.suggestions.is_empty() {
                                for suggestion in &analysis.suggestions {
                                    println!("     → {}", suggestion);
                                }
                            }
                        }
                        Err(e) => {
                            all_compatible = false;
                            println!("  └─ ATS Compatibility: ✗ ERROR - Failed to extract text: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("✗");
                println!("  └─ Error: {}", e);
                all_compatible = false;
            }
        }
    }

    println!("{:-<60}\n", "");

    if args.check_ats {
        if all_compatible {
            println!("✅ All templates passed ATS compatibility checks!\n");
        } else {
            println!("⚠️  Some templates failed ATS compatibility checks.\n");
            println!("    Recommendation: Use 'ats-simple' template for guaranteed ATS compatibility.");
            println!("    Or review template choice with 'coderesume templates'\n");
        }
    } else {
        println!("✅ All templates generated successfully!\n");
        println!("    Use --check-ats flag to validate ATS compatibility:\n");
        println!("    $ coderesume validate <file> --check-ats\n");
    }

    Ok(())
}
