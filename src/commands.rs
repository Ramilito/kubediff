use std::{
    io::{Error, ErrorKind},
    process::{Child, Command, Stdio},
};

use crate::logger::Logger;

pub struct Commands {}

impl Commands {
    pub fn get_diff() -> Child {
        Command::new("kubectl")
            .env("KUBECTL_EXTERNAL_DIFF", format!("{}", get_script()))
            .arg("diff")
            .arg("-f")
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
    }

    pub fn get_build(logger: &Logger, target: &str) -> anyhow::Result<String> {
        let output = Command::new("kustomize")
            .arg("build")
            .arg(target)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?
            .wait_with_output()?;

        match output.status.success() {
            true => Ok(String::from_utf8(output.stdout).expect("Couldn't read stdout of command")),
            false => {
                logger.log_error(
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
