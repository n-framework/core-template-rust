//! Tera-based template renderer module.

use log::warn;
use n_framework_core_template_abstractions::{TemplateContext, TemplateError, TemplateRenderer};
use tera::Tera;

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
        // Validate keys as suggested for type design safety
        for (k, _) in context.iter() {
            if !k.chars().all(|c| c.is_alphanumeric() || c == '_') {
                warn!(
                    "Template context contains key with non-alphanumeric characters: '{}'. This might fail evaluation in Tera.",
                    k
                );
            }
        }

        let mut tera = Tera::default();
        let tera_context = tera::Context::from_serialize(context.to_json())
            .map_err(|e| TemplateError::render(format!("failed to create context: {}", e)))?;

        tera.render_str(template_content, &tera_context)
            .map_err(|e| TemplateError::render(format!("failed to render template: {}", e)))
    }
}
