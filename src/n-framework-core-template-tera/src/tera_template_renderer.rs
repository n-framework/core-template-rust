//! Tera-based template renderer module.

use log::debug;
use n_framework_core_template_abstractions::{TemplateContext, TemplateError, TemplateRenderer};
use tera::Tera;

/// Macro for extracting typed values from Tera's dynamic Value type.
///
/// Provides consistent error messaging when filter arguments have unexpected types.
///
/// # Arguments
/// - `$filter_name`: Name of the filter (for error messages)
/// - `$arg_name`: Argument name being extracted
/// - `$ty`: Expected type
/// - `$value`: The tera::Value to extract from
macro_rules! try_get_value {
    ($filter_name:expr, $arg_name:expr, $ty:ty, $value:expr) => {
        match tera::from_value::<$ty>($value.clone()) {
            Ok(v) => v,
            Err(_) => {
                return Err(tera::Error::msg(format!(
                    "Filter '{}' expected a {} for arg '{}' but got {:?}",
                    $filter_name,
                    stringify!($ty),
                    $arg_name,
                    $value
                )));
            }
        }
    };
}

/// Tera-based template renderer.
///
/// Current design employs a stateless trait wrapper that spins up a new independent Tera
/// environment for each file content evaluation. This is safe, ensures zero side effects
/// between files, but should be optimized dynamically via caching if performance is impacted.
#[derive(Debug, Clone, Default)]
pub struct TeraTemplateRenderer;

impl TeraTemplateRenderer {
    /// Creates a new, stateless TeraTemplateRenderer.
    pub fn new() -> Self {
        Self
    }
}

impl TemplateRenderer for TeraTemplateRenderer {
    fn render_content(
        &self,
        template_content: &str,
        context: &TemplateContext,
    ) -> Result<String, TemplateError> {
        debug!(
            "Rendering template with {} context keys",
            context.iter().count()
        );

        let mut tera = Tera::default();

        tera.register_filter("slugify", slugify_filter);
        tera.register_filter("pascal_case", pascal_case_filter);
        tera.register_filter("snake_case", snake_case_filter);
        tera.register_filter("sentence_case", sentence_case_filter);

        let tera_context = tera::Context::from_serialize(context.to_json())
            .map_err(|e| TemplateError::render(format!("failed to create context: {}", e)))?;

        tera.render_str(template_content, &tera_context)
            .map_err(|e| TemplateError::render(format!("failed to render template: {}", e)))
    }
}

/// Converts a string to kebab-case (slug format).
///
/// # Rules
/// - Separators (`_`, `-`, space) become `-`
/// - Multiple consecutive separators are collapsed to single `-`
/// - Uppercase letters are converted to lowercase
/// - Transitions from lowercase to uppercase insert `-` (e.g., "kebabCase" → "kebab-case")
///
/// # Examples
/// - "HelloWorld" → "hello-world"
/// - "hello_world" → "hello-world"
/// - "Hello World" → "hello-world"
fn slugify_filter(
    value: &tera::Value,
    _args: &std::collections::HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let s = try_get_value!("slugify", "value", String, value);
    tera::to_value(to_kebab_case(&s))
        .map_err(|e| tera::Error::msg(format!("Failed to convert slugify result: {}", e)))
}

/// Converts a string to PascalCase.
///
/// # Rules
/// - Splits on separators (`_`, `-`, space)
/// - Capitalizes first letter of each word
/// - Removes all separators
///
/// # Examples
/// - "hello_world" → "HelloWorld"
/// - "hello-world" → "HelloWorld"
/// - "hello world" → "HelloWorld"
fn pascal_case_filter(
    value: &tera::Value,
    _args: &std::collections::HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let s = try_get_value!("pascal_case", "value", String, value);
    tera::to_value(to_pascal_case(&s))
        .map_err(|e| tera::Error::msg(format!("Failed to convert pascal_case result: {}", e)))
}

/// Converts a string to snake_case.
///
/// # Rules
/// - Separators (`_`, `-`, space) become `_`
/// - Multiple consecutive separators are collapsed to single `_`
/// - Uppercase letters are converted to lowercase
/// - Transitions from lowercase to uppercase insert `_` (e.g., "snakeCase" → "snake_case")
///
/// # Examples
/// - "HelloWorld" → "hello_world"
/// - "hello-world" → "hello_world"
/// - "Hello World" → "hello_world"
fn snake_case_filter(
    value: &tera::Value,
    _args: &std::collections::HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let s = try_get_value!("snake_case", "value", String, value);
    tera::to_value(to_snake_case(&s))
        .map_err(|e| tera::Error::msg(format!("Failed to convert snake_case result: {}", e)))
}

