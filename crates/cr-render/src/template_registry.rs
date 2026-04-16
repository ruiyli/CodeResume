use cr_core::resume::Language;
use cr_core::template::{TemplateId, TemplateMeta};
use std::path::{Path, PathBuf};

pub struct TemplateRegistry {
    base_dir: PathBuf,
    templates: Vec<TemplateMeta>,
}

impl TemplateRegistry {
    /// Scan the templates directory and load all meta.toml files.
    pub fn discover(base_dir: &Path) -> anyhow::Result<Self> {
        let mut templates = Vec::new();

        for id in TemplateId::all() {
            let meta_path = base_dir.join(id.dir_name()).join("meta.toml");
            if meta_path.exists() {
                let contents = std::fs::read_to_string(&meta_path)?;
                let meta: TemplateMeta = toml::from_str(&contents)?;
                templates.push(meta);
            } else {
                // Create a default meta from the TemplateId
                templates.push(TemplateMeta {
                    id: *id,
                    name: id.display_name().to_string(),
                    name_zh: id.display_name_zh().to_string(),
                    description: format!("{} resume template", id.display_name()),
                    description_zh: format!("{}简历模板", id.display_name_zh()),
                    supports_languages: vec![Language::En, Language::Zh],
                    page_limit_hint: Some(2),
                    preview_image: None,
                });
            }
        }

        Ok(Self {
            base_dir: base_dir.to_path_buf(),
            templates,
        })
    }

    pub fn list(&self) -> &[TemplateMeta] {
        &self.templates
    }

    pub fn get(&self, id: TemplateId) -> Option<&TemplateMeta> {
        self.templates.iter().find(|t| t.id == id)
    }

    /// Return the path to the .typ file for a given template.
    pub fn typ_path(&self, id: TemplateId) -> PathBuf {
        self.base_dir.join(id.dir_name()).join("template.typ")
    }
}
