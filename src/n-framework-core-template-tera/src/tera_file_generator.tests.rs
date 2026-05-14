use super::*;
use crate::TeraTemplateRenderer;
use n_framework_core_template_abstractions::{FileGenerator, TemplateContext};
use serde_json::json;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::symlink;
use tempfile::tempdir;

#[test]
fn test_basic_file_generation() {
    let temp = tempdir().unwrap();
    let template_root = temp.path().join("template");
    let output_root = temp.path().join("output");

    fs::create_dir_all(&template_root).unwrap();
    fs::write(template_root.join("hello.txt.tera"), "Hello {{ name }}!").unwrap();
    fs::write(template_root.join("template.yaml"), "name: 'test'").unwrap(); // verify ignore

    let generator = TeraFileGenerator::new(TeraTemplateRenderer::new());
    let mut ctx_data = std::collections::BTreeMap::new();
    ctx_data.insert("name".to_string(), json!("World"));
    let context = TemplateContext::new(ctx_data);

    generator
        .generate(&template_root, &output_root, &context)
        .expect("generation failed");

    let output_file = output_root.join("hello.txt");
    assert!(output_file.exists());
    assert_eq!(fs::read_to_string(output_file).unwrap(), "Hello World!");
    assert!(!output_root.join("template.yaml").exists());
}

#[test]
fn test_path_interpolation() {
    let temp = tempdir().unwrap();
    let template_root = temp.path().join("template");
    let output_root = temp.path().join("output");

    fs::create_dir_all(template_root.join("{{ folder_name }}")).unwrap();
    fs::write(
        template_root.join("{{ folder_name }}/{{ file_name }}.txt.tera"),
        "Content",
    )
    .unwrap();

    let generator = TeraFileGenerator::new(TeraTemplateRenderer::new());
    let mut ctx_data = std::collections::BTreeMap::new();
    ctx_data.insert("folder_name".to_string(), json!("my_folder"));
    ctx_data.insert("file_name".to_string(), json!("hello"));
    let context = TemplateContext::new(ctx_data);

    generator
        .generate(&template_root, &output_root, &context)
        .expect("generation failed");

    assert!(output_root.join("my_folder").is_dir());
    assert!(output_root.join("my_folder/hello.txt").exists());
}

#[test]
fn test_security_path_traversal() {
    let temp = tempdir().unwrap();
    let template_root = temp.path().join("template");
    let output_root = temp.path().join("output");

    fs::create_dir_all(&template_root).unwrap();
    fs::create_dir_all(&output_root).unwrap();
    // Attempt to write outside output_root
    fs::write(
        template_root.join("{{ malicious_path }}.txt.tera"),
        "malicious content",
    )
    .unwrap();

    let generator = TeraFileGenerator::new(TeraTemplateRenderer::new());
    let mut ctx_data = std::collections::BTreeMap::new();
    ctx_data.insert("malicious_path".to_string(), json!("../hacked"));
    let context = TemplateContext::new(ctx_data);

    let result = generator.generate(&template_root, &output_root, &context);
    assert!(
        result.is_err(),
        "Expected path traversal to be blocked and return Err"
    );

    if let Err(e) = result {
        assert!(e.is_security(), "Expected security error, got: {:?}", e);
    } else {
        panic!("Expected Err!");
    }
}

#[test]
fn test_security_template_root_not_dir() {
    let temp = tempdir().unwrap();
    let template_file = temp.path().join("not_a_dir.txt");
    fs::write(&template_file, "content").unwrap();
    let output_root = temp.path().join("output");

    let generator = TeraFileGenerator::new(TeraTemplateRenderer::new());
    let context = TemplateContext::empty();

    let result = generator.generate(&template_file, &output_root, &context);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("must be a directory"));
}

#[test]
fn test_security_template_root_missing() {
    let temp = tempdir().unwrap();
    let template_root = temp.path().join("missing");
    let output_root = temp.path().join("output");

    let generator = TeraFileGenerator::new(TeraTemplateRenderer::new());
    let context = TemplateContext::empty();

    let result = generator.generate(&template_root, &output_root, &context);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("does not exist"));
}

#[test]
fn test_security_symlink_escape_attempt() {
    // Note: Creating symlinks might require specific permissions on some OSes
    // This test might be skipped or simplified if symlinks aren't supported
    #[cfg(unix)]
    {
        let temp = tempdir().unwrap();
        let template_root = temp.path().join("template");
        let output_root = temp.path().join("output");
        let outside_dir = temp.path().join("outside");

        fs::create_dir_all(&template_root).unwrap();
        fs::create_dir_all(&output_root).unwrap();
        fs::create_dir_all(&outside_dir).unwrap();
        fs::write(outside_dir.join("secret.txt"), "secret").unwrap();

        // Create a symlink in the template root pointing outside
        // Even if WalkDir doesn't follow it, we want to ensure no other path leads there
        symlink(&outside_dir, template_root.join("escaped_link")).unwrap();

        let generator = TeraFileGenerator::new(TeraTemplateRenderer::new());
        let context = TemplateContext::empty();

        // Default WalkDir doesn't follow symlinks, so it should just skip it or treat it as a file
        // If it treats it as a file, it will try to render it to the output root
        generator
            .generate(&template_root, &output_root, &context)
            .expect("Should not follow symlink by default");

        assert!(!output_root.join("escaped_link/secret.txt").exists());
    }
}

#[test]
fn test_recursive_directory_generation() {
    let temp = tempdir().unwrap();
    let template_root = temp.path().join("template");
    let output_root = temp.path().join("output");

    let deep_path = template_root.join("a/b/c");
    fs::create_dir_all(&deep_path).unwrap();
    fs::write(deep_path.join("file.txt.tera"), "content").unwrap();

    let generator = TeraFileGenerator::new(TeraTemplateRenderer::new());
    let context = TemplateContext::empty();

    generator
        .generate(&template_root, &output_root, &context)
        .expect("recursive generation failed");

    assert!(output_root.join("a/b/c/file.txt").exists());
}

#[test]
fn test_error_handling_write_failure() {
    let temp = tempdir().unwrap();
    let template_root = temp.path().join("template");
    let output_root = temp.path().join("output");

    fs::create_dir_all(&template_root).unwrap();
    fs::write(template_root.join("file.txt.tera"), "content").unwrap();

    // Create output_root and make it read-only
    fs::create_dir_all(&output_root).unwrap();
    let mut perms = fs::metadata(&output_root).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(&output_root, perms).unwrap();

    let generator = TeraFileGenerator::new(TeraTemplateRenderer::new());
    let context = TemplateContext::empty();

    let result = generator.generate(&template_root, &output_root, &context);

    // Cleanup: make it writable again so tempdir can delete it
    let mut perms = fs::metadata(&output_root).unwrap().permissions();
    #[allow(clippy::permissions_set_readonly_false)]
    perms.set_readonly(false);
    fs::set_permissions(&output_root, perms).unwrap();

    assert!(
        result.is_err(),
        "Expected error when writing to read-only directory"
    );
    assert!(result.unwrap_err().to_string().contains("failed to write"));
}