/// Converts a string to sentence case.
///
/// # Rules
/// - Splits on separators (`_`, `-`, space)
/// - Capitalizes first letter of first word only
/// - Joins with spaces
/// - All other letters are lowercase
///
/// # Examples
/// - "hello_world" → "Hello world"
/// - "HELLO-WORLD" → "Hello world"
/// - "hello world" → "Hello world"
fn sentence_case_filter(
    value: &tera::Value,
    _args: &std::collections::HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let s = try_get_value!("sentence_case", "value", String, value);
    tera::to_value(to_sentence_case(&s))
        .map_err(|e| tera::Error::msg(format!("Failed to convert sentence_case result: {}", e)))
}

/// Converts a string to kebab-case (slug format).
///
/// # Rules
/// - Separators (`_`, `-`, space) become `-`
/// - Multiple consecutive separators are collapsed to single `-`
/// - Uppercase letters are converted to lowercase
/// - Transitions from lowercase to uppercase insert `-` (e.g., "kebabCase" → "kebab-case")
///
/// # Examples
/// - "HelloWorld" → "hello-world"
/// - "hello_world" → "hello-world"
/// - "Hello World" → "hello-world"
fn to_kebab_case(s: &str) -> String {
    to_case_with_separator(s, '-')
}

/// Converts a string to PascalCase.
///
/// # Rules
/// - Splits on separators (`_`, `-`, space)
/// - Capitalizes first letter of each word
/// - Removes all separators
///
/// # Examples
/// - "hello_world" → "HelloWorld"
/// - "hello-world" → "HelloWorld"
/// - "hello world" → "HelloWorld"
fn to_pascal_case(s: &str) -> String {
    s.split(['-', '_', ' '])
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => format!("{}{}", first.to_ascii_uppercase(), chars.as_str()),
                None => String::new(),
            }
        })
        .collect()
}

/// Converts a string to snake_case.
///
/// # Rules
/// - Separators (`_`, `-`, space) become `_`
/// - Multiple consecutive separators are collapsed to single `_`
/// - Uppercase letters are converted to lowercase
/// - Transitions from lowercase to uppercase insert `_` (e.g., "snakeCase" → "snake_case")
///
/// # Examples
/// - "HelloWorld" → "hello_world"
/// - "hello-world" → "hello_world"
/// - "Hello World" → "hello_world"
fn to_snake_case(s: &str) -> String {
    to_case_with_separator(s, '_')
}

/// Helper function that converts strings to separated case formats (kebab-case or snake_case).
///
/// This function implements the common logic for both kebab-case and snake_case conversions,
/// differing only in the separator character used.
///
/// # Rules
/// - All separators (`_`, `-`, space) become the specified separator
/// - Multiple consecutive separators are collapsed to single separator
/// - Uppercase letters are converted to lowercase
/// - Transitions from lowercase to uppercase insert the separator (e.g., "kebabCase" → "kebab-case")
///
/// # Arguments
/// - `s`: The input string to convert
/// - `separator`: The separator character to use ('-' for kebab-case, '_' for snake_case)
fn to_case_with_separator(s: &str, separator: char) -> String {
    let mut result = String::new();
    let mut last_was_upper = false;
    let mut last_was_separator = true;

    for (i, c) in s.chars().enumerate() {
        if c == '_' || c == '-' || c == ' ' {
            if !last_was_separator {
                result.push(separator);
                last_was_separator = true;
            }
            continue;
        }

        if c.is_uppercase() {
            if !last_was_separator && !last_was_upper && i > 0 {
                result.push(separator);
            }
            result.push(c.to_ascii_lowercase());
            last_was_upper = true;
            last_was_separator = false;
        } else {
            result.push(c);
            last_was_upper = false;
            last_was_separator = false;
        }
    }
    result
}

/// Converts a string to sentence case.
///
/// # Rules
/// - Splits on separators (`_`, `-`, space)
/// - Capitalizes first letter of first word only
/// - Joins with spaces
/// - All other letters are lowercase
///
/// # Examples
/// - "hello_world" → "Hello world"
/// - "HELLO-WORLD" → "Hello world"
/// - "hello world" → "Hello world"
fn to_sentence_case(s: &str) -> String {
    let words: Vec<String> = s
        .split(['-', '_', ' '])
        .filter(|segment| !segment.is_empty())
        .enumerate()
        .map(|(i, segment)| {
            let lower = segment.to_ascii_lowercase();
            if i == 0 {
                // Capitalize first word
                let mut chars = lower.chars();
                match chars.next() {
                    Some(first) => format!("{}{}", first.to_ascii_uppercase(), chars.as_str()),
                    None => String::new(),
                }
            } else {
                lower
            }
        })
        .collect();
    words.join(" ")
}

#[cfg(test)]
#[path = "tera_template_renderer.tests.rs"]
mod tests;
