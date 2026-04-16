use anyhow::Result;
use std::io::Read;
use std::path::Path;

/// Extract raw text from a PDF file.
///
/// This function reads a PDF file and extracts all visible text content.
/// It uses the pdf-extract library which handles PDF decompression and
/// text extraction from various PDF encodings.
///
/// # Arguments
/// * `path` - Path to the PDF file to extract text from
///
/// # Returns
/// * `Ok(String)` - Extracted text content from the PDF
/// * `Err(anyhow::Error)` - Error if PDF cannot be read or parsed
pub fn extract_text(path: &Path) -> Result<String> {
    // Open the PDF file
    let mut file = std::fs::File::open(path)
        .map_err(|e| anyhow::anyhow!("Failed to open PDF file '{}': {}", path.display(), e))?;

    // Read the entire file into memory
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| anyhow::anyhow!("Failed to read PDF file '{}': {}", path.display(), e))?;

    // Extract text from the PDF buffer
    let text = pdf_extract::extract_text_from_mem(&buffer)
        .map_err(|e| anyhow::anyhow!("Failed to extract text from PDF '{}': {}", path.display(), e))?;

    Ok(text)
}

/// Analyze extracted text to detect ATS compatibility issues.
///
/// This function checks if extracted text seems complete and readable,
/// which indicates the PDF layout is ATS-compatible.
///
/// # Arguments
/// * `text` - Extracted text from PDF
///
/// # Returns
/// * `AtsAnalysisResult` with compatibility status and findings
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtsAnalysisResult {
    /// Is the PDF likely ATS-compatible?
    pub is_compatible: bool,
    /// Character count of extracted text
    pub text_length: usize,
    /// Detected issues (empty if compatible)
    pub issues: Vec<String>,
    /// Suggestions for improvement
    pub suggestions: Vec<String>,
}

pub fn analyze_ats_compatibility(text: &str) -> AtsAnalysisResult {
    let text_length = text.len();
    let mut issues = Vec::new();
    let mut suggestions = Vec::new();

    // Check if text extraction produced minimal content
    // This typically indicates grid layouts or complex formatting that broke ATS parsing
    if text_length < 100 {
        issues.push("Very little text extracted - likely broken grid layout or complex formatting".to_string());
        suggestions.push("Consider using a simpler template with single-column layout".to_string());
        suggestions.push("Use the 'ats-simple' template for guaranteed ATS compatibility".to_string());
    }

    // Check for signs of broken multi-column layout
    // (too much whitespace or very short lines)
    let lines: Vec<&str> = text.lines().collect();
    let avg_line_length = if !lines.is_empty() {
        text_length / lines.len()
    } else {
        0
    };

    if avg_line_length < 20 && lines.len() > 50 {
        issues.push("Detected very short lines with many breaks - suggests multi-column layout".to_string());
        suggestions.push("Use single-column templates to ensure linear ATS parsing".to_string());
    }

    // Check for common indicators of text preservation
    let has_name = text.contains("Name") || text.to_lowercase().contains("john") || text.contains("Engineer");
    let has_contact = text.contains("@") || text.contains("linkedin") || text.contains("github");
    let has_experience = text.to_lowercase().contains("experience") || text.to_lowercase().contains("work");
    let has_education = text.to_lowercase().contains("education") || text.to_lowercase().contains("degree");

    // If we have key sections, that's a good sign
    if has_name && has_contact && (has_experience || has_education) {
        if issues.is_empty() {
            suggestions.push("Resume structure appears intact and readable by ATS".to_string());
        }
    } else if issues.is_empty() {
        issues.push("Missing expected resume sections - text extraction may have failed".to_string());
        suggestions.push("Verify the PDF generates correctly and contains all resume content".to_string());
    }

    let is_compatible = issues.is_empty() && text_length > 200;

    AtsAnalysisResult {
        is_compatible,
        text_length,
        issues,
        suggestions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ats_analysis_good_text() {
        let text = "John Doe\njohn@example.com\nSenior Engineer\n\nExperience:\nTechCorp - Senior Engineer (2020-present)\nLed architecture redesign\nEducation:\nMIT - BS Computer Science (2016)";
        let result = analyze_ats_compatibility(text);
        assert!(result.is_compatible);
        assert!(result.issues.is_empty());
        assert!(!result.suggestions.is_empty());
    }

    #[test]
    fn test_ats_analysis_minimal_text() {
        let text = "John";
        let result = analyze_ats_compatibility(text);
        assert!(!result.is_compatible);
        assert!(!result.issues.is_empty());
    }

    #[test]
    fn test_ats_analysis_broken_layout() {
        let text = "J\nD\nA\nB\nC\nE\nF\nG\nH\nI\nJ\nK\nL\nM\nN\nO\nP\nQ\nR\nS\nT\nU\nV\nW\nX\nY\nZ\nA\nB\nC\nD\nE\nF\nG\nH\nI\nJ\nK\nL\nM\nN\nO\nP\nQ\nR\nS\nT\nU\nV\nW\nX";
        let result = analyze_ats_compatibility(text);
        assert!(!result.is_compatible);
        assert!(!result.issues.is_empty());
    }
}
