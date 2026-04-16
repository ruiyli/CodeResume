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
