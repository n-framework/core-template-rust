use super::*;
use serde_json::Value;
use std::collections::BTreeMap;

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

#[test]
fn test_insert_and_get_string() {
    let mut context = TemplateContext::empty();
    context.insert("name", "World");
    assert_eq!(context.get_str("name"), Some("World"));
}

#[test]
fn test_insert_number() {
    let mut context = TemplateContext::empty();
    context.insert_number("count", 42.0);
    assert_eq!(context.get_number("count"), Some(42.0));
}

#[test]
fn test_insert_bool() {
    let mut context = TemplateContext::empty();
    context.insert_bool("active", true);
    assert_eq!(context.get_bool("active"), Some(true));
}

#[test]
fn test_insert_value() {
    let mut context = TemplateContext::empty();
    context.insert_value("data", serde_json::json!({"key": "value"}));
    assert!(context.get("data").is_some());
}

#[test]
fn test_iter_abstractions() {
    let mut context = TemplateContext::empty();
    context.insert("a", "1");
    context.insert("b", "2");
    let items: Vec<_> = context.iter().collect();
    assert_eq!(items.len(), 2);
}

#[test]
fn test_to_json() {
    let mut context = TemplateContext::empty();
    context.insert("name", "World");
    let json = context.to_json();
    assert_eq!(json["name"], "World");
}

#[test]
fn test_from_json() {
    let json = serde_json::json!({"name": "World", "count": 42});
    let context = TemplateContext::from_json(json);
    assert!(context.is_some());
    assert_eq!(context.unwrap().get_str("name"), Some("World"));
}
