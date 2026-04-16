use cr_ai::provider::AiProvider;
use cr_config::AppConfig;
use cr_render::engine::RenderEngine;
use cr_render::template_registry::TemplateRegistry;
use std::path::PathBuf;

pub struct ServiceContext {
    pub config: AppConfig,
    pub ai: Box<dyn AiProvider>,
    pub render_engine: RenderEngine,
    pub template_registry: TemplateRegistry,
}

impl ServiceContext {
    pub async fn from_config(config: AppConfig, template_base: PathBuf) -> anyhow::Result<Self> {
        let ai = cr_ai::provider::build_provider(&config.ai)?;
        let render_engine =
            RenderEngine::new(config.output.typst_bin.clone(), template_base.clone());
        let template_registry = TemplateRegistry::discover(&template_base)?;

        Ok(Self {
            config,
            ai,
            render_engine,
            template_registry,
        })
    }
}
