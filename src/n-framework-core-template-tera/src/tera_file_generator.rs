//! Tera template generation module.
//!
//! Provides a file generator that supports folder rendering and path interpolation securely.

use crate::TeraTemplateRenderer;
use log::{debug, error, info, warn};
use n_framework_core_template_abstractions::{
    AtomicFileGenerator, FileGenerator, OverwritePolicy, TemplateContext, TemplateError,
    TemplateRenderer,
};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Tera-based file generator that supports folder rendering and path interpolation.
/// Handles recursive traversal, renders `.tera` templates to standard files,
/// and securely restricts generated output to the boundaries of the output root directory.
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
    /// Creates a new TeraFileGenerator with the given stateless renderer.
    pub fn new(renderer: R) -> Self {
        Self { renderer }
    }

    fn generate_internal(
        &self,
        template_root: &Path,
        output_root: &Path,
        context: &TemplateContext,
        overwrite_policy: OverwritePolicy,
        atomic: bool,
    ) -> Result<(), TemplateError> {
        let mut created_paths = Vec::new();

        let result = self.execute_generation(
            template_root,
            output_root,
            context,
            overwrite_policy,
            &mut created_paths,
        );

        if result.is_err() && atomic {
            warn!(
                "Atomic generation failed. Rolling back {} created paths.",
                created_paths.len()
            );
            // Rollback in reverse order (files before their parent directories)
            for path in created_paths.into_iter().rev() {
                if path.is_file() {
                    let _ = fs::remove_file(&path);
                } else if path.is_dir() {
                    let _ = fs::remove_dir(&path);
                }
            }
        }

        result
    }

    fn execute_generation(
        &self,
        template_root: &Path,
        output_root: &Path,
        context: &TemplateContext,
        overwrite_policy: OverwritePolicy,
        created_paths: &mut Vec<PathBuf>,
    ) -> Result<(), TemplateError> {
        if !template_root.exists() {
            let msg = format!("template root does not exist: {}", template_root.display());
            error!("{}", msg);
            return Err(TemplateError::io(msg));
        }

        if !template_root.is_dir() {
            let msg = format!(
                "template root must be a directory: {}",
                template_root.display()
            );
            error!("{}", msg);
            return Err(TemplateError::io(msg));
        }

        info!(
            "Starting generation from template {} to {}",
            template_root.display(),
            output_root.display()
        );

        // Pre-create and canonicalize the output root for later security checks.
        if !output_root.exists() {
            fs::create_dir_all(output_root).map_err(|e| {
                let msg = format!(
                    "failed to create output root {}: {}",
                    output_root.display(),
                    e
                );
                error!("{}", msg);
                TemplateError::io(msg)
            })?;
            created_paths.push(output_root.to_path_buf());
        }

        let canonical_output_root = output_root.canonicalize().map_err(|e| {
            let msg = format!(
                "failed to canonicalize output root {}: {}",
                output_root.display(),
                e
            );
            error!("{}", msg);
            TemplateError::security(msg)
        })?;

        for entry in WalkDir::new(template_root) {
            let entry = entry.map_err(|e| {
                let msg = format!("failed to walk directory: {}", e);
                error!("{}", msg);
                TemplateError::io(msg)
            })?;

            let path = entry.path();

            if path == template_root {
                continue;
            }

            if let Some(file_name) = path.file_name()
                && file_name == "template.yaml"
            {
                continue;
            }

            let rel_path = path.strip_prefix(template_root).map_err(|e| {
                let msg = format!("failed to map relative path: {}", e);
                error!("{}", msg);
                TemplateError::validation(msg)
            })?;

            let rel_path_str = rel_path.to_string_lossy();
            let rendered_rel_path = self.renderer.render_content(&rel_path_str, context)?;

            let dest_path = output_root.join(rendered_rel_path);

            let canonical_dest = if dest_path.exists() {
                dest_path.canonicalize().map_err(|e| {
                    let msg = format!(
                        "failed to canonicalize dest_path {}: {}",
                        dest_path.display(),
                        e
                    );
                    error!("{}", msg);
                    TemplateError::security(msg)
                })?
            } else {
                let parent = dest_path.parent().unwrap_or(&dest_path);
                if !parent.exists() {
                    fs::create_dir_all(parent).map_err(|e| {
                        let msg = format!(
                            "failed to create parent dir for canonicalization {}: {}",
                            parent.display(),
                            e
                        );
                        error!("{}", msg);
                        TemplateError::io(msg)
                    })?;
                    // We don't track all intermediate parent dirs easily, but we can track the parent
                    created_paths.push(parent.to_path_buf());
                }
                let canonical_parent = parent.canonicalize().map_err(|e| {
                    let msg = format!(
                        "failed to canonicalize parent path {}: {}",
                        parent.display(),
                        e
                    );
                    error!("{}", msg);
                    TemplateError::security(msg)
                })?;
                canonical_parent.join(dest_path.file_name().unwrap_or_default())
            };

            let dest_str = dest_path.to_string_lossy();
            if dest_str.contains("..") || !canonical_dest.starts_with(&canonical_output_root) {
                let msg = format!(
                    "unsafe rendered path escapes output root: {}",
                    dest_path.display()
                );
                error!("{}", msg);
                return Err(TemplateError::security(msg));
            }

            if path.is_dir() {
                if !dest_path.exists() {
                    fs::create_dir_all(&dest_path).map_err(|e| {
                        let msg =
                            format!("failed to create directory {}: {}", dest_path.display(), e);
                        error!("{}", msg);
                        TemplateError::io(msg)
                    })?;
                    created_paths.push(dest_path.clone());
                    debug!("Created directory: {}", dest_path.display());
                }
            } else {
                let mut output_file_path = dest_path.clone();

                if let Some(ext) = output_file_path.extension().map(|e| e.to_string_lossy())
                    && ext == "tera"
                {
                    output_file_path.set_extension("");
                }

                // Apply overwrite policy
                if output_file_path.exists() {
                    match overwrite_policy {
                        OverwritePolicy::Overwrite => { /* Continue */ }
                        OverwritePolicy::Skip => continue,
                        OverwritePolicy::Fail => {
                            let msg = format!(
                                "File already exists and policy is Fail: {}",
                                output_file_path.display()
                            );
                            error!("{}", msg);
                            return Err(TemplateError::validation(msg));
                        }
                        OverwritePolicy::Prompt => {
                            warn!(
                                "Prompt overwrite policy not yet supported, defaulting to Skip: {}",
                                output_file_path.display()
                            );
                            continue;
                        }
                    }
                }

                if let Some(parent) = output_file_path.parent()
                    && !parent.exists()
                {
                    fs::create_dir_all(parent).map_err(|e| {
                        let msg = format!(
                            "failed to create parent directory for {}: {}",
                            output_file_path.display(),
                            e
                        );
                        error!("{}", msg);
                        TemplateError::io(msg)
                    })?;
                    created_paths.push(parent.to_path_buf());
                }

                let content = fs::read_to_string(path).map_err(|e| {
                    let msg = format!("failed to read template file {}: {}", path.display(), e);
                    error!("{}", msg);
                    TemplateError::io(msg)
                })?;

                let rendered_content = self.renderer.render_content(&content, context)?;

                fs::write(&output_file_path, rendered_content).map_err(|e| {
                    let msg = format!(
                        "failed to write output file {}: {}",
                        output_file_path.display(),
                        e
                    );
                    error!("{}", msg);
                    TemplateError::io(msg)
                })?;

                created_paths.push(output_file_path.clone());
                debug!("Generated file: {}", output_file_path.display());
            }
        }

        info!(
            "Successfully generated templates to {}",
            output_root.display()
        );
        Ok(())
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
        self.generate_internal(
            template_root,
            output_root,
            context,
            OverwritePolicy::Overwrite,
            false,
        )
    }
}

impl<R> AtomicFileGenerator for TeraFileGenerator<R>
where
    R: TemplateRenderer,
{
    fn generate_atomic(
        &self,
        template_root: &Path,
        output_root: &Path,
        context: &TemplateContext,
        overwrite_policy: OverwritePolicy,
    ) -> Result<(), TemplateError> {
        self.generate_internal(template_root, output_root, context, overwrite_policy, true)
    }
}

#[cfg(test)]
#[path = "tera_file_generator.tests.rs"]
mod tests;
