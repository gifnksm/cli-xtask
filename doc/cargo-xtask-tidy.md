# `cargo-xtask-tidy(1)`

```test
Fix the package problems

Usage: cargo xtask tidy [OPTIONS]

Options:
      --allow-no-vcs               Fix code even if a VCS was not detected
  -v, --verbose...                 More output per occurrence
      --allow-dirty                Fix code even if the working directory is dirty
  -q, --quiet...                   Less output per occurrence
      --allow-staged               Fix code even if the working directory has staged changes
      --exhaustive                 Same as `--all-workspaces --workspace --each-feature`
      --all-workspaces             Run the subcommand on all workspaces
      --exclude-current-workspace  Run the subcommand on each workspace other than the current workspace
      --workspace                  Run the subcommand for all packages in the workspace
  -p, --package <PACKAGE>          Package name to run the subcommand for
      --each-feature               Run the subcommand with each feature enabled
  -h, --help                       Print help information
```
