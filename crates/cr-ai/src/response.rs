use cr_core::jd::MatchResult;
use cr_core::resume::Resume;
use cr_core::scoring::ScoreReport;

/// Extract JSON block from AI response (handles markdown fences)
pub fn extract_json_block(text: &str) -> String {
    // Try ```json ... ``` first
    if let Some(start) = text.find("```json") {
        let after = &text[start + 7..];
        if let Some(end) = after.find("```") {
            return after[..end].trim().to_string();
        }
    }
    // Try ``` ... ``` (without json tag)
    if let Some(start) = text.find("```") {
        let after = &text[start + 3..];
        if let Some(end) = after.find("```") {
            let block = after[..end].trim();
            if block.starts_with('{') || block.starts_with('[') {
                return block.to_string();
            }
        }
    }
    // Fallback: find outermost braces
    if let (Some(start), Some(end)) = (text.find('{'), text.rfind('}')) {
        return text[start..=end].to_string();
    }
    text.to_string()
}

pub fn parse_score_report(raw: &str) -> anyhow::Result<ScoreReport> {
    let cleaned = extract_json_block(raw);
    serde_json::from_str(&cleaned)
        .map_err(|e| anyhow::anyhow!("Failed to parse AI score report: {e}\nRaw response:\n{raw}"))
}

pub fn parse_match_result(raw: &str) -> anyhow::Result<MatchResult> {
    let cleaned = extract_json_block(raw);
    serde_json::from_str(&cleaned)
        .map_err(|e| anyhow::anyhow!("Failed to parse AI match result: {e}\nRaw response:\n{raw}"))
}

pub fn parse_resume(raw: &str) -> anyhow::Result<Resume> {
    let cleaned = extract_json_block(raw);
    serde_json::from_str(&cleaned).map_err(|e| {
        anyhow::anyhow!("Failed to parse AI resume extraction: {e}\nRaw response:\n{raw}")
    })
}

/// Parse a rewrite response and merge into the original resume.
pub fn parse_and_apply_rewrite(raw: &str, original: &Resume) -> anyhow::Result<Resume> {
    let cleaned = extract_json_block(raw);
    let rewrite: serde_json::Value = serde_json::from_str(&cleaned)
        .map_err(|e| anyhow::anyhow!("Failed to parse AI rewrite: {e}\nRaw:\n{raw}"))?;

    let mut resume = original.clone();

    // Apply summary if present
    if let Some(summary) = rewrite.get("summary").and_then(|v| v.as_str()) {
        resume.summary = Some(summary.to_string());
    }

    // Apply experience highlights
    if let Some(experiences) = rewrite.get("experience").and_then(|v| v.as_array()) {
        for ai_exp in experiences {
            let company = ai_exp.get("company").and_then(|v| v.as_str()).unwrap_or("");
            if let Some(target) = resume.experience.iter_mut().find(|e| e.company == company) {
                if let Some(highlights) = ai_exp.get("highlights").and_then(|v| v.as_array()) {
                    target.highlights = highlights
                        .iter()
                        .filter_map(|h| h.as_str().map(String::from))
                        .collect();
                }
            }
        }
    }

    Ok(resume)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_json_from_markdown_fence() {
        let raw = "Here's my analysis:\n```json\n{\"score\":75}\n```\nDone.";
        assert_eq!(extract_json_block(raw), "{\"score\":75}");
    }

    #[test]
    fn extract_json_from_bare_braces() {
        let raw = "Result: {\"score\":80} extra text";
        assert_eq!(extract_json_block(raw), "{\"score\":80}");
    }

    #[test]
    fn parse_score_report_success() {
        let json = r#"{"overall_score":75,"dimensions":[],"suggestions":[],"strengths":["Good"]}"#;
        let report = parse_score_report(json).unwrap();
        assert_eq!(report.overall_score, 75);
    }
}
