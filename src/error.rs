use crate::span::Span;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ObstructError {
    pub span: Span,
    pub message: String,
    pub file: Option<String>,
}

impl ObstructError {
    pub fn new(line: usize, column: usize, message: impl Into<String>, file: Option<String>) -> Self {
        Self {
            span: Span { line, column },
            message: message.into(),
            file,
        }
    }

    pub fn file_not_found(path: String) -> Self {
        Self::new(0, 0, &path, Some(path.clone()))
    }
}

impl fmt::Display for ObstructError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[line {} column {}] Error: {}",
            self.span.line, self.span.column, self.message
        )
    }
}

impl std::error::Error for ObstructError {}
