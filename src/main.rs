mod commands;

use commands::{get_diff, get_target, get_build, pretty_print};
use serde::Deserialize;
use serde_yaml::Value;
use std::io::Write;

// include!(concat!(env!("OUT_DIR"), "/hello.rs"));

fn main() {
    // commands::printThemes();

    let target = get_target();
    let build = get_build(&target);

    for document in serde_yaml::Deserializer::from_str(build.as_str()) {
        let v = Value::deserialize(document).unwrap();
        let string = serde_yaml::to_string(&v).unwrap();
        let mut diff = get_diff();

        diff.stdin
            .as_mut()
            .unwrap()
            .write(string.as_bytes())
            .unwrap();

        let diff = diff.wait_with_output().unwrap();
        let string = String::from_utf8(diff.stdout.to_owned()).unwrap();

        pretty_print(string);
    }
}

