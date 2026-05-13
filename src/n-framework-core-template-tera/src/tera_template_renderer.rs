//! Tera-based template renderer module.

use log::info;
use n_framework_core_template_abstractions::{TemplateContext, TemplateError, TemplateRenderer};
use tera::Tera;

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
        info!(
            "Rendering template content with context: {:?}",
            context.to_json()
        );

        let mut tera = Tera::default();

        // Register custom filters
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

fn slugify_filter(
    value: &tera::Value,
    _args: &std::collections::HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let s = try_get_value!("slugify", "value", String, value);
    Ok(tera::to_value(to_kebab_case(&s)).unwrap())
}

fn pascal_case_filter(
    value: &tera::Value,
    _args: &std::collections::HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let s = try_get_value!("pascal_case", "value", String, value);
    Ok(tera::to_value(to_pascal_case(&s)).unwrap())
}

fn snake_case_filter(
    value: &tera::Value,
    _args: &std::collections::HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let s = try_get_value!("snake_case", "value", String, value);
    Ok(tera::to_value(to_snake_case(&s)).unwrap())
}

fn sentence_case_filter(
    value: &tera::Value,
    _args: &std::collections::HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let s = try_get_value!("sentence_case", "value", String, value);
    Ok(tera::to_value(to_sentence_case(&s)).unwrap())
}

fn to_kebab_case(s: &str) -> String {
    let mut result = String::new();
    let mut last_was_upper = false;
    let mut last_was_separator = true;

    for (i, c) in s.chars().enumerate() {
        if c == '_' || c == '-' || c == ' ' {
            if !last_was_separator {
                result.push('-');
                last_was_separator = true;
            }
            continue;
        }

        if c.is_uppercase() {
            if !last_was_separator && !last_was_upper && i > 0 {
                result.push('-');
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

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut last_was_upper = false;
    let mut last_was_separator = true;

    for (i, c) in s.chars().enumerate() {
        if c == '_' || c == '-' || c == ' ' {
            if !last_was_separator {
                result.push('_');
                last_was_separator = true;
            }
            continue;
        }

        if c.is_uppercase() {
            if !last_was_separator && !last_was_upper && i > 0 {
                result.push('_');
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
