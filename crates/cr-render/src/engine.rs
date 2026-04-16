use anyhow::Context;
use cr_core::resume::Resume;
use cr_core::template::{OutputFormat, RenderRequest};
use std::path::PathBuf;
use std::process::Command;

pub struct RenderEngine {
    typst_bin: String,
    template_base: PathBuf,
}

impl RenderEngine {
    pub fn new(typst_bin: Option<String>, template_base: PathBuf) -> Self {
        Self {
            typst_bin: typst_bin.unwrap_or_else(|| "typst".to_string()),
            template_base: std::fs::canonicalize(template_base)
                .unwrap_or_else(|_| PathBuf::from("templates")),
        }
    }

    /// Full render: Resume → JSON data file → Typst compile → PDF
    pub fn render(&self, resume: &Resume, request: &RenderRequest) -> anyhow::Result<Vec<PathBuf>> {
        let mut outputs = Vec::new();

        // Write resume data JSON next to the template (in template dir)
        let template_dir = self.template_base.join(request.template.dir_name());
        let data_path = super::data_bridge::write_data_json(resume, &template_dir)?;

        let typ_src = template_dir.join("template.typ");

        // If resume has a photo, copy it to the template directory so Typst can find it
        let mut photo_copy: Option<PathBuf> = None;
        if let Some(ref photo_src) = resume.personal.photo {
            let src = PathBuf::from(photo_src);
            if src.exists() {
                let filename = src
                    .file_name()
                    .unwrap_or_else(|| std::ffi::OsStr::new("photo.png"));
                let dest = template_dir.join(filename);
                std::fs::copy(&src, &dest)?;
                photo_copy = Some(dest);
            }
        }

        for format in &request.formats {
            match format {
                OutputFormat::Pdf => {
                    let pdf_path = request.output_path.with_extension("pdf");
                    if let Some(parent) = pdf_path.parent() {
                        std::fs::create_dir_all(parent)
                            .with_context(|| format!("Failed to create output directory: {:?}", parent))?;
                    }

                    let lang_str = match resume.language {
                        cr_core::resume::Language::En => "en",
                        cr_core::resume::Language::Zh => "zh",
                    };

                    let output = Command::new(&self.typst_bin)
                        .arg("compile")
                        .arg(&typ_src)
                        .arg(&pdf_path)
                        .arg("--input")
                        .arg("data-path=resume-data.json")
                        .arg("--input")
                        .arg(format!("lang={}", lang_str))
                        .output()?;

                    // Clean up temp files - log warnings if cleanup fails
                    if let Err(e) = std::fs::remove_file(&data_path) {
                        tracing::warn!("Failed to remove temporary data file {:?}: {}", data_path, e);
                    }
                    if let Some(ref p) = photo_copy {
                        if let Err(e) = std::fs::remove_file(p) {
                            tracing::warn!("Failed to remove temporary photo file {:?}: {}", p, e);
                        }
                    }

                    if !output.status.success() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        anyhow::bail!("Typst compilation failed:\n{}", stderr);
                    }
                    outputs.push(pdf_path);
                }
                OutputFormat::Markdown => {
                    let md_path = request.output_path.with_extension("md");
                    if let Some(parent) = md_path.parent() {
                        std::fs::create_dir_all(parent)
                            .with_context(|| format!("Failed to create output directory: {:?}", parent))?;
                    }
                    let md = cr_io::markdown::render_markdown(resume);
                    std::fs::write(&md_path, md)?;
                    outputs.push(md_path);
                }
            }
        }

        Ok(outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cr_core::resume::{Language, PersonalInfo, Resume};
    use cr_core::template::TemplateId;
    use std::collections::HashMap;

    fn create_test_resume() -> Resume {
        Resume {
            version: "1.0".to_string(),
            language: Language::En,
            personal: PersonalInfo {
                name: "Test User".to_string(),
                name_alt: None,
                email: "test@example.com".to_string(),
                phone: Some("+1-555-0100".to_string()),
                location: Some("San Francisco, CA".to_string()),
                title: Some("Software Engineer".to_string()),
                photo: None,
                website: None,
                linkedin: None,
                github: None,
                extra_links: HashMap::new(),
            },
            summary: Some("Experienced engineer".to_string()),
            experience: vec![],
            education: vec![],
            skills: Default::default(),
            projects: vec![],
            certifications: vec![],
            publications: vec![],
            open_source: vec![],
            custom_sections: vec![],
            meta: Default::default(),
        }
    }

    #[test]
    fn test_render_engine_new() {
        let engine = RenderEngine::new(None, PathBuf::from("templates"));
        assert_eq!(engine.typst_bin, "typst");
    }

    #[test]
    fn test_render_engine_custom_bin() {
        let engine = RenderEngine::new(Some("/usr/bin/custom-typst".to_string()), PathBuf::from("templates"));
        assert_eq!(engine.typst_bin, "/usr/bin/custom-typst");
    }

    #[test]
    fn test_render_request_construction() {
        let request = cr_core::template::RenderRequest {
            template: TemplateId::Classic,
            language: Language::En,
            output_path: PathBuf::from("/tmp/resume"),
            formats: vec![cr_core::template::OutputFormat::Markdown],
        };

        assert_eq!(request.template, TemplateId::Classic);
        assert_eq!(request.language, Language::En);
        assert_eq!(request.formats.len(), 1);
    }

    #[test]
    fn test_resume_data_minimal() {
        let resume = create_test_resume();
        assert_eq!(resume.personal.name, "Test User");
        assert_eq!(resume.personal.email, "test@example.com");
        assert_eq!(resume.language, Language::En);
    }

    #[test]
    fn test_render_markdown_output() {
        let resume = create_test_resume();
        let md = cr_io::markdown::render_markdown(&resume);
        
        // Verify markdown contains key sections
        assert!(md.contains("Test User"), "Markdown should contain user name");
        assert!(md.contains("test@example.com"), "Markdown should contain email");
        assert!(md.contains("Software Engineer"), "Markdown should contain job title");
        assert!(md.contains("Experienced engineer"), "Markdown should contain summary");
    }
}
