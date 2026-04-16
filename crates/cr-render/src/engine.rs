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
                        std::fs::create_dir_all(parent).ok();
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

                    // Clean up temp files
                    std::fs::remove_file(&data_path).ok();
                    if let Some(ref p) = photo_copy {
                        std::fs::remove_file(p).ok();
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
                        std::fs::create_dir_all(parent).ok();
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
