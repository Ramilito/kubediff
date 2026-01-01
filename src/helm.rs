use std::path::Path;
use std::process::{Command, Stdio};

/// Check if a directory is a Helm chart (contains Chart.yaml)
pub fn is_helm_directory(target: &str) -> bool {
    Path::new(target).join("Chart.yaml").exists()
}

/// Run helm template on a chart directory with a values file
pub fn build(target: &str, values_file: &str) -> anyhow::Result<String> {
    // Verify the values file exists
    if !Path::new(values_file).exists() {
        return Err(anyhow::anyhow!(
            "Helm values file not found: {}",
            values_file
        ));
    }

    let output = Command::new("helm")
        .args(["template", "release", target, "--values", values_file])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        let stderr = String::from_utf8(output.stderr)?;
        Err(anyhow::anyhow!("Helm template failed: {}", stderr))
    }
}
