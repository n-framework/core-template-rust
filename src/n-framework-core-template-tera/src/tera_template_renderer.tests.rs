//! Tests for Tera template renderer with custom string case filters.

use super::*;
use n_framework_core_template_abstractions::TemplateContext;

#[test]
fn test_slugify_filter_basic() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "HelloWorld");

    let template = "{{ input | slugify }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "hello-world");
}

#[test]
fn test_slugify_filter_with_spaces() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "Hello World");

    let template = "{{ input | slugify }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "hello-world");
}

#[test]
fn test_slugify_filter_with_underscores() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "hello_world");

    let template = "{{ input | slugify }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "hello-world");
}

#[test]
fn test_slugify_filter_empty_string() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "");

    let template = "{{ input | slugify }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
}

#[test]
fn test_slugify_filter_only_separators() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "---");

    let template = "{{ input | slugify }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
}

#[test]
fn test_slugify_filter_mixed_separators() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "hello_-_world");

    let template = "{{ input | slugify }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "hello-world");
}

#[test]
fn test_slugify_filter_camel_case() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "helloWorld");

    let template = "{{ input | slugify }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "hello-world");
}

#[test]
fn test_slugify_filter_with_number() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "123");

    let template = "{{ input | slugify }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "123");
}

#[test]
fn test_pascal_case_filter_basic() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "hello_world");

    let template = "{{ input | pascal_case }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "HelloWorld");
}

#[test]
fn test_pascal_case_filter_with_spaces() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "hello world");

    let template = "{{ input | pascal_case }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "HelloWorld");
}

#[test]
fn test_pascal_case_filter_empty_string() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "");

    let template = "{{ input | pascal_case }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
}

#[test]
fn test_pascal_case_filter_single_word() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "hello");

    let template = "{{ input | pascal_case }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello");
}

#[test]
fn test_pascal_case_filter_mixed_separators() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "hello_-_world");

    let template = "{{ input | pascal_case }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "HelloWorld");
}

#[test]
fn test_snake_case_filter_basic() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "HelloWorld");

    let template = "{{ input | snake_case }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "hello_world");
}

#[test]
fn test_snake_case_filter_with_hyphens() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "hello-world");

    let template = "{{ input | snake_case }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "hello_world");
}

#[test]
fn test_snake_case_filter_empty_string() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "");

    let template = "{{ input | snake_case }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
}

#[test]
fn test_snake_case_filter_only_separators() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "___");

    let template = "{{ input | snake_case }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
}

#[test]
fn test_snake_case_filter_camel_case() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "helloWorld");

    let template = "{{ input | snake_case }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "hello_world");
}

#[test]
fn test_sentence_case_filter_basic() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "hello_world");

    let template = "{{ input | sentence_case }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello world");
}

#[test]
fn test_sentence_case_filter_all_caps() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "HELLO_WORLD");

    let template = "{{ input | sentence_case }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello world");
}

#[test]
fn test_sentence_case_filter_empty_string() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "");

    let template = "{{ input | sentence_case }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
}

#[test]
fn test_sentence_case_filter_single_word() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "hello");

    let template = "{{ input | sentence_case }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello");
}

#[test]
fn test_multiple_filters_chained() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "Hello World");

    let template = "{{ input | slugify | upper }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "HELLO-WORLD");
}

#[test]
fn test_filter_with_template_variables() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("first_name", "John");
    context.insert("last_name", "Doe");

    let template = "{{ first_name | slugify }}-{{ last_name | slugify }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "john-doe");
}

#[test]
fn test_unicode_with_accented_characters() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "CaféAuLait");

    let template = "{{ input | slugify }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "café-au-lait");
}

#[test]
fn test_already_formatted_input_idempotent() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "hello-world");

    let template = "{{ input | slugify }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "hello-world");
}

#[test]
fn test_string_with_numbers() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "hello123world");

    let template = "{{ input | slugify }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "hello123world");
}

#[test]
fn test_special_characters() {
    let renderer = TeraTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("input", "hello@world#test");

    let template = "{{ input | slugify }}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "hello@world#test");
}

// Unit tests for helper functions
#[test]
fn test_to_kebab_case_directly() {
    assert_eq!(to_kebab_case("HelloWorld"), "hello-world");
    assert_eq!(to_kebab_case("hello_world"), "hello-world");
    assert_eq!(to_kebab_case(""), "");
    assert_eq!(to_kebab_case("---"), "");
    assert_eq!(to_kebab_case("hello_-_world"), "hello-world");
    assert_eq!(to_kebab_case("helloWorld"), "hello-world");
}

#[test]
fn test_to_pascal_case_directly() {
    assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
    assert_eq!(to_pascal_case("hello world"), "HelloWorld");
    assert_eq!(to_pascal_case(""), "");
    assert_eq!(to_pascal_case("hello"), "Hello");
    assert_eq!(to_pascal_case("hello_-_world"), "HelloWorld");
}

#[test]
fn test_to_snake_case_directly() {
    assert_eq!(to_snake_case("HelloWorld"), "hello_world");
    assert_eq!(to_snake_case("hello-world"), "hello_world");
    assert_eq!(to_snake_case(""), "");
    assert_eq!(to_snake_case("___"), "");
    assert_eq!(to_snake_case("helloWorld"), "hello_world");
}

#[test]
fn test_to_sentence_case_directly() {
    assert_eq!(to_sentence_case("hello_world"), "Hello world");
    assert_eq!(to_sentence_case("HELLO_WORLD"), "Hello world");
    assert_eq!(to_sentence_case(""), "");
    assert_eq!(to_sentence_case("hello"), "Hello");
    assert_eq!(to_sentence_case("hello_-_world"), "Hello world");
}

#[test]
fn test_to_case_with_separator_edge_cases() {
    assert_eq!(to_case_with_separator("", '-'), "");
    assert_eq!(to_case_with_separator("---", '-'), "");
    assert_eq!(to_case_with_separator("___", '_'), "");
    assert_eq!(to_case_with_separator("   ", ' '), "");
    assert_eq!(to_case_with_separator("a", '-'), "a");
    assert_eq!(to_case_with_separator("A", '-'), "a");
}
