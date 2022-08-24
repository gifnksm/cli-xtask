# `cargo-xtask-lint(1)`

```test
cargo-xtask-lint 
Run lint commands at once

USAGE:
    cargo xtask lint [OPTIONS]

OPTIONS:
        --all-workspaces               Run the subcommand on all workspaces
        --each-feature                 Run the subcommand with each feature enabled
        --exclude-current-workspace    Run the subcommand on each workspace other than the current
                                       workspace
        --exhaustive                   Same as `--all-workspaces --workspace --each-feature`
    -h, --help                         Print help information
    -p, --package <PACKAGE>            Package name to run the subcommand for
    -q, --quiet                        Less output per occurrence
    -v, --verbose                      More output per occurrence
        --workspace                    Run the subcommand for all packages in the workspace
```
