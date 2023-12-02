use std::process::{Child, Command, Stdio};

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

    pub fn get_build(target: &String) -> String {
        let output = Command::new("kustomize")
            .arg("build")
            .arg(target)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
            .wait_with_output()
            .unwrap();

        String::from_utf8(output.stdout.to_owned()).unwrap()
    }
}

fn get_script() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.push("diff.sh");
    path.to_str().unwrap().to_string()
}
