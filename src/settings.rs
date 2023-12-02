use glob::glob;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io::{BufReader, Error};
use std::path::Path;
use std::{fs::File, io};

use crate::enums::LogLevel;

#[derive(Debug, Serialize, Deserialize)]
pub struct Configs {
    #[serde(default)]
    pub include: Vec<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub env: String,
    #[serde(default)]
    pub log: LogLevel,
}

pub fn expanduser(path: &str) -> String {
    if path.starts_with("~/") {
        format!("{}/{}", dirs::home_dir().unwrap().display(), &path[2..])
    } else {
        path.to_string()
    }
}

#[derive(Debug, Deserialize)]
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
        let file = File::open(settings_path)?;
        let reader = BufReader::new(file);
        let settings: Settings = serde_yaml::from_reader(reader).unwrap();

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
}
