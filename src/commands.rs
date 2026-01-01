use std::{
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use crate::kustomize;

pub struct Commands;

impl Commands {
    pub fn get_diff(input: &str) -> anyhow::Result<String> {
        let mut diff = Command::new("kubectl")
            .arg("diff")
            .arg("-f")
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        if let Some(stdin) = diff.stdin.as_mut() {
            stdin.write_all(input.as_bytes())?;
        }

        let output = diff.wait_with_output()?;
        let result = String::from_utf8(output.stdout)?;
        Ok(result)
    }

    pub fn get_build(target: &str) -> anyhow::Result<String> {
        let path = Path::new(target);

        // Single file - just read it
        if path.is_file() {
            return Ok(fs::read_to_string(path)?);
        }

        // Kustomize directory - use embedded kustomize
        if is_kustomize_directory(target)? {
            return kustomize::build(target);
        }

        // Regular directory - concatenate all YAML files
        let mut combined_output = String::new();
        for entry in fs::read_dir(target)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "yaml" || ext == "yml" {
                        let content = fs::read_to_string(&path)?;
                        combined_output.push_str(&content);
                        // Ensure documents are separated
                        if !combined_output.ends_with('\n') {
                            combined_output.push('\n');
                        }
                        combined_output.push_str("---\n");
                    }
                }
            }
        }

        Ok(combined_output)
    }
}

fn is_kustomize_directory(target: &str) -> anyhow::Result<bool> {
    for entry in fs::read_dir(target)? {
        let entry = entry?;
        let file_name = entry.file_name();
        if file_name == "kustomization.yaml"
            || file_name == "kustomization.yml"
            || file_name == "Kustomization"
        {
            return Ok(true);
        }
    }
    Ok(false)
}
