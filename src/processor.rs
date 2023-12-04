use rayon::prelude::*;
use serde::Deserialize;
use serde_yaml::Value;

use std::{
    collections::HashSet,
    env,
    sync::{Arc, Mutex},
};

use crate::{commands::Commands, logger::Logger, print::Pretty, settings::Settings, Cli};
pub struct Process {}

impl Process {
    pub fn get_entries(args: Cli, mut settings: Settings) -> HashSet<String> {
        let mut targets = HashSet::new();

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

    pub fn process_target(logger: &Arc<Mutex<Logger>>, target: &str) -> anyhow::Result<()> {
        Pretty::print_path(format!("Path: {}", target.to_string()));
        let build = Commands::get_build(logger.clone(), target)?;

        let handles: Vec<_> = serde_yaml::Deserializer::from_str(build.as_str())
            .filter_map(|document| {
                let v_result = Value::deserialize(document);

                match handle_deserialization_result(v_result) {
                    Ok(v) => Some(v),
                    Err(error) => {
                        Pretty::print_info(error.to_string());
                        None
                    }
                }
            })
            .collect();

        handles.par_iter().for_each(|v| {
            let logger_clone = Arc::clone(&logger);
            let string = serde_yaml::to_string(&v).unwrap();
            let diff = Commands::get_diff(&string).unwrap();

            if diff.len() > 0 {
                Pretty::print(diff);
            } else {
                let logger = logger_clone.lock().unwrap();
                handle_no_changes(&logger, &v);
            }
        });
        Ok(())
    }
}

fn handle_no_changes(logger: &Logger, v: &Value) {
    logger.log_info(format!(
        "No changes in: {:?} {:?} {:?}\n",
        v["apiVersion"].as_str().unwrap(),
        v["kind"].as_str().unwrap(),
        v["metadata"]["name"].as_str().unwrap()
    ));
}

fn handle_deserialization_result(
    v_result: Result<Value, serde_yaml::Error>,
) -> Result<Value, String> {
    match v_result {
        Ok(result) => Ok(result),
        Err(error) => Err(error.to_string()),
    }
}
