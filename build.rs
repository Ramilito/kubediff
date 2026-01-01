use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

const KUSTOMIZE_VERSION: &str = "v5.8.0";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();
    let target = env::var("TARGET").unwrap();

    // Determine the correct kustomize binary URL for the target platform
    let (url, archive_type, binary_name) = get_kustomize_url(&target);

    let archive_path = Path::new(&out_dir).join(format!("kustomize_archive.{}", archive_type));
    let binary_path = Path::new(&out_dir).join("kustomize_bin");

    // Skip download if binary already exists (for faster rebuilds)
    if binary_path.exists() {
        println!("cargo:warning=Using cached kustomize binary");
        return;
    }

    println!("cargo:warning=Downloading kustomize {} for {}", KUSTOMIZE_VERSION, target);

    // Download the archive
    download_file(&url, &archive_path).expect("Failed to download kustomize");

    // Extract the binary
    extract_kustomize(&archive_path, &binary_path, archive_type, binary_name)
        .expect("Failed to extract kustomize");

    // Clean up archive
    let _ = fs::remove_file(&archive_path);

    println!("cargo:warning=Kustomize binary ready at {:?}", binary_path);
}

fn get_kustomize_url(target: &str) -> (String, &'static str, &'static str) {
    let base_url = format!(
        "https://github.com/kubernetes-sigs/kustomize/releases/download/kustomize%2F{}/kustomize_{}",
        KUSTOMIZE_VERSION, KUSTOMIZE_VERSION
    );

    let (platform, archive_type, binary_name) = if target.contains("x86_64") && target.contains("linux") {
        ("linux_amd64.tar.gz", "tar.gz", "kustomize")
    } else if target.contains("aarch64") && target.contains("linux") {
        ("linux_arm64.tar.gz", "tar.gz", "kustomize")
    } else if target.contains("x86_64") && target.contains("darwin") {
        ("darwin_amd64.tar.gz", "tar.gz", "kustomize")
    } else if target.contains("aarch64") && target.contains("darwin") {
        ("darwin_arm64.tar.gz", "tar.gz", "kustomize")
    } else if target.contains("x86_64") && target.contains("windows") {
        ("windows_amd64.zip", "zip", "kustomize.exe")
    } else if target.contains("aarch64") && target.contains("windows") {
        ("windows_arm64.zip", "zip", "kustomize.exe")
    } else {
        panic!("Unsupported target platform: {}. Supported: linux/darwin/windows for x86_64/aarch64", target);
    };

    (format!("{}_{}", base_url, platform), archive_type, binary_name)
}

fn download_file(url: &str, dest: &Path) -> io::Result<()> {
    // Use curl or wget via command line for simplicity in build script
    let status = std::process::Command::new("curl")
        .args(["-L", "-o", dest.to_str().unwrap(), url])
        .status()?;

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to download from {}", url),
        ));
    }

    Ok(())
}

fn extract_kustomize(
    archive_path: &Path,
    dest: &Path,
    archive_type: &str,
    binary_name: &str,
) -> io::Result<()> {
    match archive_type {
        "tar.gz" => extract_tar_gz(archive_path, dest, binary_name),
        "zip" => extract_zip(archive_path, dest, binary_name),
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            "Unknown archive type",
        )),
    }
}

fn extract_tar_gz(archive_path: &Path, dest: &Path, binary_name: &str) -> io::Result<()> {
    use flate2::read::GzDecoder;
    use tar::Archive;

    let file = File::open(archive_path)?;
    let gz = GzDecoder::new(file);
    let mut archive = Archive::new(gz);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        if path.file_name().map(|n| n.to_str()) == Some(Some(binary_name)) {
            let mut contents = Vec::new();
            entry.read_to_end(&mut contents)?;
            let mut out_file = File::create(dest)?;
            out_file.write_all(&contents)?;
            return Ok(());
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("Binary '{}' not found in archive", binary_name),
    ))
}

fn extract_zip(archive_path: &Path, dest: &Path, binary_name: &str) -> io::Result<()> {
    use zip::ZipArchive;

    let file = File::open(archive_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        if file.name() == binary_name {
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)?;
            let mut out_file = File::create(dest)?;
            out_file.write_all(&contents)?;
            return Ok(());
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("Binary '{}' not found in archive", binary_name),
    ))
}
