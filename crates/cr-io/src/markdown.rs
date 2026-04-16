use cr_core::resume::{Language, Resume};

/// Render a Resume struct as a Markdown string.
pub fn render_markdown(resume: &Resume) -> String {
    let mut md = String::new();
    let lang = resume.language;

    // Header
    md.push_str(&format!("# {}\n\n", resume.personal.name));

    if let Some(ref title) = resume.personal.title {
        md.push_str(&format!("**{}**\n\n", title));
    }

    // Contact info
    let mut contacts = vec![resume.personal.email.clone()];
    if let Some(ref phone) = resume.personal.phone {
        contacts.push(phone.clone());
    }
    if let Some(ref location) = resume.personal.location {
        contacts.push(location.clone());
    }
    if let Some(ref github) = resume.personal.github {
        contacts.push(format!("[GitHub]({})", github));
    }
    if let Some(ref linkedin) = resume.personal.linkedin {
        contacts.push(format!("[LinkedIn]({})", linkedin));
    }
    if let Some(ref website) = resume.personal.website {
        contacts.push(format!("[Website]({})", website));
    }
    md.push_str(&contacts.join(" | "));
    md.push_str("\n\n");

    // Summary
    if let Some(ref summary) = resume.summary {
        md.push_str(&format!(
            "## {}\n\n{}\n\n",
            section_title("Summary", "个人简介", lang),
            summary
        ));
    }

    // Experience
    if !resume.experience.is_empty() {
        md.push_str(&format!(
            "## {}\n\n",
            section_title("Experience", "工作经历", lang)
        ));
        for exp in &resume.experience {
            let end = exp
                .end_date
                .as_ref()
                .map(|d| d.display(lang))
                .unwrap_or_else(|| present_text(lang).to_string());
            md.push_str(&format!(
                "### {} | {}\n\n*{} — {}*\n\n",
                exp.title,
                exp.company,
                exp.start_date.display(lang),
                end,
            ));
            for bullet in &exp.highlights {
                md.push_str(&format!("- {}\n", bullet));
            }
            if !exp.technologies.is_empty() {
                md.push_str(&format!("\n**Tech:** {}\n", exp.technologies.join(", ")));
            }
            md.push('\n');
        }
    }

    // Education
    if !resume.education.is_empty() {
        md.push_str(&format!(
            "## {}\n\n",
            section_title("Education", "教育背景", lang)
        ));
        for edu in &resume.education {
            let end = edu
                .end_date
                .as_ref()
                .map(|d| d.display(lang))
                .unwrap_or_default();
            md.push_str(&format!(
                "### {} | {}\n\n*{} — {}*",
                edu.institution,
                edu.degree,
                edu.start_date.display(lang),
                end,
            ));
            if let Some(ref gpa) = edu.gpa {
                md.push_str(&format!(" | GPA: {}", gpa));
            }
            md.push_str("\n\n");
            for bullet in &edu.highlights {
                md.push_str(&format!("- {}\n", bullet));
            }
            md.push('\n');
        }
    }

    // Skills
    if !resume.skills.groups.is_empty() {
        md.push_str(&format!(
            "## {}\n\n",
            section_title("Skills", "专业技能", lang)
        ));
        for group in &resume.skills.groups {
            md.push_str(&format!(
                "- **{}:** {}\n",
                group.category,
                group.skills.join(", ")
            ));
        }
        md.push('\n');
    }

    // Projects
    if !resume.projects.is_empty() {
        md.push_str(&format!(
            "## {}\n\n",
            section_title("Projects", "项目经历", lang)
        ));
        for proj in &resume.projects {
            let mut header = format!("### {}", proj.name);
            if let Some(ref url) = proj.url {
                header.push_str(&format!(" [↗]({})", url));
            }
            md.push_str(&header);
            md.push_str("\n\n");
            md.push_str(&format!("{}\n\n", proj.description));
            for bullet in &proj.highlights {
                md.push_str(&format!("- {}\n", bullet));
            }
            if !proj.technologies.is_empty() {
                md.push_str(&format!("\n**Tech:** {}\n", proj.technologies.join(", ")));
            }
            md.push('\n');
        }
    }

    // Certifications
    if !resume.certifications.is_empty() {
        md.push_str(&format!(
            "## {}\n\n",
            section_title("Certifications", "证书", lang)
        ));
        for cert in &resume.certifications {
            md.push_str(&format!("- **{}** — {}", cert.name, cert.issuer));
            if let Some(ref date) = cert.date {
                md.push_str(&format!(" ({})", date.display(lang)));
            }
            md.push('\n');
        }
        md.push('\n');
    }

    // Publications
    if !resume.publications.is_empty() {
        md.push_str(&format!(
            "## {}\n\n",
            section_title("Publications", "发表文章", lang)
        ));
        for pub_item in &resume.publications {
            md.push_str(&format!("- **{}** — {}", pub_item.title, pub_item.venue));
            if let Some(ref date) = pub_item.date {
                md.push_str(&format!(" ({})", date.display(lang)));
            }
            md.push('\n');
        }
        md.push('\n');
    }

    // Open Source
    if !resume.open_source.is_empty() {
        md.push_str(&format!(
            "## {}\n\n",
            section_title("Open Source", "开源贡献", lang)
        ));
        for os in &resume.open_source {
            md.push_str(&format!("- **{}** — {}\n", os.project, os.description));
            if let Some(stars) = os.stars {
                md.push_str(&format!("  ⭐ {} stars\n", stars));
            }
        }
        md.push('\n');
    }

    // Custom Sections
    for section in &resume.custom_sections {
        md.push_str(&format!("## {}\n\n", section.title));
        
        for item in &section.items {
            if let Some(ref heading) = item.heading {
                if let Some(ref subheading) = item.subheading {
                    md.push_str(&format!("### {} | {}\n\n", heading, subheading));
                } else {
                    md.push_str(&format!("### {}\n\n", heading));
                }
            } else if let Some(ref subheading) = item.subheading {
                md.push_str(&format!("### {}\n\n", subheading));
            }

            if let Some(ref date) = item.date {
                md.push_str(&format!("*{}*\n\n", date));
            }

            if let Some(ref body) = item.body {
                md.push_str(&format!("{}\n\n", body));
            }

            for bullet in &item.bullets {
                md.push_str(&format!("- {}\n", bullet));
            }

            if !item.bullets.is_empty() {
                md.push('\n');
            }
        }

        md.push('\n');
    }

    md
}

