use std::error::Error;
use std::fmt::{Display, Formatter};

/// Error variants for template operations.
#[derive(Debug)]
pub enum TemplateErrorKind {
    /// Failed to parse a template.
    Parse(String),
    /// Failed to render a template.
    Render(String),
    /// I/O error (file not found, permission denied, etc.).
    Io(String),
    /// Validation error (invalid path, missing required field, etc.).
    Validation(String),
    /// Path traversal attempt detected.
    Security(String),
    /// Unknown error.
    Other(String),
}

/// Error type for template rendering and file generation operations.
///
/// # Example
/// ```
/// use n_framework_core_template_abstractions::{TemplateError, TemplateErrorKind};
///
/// let error = TemplateError::new(TemplateErrorKind::Parse("invalid template".to_string()));
/// assert!(error.message().contains("invalid template"));
/// ```
#[derive(Debug)]
pub struct TemplateError {
    kind: TemplateErrorKind,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl Clone for TemplateError {
    fn clone(&self) -> Self {
        Self {
            kind: match &self.kind {
                TemplateErrorKind::Parse(s) => TemplateErrorKind::Parse(s.clone()),
                TemplateErrorKind::Render(s) => TemplateErrorKind::Render(s.clone()),
                TemplateErrorKind::Io(s) => TemplateErrorKind::Io(s.clone()),
                TemplateErrorKind::Validation(s) => TemplateErrorKind::Validation(s.clone()),
                TemplateErrorKind::Security(s) => TemplateErrorKind::Security(s.clone()),
                TemplateErrorKind::Other(s) => TemplateErrorKind::Other(s.clone()),
            },
            source: None,
        }
    }
}

impl TemplateError {
    /// Creates a new TemplateError with the given kind.
    pub fn new(kind: TemplateErrorKind) -> Self {
        Self { kind, source: None }
    }

    /// Creates a new TemplateError with a simple message (wraps in Other).
    pub fn msg(message: impl Into<String>) -> Self {
        Self::new(TemplateErrorKind::Other(message.into()))
    }

    /// Creates a new TemplateError with an underlying source error.
    pub fn with_source<E>(kind: TemplateErrorKind, source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self {
            kind,
            source: Some(Box::new(source)),
        }
    }

    /// Returns the error message.
    pub fn message(&self) -> String {
        match &self.kind {
            TemplateErrorKind::Parse(s) => format!("parse error: {}", s),
            TemplateErrorKind::Render(s) => format!("render error: {}", s),
            TemplateErrorKind::Io(s) => format!("I/O error: {}", s),
            TemplateErrorKind::Validation(s) => format!("validation error: {}", s),
            TemplateErrorKind::Security(s) => format!("security error: {}", s),
            TemplateErrorKind::Other(s) => s.clone(),
        }
    }

    /// Returns the error kind.
    pub fn kind(&self) -> &TemplateErrorKind {
        &self.kind
    }

    /// Returns true if this is a security-related error.
    pub fn is_security(&self) -> bool {
        matches!(self.kind, TemplateErrorKind::Security(_))
    }

    /// Returns true if this is an I/O error.
    pub fn is_io(&self) -> bool {
        matches!(self.kind, TemplateErrorKind::Io(_))
    }

    /// Creates a parse error.
    pub fn parse(message: impl Into<String>) -> Self {
        Self::new(TemplateErrorKind::Parse(message.into()))
    }

    /// Creates a render error.
    pub fn render(message: impl Into<String>) -> Self {
        Self::new(TemplateErrorKind::Render(message.into()))
    }

    /// Creates an I/O error.
    pub fn io(message: impl Into<String>) -> Self {
        Self::new(TemplateErrorKind::Io(message.into()))
    }

    /// Creates a validation error.
    pub fn validation(message: impl Into<String>) -> Self {
        Self::new(TemplateErrorKind::Validation(message.into()))
    }

    /// Creates a security error.
    pub fn security(message: impl Into<String>) -> Self {
        Self::new(TemplateErrorKind::Security(message.into()))
    }
}

impl Display for TemplateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Error for TemplateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as _)
    }
}

#[cfg(test)]
#[path = "template_error.tests.rs"]
mod tests;
