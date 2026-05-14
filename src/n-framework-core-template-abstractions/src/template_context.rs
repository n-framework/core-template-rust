use crate::errors::TemplateError;
use serde_json::Value;
use std::collections::BTreeMap;

/// A context for template rendering, containing key-value pairs
/// that are substituted into templates.
///
/// Supports structured data including strings, numbers, booleans, arrays, and objects.
/// Keys are validated to ensure compatibility with most template engines (alphanumeric,
/// underscores, hyphens, and must start with a letter or underscore).
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
    ///
    /// # Note
    /// This method does not validate keys. Use `try_insert` for validated insertion.
    pub fn new(values: BTreeMap<String, Value>) -> Self {
        Self { values }
    }

    /// Creates an empty TemplateContext.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Validates if a key is safe for template engines.
    ///
    /// Rules:
    /// - Must not be empty
    /// - Must start with an ASCII letter or underscore
    /// - Must contain only ASCII alphanumeric characters, underscores, or hyphens
    pub fn is_valid_key(key: &str) -> bool {
        if key.is_empty() {
            return false;
        }

        let first = key.chars().next().unwrap();
        if !first.is_ascii_alphabetic() && first != '_' {
            return false;
        }

        key.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    }

    /// Inserts a string value into the context.
    ///
    /// # Panics
    /// Panics if the key is invalid. Use `try_insert` for safe insertion.
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        let key_str = key.into();
        if !Self::is_valid_key(&key_str) {
            panic!("Invalid template context key: '{}'", key_str);
        }
        self.values.insert(key_str, Value::String(value.into()));
    }

    /// Tries to insert a string value into the context.
    ///
    /// Returns an error if the key is invalid.
    pub fn try_insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), TemplateError> {
        let key_str = key.into();
        if !Self::is_valid_key(&key_str) {
            return Err(TemplateError::validation(format!(
                "invalid context key: '{}'",
                key_str
            )));
        }
        self.values.insert(key_str, Value::String(value.into()));
        Ok(())
    }

    /// Inserts a numeric value into the context.
    ///
    /// # Panics
    /// Panics if the key is invalid.
    pub fn insert_number(&mut self, key: impl Into<String>, value: f64) {
        let key_str = key.into();
        if !Self::is_valid_key(&key_str) {
            panic!("Invalid template context key: '{}'", key_str);
        }
        self.values.insert(
            key_str,
            serde_json::Number::from_f64(value)
                .map(Value::Number)
                .unwrap_or(Value::Null),
        );
    }

    /// Tries to insert a numeric value into the context.
    pub fn try_insert_number(
        &mut self,
        key: impl Into<String>,
        value: f64,
    ) -> Result<(), TemplateError> {
        let key_str = key.into();
        if !Self::is_valid_key(&key_str) {
            return Err(TemplateError::validation(format!(
                "invalid context key: '{}'",
                key_str
            )));
        }
        self.values.insert(
            key_str,
            serde_json::Number::from_f64(value)
                .map(Value::Number)
                .unwrap_or(Value::Null),
        );
        Ok(())
    }

    /// Inserts a boolean value into the context.
    ///
    /// # Panics
    /// Panics if the key is invalid.
    pub fn insert_bool(&mut self, key: impl Into<String>, value: bool) {
        let key_str = key.into();
        if !Self::is_valid_key(&key_str) {
            panic!("Invalid template context key: '{}'", key_str);
        }
        self.values.insert(key_str, Value::Bool(value));
    }

    /// Tries to insert a boolean value into the context.
    pub fn try_insert_bool(
        &mut self,
        key: impl Into<String>,
        value: bool,
    ) -> Result<(), TemplateError> {
        let key_str = key.into();
        if !Self::is_valid_key(&key_str) {
            return Err(TemplateError::validation(format!(
                "invalid context key: '{}'",
                key_str
            )));
        }
        self.values.insert(key_str, Value::Bool(value));
        Ok(())
    }

    /// Inserts a JSON value directly into the context.
    ///
    /// # Panics
    /// Panics if the key is invalid.
    pub fn insert_value(&mut self, key: impl Into<String>, value: Value) {
        let key_str = key.into();
        if !Self::is_valid_key(&key_str) {
            panic!("Invalid template context key: '{}'", key_str);
        }
        self.values.insert(key_str, value);
    }

    /// Tries to insert a JSON value directly into the context.
    pub fn try_insert_value(
        &mut self,
        key: impl Into<String>,
        value: Value,
    ) -> Result<(), TemplateError> {
        let key_str = key.into();
        if !Self::is_valid_key(&key_str) {
            return Err(TemplateError::validation(format!(
                "invalid context key: '{}'",
                key_str
            )));
        }
        self.values.insert(key_str, value);
        Ok(())
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
#[path = "template_context.tests.rs"]
mod tests;
