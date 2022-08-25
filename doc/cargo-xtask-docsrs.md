# `cargo-xtask-docsrs(1)`

```test
cargo-xtask-docsrs 
`cargo doc` with docs.rs specific options

USAGE:
    cargo xtask docsrs [OPTIONS] [EXTRA_OPTIONS]...

ARGS:
    <EXTRA_OPTIONS>...    Options to pass to the `cargo doc`

OPTIONS:
        --all-targets                  Build documents for all supported targets
        --all-workspaces               Run the subcommand on all workspaces
        --default-target               Build documents for docs.rs's default target
    -e, --env <KEY>=<VALUE>            Environment variables to set for the subcommand
        --exclude-current-workspace    Run the subcommand on each workspace other than the current
                                       workspace
        --exhaustive                   Same as `--all-workspaces --workspace --each-feature`
    -h, --help                         Print help information
    -p, --package <PACKAGE>            Package name to run the subcommand for
    -q, --quiet                        Less output per occurrence
    -v, --verbose                      More output per occurrence
        --workspace                    Run the subcommand for all packages in the workspace
```
