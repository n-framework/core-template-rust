use crate::TeraTemplateRenderer;
use n_framework_core_template_abstractions::{
    FileGenerator, TemplateContext, TemplateError, TemplateRenderer,
};
use std::fs;
use std::path::Path;
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
            if let Some(file_name) = path.file_name()
                && file_name == "template.yaml"
            {
                continue;
            }

            let rel_path = path.strip_prefix(template_root).map_err(|e| {
                TemplateError::validation(format!("failed to map relative path: {}", e))
            })?;

            // Render the path itself using the renderer (path interpolation)
            let rel_path_str = rel_path.to_string_lossy();
            let rendered_rel_path = self.renderer.render_content(&rel_path_str, context)?;

            let dest_path = output_root.join(rendered_rel_path);
            
            // SECURITY: Ensure the normalized destination path does not escape output_root
            let canonical_output_root = output_root.canonicalize().unwrap_or(output_root.to_path_buf());
            let canonical_dest = if dest_path.exists() {
                dest_path.canonicalize().unwrap_or(dest_path.clone())
            } else {
                let parent = dest_path.parent().unwrap_or(&dest_path);
                let canonical_parent = parent.canonicalize().unwrap_or(parent.to_path_buf());
                canonical_parent.join(dest_path.file_name().unwrap_or_default())
            };
            
            // Fallback string matching if canonicalize fails
            let dest_str = dest_path.to_string_lossy();
            if dest_str.contains("..") || !canonical_dest.starts_with(&canonical_output_root) {
                 return Err(TemplateError::validation(format!(
                    "unsafe rendered path escapes output root: {}", dest_path.display()
                )));
            }
            
            let mut dest_path_mut = dest_path.clone();

            if path.is_dir() {
                fs::create_dir_all(&dest_path_mut).map_err(|e| {
                    TemplateError::io(format!(
                        "failed to create directory {}: {}",
                        dest_path_mut.display(),
                        e
                    ))
                })?;
            } else {
                // Strip .tera extension if present
                if let Some(ext) = dest_path_mut.extension().map(|e| e.to_string_lossy())
                    && ext == "tera"
                {
                    dest_path_mut.set_extension("");
                }

                // Ensure parent directory exists
                if let Some(parent) = dest_path_mut.parent() {

                    fs::create_dir_all(parent).map_err(|e| {
                        TemplateError::io(format!(
                            "failed to create parent directory for {}: {}",
                            dest_path_mut.display(),
                            e
                        ))
                    })?;
                }

                // Render content
                let content = fs::read_to_string(path).map_err(|e| {
                    TemplateError::io(format!(
                        "failed to read template file {}: {}",
                        path.display(),
                        e
                    ))
                })?;

                let rendered_content = self.renderer.render_content(&content, context)?;

                fs::write(&dest_path_mut, rendered_content).map_err(|e| {
                    TemplateError::io(format!(
                        "failed to write output file {}: {}",
                        dest_path_mut.display(),
                        e
                    ))
                })?;
            }
        }

        Ok(())
    }
}
