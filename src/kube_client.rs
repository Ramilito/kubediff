//! Kubernetes API client wrapper for fetching live resources.
//!
//! This module provides a client that can fetch any Kubernetes resource
//! using dynamic API discovery.

use anyhow::{anyhow, Result};
use kube::{
    api::{Api, DynamicObject, Patch, PatchParams},
    discovery::{ApiCapabilities, ApiResource, Discovery, Scope},
    Client,
};
use serde_json::Value;

/// A Kubernetes client wrapper with API discovery capabilities.
pub struct KubeClient {
    client: Client,
    discovery: Discovery,
}

impl KubeClient {
    /// Create a new KubeClient using the default kubeconfig.
    ///
    /// This will read from:
    /// - `KUBECONFIG` environment variable
    /// - `~/.kube/config`
    /// - In-cluster service account (if running in a pod)
    pub async fn new() -> Result<Self> {
        let client = Client::try_default().await?;
        let discovery = Discovery::new(client.clone()).run().await?;
        Ok(Self { client, discovery })
    }

    /// Find the API resource definition for a given apiVersion and kind.
    fn find_api_resource(
        &self,
        api_version: &str,
        kind: &str,
    ) -> Option<(ApiResource, ApiCapabilities)> {
        // Parse apiVersion into group and version
        // e.g., "apps/v1" -> ("apps", "v1"), "v1" -> ("", "v1")
        let (group, version) = if let Some(pos) = api_version.find('/') {
            (&api_version[..pos], &api_version[pos + 1..])
        } else {
            ("", api_version)
        };

        // Search through discovered API groups
        for group_info in self.discovery.groups() {
            if group_info.name() == group {
                for (ar, caps) in group_info.recommended_resources() {
                    if ar.kind == kind && ar.version == version {
                        return Some((ar, caps));
                    }
                }
            }
        }

        None
    }

    /// Fetch a live resource from the Kubernetes cluster.
    ///
    /// # Arguments
    /// * `api_version` - The API version (e.g., "v1", "apps/v1")
    /// * `kind` - The resource kind (e.g., "ConfigMap", "Deployment")
    /// * `namespace` - The namespace (None for cluster-scoped resources)
    /// * `name` - The resource name
    ///
    /// # Returns
    /// * `Ok(Some(object))` - Resource found
    /// * `Ok(None)` - Resource not found (doesn't exist in cluster)
    /// * `Err(_)` - API error or unknown resource type
    pub async fn get_live_resource(
        &self,
        api_version: &str,
        kind: &str,
        namespace: Option<&str>,
        name: &str,
    ) -> Result<Option<DynamicObject>> {
        let (ar, caps) = self
            .find_api_resource(api_version, kind)
            .ok_or_else(|| anyhow!("Unknown resource type: {}/{}", api_version, kind))?;

        let api: Api<DynamicObject> = match caps.scope {
            Scope::Namespaced => {
                let ns = namespace.unwrap_or("default");
                Api::namespaced_with(self.client.clone(), ns, &ar)
            }
            Scope::Cluster => Api::all_with(self.client.clone(), &ar),
        };

        match api.get_opt(name).await {
            Ok(obj) => Ok(obj),
            Err(kube::Error::Api(err)) if err.code == 404 => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Apply a resource with server-side dry-run to get the normalized result.
    ///
    /// This sends the manifest to the API server which applies all defaults,
    /// validation, and normalization - exactly like kubectl diff does.
    ///
    /// # Arguments
    /// * `manifest` - The resource manifest as a JSON Value
    ///
    /// # Returns
    /// * `Ok(object)` - The normalized resource as it would exist after apply
    /// * `Err(_)` - API error or validation failure
    pub async fn apply_dry_run(&self, manifest: &Value) -> Result<DynamicObject> {
        let api_version = manifest["apiVersion"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing apiVersion"))?;
        let kind = manifest["kind"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing kind"))?;
        let name = manifest["metadata"]["name"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing metadata.name"))?;
        let namespace = manifest["metadata"]["namespace"].as_str();

        let (ar, caps) = self
            .find_api_resource(api_version, kind)
            .ok_or_else(|| anyhow!("Unknown resource type: {}/{}", api_version, kind))?;

        let api: Api<DynamicObject> = match caps.scope {
            Scope::Namespaced => {
                let ns = namespace.unwrap_or("default");
                Api::namespaced_with(self.client.clone(), ns, &ar)
            }
            Scope::Cluster => Api::all_with(self.client.clone(), &ar),
        };

        // Use server-side apply with dry-run - this applies all server defaults
        // Force is needed to bypass field ownership conflicts (safe since it's dry-run only)
        let patch_params = PatchParams::apply("kubediff").dry_run().force();
        let result = api
            .patch(name, &patch_params, &Patch::Apply(manifest))
            .await?;

        Ok(result)
    }
}
