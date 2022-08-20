mod commands;

use serde::Deserialize;
use serde_yaml::Value;
use std::{
    io::Write,
    process::{Command, Stdio},
};

// include!(concat!(env!("OUT_DIR"), "/hello.rs"));

fn main() {
    // commands::printThemes();

    let target = commands::get_target();
    let build = commands::get_build(&target);

    for document in serde_yaml::Deserializer::from_str(build.as_str()) {
        let v = Value::deserialize(document).unwrap();
        let string = serde_yaml::to_string(&v).unwrap();
        let mut diff = Command::new("kubectl")
            .env("KUBECTL_EXTERNAL_DIFF", format!("{}", commands::get_script()))
            .arg("diff")
            .arg("-f")
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        diff.stdin
            .as_mut()
            .unwrap()
            .write(string.as_bytes())
            .unwrap();

        let diff = diff.wait_with_output().unwrap();
        let string = String::from_utf8(diff.stdout.to_owned()).unwrap();
        commands::pretty_print(string);
    }
}
