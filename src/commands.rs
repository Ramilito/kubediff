use std::{fs, path::Path};

use serde_json::Value;

use crate::{diff::generate_diff, filter::filter_resource, kube_client::KubeClient, kustomize};

pub struct Commands;

impl Commands {
    /// Get diff for a single Kubernetes resource using kube.rs.
    ///
    /// Uses server-side dry-run apply to get the normalized local manifest with
    /// all server defaults applied, then compares it to the live resource.
    /// This matches kubectl diff behavior exactly.
    pub async fn get_diff(client: &KubeClient, input: &str) -> anyhow::Result<String> {
        // Parse local YAML to JSON
        let local_value: Value = serde_yaml::from_str(input)?;

        // Extract resource identifiers for diff header
        let kind = local_value["kind"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing kind"))?;
        let name = local_value["metadata"]["name"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing metadata.name"))?;
        let resource_id = format!("{}/{}", kind, name);

        // Apply local manifest with dry-run to get server-normalized version
        // This applies all server defaults, just like kubectl diff does
        let dry_run_result = client.apply_dry_run(&local_value).await?;
        let mut local_normalized: Value = serde_json::to_value(&dry_run_result)?;

        // Fetch live resource from cluster
        let api_version = local_value["apiVersion"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing apiVersion"))?;
        let namespace = local_value["metadata"]["namespace"].as_str();
        let live = client
            .get_live_resource(api_version, kind, namespace, name)
            .await?;

        match live {
            None => {
                // Resource doesn't exist in cluster - show as new
                filter_resource(&mut local_normalized);
                let local_yaml = serde_yaml::to_string(&local_normalized)?;

                Ok(generate_diff(&resource_id, "", &local_yaml).unwrap_or_default())
            }
            Some(live_obj) => {
                // Compare normalized local with live
                let mut live_value: Value = serde_json::to_value(&live_obj)?;

                // Apply filters to both (remove status, managedFields, etc.)
                filter_resource(&mut live_value);
                filter_resource(&mut local_normalized);

                // Convert to YAML for diff
                let live_yaml = serde_yaml::to_string(&live_value)?;
                let local_yaml = serde_yaml::to_string(&local_normalized)?;

                Ok(generate_diff(&resource_id, &live_yaml, &local_yaml).unwrap_or_default())
            }
        }
    }

    /// Build Kubernetes manifests from a target path.
    ///
    /// Handles:
    /// - Single YAML files
    /// - Kustomize directories (uses embedded kustomize binary)
    /// - Regular directories (concatenates all YAML files)
    pub fn get_build(target: &str) -> anyhow::Result<String> {
        let path = Path::new(target);

        // Single file - just read it
        if path.is_file() {
            return Ok(fs::read_to_string(path)?);
        }

        // Kustomize directory - use embedded kustomize
        if is_kustomize_directory(target)? {
            return kustomize::build(target);
        }

        // Regular directory - concatenate all YAML files
        let mut combined_output = String::new();
        for entry in fs::read_dir(target)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "yaml" || ext == "yml" {
                        let content = fs::read_to_string(&path)?;
                        combined_output.push_str(&content);
                        // Ensure documents are separated
                        if !combined_output.ends_with('\n') {
                            combined_output.push('\n');
                        }
                        combined_output.push_str("---\n");
                    }
                }
            }
        }

        Ok(combined_output)
    }
}

fn is_kustomize_directory(target: &str) -> anyhow::Result<bool> {
    for entry in fs::read_dir(target)? {
        let entry = entry?;
        let file_name = entry.file_name();
        if file_name == "kustomization.yaml"
            || file_name == "kustomization.yml"
            || file_name == "Kustomization"
        {
            return Ok(true);
        }
    }
    Ok(false)
}
