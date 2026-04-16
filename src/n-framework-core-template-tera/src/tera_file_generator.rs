use std::fs;
use std::path::Path;
use n_framework_core_template_abstractions::{FileGenerator, TemplateContext, TemplateError, TemplateRenderer};
use crate::TeraTemplateRenderer;
use walkdir::WalkDir;

/// Tera-based file generator that supports folder rendering and path interpolation.
#[derive(Debug, Clone)]
pub struct TeraFileGenerator<R>
where
    R: TemplateRenderer,
{
    renderer: R,
}

impl<R> TeraFileGenerator<R>
where
    R: TemplateRenderer,
{
    pub fn new(renderer: R) -> Self {
        Self { renderer }
    }
}

impl Default for TeraFileGenerator<TeraTemplateRenderer> {
    fn default() -> Self {
        Self::new(TeraTemplateRenderer::new())
    }
}

impl<R> FileGenerator for TeraFileGenerator<R>
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
            return Err(TemplateError::io(format!(
                "template root does not exist: {}",
                template_root.display()
            )));
        }

        // We use WalkDir for efficient recursive traversal
        for entry in WalkDir::new(template_root)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            // Skip the root itself
            if path == template_root {
                continue;
            }

            // Skip template.yaml - it's configuration, not output content
            if let Some(file_name) = path.file_name() {
                if file_name == "template.yaml" {
                    continue;
                }
            }

            let rel_path = path.strip_prefix(template_root).map_err(|e| {
                TemplateError::validation(format!("failed to map relative path: {}", e))
            })?;

            // Render the path itself using the renderer (path interpolation)
            let rel_path_str = rel_path.to_string_lossy();
            let rendered_rel_path = self.renderer.render_content(&rel_path_str, context)?;
            
            let mut dest_path = output_root.join(rendered_rel_path);

            if path.is_dir() {
                fs::create_dir_all(&dest_path).map_err(|e| {
                    TemplateError::io(format!("failed to create directory {}: {}", dest_path.display(), e))
                })?;
            } else {
                // Strip .tera extension if present
                if let Some(ext) = dest_path.extension().map(|e| e.to_string_lossy()) {
                    if ext == "tera" {
                        dest_path.set_extension("");
                    }
                }

                // Ensure parent directory exists
                if let Some(parent) = dest_path.parent() {
                    fs::create_dir_all(parent).map_err(|e| {
                        TemplateError::io(format!("failed to create parent directory for {}: {}", dest_path.display(), e))
                    })?;
                }

                // Render content
                let content = fs::read_to_string(path).map_err(|e| {
                    TemplateError::io(format!("failed to read template file {}: {}", path.display(), e))
                })?;
                
                let rendered_content = self.renderer.render_content(&content, context)?;

                fs::write(&dest_path, rendered_content).map_err(|e| {
                    TemplateError::io(format!("failed to write output file {}: {}", dest_path.display(), e))
                })?;
            }
        }

        Ok(())
    }
}
