use std::fs::{self, File, Permissions};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

const KUSTOMIZE_VERSION: &str = "v5.8.0";

// Embed the kustomize binary at compile time
const KUSTOMIZE_BINARY: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/kustomize_bin"));

/// Get the path to the kustomize binary, extracting it if necessary
pub fn get_kustomize_path() -> anyhow::Result<PathBuf> {
    let cache_dir = get_cache_dir()?;
    let binary_name = if cfg!(windows) {
        "kustomize.exe"
    } else {
        "kustomize"
    };
    let binary_path = cache_dir.join(binary_name);

    // Extract if not already cached
    if !binary_path.exists() {
        fs::create_dir_all(&cache_dir)?;

        let mut file = File::create(&binary_path)?;
        file.write_all(KUSTOMIZE_BINARY)?;

        // Make executable on Unix
        #[cfg(unix)]
        {
            let permissions = Permissions::from_mode(0o755);
            fs::set_permissions(&binary_path, permissions)?;
        }
    }

    Ok(binary_path)
}

/// Run kustomize build on a directory
pub fn build(target: &str) -> anyhow::Result<String> {
    let kustomize_path = get_kustomize_path()?;

    let output = Command::new(&kustomize_path)
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

fn get_cache_dir() -> anyhow::Result<PathBuf> {
    let cache_base = dirs::cache_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine cache directory"))?;

    Ok(cache_base.join("kubediff").join(format!("kustomize-{}", KUSTOMIZE_VERSION)))
}
