use n_framework_core_template_abstractions::{TemplateContext, TemplateError, TemplateRenderer};
use tera::Tera;

/// Tera-based template renderer.
#[derive(Debug, Clone, Default)]
pub struct TeraTemplateRenderer;

impl TeraTemplateRenderer {
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
        let mut tera = Tera::default();
        let tera_context = tera::Context::from_serialize(context.to_json())
            .map_err(|e| TemplateError::render(format!("failed to create context: {}", e)))?;

        tera.render_str(template_content, &tera_context)
            .map_err(|e| TemplateError::render(format!("failed to render template: {}", e)))
    }
}
