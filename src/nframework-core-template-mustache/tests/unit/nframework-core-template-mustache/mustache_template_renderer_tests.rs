use std::sync::{Arc, Mutex};
use std::thread;

use nframework_core_template_abstractions::{TemplateContext, TemplateRenderer};

use nframework_core_template_mustache::MustacheTemplateRenderer;

#[test]
fn test_mustache_template_renderer_renders_simple_variable() {
    let renderer = MustacheTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("name", "World");

    let template = "Hello {{name}}!";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello World!");
}

#[test]
fn test_mustache_template_renderer_renders_multiple_variables() {
    let renderer = MustacheTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("first", "Hello");
    context.insert("second", "World");

    let template = "{{first}} {{second}}!";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello World!");
}

#[test]
fn test_mustache_template_renderer_renders_section() {
    let renderer = MustacheTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("show_greeting", "true");
    context.insert("name", "Alice");

    let template = "{{#show_greeting}}Hello {{name}}!{{/show_greeting}}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello Alice!");
}

#[test]
fn test_mustache_template_renderer_renders_inverted_section() {
    let renderer = MustacheTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("show_greeting", "true");

    let template = "{{^show_greeting}}No greeting{{/show_greeting}}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
}

#[test]
fn test_mustache_template_renderer_handles_empty_context() {
    let renderer = MustacheTemplateRenderer::new();
    let context = TemplateContext::empty();

    let template = "Hello {{name}}!";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello !");
}

#[test]
fn test_mustache_template_renderer_escapes_html_by_default() {
    let renderer = MustacheTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("content", "<script>alert('xss')</script>");

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
    let mut context = TemplateContext::empty();
    context.insert("content", "<b>Bold</b>");

    let template = "{{{content}}}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "<b>Bold</b>");
}

#[test]
fn test_mustache_template_renderer_returns_error_for_invalid_template() {
    let renderer = MustacheTemplateRenderer::new();
    let context = TemplateContext::empty();

    let template = "{{#invalid";
    let result = renderer.render_content(template, &context);

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.message().contains("parse error"));
}

#[test]
fn test_mustache_template_renderer_default() {
    let renderer = MustacheTemplateRenderer::default();
    let context = TemplateContext::empty();

    let template = "{{name}}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
}

// === Additional Mustache Feature Tests ===

#[test]
fn test_mustache_comments_are_removed() {
    let renderer = MustacheTemplateRenderer::new();
    let context = TemplateContext::empty();

    // Comments {{! comment }} should be removed from output
    let template = "Hello {{! This is a comment }}World!";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello World!");
}

#[test]
fn test_mustache_set_delimiter() {
    let renderer = MustacheTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("name", "World");

    // Change delimiters using {{= =}}
    let template = "{{= << >> =}}Hello <<name>>!";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello World!");
}

#[test]
fn test_mustache_nested_variables() {
    let renderer = MustacheTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert_value("person", serde_json::json!({"name": "Alice", "age": 30}));

    let template = "{{person.name}} is {{person.age}} years old";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Alice is 30 years old");
}

#[test]
fn test_mustache_array_iteration() {
    let renderer = MustacheTemplateRenderer::new();
    let context = TemplateContext::from_json(serde_json::json!({
        "items": ["one", "two", "three"]
    }))
    .unwrap();

    let template = "{{#items}}{{.}},{{/items}}";
    let result = renderer.render_content(template, &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "one,two,three,");
}

#[test]
fn test_template_caching() {
    let renderer = MustacheTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("name", "World");

    let template = "Hello {{name}}!";

    // First render - cache miss
    let result1 = renderer.render_content(template, &context);
    assert!(result1.is_ok());
    assert_eq!(renderer.cache_size().unwrap(), 1);

    // Second render - cache hit
    let result2 = renderer.render_content(template, &context);
    assert!(result2.is_ok());
    assert_eq!(renderer.cache_size().unwrap(), 1);
}

#[test]
fn test_template_cache_clear() {
    let renderer = MustacheTemplateRenderer::new();
    let mut context = TemplateContext::empty();
    context.insert("name", "World");

    // Render some templates
    renderer
        .render_content("Hello {{name}}!", &context)
        .unwrap();
    assert_eq!(renderer.cache_size().unwrap(), 1);

    // Clear cache
    renderer.clear_cache().unwrap();
    assert_eq!(renderer.cache_size().unwrap(), 0);
}

#[test]
fn test_thread_safety_concurrent_renders() {
    let renderer = Arc::new(MustacheTemplateRenderer::new());
    let mut context = TemplateContext::empty();
    context.insert("name", "World");

    let template = "Hello {{name}}!".to_string();
    let context_clone = context.clone();

    // Spawn multiple threads that render concurrently
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let renderer = Arc::clone(&renderer);
            let template = template.clone();
            let context = context_clone.clone();
            thread::spawn(move || {
                for _ in 0..100 {
                    let result = renderer.render_content(&template, &context);
                    assert!(result.is_ok());
                }
                Ok::<(), ()>(())
            })
        })
        .collect();

    for handle in handles {
        let _ = handle.join().unwrap();
    }
}

#[test]
fn test_thread_safety_shared_renderer() {
    let renderer = Arc::new(MustacheTemplateRenderer::new());
    let result = Arc::new(Mutex::new(String::new()));

    let template = "Hello {{name}}!".to_string();
    let mut context = TemplateContext::empty();
    context.insert("name", "World");

    // Multiple threads accessing the same renderer
    let _result_clone = Arc::clone(&result);
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let renderer = Arc::clone(&renderer);
            let template = template.clone();
            let context = context.clone();
            let result = Arc::clone(&result);
            thread::spawn(move || {
                let output = renderer.render_content(&template, &context).unwrap();
                let mut result = result.lock().unwrap();
                *result = output;
                Ok::<(), ()>(())
            })
        })
        .collect();

    for handle in handles {
        let _ = handle.join().unwrap();
    }

    let final_result = result.lock().unwrap();
    assert_eq!(*final_result, "Hello World!");
}
