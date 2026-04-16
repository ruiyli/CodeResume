use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobDescription {
    pub raw_text: String,
    pub company: Option<String>,
    pub title: Option<String>,
    /// AI-extracted structured data
    pub parsed: Option<ParsedJD>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedJD {
    pub required_skills: Vec<String>,
    pub preferred_skills: Vec<String>,
    pub responsibilities: Vec<String>,
    pub experience_years: Option<String>,
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    pub match_score: u8,
    pub matched_keywords: Vec<String>,
    pub missing_keywords: Vec<String>,
    pub rewrite_suggestions: Vec<RewriteSuggestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewriteSuggestion {
    pub section: String,
    pub original: String,
    pub suggested: String,
    pub rationale: String,
}
