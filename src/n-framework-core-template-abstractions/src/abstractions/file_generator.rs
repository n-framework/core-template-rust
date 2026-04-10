use std::path::Path;

use crate::errors::TemplateError;
use crate::template_context::TemplateContext;

/// Policy for handling existing output files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverwritePolicy {
    /// Overwrite existing files.
    Overwrite,
    /// Skip existing files.
    Skip,
    /// Fail if any output file exists.
    Fail,
    /// Prompt for each conflict (requires interactive mode - not yet implemented).
    Prompt,
}

/// Trait for generating files from templates.
///
/// Implementors provide the logic to recursively process template directories,
/// render template files with a context, and write output to a destination directory.
///
/// # Overwrite Policy
///
/// By default, existing files are overwritten. Use `AtomicFileGenerator` for
/// transactional guarantees where partial failures can be rolled back.
///
/// # Example
/// ```
/// use n_framework_core_template_abstractions::{FileGenerator, TemplateContext};
/// use std::path::Path;
///
/// fn example_usage<G: FileGenerator>(generator: &G, context: &TemplateContext) {
///     // Generate files from templates to output
///     generator.generate(
///         Path::new("./templates"),
///         Path::new("./output"),
///         context,
///     ).expect("Failed to generate files");
/// }
/// ```
pub trait FileGenerator {
    /// Generates files from templates in the template_root directory to output_root.
    ///
    /// # Arguments
    /// * `template_root` - The root directory containing template files
    /// * `output_root` - The root directory where rendered files will be written
    /// * `context` - The template context containing variable values
    ///
    /// # Returns
    /// * `Ok(())` - All files generated successfully
    /// * `Err(TemplateError)` - If generation fails
    ///
    /// # Overwrite Policy
    /// By default, existing output files are overwritten. Use `AtomicFileGenerator`
    /// for transactional behavior with rollback on failure.
    fn generate(
        &self,
        template_root: &Path,
        output_root: &Path,
        context: &TemplateContext,
    ) -> Result<(), TemplateError>;
}

/// Trait for atomic file generation with rollback capability.
///
/// Implementors provide transactional file generation where partial failures
/// can be rolled back, ensuring the output directory is not left in a
/// partially modified state.
pub trait AtomicFileGenerator: FileGenerator {
    /// Generates files atomically with rollback on failure.
    ///
    /// If any file fails to generate, already-created files in output_root
    /// will be removed to maintain consistency.
    ///
    /// # Arguments
    /// * `template_root` - The root directory containing template files
    /// * `output_root` - The root directory where rendered files will be written
    /// * `context` - The template context containing variable values
    /// * `overwrite_policy` - Policy for handling existing files
    ///
    /// # Returns
    /// * `Ok(())` - All files generated successfully
    /// * `Err(TemplateError)` - If generation fails (output rolled back)
    fn generate_atomic(
        &self,
        template_root: &Path,
        output_root: &Path,
        context: &TemplateContext,
        overwrite_policy: OverwritePolicy,
    ) -> Result<(), TemplateError>;
}
