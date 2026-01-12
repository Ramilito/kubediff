# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

kubediff is a Rust CLI tool that wraps Kubernetes manifests to provide enhanced diff capabilities. It compares local Kubernetes manifests against live cluster state with clean, beautified output by removing server-managed noise (managedFields, status, etc.).

## Build Commands

```bash
make build          # Build debug binary, copy config.yaml to target/debug/
make build_local    # Build release binary, install to ~/.kube/kubediff/
make run            # Run with -e local flag
cargo build         # Standard Rust debug build
cargo build --release  # Standard Rust release build
```

## Running the CLI

```bash
kubediff -e <environment>    # Diff using config paths with environment suffix
kubediff -p <path>           # Diff specific path (overrides config)
kubediff -i                  # In-place mode (uses current directory)
kubediff --log info          # Set log level
```

Configuration lives at `~/.kube/kubediff/config.yaml`.

## Architecture

The project is split into a library crate (`kubediff`) and a binary crate with CLI-only features.

**Core data flow:**
```
CLI args → Settings (config.yaml) → Process::get_entries() → resolve paths
    → Commands::get_build() (kustomize build) → parse YAML documents
    → Commands::get_diff() for each resource:
        - apply_dry_run() → normalized local manifest
        - get_live_resource() → fetch from cluster
        - filter_resource() → remove noise fields
        - generate_diff() → unified diff output
```

**Key modules:**
- `processor.rs` - Main orchestration, processes targets and aggregates results
- `kube_client.rs` - Kubernetes client wrapper using kube.rs with dynamic API discovery
- `commands.rs` - Executes kustomize build and generates diffs
- `kustomize.rs` - Manages embedded kustomize binary (extracted to ~/.cache/kubediff/)
- `filter.rs` - Removes server-managed fields (status, managedFields, generation, resourceVersion, uid, specific annotations)
- `settings.rs` - Config loading and glob pattern resolution with environment suffix support
- `diff.rs` - Unified diff generation using the `similar` crate

**CLI-only modules (feature-gated):**
- `main.rs` - CLI entry point with clap argument parsing
- `print.rs` - Syntax-highlighted output using bat
- `logger.rs` - Conditional logging

## Build System Notes

- `build.rs` downloads kustomize v5.8.0 at compile time and embeds it into the binary
- Cross-compilation configured in `Cross.toml` for ARM64 Linux targets
- No external tool dependencies at runtime (kustomize is embedded)

## Testing

No test suite currently exists. When adding tests, focus on:
- Filter logic (field removal in `filter.rs`)
- Diff generation (`diff.rs`)
- Glob pattern resolution (`settings.rs`)
- Config loading (`settings.rs`)

Run tests with: `cargo test`

## Release Process

- Semantic release via `.releaserc.json` on main branch push
- Multi-platform builds via GitHub Actions (Linux x86_64/ARM64, macOS x86_64/ARM64, Windows x86_64)
- Publishes to crates.io and GitHub releases
