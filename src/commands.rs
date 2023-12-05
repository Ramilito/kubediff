use std::{
    io::{Error, ErrorKind, Write},
    process::{Child, Command, Stdio},
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
        let output = Command::new("kubectl")
            .arg("kustomize")
            .arg(target)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?
            .wait_with_output()?;

        match output.status.success() {
            true => Ok(String::from_utf8(output.stdout).expect("Couldn't read stdout of command")),
            false => {
                logger.lock().unwrap().log_error(
                    String::from_utf8(output.stderr).expect("Couldn't read stderr of command"),
                );
                Err(Error::new(
                    ErrorKind::Other,
                    "Kustomize build failed with above error 👆",
                )
                .into())
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
