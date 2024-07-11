use std::{
    fs,
    io::{Error, ErrorKind, Write},
    path::Path,
    process::{Command, Output, Stdio},
    sync::{Arc, Mutex},
};

use crate::logger::Logger;

pub struct Commands {}

impl Commands {
    pub fn get_diff(input: &String) -> anyhow::Result<String> {
        let mut diff = Command::new("kubectl")
            .env("KUBECTL_EXTERNAL_DIFF", format!("{}", get_script()))
            .arg("diff")
            .arg("-f")
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let stdin = diff.stdin.as_mut().unwrap().write_all(input.as_bytes());
        drop(stdin);

        let diff = diff.wait_with_output().unwrap();
        let string = String::from_utf8(diff.stdout.to_owned()).unwrap();
        Ok(string)
    }

    pub fn get_build(logger: Arc<Mutex<Logger>>, target: &str) -> anyhow::Result<String> {
        let mut args = vec![];
        let output: Output;
        if Path::new(&target).is_file() {
            args.push(target);
            output = Command::new("cat")
                .args(args)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?
                .wait_with_output()?;
        } else {
            let file_exists = fs::read_dir(target)?.filter_map(Result::ok).any(|entry| {
                entry.path().is_file()
                    && (entry.file_name() == "kustomization.yaml"
                        || entry.file_name() == "kustomization.yml"
                        || entry.file_name() == "Kustomization")
            });
            if file_exists {
                args.push("kustomize");
                args.push(target);
                output = Command::new("kubectl")
                    .args(args)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()?
                    .wait_with_output()?;
            } else {
                let mut combined_output = String::new();
                for entry in fs::read_dir(target)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_file() {
                        let file_args = vec![path.to_str().unwrap()];
                        let file_output = Command::new("cat")
                            .args(&file_args)
                            .stdin(Stdio::piped())
                            .stdout(Stdio::piped())
                            .stderr(Stdio::piped())
                            .spawn()?
                            .wait_with_output()?;

                        if !file_output.status.success() {
                            logger.lock().unwrap().log_error(
                                String::from_utf8(file_output.stderr)
                                    .expect("Couldn't read stderr of command"),
                            );
                            return Err(Error::new(
                                ErrorKind::Other,
                                "Build failed with above error 👆",
                            )
                            .into());
                        }

                        combined_output.push_str(&String::from_utf8(file_output.stdout)?);
                    }
                }
                return Ok(combined_output);
            }
        }

        match output.status.success() {
            true => Ok(String::from_utf8(output.stdout).expect("Couldn't read stdout of command")),
            false => {
                logger.lock().unwrap().log_error(
                    String::from_utf8(output.stderr).expect("Couldn't read stderr of command"),
                );
                Err(Error::new(ErrorKind::Other, "Build failed with above error 👆").into())
            }
        }
    }
}

fn get_script() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.push("diff.sh");
    path.to_str().unwrap().to_string()
}
