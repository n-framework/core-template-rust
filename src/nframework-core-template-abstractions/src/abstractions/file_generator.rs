use std::path::Path;

use crate::errors::TemplateError;
use crate::template_context::TemplateContext;

pub trait FileGenerator {
    fn generate(
        &self,
        template_root: &Path,
        output_root: &Path,
        context: &TemplateContext,
    ) -> Result<(), TemplateError>;
}
