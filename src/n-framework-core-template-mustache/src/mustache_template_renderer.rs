use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, RwLock};

use log::{debug, error, info, warn};
use n_framework_core_template_abstractions::{
    FileGenerator, TemplateContext, TemplateError, TemplateRenderer,
};

/// Context struct for recursive file generation to reduce parameter count.
struct GenerateContext<'a> {
    root: &'a Path,
    output_root: &'a Path,
    context: &'a TemplateContext,
}

/// Mustache-based template renderer with caching for performance.
///
/// Uses an RwLock-protected HashMap to cache compiled templates,
/// avoiding recompilation of identical templates.
#[derive(Debug)]
pub struct MustacheTemplateRenderer {
    cache: Arc<RwLock<HashMap<String, mustache::Template>>>,
}

impl MustacheTemplateRenderer {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn get_or_compile(&self, template_content: &str) -> Result<mustache::Template, TemplateError> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // Create a hash of the template content for cache key
        let mut hasher = DefaultHasher::new();
        template_content.hash(&mut hasher);
        let cache_key = format!("{:x}", hasher.finish());

        // Try to get from cache first
        {
            let cache = self
                .cache
                .read()
                .map_err(|_| TemplateError::io("failed to acquire cache read lock"))?;
            if let Some(template) = cache.get(&cache_key) {
                debug!("Template cache hit for key: {}", cache_key);
                return Ok(template.clone());
            }
        }

        // Compile and cache
        debug!("Template cache miss, compiling new template");
        let template = mustache::compile_str(template_content)
            .map_err(|e| TemplateError::parse(format!("failed to compile template: {}", e)))?;

        {
            let mut cache = self
                .cache
                .write()
                .map_err(|_| TemplateError::io("failed to acquire cache write lock"))?;
            cache.insert(cache_key, template.clone());
        }

        Ok(template)
    }

    /// Clears the template cache.
    pub fn clear_cache(&self) -> Result<(), TemplateError> {
        let mut cache = self
            .cache
            .write()
            .map_err(|_| TemplateError::io("failed to acquire cache write lock"))?;
        cache.clear();
        debug!("Template cache cleared");
        Ok(())
    }

    /// Returns the number of cached templates.
    pub fn cache_size(&self) -> Result<usize, TemplateError> {
        let cache = self
            .cache
            .read()
            .map_err(|_| TemplateError::io("failed to acquire cache read lock"))?;
        Ok(cache.len())
    }
}

impl Clone for MustacheTemplateRenderer {
    fn clone(&self) -> Self {
        Self {
            cache: Arc::clone(&self.cache),
        }
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
        info!(
            "Rendering template with {} context values",
            context.iter().count()
        );

        let data: serde_json::Map<String, serde_json::Value> = context
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect();

        let template = self.get_or_compile(template_content)?;

        template.render_to_string(&data).map_err(|e| {
            error!("Template render failed: {}", e);
            TemplateError::render(format!("failed to render template: {}", e))
        })
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
    /// Creates a new MustacheFileGenerator.
    ///
    /// # Arguments
    /// * `renderer` - The template renderer to use for rendering content
    pub fn new(renderer: R) -> Self {
        info!("Creating new MustacheFileGenerator");
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
            warn!("Template root does not exist: {}", template_root.display());
            return Err(TemplateError::validation(format!(
                "template root does not exist: {}",
                template_root.display()
            )));
        }

        if !template_root.is_dir() {
            warn!(
                "Template root is not a directory: {}",
                template_root.display()
            );
            return Err(TemplateError::validation(format!(
                "template root is not a directory: {}",
                template_root.display()
            )));
        }

        if let Some(parent) = output_root.parent()
            && !parent.exists()
        {
            warn!(
                "Output root parent directory does not exist: {}",
                parent.display()
            );
            return Err(TemplateError::validation(format!(
                "output root parent directory does not exist: {}",
                parent.display()
            )));
        }

        info!(
            "Starting file generation from {} to {}",
            template_root.display(),
            output_root.display()
        );

        let gen_ctx = GenerateContext {
            root: template_root,
            output_root,
            context,
        };

        self.generate_recursive(template_root, &gen_ctx)
    }
}

impl<R> MustacheFileGenerator<R>
where
    R: TemplateRenderer,
{
    fn generate_recursive(
        &self,
        current: &Path,
        ctx: &GenerateContext<'_>,
    ) -> Result<(), TemplateError> {
        use std::fs;

        for entry in fs::read_dir(current).map_err(|error| {
            error!("Failed to read directory {}: {}", current.display(), error);
            TemplateError::io(format!("failed to read directory: {}", error))
        })? {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    warn!("Skipping invalid directory entry: {}", e);
                    continue;
                }
            };

            let path = entry.path();
            let file_type = match entry.file_type() {
                Ok(ft) => ft,
                Err(e) => {
                    warn!("Failed to get file type for {}: {}", path.display(), e);
                    continue;
                }
            };

            if file_type.is_symlink() {
                warn!("Skipping symbolic link: {}", path.display());
                continue;
            }

            if file_type.is_dir() {
                debug!("Processing directory: {}", path.display());
                self.generate_recursive(&path, ctx)?;
                continue;
            }

            if !file_type.is_file() {
                debug!("Skipping non-file entry: {}", path.display());
                continue;
            }

            let relative_path = path.strip_prefix(ctx.root).map_err(|error| {
                error!(
                    "Failed to map relative path for {}: {}",
                    path.display(),
                    error
                );
                TemplateError::io(format!("failed to map relative path: {}", error))
            })?;

            let rendered_relative_path = self.render_path(relative_path, ctx.context)?;
            let output_path = ctx.output_root.join(rendered_relative_path);

            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent).map_err(|error| {
                    error!(
                        "Failed to create output directory {}: {}",
                        parent.display(),
                        error
                    );
                    TemplateError::io(format!("failed to create output directory: {}", error))
                })?;
            }

            debug!("Rendering file: {}", path.display());

            let content = fs::read_to_string(&path).map_err(|error| {
                error!("Failed to read template file {}: {}", path.display(), error);
                TemplateError::io(format!("failed to read template file: {}", error))
            })?;
            let rendered_content = self.renderer.render_content(&content, ctx.context)?;

            fs::write(&output_path, &rendered_content).map_err(|error| {
                error!(
                    "Failed to write output file {}: {}",
                    output_path.display(),
                    error
                );
                TemplateError::io(format!("failed to write output file: {}", error))
            })?;

            debug!("Successfully generated: {}", output_path.display());
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

        let rendered_path = std::path::PathBuf::from(&rendered);

        if rendered_path.components().any(|c| {
            matches!(
                c,
                std::path::Component::ParentDir | std::path::Component::RootDir
            )
        }) {
            error!(
                "Path traversal attempt detected in rendered path: {}",
                rendered
            );
            return Err(TemplateError::security(format!(
                "path traversal attempt detected in rendered path: {}",
                rendered
            )));
        }

        Ok(rendered_path)
    }
}

#[cfg(test)]
#[path = "mustache_template_renderer.tests.rs"]
mod tests;
