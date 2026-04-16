use crate::provider::{ChatMessage, Role};
use cr_core::resume::{Language, Resume};

pub fn build_rewrite_prompt(resume: &Resume) -> Vec<ChatMessage> {
    let lang_instruction = match resume.language {
        Language::En => "Respond in English.",
        Language::Zh => "请用中文回复。",
    };

    let system = format!(
        r#"You are an expert technical resume writer. Your task is to improve resume
bullet points for a software engineer. Follow these rules:
1. Start each bullet with a strong action verb
2. Quantify impact wherever possible (%, $, users, latency)
3. Use the XYZ formula: "Accomplished [X] as measured by [Y], by doing [Z]"
4. Keep each bullet to 1-2 lines
5. Preserve technical accuracy — never invent metrics the candidate didn't provide
6. {lang_instruction}

Output format: JSON object matching this schema:
{{
  "experience": [
    {{
      "company": "...",
      "highlights": ["improved bullet 1", "improved bullet 2"]
    }}
  ],
  "summary": "improved summary"
}}"#
    );

    let user = format!(
        "Here is the resume data in YAML:\n```yaml\n{}\n```\nRewrite all bullet points and the summary.",
        serde_yaml::to_string(resume).unwrap_or_default()
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
