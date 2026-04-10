use std::collections::BTreeMap;

use serde_json::Value;

/// A context for template rendering, containing key-value pairs
/// that are substituted into templates.
///
/// Supports structured data including strings, numbers, booleans, arrays, and objects.
///
/// # Example
/// ```
/// use n_framework_core_template_abstractions::TemplateContext;
///
/// let mut context = TemplateContext::empty();
/// context.insert("name", "World");
/// context.insert_number("count", 42.0);
/// context.insert_bool("active", true);
///
/// assert_eq!(context.get_str("name"), Some("World"));
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TemplateContext {
    values: BTreeMap<String, Value>,
}

impl TemplateContext {
    /// Creates a new TemplateContext with the given values.
    pub fn new(values: BTreeMap<String, Value>) -> Self {
        Self { values }
    }

    /// Creates an empty TemplateContext.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Inserts a string value into the context.
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.values.insert(key.into(), Value::String(value.into()));
    }

    /// Inserts a numeric value into the context.
    pub fn insert_number(&mut self, key: impl Into<String>, value: f64) {
        self.values.insert(
            key.into(),
            serde_json::Number::from_f64(value)
                .map(Value::Number)
                .unwrap_or(Value::Null),
        );
    }

    /// Inserts a boolean value into the context.
    pub fn insert_bool(&mut self, key: impl Into<String>, value: bool) {
        self.values.insert(key.into(), Value::Bool(value));
    }

    /// Inserts a JSON value directly into the context.
    pub fn insert_value(&mut self, key: impl Into<String>, value: Value) {
        self.values.insert(key.into(), value);
    }

    /// Gets a value by key, returning None if not found.
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.get(key)
    }

    /// Gets a string value by key, returning None if not found or not a string.
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.values.get(key).and_then(|v| v.as_str())
    }

    /// Gets a number value by key, returning None if not found or not a number.
    pub fn get_number(&self, key: &str) -> Option<f64> {
        self.values.get(key).and_then(|v| v.as_f64())
    }

    /// Gets a boolean value by key, returning None if not found or not a boolean.
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.values.get(key).and_then(|v| v.as_bool())
    }

    /// Iterates over all key-value pairs in the context.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Value)> {
        self.values.iter().map(|(k, v)| (k.as_str(), v))
    }

    /// Converts the context to a JSON value for template rendering.
    pub fn to_json(&self) -> Value {
        Value::Object(
            self.values
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        )
    }

    /// Creates a context from a JSON value.
    pub fn from_json(json: Value) -> Option<Self> {
        match json {
            Value::Object(map) => Some(Self {
                values: map.into_iter().collect(),
            }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_iter() {
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
}
