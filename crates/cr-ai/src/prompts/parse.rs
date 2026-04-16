use crate::provider::{ChatMessage, Role};

pub fn build_parse_prompt(pdf_text: &str) -> Vec<ChatMessage> {
    let system = r#"Extract structured resume data from the following text (extracted from a PDF).
The text may have formatting artifacts. Do your best to identify:
- Personal info (name, email, phone, links)
- Work experience (company, title, dates, bullet points)
- Education
- Skills
- Projects

Output a JSON object matching this schema:
{
  "version": "1.0",
  "language": "en",
  "personal": {"name": "...", "email": "...", "phone": "...", "github": "...", "linkedin": "...", "title": "..."},
  "summary": "...",
  "experience": [{"company": "...", "title": "...", "start_date": {"year": 2020, "month": 1}, "end_date": null, "highlights": ["..."], "technologies": ["..."]}],
  "education": [{"institution": "...", "degree": "...", "start_date": 2016, "end_date": 2020}],
  "skills": {"groups": [{"category": "...", "skills": ["..."]}]},
  "projects": [{"name": "...", "description": "...", "highlights": ["..."]}]
}
Use null for fields you cannot determine. For dates, use {"year": YYYY, "month": MM} format."#
        .to_string();

    let user = format!("Here is the extracted PDF text:\n\n{}", pdf_text);

    vec![
        ChatMessage {
            role: Role::System,
            content: system,
        },
        ChatMessage {
            role: Role::User,
            content: user,
        },
    ]
}
