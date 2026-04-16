use crate::context::ServiceContext;
use cr_ai::prompts::review as review_prompts;
use cr_ai::provider::ChatRequest;
use cr_ai::response;
use cr_core::resume::Resume;
use cr_core::scoring::ScoreReport;

pub async fn run(ctx: &ServiceContext, resume: &Resume) -> anyhow::Result<ScoreReport> {
    let messages = review_prompts::build_review_prompt(resume);
    let request = ChatRequest {
        messages,
        max_tokens: ctx.config.ai.max_tokens,
        temperature: ctx.config.ai.temperature,
        json_mode: true,
    };

    let resp = ctx.ai.chat(request).await?;
    let report = response::parse_score_report(&resp.content)?;
    Ok(report)
}
