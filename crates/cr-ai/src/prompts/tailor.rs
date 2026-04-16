use crate::provider::{ChatMessage, Role};
use cr_core::jd::JobDescription;
use cr_core::resume::Resume;

pub fn build_tailor_prompt(resume: &Resume, jd: &JobDescription) -> Vec<ChatMessage> {
    let system = r#"You are a resume optimization expert. Given a resume and a job description,
analyze keyword gaps and rewrite the resume to maximize ATS match rate while
maintaining honesty.

Output JSON:
{
  "match_score": 0-100,
  "matched_keywords": [...],
  "missing_keywords": [...],
  "rewrite_suggestions": [
    {
      "section": "experience[0].highlights[2]",
      "original": "...",
      "suggested": "...",
      "rationale": "..."
    }
  ]
}"#
    .to_string();

    let user = format!(
        "## Resume\n```yaml\n{}\n```\n\n## Job Description\n```\n{}\n```",
        serde_yaml::to_string(resume).unwrap_or_default(),
        jd.raw_text
    );

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
