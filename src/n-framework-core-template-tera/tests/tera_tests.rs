use std::fs;
use tempfile::tempdir;
use serde_json::json;
use n_framework_core_template_abstractions::{FileGenerator, TemplateContext};
use n_framework_core_template_tera::{TeraFileGenerator, TeraTemplateRenderer};

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
    
    generator.generate(&template_root, &output_root, &context).expect("generation failed");
    
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
    fs::write(template_root.join("{{ folder_name }}/{{ file_name }}.txt.tera"), "Content").unwrap();
    
    let generator = TeraFileGenerator::new(TeraTemplateRenderer::new());
    let mut ctx_data = std::collections::BTreeMap::new();
    ctx_data.insert("folder_name".to_string(), json!("my_folder"));
    ctx_data.insert("file_name".to_string(), json!("hello"));
    let context = TemplateContext::new(ctx_data);
    
    generator.generate(&template_root, &output_root, &context).expect("generation failed");
    
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
    fs::write(template_root.join("{{ malicious_path }}.txt.tera"), "malicious content").unwrap();
    
    let generator = TeraFileGenerator::new(TeraTemplateRenderer::new());
    let mut ctx_data = std::collections::BTreeMap::new();
    ctx_data.insert("malicious_path".to_string(), json!("../hacked"));
    let context = TemplateContext::new(ctx_data);
    
    let result = generator.generate(&template_root, &output_root, &context);
    assert!(result.is_err(), "Expected path traversal to be blocked and return Err");
    
    if let Err(e) = result {
        assert!(e.is_security(), "Expected security error, got: {:?}", e);
    } else {
        panic!("Expected Err!");
    }
}
