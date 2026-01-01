//! kubediff - A library for comparing Kubernetes manifests against live cluster state
//!
//! This library provides the core diff logic for comparing local Kubernetes YAML manifests
//! against what's currently deployed in your cluster using the kube.rs client library.
//!
//! # Example
//!
//! ```rust,no_run
//! use kubediff::{KubeClient, Settings, Process, DiffResult};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let mut settings = Settings::load()?;
//!
//!     // Initialize the Kubernetes client
//!     let client = KubeClient::new().await?;
//!
//!     // Get targets from settings or specify directly
//!     let targets = Process::get_entries(
//!         Some("production".to_string()),  // env
//!         false,                             // inplace
//!         None,                              // path
//!         &mut settings,
//!     );
//!
//!     for target in targets {
//!         let result = Process::process_target(&client, &target).await;
//!
//!         if let Some(error) = result.build_error {
//!             eprintln!("Build error for {}: {}", target, error);
//!             continue;
//!         }
//!
//!         for diff_result in result.results {
//!             if let Some(diff) = &diff_result.diff {
//!                 println!("Changes in {} {}: {}",
//!                     diff_result.kind,
//!                     diff_result.resource_name,
//!                     diff
//!                 );
//!             } else if diff_result.error.is_none() {
//!                 println!("No changes in {} {}",
//!                     diff_result.kind,
//!                     diff_result.resource_name
//!                 );
//!             }
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod commands;
pub mod diff;
pub mod enums;
pub mod filter;
pub mod helm;
pub mod kube_client;
pub mod kustomize;
pub mod processor;
pub mod settings;

// Re-export main types for convenience
pub use enums::LogLevel;
pub use kube_client::KubeClient;
pub use processor::{DiffResult, Process, TargetResult};
pub use settings::Settings;
