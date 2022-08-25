mod commands;
mod settings;

use commands::{get_build, get_diff, pretty_print};
use serde::Deserialize;
use serde_yaml::Value;
use settings::Settings;
use std::io::{self, Write};

// include!(concat!(env!("OUT_DIR"), "/hello.rs"));

fn main() -> Result<(), io::Error> {
    // commands::printThemes();
    let settings = Settings::load()?;

    let entry = Settings::get_service_paths(&settings)?;
    // println!("{:?}", entry);

    for path in entry {
        let build = get_build(&path);
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

    Ok(())
}
