use cr_core::resume::{Language, PersonalInfo, Resume};
use std::collections::HashMap;

/// User input data for resume creation
#[derive(Debug, Clone)]
pub struct CreateInput {
    pub language: Language,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub title: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub linkedin: Option<String>,
    pub github: Option<String>,
    pub summary: Option<String>,
}

impl CreateInput {
    /// Validate all required fields are present and well-formed
    pub fn validate(&self) -> anyhow::Result<()> {
        // Name validation
        if self.name.trim().is_empty() {
            anyhow::bail!("Name cannot be empty");
        }
        if self.name.len() > 200 {
            anyhow::bail!("Name is too long (max 200 characters)");
        }

        // Email validation (basic check)
        if self.email.trim().is_empty() {
            anyhow::bail!("Email cannot be empty");
        }
        if !self.email.contains('@') {
            anyhow::bail!("Invalid email format");
        }
        if self.email.len() > 254 {
            anyhow::bail!("Email is too long (max 254 characters)");
        }

        // Optional fields validation
        if let Some(ref phone) = self.phone {
            if phone.len() > 20 {
                anyhow::bail!("Phone number is too long");
            }
        }

        if let Some(ref title) = self.title {
            if title.len() > 200 {
                anyhow::bail!("Job title is too long (max 200 characters)");
            }
        }

        if let Some(ref location) = self.location {
            if location.len() > 200 {
                anyhow::bail!("Location is too long (max 200 characters)");
            }
        }

        if let Some(ref website) = self.website {
            if website.len() > 500 {
                anyhow::bail!("Website URL is too long");
            }
        }

        if let Some(ref linkedin) = self.linkedin {
            if linkedin.len() > 500 {
                anyhow::bail!("LinkedIn URL is too long");
            }
        }

        if let Some(ref github) = self.github {
            if github.len() > 500 {
                anyhow::bail!("GitHub URL is too long");
            }
        }

        if let Some(ref summary) = self.summary {
            if summary.len() > 2000 {
                anyhow::bail!("Summary is too long (max 2000 characters)");
            }
        }

        Ok(())
    }
}

/// Create a new resume from user input with validation
pub fn create(input: CreateInput) -> anyhow::Result<Resume> {
    // Validate input first
    input.validate()?;

    let personal = PersonalInfo {
        name: input.name.trim().to_string(),
        name_alt: None,
        email: input.email.trim().to_string(),
        phone: input.phone.map(|p| p.trim().to_string()),
        location: input.location.map(|l| l.trim().to_string()),
        title: input.title.map(|t| t.trim().to_string()),
        photo: None,
        website: input.website.map(|w| w.trim().to_string()),
        linkedin: input.linkedin.map(|l| l.trim().to_string()),
        github: input.github.map(|g| g.trim().to_string()),
        extra_links: HashMap::new(),
    };

    let resume = Resume {
        version: "1.0".to_string(),
        language: input.language,
        personal,
        summary: input.summary.map(|s| s.trim().to_string()),
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

    Ok(resume)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_valid_input() -> CreateInput {
        CreateInput {
            language: Language::En,
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            phone: Some("+1 555-0100".to_string()),
            title: Some("Senior Software Engineer".to_string()),
            location: Some("San Francisco, CA".to_string()),
            website: Some("https://johndoe.com".to_string()),
            linkedin: Some("https://linkedin.com/in/johndoe".to_string()),
            github: Some("https://github.com/johndoe".to_string()),
            summary: Some("Experienced software engineer with 10 years of experience.".to_string()),
        }
    }

    #[test]
    fn test_create_valid_resume() {
        let input = make_valid_input();
        let resume = create(input.clone()).expect("Failed to create resume");

        assert_eq!(resume.personal.name, "John Doe");
        assert_eq!(resume.personal.email, "john@example.com");
        assert_eq!(resume.language, Language::En);
        assert_eq!(resume.version, "1.0");
        assert!(resume.experience.is_empty());
        assert!(resume.education.is_empty());
    }

    #[test]
    fn test_validate_empty_name() {
        let mut input = make_valid_input();
        input.name = "  ".to_string();
        assert!(input.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_email() {
        let mut input = make_valid_input();
        input.email = "not-an-email".to_string();
        assert!(input.validate().is_err());
    }

    #[test]
    fn test_validate_empty_email() {
        let mut input = make_valid_input();
        input.email = "  ".to_string();
        assert!(input.validate().is_err());
    }

    #[test]
    fn test_validate_fields_trimmed() {
        let mut input = make_valid_input();
        input.name = "  John Doe  ".to_string();
        input.email = "  john@example.com  ".to_string();
        input.title = Some("  Senior Software Engineer  ".to_string());

        let resume = create(input).expect("Failed to create resume");
        assert_eq!(resume.personal.name, "John Doe");
        assert_eq!(resume.personal.email, "john@example.com");
        assert_eq!(resume.personal.title.unwrap(), "Senior Software Engineer");
    }

    #[test]
    fn test_validate_name_too_long() {
        let mut input = make_valid_input();
        input.name = "a".repeat(201);
        assert!(input.validate().is_err());
    }

    #[test]
    fn test_validate_summary_too_long() {
        let mut input = make_valid_input();
        input.summary = Some("a".repeat(2001));
        assert!(input.validate().is_err());
    }

    #[test]
    fn test_create_with_chinese_language() {
        let mut input = make_valid_input();
        input.language = Language::Zh;
        input.name = "张三".to_string();

        let resume = create(input).expect("Failed to create resume");
        assert_eq!(resume.language, Language::Zh);
        assert_eq!(resume.personal.name, "张三");
    }

    #[test]
    fn test_create_minimal_input() {
        let input = CreateInput {
            language: Language::En,
            name: "Jane Doe".to_string(),
            email: "jane@example.com".to_string(),
            phone: None,
            title: None,
            location: None,
            website: None,
            linkedin: None,
            github: None,
            summary: None,
        };

        let resume = create(input).expect("Failed to create resume");
        assert_eq!(resume.personal.name, "Jane Doe");
        assert_eq!(resume.personal.email, "jane@example.com");
        assert!(resume.personal.title.is_none());
        assert!(resume.summary.is_none());
    }
}

impl CreateInput {
    /// Create a minimal input with just required fields
    #[cfg(test)]
    pub fn minimal() -> Self {
        Self {
            language: Language::En,
            name: "Test Name".to_string(),
            email: "test@example.com".to_string(),
            phone: None,
            title: None,
            location: None,
            website: None,
            linkedin: None,
            github: None,
            summary: None,
        }
    }
}
