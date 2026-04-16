use crate::context::ServiceContext;
use cr_core::resume::Resume;
use cr_core::template::{OutputFormat, RenderRequest, TemplateId};
use std::path::PathBuf;

pub fn run(
    ctx: &ServiceContext,
    resume: &Resume,
    template: TemplateId,
    output_path: PathBuf,
    formats: Vec<OutputFormat>,
) -> anyhow::Result<Vec<PathBuf>> {
    let request = RenderRequest {
        template,
        language: resume.language,
        output_path,
        formats,
    };
    ctx.render_engine.render(resume, &request)
}
