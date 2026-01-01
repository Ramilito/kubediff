//! Field filtering for Kubernetes resources before diffing.
//!
//! This module removes server-managed fields from Kubernetes resources
//! so that diffs only show meaningful changes.

use serde_json::Value;

/// Annotations to remove from resources before diffing
const ANNOTATIONS_TO_REMOVE: &[&str] = &[
    "kubectl.kubernetes.io/last-applied-configuration",
    "argocd.argoproj.io/tracking-id",
];

/// Filter out server-managed fields from a Kubernetes resource.
///
/// This removes fields that are set by the Kubernetes API server and
/// shouldn't be compared when diffing local manifests against live state.
pub fn filter_resource(value: &mut Value) {
    let Some(obj) = value.as_object_mut() else {
        return;
    };

    // Remove status field (always server-managed)
    obj.remove("status");

    // Remove webhooks (often has server-injected caBundle)
    obj.remove("webhooks");

    // Filter metadata fields
    if let Some(metadata) = obj.get_mut("metadata").and_then(|m| m.as_object_mut()) {
        metadata.remove("managedFields");
        metadata.remove("ownerReferences");
        metadata.remove("generation");
        metadata.remove("creationTimestamp");
        metadata.remove("resourceVersion");
        metadata.remove("uid");
        metadata.remove("selfLink");

        // Remove specific annotations
        if let Some(annotations) = metadata.get_mut("annotations").and_then(|a| a.as_object_mut())
        {
            for annotation in ANNOTATIONS_TO_REMOVE {
                annotations.remove(*annotation);
            }
            // Remove empty annotations object
            if annotations.is_empty() {
                metadata.remove("annotations");
            }
        }
    }

    // Remove caBundle from spec if present
    if let Some(spec) = obj.get_mut("spec").and_then(|s| s.as_object_mut()) {
        spec.remove("caBundle");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_filter_removes_managed_fields() {
        let mut value = json!({
            "apiVersion": "v1",
            "kind": "ConfigMap",
            "metadata": {
                "name": "test",
                "namespace": "default",
                "managedFields": [{"manager": "kubectl"}],
                "generation": 1,
                "creationTimestamp": "2024-01-01T00:00:00Z",
                "resourceVersion": "12345",
                "uid": "abc-123"
            },
            "data": {
                "key": "value"
            }
        });

        filter_resource(&mut value);

        let metadata = value["metadata"].as_object().unwrap();
        assert!(metadata.get("managedFields").is_none());
        assert!(metadata.get("generation").is_none());
        assert!(metadata.get("creationTimestamp").is_none());
        assert!(metadata.get("resourceVersion").is_none());
        assert!(metadata.get("uid").is_none());
        assert_eq!(metadata.get("name").unwrap(), "test");
        assert_eq!(metadata.get("namespace").unwrap(), "default");
    }

    #[test]
    fn test_filter_removes_status() {
        let mut value = json!({
            "apiVersion": "apps/v1",
            "kind": "Deployment",
            "metadata": {"name": "test"},
            "spec": {"replicas": 1},
            "status": {"readyReplicas": 1}
        });

        filter_resource(&mut value);

        assert!(value.get("status").is_none());
        assert!(value.get("spec").is_some());
    }

    #[test]
    fn test_filter_removes_annotations() {
        let mut value = json!({
            "apiVersion": "v1",
            "kind": "ConfigMap",
            "metadata": {
                "name": "test",
                "annotations": {
                    "kubectl.kubernetes.io/last-applied-configuration": "{}",
                    "argocd.argoproj.io/tracking-id": "abc",
                    "custom-annotation": "keep-me"
                }
            }
        });

        filter_resource(&mut value);

        let annotations = value["metadata"]["annotations"].as_object().unwrap();
        assert!(annotations
            .get("kubectl.kubernetes.io/last-applied-configuration")
            .is_none());
        assert!(annotations.get("argocd.argoproj.io/tracking-id").is_none());
        assert_eq!(annotations.get("custom-annotation").unwrap(), "keep-me");
    }

    #[test]
    fn test_filter_removes_empty_annotations() {
        let mut value = json!({
            "apiVersion": "v1",
            "kind": "ConfigMap",
            "metadata": {
                "name": "test",
                "annotations": {
                    "kubectl.kubernetes.io/last-applied-configuration": "{}"
                }
            }
        });

        filter_resource(&mut value);

        let metadata = value["metadata"].as_object().unwrap();
        assert!(metadata.get("annotations").is_none());
    }
}
