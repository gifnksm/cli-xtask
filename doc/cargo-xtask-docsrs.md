# `cargo-xtask-docsrs(1)`

```test
`cargo doc` with docs.rs specific options

Usage: cargo xtask docsrs [OPTIONS] [EXTRA_OPTIONS]...

Arguments:
  [EXTRA_OPTIONS]...  Options to pass to the `cargo doc`

Options:
  -e, --env <KEY>=<VALUE>          Environment variables to set for the subcommand
  -v, --verbose...                 More output per occurrence
      --exhaustive                 Same as `--all-workspaces --workspace --each-feature`
  -q, --quiet...                   Less output per occurrence
      --all-workspaces             Run the subcommand on all workspaces
      --exclude-current-workspace  Run the subcommand on each workspace other than the current workspace
      --workspace                  Run the subcommand for all packages in the workspace
  -p, --package <PACKAGE>          Package name to run the subcommand for
      --default-target             Build documents for docs.rs's default target
      --all-targets                Build documents for all supported targets
  -h, --help                       Print help information
```
