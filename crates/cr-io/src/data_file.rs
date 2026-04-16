use anyhow::{Context, Result};
use cr_core::resume::Resume;
use std::path::Path;

/// Auto-detect file format and load resume data.
pub fn load(path: &Path) -> Result<Resume> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "yaml" | "yml" => crate::yaml::read(path),
        "json" => crate::json::read(path),
        _ => {
            // Try YAML first, then JSON
            crate::yaml::read(path)
                .or_else(|_| crate::json::read(path))
                .context(format!(
                    "Failed to parse {} as YAML or JSON",
                    path.display()
                ))
        }
    }
}

/// Auto-detect format and save resume data.
pub fn save(resume: &Resume, path: &Path) -> Result<()> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "json" => crate::json::write(resume, path),
        _ => crate::yaml::write(resume, path), // Default to YAML
    }
}

/// Load raw text from a file (for JD text files).
pub fn load_text(path: &Path) -> Result<String> {
    std::fs::read_to_string(path).context(format!("Failed to read {}", path.display()))
}
