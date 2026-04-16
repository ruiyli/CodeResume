use serde::{Deserialize, Serialize};

use crate::resume::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TemplateId {
    Classic,
    Modern,
    Minimal,
    TwoColumn,
    Academic,
    #[serde(rename = "ats-simple")]
    AtSimple,
    Brilliant,
}

impl TemplateId {
    pub fn all() -> &'static [TemplateId] {
        &[
            Self::Classic,
            Self::Modern,
            Self::Minimal,
            Self::TwoColumn,
            Self::Academic,
            Self::AtSimple,
            Self::Brilliant,
        ]
    }

    pub fn dir_name(&self) -> &'static str {
        match self {
            Self::Classic => "classic",
            Self::Modern => "modern",
            Self::Minimal => "minimal",
            Self::TwoColumn => "two-column",
            Self::Academic => "academic",
            Self::AtSimple => "ats-simple",
            Self::Brilliant => "brilliant",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Classic => "Classic",
            Self::Modern => "Modern Tech",
            Self::Minimal => "Minimal",
            Self::TwoColumn => "Two Column",
            Self::Academic => "Academic",
            Self::AtSimple => "ATS-Simple",
            Self::Brilliant => "Brilliant",
        }
    }

    pub fn display_name_zh(&self) -> &'static str {
        match self {
            Self::Classic => "经典",
            Self::Modern => "现代科技",
            Self::Minimal => "极简",
            Self::TwoColumn => "双栏",
            Self::Academic => "学术",
            Self::AtSimple => "ATS兼容",
            Self::Brilliant => "炫彩",
        }
    }
}

impl std::fmt::Display for TemplateId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.dir_name())
    }
}

impl std::str::FromStr for TemplateId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "classic" => Ok(Self::Classic),
            "modern" => Ok(Self::Modern),
            "minimal" => Ok(Self::Minimal),
            "two-column" | "twocolumn" => Ok(Self::TwoColumn),
            "academic" => Ok(Self::Academic),
            "ats-simple" | "atssimple" | "ats" => Ok(Self::AtSimple),
            "brilliant" => Ok(Self::Brilliant),
            _ => Err(format!("Unknown template: {s}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMeta {
    pub id: TemplateId,
    pub name: String,
    pub name_zh: String,
    pub description: String,
    pub description_zh: String,
    pub supports_languages: Vec<Language>,
    pub page_limit_hint: Option<u8>,
    pub preview_image: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RenderRequest {
    pub template: TemplateId,
    pub language: Language,
    pub output_path: std::path::PathBuf,
    pub formats: Vec<OutputFormat>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Pdf,
    Markdown,
}
