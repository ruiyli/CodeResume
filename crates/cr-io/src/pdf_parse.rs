use anyhow::Result;
use std::path::Path;

/// Extract raw text from a PDF file.
pub fn extract_text(path: &Path) -> Result<String> {
    // TODO: Implement PDF text extraction using pdf-extract crate
    // For now, return an error indicating the feature is not yet implemented
    anyhow::bail!(
        "PDF text extraction not yet implemented. File: {}",
        path.display()
    )
}
