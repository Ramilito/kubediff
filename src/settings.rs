use glob::glob;
use std::collections::HashSet;
use std::io::{BufReader, Error, Read};
use std::path::Path;
use std::{fs::File, io};
use yaml_rust::{Yaml, YamlLoader};

use crate::enums::LogLevel;

#[derive(Debug)]
pub struct Configs {
    pub include: Vec<String>,
    pub exclude: Vec<String>,
    pub env: String,
    pub log: LogLevel,
}

pub fn expanduser(path: &str) -> String {
    if path.starts_with("~/") {
        format!("{}/{}", dirs::home_dir().unwrap().display(), &path[2..])
    } else {
        path.to_string()
    }
}

#[derive(Debug)]
pub struct Settings {
    pub configs: Configs,
}

impl Settings {
    pub fn path() -> String {
        format!(
            "{}/.kube/kubediff/config.yaml",
            dirs::home_dir().unwrap().display()
        )
    }

    pub fn load() -> Result<Settings, io::Error> {
        let settings_path_str = Self::path();
        let settings_path = Path::new(&settings_path_str);
        let mut file = File::open(settings_path)?;

        // let reader = BufReader::new(file);
        let mut yaml_str = String::new();
        _ = file.read_to_string(&mut yaml_str);

        let yaml_docs = YamlLoader::load_from_str(&yaml_str).unwrap();
        let settings = Settings {
            configs: Settings::deserialize_configs(&yaml_docs[0]),
        };

        Ok(settings)
    }

    pub fn get_service_paths(&self) -> Result<HashSet<String>, Error> {
        let mut paths = HashSet::new();
        let env: String = self.configs.env.to_string();
        for inc in &self.configs.include {
            let expanded = expanduser(&inc);
            for entry in glob(&expanded).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        paths.insert(format!("{}/{}", path.display().to_string(), env));
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            }
        }
        for exc in &self.configs.exclude {
            let expanded = expanduser(&exc);
            for entry in glob(&expanded).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        paths.remove(&format!("{}/{}", path.display().to_string(), env));
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            }
        }

        Ok(paths)
    }
    fn deserialize_configs(yaml: &Yaml) -> Configs {
        let include = yaml["configs"]["include"].as_vec().map_or_else(
            || Vec::new(),
            |v| v.iter().map(|y| y.as_str().unwrap().to_owned()).collect(),
        );

        let exclude = yaml["configs"]["exclude"].as_vec().map_or_else(
            || Vec::new(),
            |v| v.iter().map(|y| y.as_str().unwrap().to_owned()).collect(),
        );

        let env = yaml["configs"]["env"].as_str().unwrap_or_default().to_owned();

        let test = yaml["configs"]["log"].as_str().unwrap_or_default().to_owned();
        println!("{:?}", include);
        // let log = yaml["log"] as LogLevel;
            // .as_i64()
            // .map_or(LogLevel::default(), |l| l );
        let log = LogLevel::Info;

        Configs {
            include,
            exclude,
            env,
            log,
        }
    }
}
