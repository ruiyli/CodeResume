use cr_config::AppConfig;
use cr_core::template::TemplateId;
use cr_render::template_registry::TemplateRegistry;
use std::path::PathBuf;

fn templates_dir() -> PathBuf {
    // Look for templates relative to executable, or in known locations
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()));

    // Try: ./templates, ../templates, exe_dir/templates
    let candidates = vec![
        PathBuf::from("templates"),
        PathBuf::from("../templates"),
        exe_dir
            .clone()
            .map(|d| d.join("templates"))
            .unwrap_or_default(),
        exe_dir.map(|d| d.join("../templates")).unwrap_or_default(),
    ];

    for path in candidates {
        if path.exists() {
            return path;
        }
    }

    PathBuf::from("templates")
}

pub fn run(config: &AppConfig) -> anyhow::Result<()> {
    let base = templates_dir();
    let registry = TemplateRegistry::discover(&base)?;

    let is_zh = config.output.language == "zh";

    println!("Available Templates");
    println!("===================\n");

    for meta in registry.list() {
        let name = if is_zh { &meta.name_zh } else { &meta.name };
        let desc = if is_zh {
            &meta.description_zh
        } else {
            &meta.description
        };
        let default_marker = if meta.id == TemplateId::Modern {
            " (default)"
        } else {
            ""
        };

        println!("  {} {}{}", meta.id.dir_name(), name, default_marker);
        println!("    {}", desc);
        if let Some(hint) = meta.page_limit_hint {
            println!("    Pages: up to {}", hint);
        }
        let langs: Vec<&str> = meta
            .supports_languages
            .iter()
            .map(|l| match l {
                cr_core::resume::Language::En => "English",
                cr_core::resume::Language::Zh => "中文",
            })
            .collect();
        println!("    Languages: {}", langs.join(", "));
        println!();
    }

    Ok(())
}
