# `cargo-xtask-test(1)`

```test
`cargo test` with options useful for testing and continuous integration

Usage: cargo xtask test [OPTIONS] [EXTRA_OPTIONS]...

Arguments:
  [EXTRA_OPTIONS]...  Options to pass to the `cargo test`

Options:
  -e, --env <KEY>=<VALUE>          Environment variables to set for the subcommand
  -v, --verbose...                 More output per occurrence
      --exhaustive                 Same as `--all-workspaces --workspace --each-feature`
  -q, --quiet...                   Less output per occurrence
      --all-workspaces             Run the subcommand on all workspaces
      --exclude-current-workspace  Run the subcommand on each workspace other than the current workspace
      --workspace                  Run the subcommand for all packages in the workspace
  -p, --package <PACKAGE>          Package name to run the subcommand for
      --each-feature               Run the subcommand with each feature enabled
  -h, --help                       Print help information
```
