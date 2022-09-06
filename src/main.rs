mod commands;
mod print;
mod settings;

use clap::Parser;
use commands::{get_build, get_diff};
use print::{pretty_print, pretty_print_info, pretty_print_path};
use serde::Deserialize;
use serde_yaml::Value;
use settings::Settings;
use std::io::{self, Write};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser)]
    env: Option<String>,
    #[clap(short, long, action)]
    verbose: bool,
}

fn main() -> Result<(), io::Error> {
    let args = Cli::parse();
    let mut settings = Settings::load()?;

    // settings.configs.env = args.env.unwrap_or_default();
    // let entry = Settings::get_service_paths(&settings)?;

    // for path in entry {
    //     pretty_print_path(format!("Path: {}", path.to_string()));

    //     let build = get_build(&path);
    //     for document in serde_yaml::Deserializer::from_str(build.as_str()) {
    //         let v = Value::deserialize(document).unwrap();

    //         let string = serde_yaml::to_string(&v).unwrap();
    //         let mut diff = get_diff();

    //         diff.stdin
    //             .as_mut()
    //             .unwrap()
    //             .write(string.as_bytes())
    //             .unwrap();

    //         let diff = diff.wait_with_output().unwrap();
    //         let string = String::from_utf8(diff.stdout.to_owned()).unwrap();

    //         if string.len() > 0 {
    //             pretty_print(string);
    //         } else if args.verbose {
    //             pretty_print_info(format!(
    //                 "No changes in: {:?} {:?} {:?}\n",
    //                 v["apiVersion"].as_str().unwrap(),
    //                 v["kind"].as_str().unwrap(),
    //                 v["metadata"]["name"].as_str().unwrap()
    //             ));
    //         }
    //     }
    // }

    let api_resources = commands::get_api_resources();
    for resource in api_resources {
        let result = commands::get_resources(&resource);
        if result.len() > 0 {
            let yaml = serde_yaml::to_value(&result).unwrap();
            pretty_print(yaml.as_str().unwrap().to_string());
        }
    }

    Ok(())
}
