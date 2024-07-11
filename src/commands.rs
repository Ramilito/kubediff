use std::{
    fs,
    io::{Error, ErrorKind, Write},
    path::Path,
    process::{Command, Stdio},
    sync::{Arc, Mutex},
};

use crate::logger::Logger;

pub struct Commands;

impl Commands {
    pub fn get_diff(input: &str) -> anyhow::Result<String> {
        let script_path = get_script();
        let mut diff = Command::new("kubectl")
            .env("KUBECTL_EXTERNAL_DIFF", script_path)
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

    pub fn get_build(logger: Arc<Mutex<Logger>>, target: &str) -> anyhow::Result<String> {
        if Path::new(target).is_file() {
            return run_command(&["cat", target], logger.clone());
        }

        if is_kustomize_directory(target)? {
            return run_command(&["kubectl", "kustomize", target], logger.clone());
        }

        let mut combined_output = String::new();
        for entry in fs::read_dir(target)? {
            let entry = entry?;
            if entry.path().is_file() {
                let e = entry.path();
                let file_path = e.to_str().unwrap();
                let output = run_command(&["cat", file_path], logger.clone())?;
                combined_output.push_str(&output);
            }
        }

        Ok(combined_output)
    }
}

fn get_script() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.push("diff.sh");
    path.to_str().unwrap().to_string()
}

fn run_command(args: &[&str], logger: Arc<Mutex<Logger>>) -> anyhow::Result<String> {
    let output = Command::new(args[0])
        .args(&args[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        let stderr = String::from_utf8(output.stderr)?;
        logger.lock().unwrap().log_error(stderr.clone());
        Err(Error::new(
            ErrorKind::Other,
            format!("Build failed with error: {}", stderr),
        )
        .into())
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
