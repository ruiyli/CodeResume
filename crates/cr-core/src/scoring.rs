use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreReport {
    pub overall_score: u8,
    pub dimensions: Vec<ScoreDimension>,
    pub suggestions: Vec<Suggestion>,
    pub strengths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreDimension {
    pub name: String,
    pub score: u8,
    pub feedback: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    pub severity: Severity,
    pub section: String,
    pub message: String,
    pub example_fix: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Critical,
    Warning,
    Info,
}
