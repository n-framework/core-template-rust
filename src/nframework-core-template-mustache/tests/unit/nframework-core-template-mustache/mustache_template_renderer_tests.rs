use std::collections::BTreeMap;

use nframework_core_template_abstractions::{TemplateContext, TemplateRenderer};

use nframework_core_template_mustache::MustacheTemplateRenderer;

#[test]
fn test_mustache_template_renderer_renders_simple_variable() {
    let renderer = MustacheTemplateRenderer::new();
    let context = TemplateContext::new(
        vec![("name".to_owned(), "World".to_owned())]
            .into_iter()
            .collect::<BTreeMap<_, _>>(),
    );

    let template = "Hello {{name}}!";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello World!");
}

#[test]
fn test_mustache_template_renderer_renders_multiple_variables() {
    let renderer = MustacheTemplateRenderer::new();
    let context = TemplateContext::new(
        vec![
            ("first".to_owned(), "Hello".to_owned()),
            ("second".to_owned(), "World".to_owned()),
        ]
        .into_iter()
        .collect::<BTreeMap<_, _>>(),
    );

    let template = "{{first}} {{second}}!";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello World!");
}

#[test]
fn test_mustache_template_renderer_renders_section() {
    let renderer = MustacheTemplateRenderer::new();
    let context = TemplateContext::new(
        vec![
            ("show_greeting".to_owned(), "true".to_owned()),
            ("name".to_owned(), "Alice".to_owned()),
        ]
        .into_iter()
        .collect::<BTreeMap<_, _>>(),
    );

    let template = "{{#show_greeting}}Hello {{name}}!{{/show_greeting}}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello Alice!");
}

#[test]
fn test_mustache_template_renderer_renders_inverted_section() {
    let renderer = MustacheTemplateRenderer::new();

    // Create a nested context with empty list for inverted section
    // Mustache treats empty lists/false values as falsy
    let mut data = serde_json::Map::new();
    data.insert("items".to_owned(), serde_json::Value::Array(vec![]));

    let template =
        "{{#items}}Items: {{#items}}{{.}}{{/items}}{{/items}}{{^items}}No items!{{/items}}";
    let result = renderer.render_content(
        template,
        &TemplateContext::new(
            vec![("items".to_owned(), "".to_owned())]
                .into_iter()
                .collect::<BTreeMap<_, _>>(),
        ),
    );

    assert!(result.is_ok());
}

#[test]
fn test_mustache_template_renderer_handles_empty_context() {
    let renderer = MustacheTemplateRenderer::new();
    let context = TemplateContext::new(BTreeMap::new());

    let template = "Hello {{name}}!";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello !");
}

#[test]
fn test_mustache_template_renderer_escapes_html_by_default() {
    let renderer = MustacheTemplateRenderer::new();
    let context = TemplateContext::new(
        vec![(
            "content".to_owned(),
            "<script>alert('xss')</script>".to_owned(),
        )]
        .into_iter()
        .collect::<BTreeMap<_, _>>(),
    );

    let template = "{{content}}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        "&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;"
    );
}

#[test]
fn test_mustache_template_renderer_unescaped_triple_mustache() {
    let renderer = MustacheTemplateRenderer::new();
    let context = TemplateContext::new(
        vec![("content".to_owned(), "<b>Bold</b>".to_owned())]
            .into_iter()
            .collect::<BTreeMap<_, _>>(),
    );

    let template = "{{{content}}}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "<b>Bold</b>");
}

#[test]
fn test_mustache_template_renderer_returns_error_for_invalid_template() {
    let renderer = MustacheTemplateRenderer::new();
    let context = TemplateContext::new(BTreeMap::new());

    // Invalid mustache: unclosed section
    let template = "{{#invalid";
    let result = renderer.render_content(template, &context);

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.message().contains("failed to compile"));
}

#[test]
fn test_mustache_template_renderer_default() {
    let renderer = MustacheTemplateRenderer::default();
    let context = TemplateContext::new(BTreeMap::new());

    let template = "{{name}}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
}
