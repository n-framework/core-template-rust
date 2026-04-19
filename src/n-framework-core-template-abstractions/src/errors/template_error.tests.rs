use super::*;
use std::error::Error;

#[test]
fn test_template_error_new_parse() {
    let error = TemplateError::parse("invalid template syntax");
    assert!(error.message().contains("parse error"));
    assert!(error.message().contains("invalid template syntax"));
}

#[test]
fn test_template_error_new_render() {
    let error = TemplateError::render("template rendering failed");
    assert!(error.message().contains("render error"));
    assert!(error.message().contains("template rendering failed"));
}

#[test]
fn test_template_error_new_io() {
    let error = TemplateError::io("file not found");
    assert!(error.message().contains("I/O error"));
    assert!(error.is_io());
}

#[test]
fn test_template_error_new_validation() {
    let error = TemplateError::validation("invalid path");
    assert!(error.message().contains("validation error"));
}

#[test]
fn test_template_error_new_security() {
    let error = TemplateError::security("path traversal");
    assert!(error.message().contains("security error"));
    assert!(error.is_security());
}

#[test]
fn test_template_error_display() {
    let error = TemplateError::msg("error message");
    let display = format!("{}", error);
    assert_eq!(display, "error message");
}

#[test]
fn test_template_error_clone() {
    let error = TemplateError::msg("original error");
    let cloned = error.clone();
    assert_eq!(cloned.message(), "original error");
}

#[test]
fn test_template_error_debug() {
    let error = TemplateError::msg("test error");
    let debug = format!("{:?}", error);
    assert!(debug.contains("test error"));
}

#[test]
fn test_template_error_with_source() {
    let inner = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let error = TemplateError::with_source(
        TemplateErrorKind::Io("failed to read file".to_string()),
        inner,
    );
    assert!(error.message().contains("I/O error"));
    assert!(error.source().is_some());
}

#[test]
fn test_template_error_kind() {
    let error = TemplateError::parse("test");
    assert!(matches!(error.kind(), TemplateErrorKind::Parse(_)));
}
