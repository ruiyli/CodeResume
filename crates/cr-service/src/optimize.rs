use crate::context::ServiceContext;
use cr_ai::prompts::rewrite;
use cr_ai::provider::ChatRequest;
use cr_ai::response;
use cr_core::resume::Resume;

pub async fn run(ctx: &ServiceContext, resume: &Resume) -> anyhow::Result<Resume> {
    let messages = rewrite::build_rewrite_prompt(resume);
    let request = ChatRequest {
        messages,
        max_tokens: ctx.config.ai.max_tokens,
        temperature: ctx.config.ai.temperature,
        json_mode: true,
    };

    let resp = ctx.ai.chat(request).await?;
    let optimized = response::parse_and_apply_rewrite(&resp.content, resume)?;
    Ok(optimized)
}
