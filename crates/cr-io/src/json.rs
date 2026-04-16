use anyhow::Result;
use cr_core::resume::Resume;
use std::path::Path;

pub fn read(path: &Path) -> Result<Resume> {
    let contents = std::fs::read_to_string(path)?;
    let resume: Resume = serde_json::from_str(&contents)?;
    Ok(resume)
}

pub fn write(resume: &Resume, path: &Path) -> Result<()> {
    let contents = serde_json::to_string_pretty(resume)?;
    std::fs::write(path, contents)?;
    Ok(())
}
