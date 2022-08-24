# `cargo-xtask-tidy(1)`

```test
cargo-xtask-tidy 
Fix the package problems

USAGE:
    cargo xtask tidy [OPTIONS]

OPTIONS:
        --all-workspaces               Run the subcommand on all workspaces
        --allow-dirty                  Fix code even if the working directory is dirty
        --allow-no-vcs                 Fix code even if a VCS was not detected
        --allow-staged                 Fix code even if the working directory has staged changes
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
