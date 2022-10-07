# `cargo-xtask-lint(1)`

```test
Run lint commands at once

Usage: cargo xtask lint [OPTIONS]

Options:
      --exhaustive                 Same as `--all-workspaces --workspace --each-feature`
  -v, --verbose...                 More output per occurrence
      --all-workspaces             Run the subcommand on all workspaces
  -q, --quiet...                   Less output per occurrence
      --exclude-current-workspace  Run the subcommand on each workspace other than the current workspace
      --workspace                  Run the subcommand for all packages in the workspace
  -p, --package <PACKAGE>          Package name to run the subcommand for
      --each-feature               Run the subcommand with each feature enabled
  -h, --help                       Print help information
```
