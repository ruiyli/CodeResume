use crate::provider::{ChatMessage, Role};
use cr_core::resume::Experience;

pub fn build_expand_prompt(raw_notes: &str, context: &Experience) -> Vec<ChatMessage> {
    let system = r#"You are a technical resume writer. The user has provided rough notes about
their work at a company. Expand these into 3-5 professional resume bullet points.
Each bullet should:
- Start with a past-tense action verb
- Include technical specifics (technologies, scale, approach)
- Quantify impact where inferable

Output JSON: { "highlights": ["bullet1", "bullet2", ...] }"#
        .to_string();

    let user = format!(
        "Company: {}\nTitle: {}\nRaw notes:\n{}",
        context.company, context.title, raw_notes
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
