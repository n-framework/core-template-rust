use std::collections::HashMap;
use std::path::Path;

use nframework_core_template_abstractions::{
    FileGenerator, TemplateContext, TemplateError, TemplateRenderer,
};

/// Mustache-based template renderer.
#[derive(Debug, Clone, Copy)]
pub struct MustacheTemplateRenderer;

impl MustacheTemplateRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MustacheTemplateRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateRenderer for MustacheTemplateRenderer {
    fn render_content(
        &self,
        template_content: &str,
        context: &TemplateContext,
    ) -> Result<String, TemplateError> {
        // Convert TemplateContext to a HashMap for serde serialization
        let data: HashMap<String, serde_json::Value> = context
            .iter()
            .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
            .collect();

        // Compile and render the mustache template
        let template = mustache::compile_str(template_content)
            .map_err(|e| TemplateError::new(format!("failed to compile template: {}", e)))?;

        template
            .render_to_string(&data)
            .map_err(|e| TemplateError::new(format!("failed to render template: {}", e)))
    }
}

/// Mustache-based file generator.
#[derive(Debug, Clone)]
pub struct MustacheFileGenerator<R>
where
    R: TemplateRenderer,
{
    renderer: R,
}

impl<R> MustacheFileGenerator<R>
where
    R: TemplateRenderer,
{
    pub fn new(renderer: R) -> Self {
        Self { renderer }
    }
}

impl<R> FileGenerator for MustacheFileGenerator<R>
where
    R: TemplateRenderer,
{
    fn generate(
        &self,
        template_root: &Path,
        output_root: &Path,
        context: &TemplateContext,
    ) -> Result<(), TemplateError> {
        if !template_root.exists() {
            return Err(TemplateError::new(format!(
                "template root does not exist: {}",
                template_root.display()
            )));
        }

        self.generate_recursive(template_root, template_root, output_root, context)
    }
}

impl<R> MustacheFileGenerator<R>
where
    R: TemplateRenderer,
{
    fn generate_recursive(
        &self,
        root: &Path,
        current: &Path,
        output_root: &Path,
        context: &TemplateContext,
    ) -> Result<(), TemplateError> {
        use std::fs;

        for entry in fs::read_dir(current)
            .map_err(|error| TemplateError::new(format!("failed to read directory: {error}")))?
        {
            let entry = entry
                .map_err(|error| TemplateError::new(format!("invalid directory entry: {error}")))?;
            let path = entry.path();

            if path.is_dir() {
                self.generate_recursive(root, &path, output_root, context)?;
                continue;
            }

            let relative_path = path.strip_prefix(root).map_err(|error| {
                TemplateError::new(format!("failed to map relative path: {}", error))
            })?;

            let rendered_relative_path = self.render_path(relative_path, context)?;
            let output_path = output_root.join(rendered_relative_path);

            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent).map_err(|error| {
                    TemplateError::new(format!("failed to create output directory: {error}"))
                })?;
            }

            let content = fs::read_to_string(&path).map_err(|error| {
                TemplateError::new(format!("failed to read template file: {error}"))
            })?;
            let rendered_content = self.renderer.render_content(&content, context)?;

            fs::write(&output_path, rendered_content).map_err(|error| {
                TemplateError::new(format!("failed to write output file: {error}"))
            })?;
        }

        Ok(())
    }

    fn render_path(
        &self,
        relative_path: &Path,
        context: &TemplateContext,
    ) -> Result<std::path::PathBuf, TemplateError> {
        let original = relative_path.to_string_lossy();
        let rendered = self.renderer.render_content(&original, context)?;
        Ok(std::path::PathBuf::from(rendered))
    }
}
