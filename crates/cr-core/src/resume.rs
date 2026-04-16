use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Top-level resume document — the canonical data format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resume {
    /// Schema version for forward-compatible migration
    #[serde(default = "default_version")]
    pub version: String,
    pub language: Language,
    pub personal: PersonalInfo,
    pub summary: Option<String>,
    #[serde(default)]
    pub experience: Vec<Experience>,
    #[serde(default)]
    pub education: Vec<Education>,
    #[serde(default)]
    pub skills: SkillSet,
    #[serde(default)]
    pub projects: Vec<Project>,
    #[serde(default)]
    pub certifications: Vec<Certification>,
    #[serde(default)]
    pub publications: Vec<Publication>,
    #[serde(default)]
    pub open_source: Vec<OpenSourceContribution>,
    #[serde(default)]
    pub custom_sections: Vec<CustomSection>,
    /// Internal metadata — not rendered, used by AI features
    #[serde(default)]
    pub meta: ResumeMeta,
}

fn default_version() -> String {
    "1.0".to_string()
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    En,
    Zh,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::En => write!(f, "en"),
            Language::Zh => write!(f, "zh"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalInfo {
    pub name: String,
    /// Optional name in secondary language
    pub name_alt: Option<String>,
    pub email: String,
    pub phone: Option<String>,
    pub location: Option<String>,
    /// Job title, e.g. "Senior Backend Engineer"
    pub title: Option<String>,
    /// Path to photo/avatar image file
    pub photo: Option<String>,
    pub website: Option<String>,
    pub linkedin: Option<String>,
    pub github: Option<String>,
    #[serde(default)]
    pub extra_links: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub company: String,
    pub title: String,
    pub location: Option<String>,
    pub start_date: DateValue,
    /// None means "Present"
    pub end_date: Option<DateValue>,
    #[serde(default)]
    pub highlights: Vec<String>,
    #[serde(default)]
    pub technologies: Vec<String>,
    /// Raw draft the user typed — AI rewrites into `highlights`
    pub raw_notes: Option<String>,
}

/// Flexible date: full date, year-month, or year-only.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DateValue {
    Full(NaiveDate),
    YearMonth { year: u16, month: u8 },
    Year(u16),
}

impl DateValue {
    pub fn display(&self, lang: Language) -> String {
        match (self, lang) {
            (DateValue::Full(d), Language::En) => d.format("%B %Y").to_string(),
            (DateValue::Full(d), Language::Zh) => d.format("%Y年%m月").to_string(),
            (DateValue::YearMonth { year, month }, Language::En) => {
                let month_names = [
                    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov",
                    "Dec",
                ];
                let idx = (*month as usize).saturating_sub(1).min(11);
                format!("{} {}", month_names[idx], year)
            }
            (DateValue::YearMonth { year, month }, Language::Zh) => {
                format!("{}年{}月", year, month)
            }
            (DateValue::Year(y), _) => y.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Education {
    pub institution: String,
    pub degree: String,
    pub start_date: DateValue,
    pub end_date: Option<DateValue>,
    pub gpa: Option<String>,
    #[serde(default)]
    pub highlights: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SkillSet {
    #[serde(default)]
    pub groups: Vec<SkillGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillGroup {
    pub category: String,
    pub skills: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub url: Option<String>,
    pub description: String,
    #[serde(default)]
    pub highlights: Vec<String>,
    #[serde(default)]
    pub technologies: Vec<String>,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certification {
    pub name: String,
    pub issuer: String,
    pub date: Option<DateValue>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Publication {
    pub title: String,
    pub venue: String,
    pub date: Option<DateValue>,
    pub url: Option<String>,
    pub authors: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenSourceContribution {
    pub project: String,
    pub url: Option<String>,
    pub role: String,
    pub description: String,
    pub stars: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomSection {
    pub title: String,
    pub items: Vec<CustomItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomItem {
    pub heading: Option<String>,
    pub subheading: Option<String>,
    pub date: Option<String>,
    pub body: Option<String>,
    #[serde(default)]
    pub bullets: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResumeMeta {
    /// Timestamp of last AI optimization
    pub last_optimized: Option<String>,
    /// Target JD hash — detect stale tailoring
    pub tailored_for_jd_hash: Option<String>,
    /// Model used for last AI operation
    pub last_model: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_value_display_english() {
        let d = DateValue::YearMonth {
            year: 2023,
            month: 3,
        };
        assert_eq!(d.display(Language::En), "Mar 2023");
    }

    #[test]
    fn date_value_display_chinese() {
        let d = DateValue::YearMonth {
            year: 2023,
            month: 3,
        };
        assert_eq!(d.display(Language::Zh), "2023年3月");
    }

    #[test]
    fn resume_deserialize_minimal() {
        let yaml = r#"
version: "1.0"
language: en
personal:
  name: "Test User"
  email: "test@example.com"
"#;
        let resume: Resume = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(resume.personal.name, "Test User");
        assert!(resume.experience.is_empty());
        assert!(resume.summary.is_none());
    }

    #[test]
    fn resume_serde_roundtrip() {
        let yaml = r#"
version: "1.0"
language: en
personal:
  name: "John Doe"
  email: "john@example.com"
  phone: "+1-555-0100"
  title: "Senior Software Engineer"
  github: "https://github.com/johndoe"
summary: "Experienced engineer with 8 years in distributed systems."
experience:
  - company: "TechCorp"
    title: "Senior Engineer"
    start_date:
      year: 2020
      month: 1
    end_date: null
    highlights:
      - "Led migration to microservices, reducing deploy time by 40%"
      - "Designed real-time data pipeline processing 1M events/sec"
    technologies:
      - "Rust"
      - "Kubernetes"
      - "Kafka"
education:
  - institution: "MIT"
    degree: "B.S. Computer Science"
    start_date: 2012
    end_date: 2016
    gpa: "3.9"
skills:
  groups:
    - category: "Languages"
      skills: ["Rust", "Go", "Python", "TypeScript"]
    - category: "Infrastructure"
      skills: ["Kubernetes", "Docker", "AWS", "Terraform"]
"#;
        let resume: Resume = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(resume.personal.name, "John Doe");
        assert_eq!(resume.experience.len(), 1);
        assert_eq!(resume.skills.groups.len(), 2);

        // Roundtrip
        let serialized = serde_yaml::to_string(&resume).unwrap();
        let parsed: Resume = serde_yaml::from_str(&serialized).unwrap();
        assert_eq!(parsed.personal.email, resume.personal.email);
        assert_eq!(parsed.experience.len(), resume.experience.len());
    }
}
