use crate::provider::{ChatMessage, Role};
use cr_core::resume::Resume;

pub fn build_review_prompt(resume: &Resume) -> Vec<ChatMessage> {
    let system = r#"You are a senior tech recruiter and resume reviewer. Score this resume on
these dimensions (0-100 each):
1. Impact Quantification — are achievements measurable?
2. Technical Depth — does it demonstrate real expertise?
3. Clarity & Conciseness — easy to scan in 6 seconds?
4. ATS Compatibility — proper keywords, no tables/graphics references?
5. Overall Impression

For each dimension, give a score and 1-sentence feedback.
Then provide an ordered list of specific, actionable suggestions.

Output JSON matching this schema:
{
  "overall_score": 75,
  "dimensions": [{"name": "...", "score": 80, "feedback": "..."}],
  "suggestions": [{"severity": "warning", "section": "...", "message": "...", "example_fix": "..."}],
  "strengths": ["..."]
}"#
    .to_string();

    let user = format!(
        "Here is the resume data in YAML:\n```yaml\n{}\n```\nReview and score this resume.",
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
