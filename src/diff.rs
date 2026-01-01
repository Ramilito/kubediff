//! Diff generation for Kubernetes resources.
//!
//! This module uses the `similar` crate to generate unified diff output
//! between the live cluster state and local manifest.

use similar::{ChangeTag, TextDiff};

/// Generate a unified diff between live (original) and local (modified) YAML.
///
/// Returns `None` if there are no changes, otherwise returns the diff string.
///
/// # Arguments
/// * `name` - Resource identifier for the diff header (e.g., "Deployment/my-app")
/// * `live` - The live cluster state as YAML (empty string if resource doesn't exist)
/// * `local` - The local manifest as YAML
pub fn generate_diff(name: &str, live: &str, local: &str) -> Option<String> {
    let diff = TextDiff::from_lines(live, local);

    // Check if there are any changes
    let has_changes = diff
        .iter_all_changes()
        .any(|c| c.tag() != ChangeTag::Equal);
    if !has_changes {
        return None;
    }

    let mut output = String::new();

    // Header lines (like git diff)
    output.push_str(&format!("--- a/{}\n", name));
    output.push_str(&format!("+++ b/{}\n", name));

    // Generate unified diff with context
    for hunk in diff.unified_diff().context_radius(3).iter_hunks() {
        output.push_str(&format!("{}", hunk));
    }

    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_changes_returns_none() {
        let yaml = "apiVersion: v1\nkind: ConfigMap\n";
        let result = generate_diff("ConfigMap/test", yaml, yaml);
        assert!(result.is_none());
    }

    #[test]
    fn test_new_resource() {
        let local = "apiVersion: v1\nkind: ConfigMap\nmetadata:\n  name: test\n";
        let result = generate_diff("ConfigMap/test", "", local);

        assert!(result.is_some());
        let diff = result.unwrap();
        assert!(diff.contains("--- a/ConfigMap/test"));
        assert!(diff.contains("+++ b/ConfigMap/test"));
        assert!(diff.contains("+apiVersion: v1"));
    }

    #[test]
    fn test_modified_resource() {
        let live = "apiVersion: v1\nkind: ConfigMap\ndata:\n  key: old-value\n";
        let local = "apiVersion: v1\nkind: ConfigMap\ndata:\n  key: new-value\n";
        let result = generate_diff("ConfigMap/test", live, local);

        assert!(result.is_some());
        let diff = result.unwrap();
        assert!(diff.contains("-  key: old-value"));
        assert!(diff.contains("+  key: new-value"));
    }

    #[test]
    fn test_deleted_lines() {
        let live = "apiVersion: v1\nkind: ConfigMap\ndata:\n  key1: value1\n  key2: value2\n";
        let local = "apiVersion: v1\nkind: ConfigMap\ndata:\n  key1: value1\n";
        let result = generate_diff("ConfigMap/test", live, local);

        assert!(result.is_some());
        let diff = result.unwrap();
        assert!(diff.contains("-  key2: value2"));
    }
}
