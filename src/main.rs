mod commands;
mod enums;
mod logger;
mod print;
mod settings;

use crate::{commands::Commands, enums::LogLevel, print::Pretty, settings::Settings};
use clap::Parser;
use serde::Deserialize;
use serde_yaml::Value;
use std::{
    collections::HashSet,
    env,
    io::{self, Write},
};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser)]
    env: Option<String>,
    #[clap(short, long, value_parser)]
    inplace: bool,
    #[clap(short, long, value_parser)]
    path: Option<String>,
    #[clap(short, long, arg_enum)]
    log: Option<LogLevel>,
}

fn main() -> Result<(), io::Error> {
    let args = Cli::parse();
    let targets = get_entries(args);

    for target in targets {
        process_target(&target);
    }

    Ok(())
}

fn get_entries(args: Cli) -> HashSet<String> {
    let mut targets = HashSet::new();
    let mut settings = Settings::load().expect("Failed to load config file!");

    if args.inplace {
        let cwd = env::current_dir().unwrap().to_str().unwrap().to_string();
        targets.insert(cwd);
    } else if args.path.is_some() {
        let path = args.path.unwrap();
        targets.insert(path);
    } else {
        settings.configs.env = args.env.unwrap_or_default();
        targets = Settings::get_service_paths(&settings).expect("");
    }
    return targets;
}

fn process_target(target: &str) {
    Pretty::print_path(format!("Path: {}", target.to_string()));

    let build = Commands::get_build(target);
    for document in serde_yaml::Deserializer::from_str(build.as_str()) {
        let v_result = Value::deserialize(document);

        let v = match v_result {
            Ok(result) => result,
            Err(error) => {
                println!("Handle error");
                Pretty::print_info(error.to_string());
                panic!();
            }
        };

        let string = serde_yaml::to_string(&v).unwrap();
        let mut diff = Commands::get_diff();

        diff.stdin
            .as_mut()
            .unwrap()
            .write(string.as_bytes())
            .unwrap();

        let diff = diff.wait_with_output().unwrap();
        let string = String::from_utf8(diff.stdout.to_owned()).unwrap();

        if string.len() > 0 {
            Pretty::print(string);
        } else {
            // logger::log(
            //     args.log,
            //     settings.configs.log,
            //     format!(
            //         "No changes in: {:?} {:?} {:?}\n",
            //         v["apiVersion"].as_str().unwrap(),
            //         v["kind"].as_str().unwrap(),
            //         v["metadata"]["name"].as_str().unwrap()
            //     ),
            // );
        }
    }
}