fn section_title<'a>(en: &'a str, zh: &'a str, lang: Language) -> &'a str {
    match lang {
        Language::En => en,
        Language::Zh => zh,
    }
}

fn present_text(lang: Language) -> &'static str {
    match lang {
        Language::En => "Present",
        Language::Zh => "至今",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cr_core::resume::{CustomItem, CustomSection, Language, PersonalInfo, SkillGroup, Resume};
    use std::collections::HashMap;

    fn create_minimal_resume() -> Resume {
        Resume {
            version: "1.0".to_string(),
            language: Language::En,
            personal: PersonalInfo {
                name: "Test User".to_string(),
                name_alt: None,
                email: "test@example.com".to_string(),
                phone: None,
                location: None,
                title: None,
                photo: None,
                website: None,
                linkedin: None,
                github: None,
                extra_links: HashMap::new(),
            },
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
        }
    }

    #[test]
    fn test_render_markdown_minimal_resume() {
        let resume = create_minimal_resume();
        let md = render_markdown(&resume);
        
        assert!(md.contains("Test User"));
        assert!(md.contains("test@example.com"));
    }

    #[test]
    fn test_render_markdown_with_title() {
        let mut resume = create_minimal_resume();
        resume.personal.title = Some("Senior Engineer".to_string());
        
        let md = render_markdown(&resume);
        assert!(md.contains("Senior Engineer"));
    }

    #[test]
    fn test_render_markdown_with_summary() {
        let mut resume = create_minimal_resume();
        resume.summary = Some("Experienced professional".to_string());
        
        let md = render_markdown(&resume);
        assert!(md.contains("Summary"));
        assert!(md.contains("Experienced professional"));
    }

    #[test]
    fn test_render_markdown_with_skills() {
        let mut resume = create_minimal_resume();
        resume.skills.groups.push(SkillGroup {
            category: "Languages".to_string(),
            skills: vec!["Rust".to_string(), "Go".to_string()],
        });
        
        let md = render_markdown(&resume);
        assert!(md.contains("Skills"));
        assert!(md.contains("Languages"));
        assert!(md.contains("Rust"));
        assert!(md.contains("Go"));
    }

    #[test]
    fn test_render_markdown_with_custom_sections() {
        let mut resume = create_minimal_resume();
        resume.custom_sections.push(CustomSection {
            title: "Awards".to_string(),
            items: vec![
                CustomItem {
                    heading: Some("Best Contribution".to_string()),
                    subheading: Some("2023".to_string()),
                    date: None,
                    body: Some("Awarded for exceptional work".to_string()),
                    bullets: vec![],
                },
            ],
        });
        
        let md = render_markdown(&resume);
        assert!(md.contains("Awards"));
        assert!(md.contains("Best Contribution"));
        assert!(md.contains("2023"));
        assert!(md.contains("Awarded for exceptional work"));
    }

    #[test]
    fn test_render_markdown_with_custom_section_bullets() {
        let mut resume = create_minimal_resume();
        resume.custom_sections.push(CustomSection {
            title: "Volunteering".to_string(),
            items: vec![
                CustomItem {
                    heading: Some("Open Source Mentor".to_string()),
                    subheading: None,
                    date: Some("2023-2024".to_string()),
                    body: None,
                    bullets: vec!["Mentored 5 junior developers".to_string()],
                },
            ],
        });
        
        let md = render_markdown(&resume);
        assert!(md.contains("Volunteering"));
        assert!(md.contains("Open Source Mentor"));
        assert!(md.contains("2023-2024"));
        assert!(md.contains("Mentored 5 junior developers"));
    }

    #[test]
    fn test_render_markdown_english_section_titles() {
        let _resume = create_minimal_resume();
        assert_eq!(section_title("Experience", "工作经历", Language::En), "Experience");
    }

    #[test]
    fn test_render_markdown_chinese_section_titles() {
        let _resume = create_minimal_resume();
        assert_eq!(section_title("Experience", "工作经历", Language::Zh), "工作经历");
    }

    #[test]
    fn test_render_markdown_english_present_text() {
        assert_eq!(present_text(Language::En), "Present");
    }

    #[test]
    fn test_render_markdown_chinese_present_text() {
        assert_eq!(present_text(Language::Zh), "至今");
    }

    #[test]
    fn test_render_markdown_with_contacts() {
        let mut resume = create_minimal_resume();
        resume.personal.phone = Some("+1-555-0100".to_string());
        resume.personal.location = Some("San Francisco, CA".to_string());
        resume.personal.github = Some("https://github.com/user".to_string());
        resume.personal.linkedin = Some("https://linkedin.com/in/user".to_string());
        
        let md = render_markdown(&resume);
        assert!(md.contains("+1-555-0100"));
        assert!(md.contains("San Francisco, CA"));
        assert!(md.contains("GitHub"));
        assert!(md.contains("LinkedIn"));
    }
}
