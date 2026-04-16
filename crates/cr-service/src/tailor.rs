use crate::context::ServiceContext;
use cr_ai::prompts::tailor as tailor_prompts;
use cr_ai::provider::ChatRequest;
use cr_ai::response;
use cr_core::jd::{JobDescription, MatchResult};
use cr_core::resume::Resume;

pub async fn run(
    ctx: &ServiceContext,
    resume: &Resume,
    jd: &JobDescription,
) -> anyhow::Result<(MatchResult, Resume)> {
    let messages = tailor_prompts::build_tailor_prompt(resume, jd);
    let request = ChatRequest {
        messages,
        max_tokens: ctx.config.ai.max_tokens,
        temperature: ctx.config.ai.temperature,
        json_mode: true,
    };

    let resp = ctx.ai.chat(request).await?;
    let match_result = response::parse_match_result(&resp.content)?;

    // Apply suggestions to create tailored resume
    let mut tailored = resume.clone();
    for suggestion in &match_result.rewrite_suggestions {
        // Simple application: find matching highlight and replace
        for exp in &mut tailored.experience {
            for highlight in &mut exp.highlights {
                if *highlight == suggestion.original {
                    *highlight = suggestion.suggested.clone();
                }
            }
        }
    }

    Ok((match_result, tailored))
}
