use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodeResumeError {
    #[error("Resume data file not found: {path}")]
    FileNotFound { path: String },

    #[error("Invalid resume data format: {message}")]
    InvalidFormat { message: String },

    #[error("Template not found: {id}")]
    TemplateNotFound { id: String },

    #[error("AI provider error: {message}")]
    AiProviderError { message: String },

    #[error("PDF parsing error: {message}")]
    PdfParseError { message: String },

    #[error("Render error: {message}")]
    RenderError { message: String },

    #[error("Configuration error: {message}")]
    ConfigError { message: String },
}
