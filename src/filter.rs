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
