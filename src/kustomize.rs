use std::process::{Command, Stdio};

/// Run kustomize build on a directory
pub fn build(target: &str) -> anyhow::Result<String> {
    let output = Command::new("kustomize")
        .arg("build")
        .arg(target)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        let stderr = String::from_utf8(output.stderr)?;
        Err(anyhow::anyhow!("Kustomize build failed: {}", stderr))
    }
}
