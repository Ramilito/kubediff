use rayon::prelude::*;
use serde::Deserialize;
use serde_yaml::Value;

use std::{collections::HashSet, env};

use crate::{commands::Commands, settings::Settings};

/// Result of diffing a single Kubernetes resource
#[derive(Debug, Clone)]
pub struct DiffResult {
    /// The target path that was processed
    pub target: String,
    /// The Kubernetes resource name (from metadata.name)
    pub resource_name: String,
    /// The API version of the resource
    pub api_version: String,
    /// The kind of the resource (Deployment, Service, etc.)
    pub kind: String,
    /// The diff output if changes exist, None if no changes
    pub diff: Option<String>,
    /// Error message if processing failed for this resource
    pub error: Option<String>,
}

/// Result of processing a single target path
#[derive(Debug, Clone)]
pub struct TargetResult {
    /// The target path that was processed
    pub target: String,
    /// Results for each resource in the target
    pub results: Vec<DiffResult>,
    /// Build errors that occurred before diffing (e.g., kustomize failures)
    pub build_error: Option<String>,
}

pub struct Process;

impl Process {
    /// Get target paths to process based on options and settings
    pub fn get_entries(
        env: Option<String>,
        inplace: bool,
        path: Option<String>,
        settings: &mut Settings,
    ) -> HashSet<String> {
        let mut targets = HashSet::new();

        if inplace {
            let cwd = env::current_dir().unwrap().to_str().unwrap().to_string();
            targets.insert(cwd);
        } else if let Some(p) = path {
            targets.insert(p);
        } else {
            settings.configs.env = env.unwrap_or_default();
            targets = Settings::get_service_paths(settings).expect("");
        }
        targets
    }

    /// Process a single target and return structured results
    pub fn process_target(target: &str) -> TargetResult {
        // Try to get the build output
        let build = match Commands::get_build(target) {
            Ok(b) => b,
            Err(e) => {
                return TargetResult {
                    target: target.to_string(),
                    results: vec![],
                    build_error: Some(e.to_string()),
                };
            }
        };

        // Parse YAML documents, collecting any deserialization errors
        let mut deserialization_errors: Vec<DiffResult> = vec![];
        let documents: Vec<Value> = serde_yaml::Deserializer::from_str(build.as_str())
            .filter_map(|document| {
                let v_result = Value::deserialize(document);
                match v_result {
                    Ok(v) => Some(v),
                    Err(error) => {
                        deserialization_errors.push(DiffResult {
                            target: target.to_string(),
                            resource_name: "unknown".to_string(),
                            api_version: "unknown".to_string(),
                            kind: "unknown".to_string(),
                            diff: None,
                            error: Some(error.to_string()),
                        });
                        None
                    }
                }
            })
            .collect();

        // Process documents in parallel, collecting results
        let mut results: Vec<DiffResult> = documents
            .par_iter()
            .map(|v| {
                let string = serde_yaml::to_string(&v).unwrap();
                let resource_name = v["metadata"]["name"]
                    .as_str()
                    .unwrap_or("unknown")
                    .to_string();
                let api_version = v["apiVersion"].as_str().unwrap_or("unknown").to_string();
                let kind = v["kind"].as_str().unwrap_or("unknown").to_string();

                match Commands::get_diff(&string) {
                    Ok(diff) => {
                        let diff_option = if diff.is_empty() { None } else { Some(diff) };
                        DiffResult {
                            target: target.to_string(),
                            resource_name,
                            api_version,
                            kind,
                            diff: diff_option,
                            error: None,
                        }
                    }
                    Err(e) => DiffResult {
                        target: target.to_string(),
                        resource_name,
                        api_version,
                        kind,
                        diff: None,
                        error: Some(e.to_string()),
                    },
                }
            })
            .collect();

        // Add any deserialization errors to the results
        results.extend(deserialization_errors);

        TargetResult {
            target: target.to_string(),
            results,
            build_error: None,
        }
    }

    /// Process multiple targets and return all results
    pub fn process_targets(targets: HashSet<String>) -> Vec<TargetResult> {
        targets
            .into_iter()
            .map(|target| Self::process_target(&target))
            .collect()
    }
}
