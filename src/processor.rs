use rayon::prelude::*;
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

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

        let handles: Vec<_> = YamlLoader::load_from_str(build.as_str())?
            .into_iter()
            .filter_map(|yaml| Some(yaml.clone()))
            .collect();

        handles.par_iter().for_each(|v| {
            let logger_clone = Arc::clone(&logger);
            let mut string = String::new();
            {
                let mut emitter = YamlEmitter::new(&mut string);
                emitter.dump(v).unwrap();
            }
            let diff = Commands::get_diff(&string).unwrap();

            if !diff.is_empty() {
                Pretty::print(diff);
            } else {
                let logger = logger_clone.lock().unwrap();
                handle_no_changes(&logger, &v);
            }
        });

        Ok(())
    }
}

fn handle_no_changes(logger: &Logger, v: &Yaml) {
    logger.log_info(format!(
        "No changes in: {:?} {:?} {:?}\n",
        v["apiVersion"].as_str().unwrap(),
        v["kind"].as_str().unwrap(),
        v["metadata"]["name"].as_str().unwrap()
    ));
}
