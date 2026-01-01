use std::{env, fs, path::PathBuf, process::Command};

const KUSTOMIZE_VERSION: &str = "v5.8.0";
const BASE_URL: &str = "https://github.com/kubernetes-sigs/kustomize/releases/download";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let binary_path = out_dir.join("kustomize_bin");

    // Skip if already downloaded
    if binary_path.exists() {
        return;
    }

    let target = env::var("TARGET").unwrap();
    let platform = get_platform(&target);

    println!("cargo:warning=Downloading kustomize {KUSTOMIZE_VERSION} for {target}");

    download_and_extract(&platform, &out_dir, &binary_path);
}

/// Map Rust target triple to kustomize platform string
fn get_platform(target: &str) -> &'static str {
    match target {
        t if t.contains("x86_64-unknown-linux") => "linux_amd64",
        t if t.contains("aarch64-unknown-linux") => "linux_arm64",
        t if t.contains("x86_64-apple-darwin") => "darwin_amd64",
        t if t.contains("aarch64-apple-darwin") => "darwin_arm64",
        t if t.contains("x86_64-pc-windows") => "windows_amd64",
        t if t.contains("aarch64-pc-windows") => "windows_arm64",
        _ => panic!("Unsupported platform: {target}"),
    }
}

/// Download and extract kustomize binary
fn download_and_extract(platform: &str, out_dir: &PathBuf, binary_path: &PathBuf) {
    let is_windows = platform.contains("windows");
    let ext = if is_windows { "zip" } else { "tar.gz" };
    let url = format!("{BASE_URL}/kustomize%2F{KUSTOMIZE_VERSION}/kustomize_{KUSTOMIZE_VERSION}_{platform}.{ext}");
    let archive = out_dir.join(format!("kustomize.{ext}"));

    // Download
    let status = Command::new("curl")
        .args(["-sfL", "-o", archive.to_str().unwrap(), &url])
        .status()
        .expect("curl failed");

    assert!(status.success(), "Failed to download kustomize from {url}");

    // Extract
    if is_windows {
        Command::new("tar")
            .args(["-xf", archive.to_str().unwrap(), "-C", out_dir.to_str().unwrap()])
            .status()
            .expect("Failed to extract zip");
        fs::rename(out_dir.join("kustomize.exe"), binary_path).unwrap();
    } else {
        Command::new("tar")
            .args(["-xzf", archive.to_str().unwrap(), "-C", out_dir.to_str().unwrap()])
            .status()
            .expect("Failed to extract tar.gz");
        fs::rename(out_dir.join("kustomize"), binary_path).unwrap();
    }

    fs::remove_file(archive).ok();
}
