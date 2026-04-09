use crate::errors::TemplateError;
use crate::template_context::TemplateContext;

/// Trait for rendering template content with a context.
///
/// Implementors provide the logic to parse and render template strings
/// using a specific template engine (e.g., Mustache, Handlebars, Jinja2).
pub trait TemplateRenderer {
    /// Renders the given template content with the provided context.
    ///
    /// # Arguments
    /// * `template_content` - The raw template string to render
    /// * `context` - The template context containing variable values
    ///
    /// # Returns
    /// * `Ok(String)` - The rendered content with variables substituted
    /// * `Err(TemplateError)` - If rendering fails
    fn render_content(
        &self,
        template_content: &str,
        context: &TemplateContext,
    ) -> Result<String, TemplateError>;
}
