use crate::errors::TemplateError;
use crate::template_context::TemplateContext;

pub trait TemplateRenderer {
    fn render_content(
        &self,
        template_content: &str,
        context: &TemplateContext,
    ) -> Result<String, TemplateError>;
}
