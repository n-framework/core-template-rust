use std::collections::BTreeMap;

use nframework_core_template_abstractions::TemplateContext;
use serde_json::Value;

#[test]
fn test_template_context_new() {
    let mut values = BTreeMap::new();
    values.insert("name".to_owned(), Value::String("World".to_owned()));
    values.insert(
        "count".to_owned(),
        Value::Number(serde_json::Number::from(42)),
    );

    let context = TemplateContext::new(values);

    assert_eq!(context.get_str("name"), Some("World"));
    assert_eq!(context.get_number("count"), Some(42.0));
    assert_eq!(context.get("missing"), None);
}

#[test]
fn test_template_context_empty() {
    let context = TemplateContext::empty();
    assert_eq!(context.get("anything"), None);
}

#[test]
fn test_template_context_insert() {
    let mut context = TemplateContext::empty();

    context.insert("name", "Alice");
    context.insert_number("age", 30.0);

    assert_eq!(context.get_str("name"), Some("Alice"));
    assert_eq!(context.get_number("age"), Some(30.0));
}

#[test]
fn test_template_context_iter() {
    let mut context = TemplateContext::empty();
    context.insert("a", "1");
    context.insert("b", "2");
    let items: Vec<_> = context.iter().collect();
    assert_eq!(items.len(), 2);
    assert!(
        items
            .iter()
            .any(|(k, v)| *k == "a" && **v == Value::String("1".to_string()))
    );
    assert!(
        items
            .iter()
            .any(|(k, v)| *k == "b" && **v == Value::String("2".to_string()))
    );
}

#[test]
fn test_template_context_insert_overwrites() {
    let mut context = TemplateContext::empty();

    context.insert("key", "first");
    context.insert("key", "second");

    assert_eq!(context.get_str("key"), Some("second"));
}
