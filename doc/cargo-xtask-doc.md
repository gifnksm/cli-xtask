# `cargo-xtask-doc(1)`

```test
cargo-xtask-doc 
`cargo doc` with options useful for testing and continuous integration

USAGE:
    cargo xtask doc [OPTIONS] [EXTRA_OPTIONS]...

ARGS:
    <EXTRA_OPTIONS>...    Options to pass to the `cargo build`

OPTIONS:
        --all-workspaces       Run the subcommand on all workspaces
    -e, --env <KEY>=<VALUE>    Environment variables to set for the subcommand
        --exhaustive           Run the subcommand on all option combinations (workspaces, packages,
                               features if available)
    -h, --help                 Print help information
    -p, --package <PACKAGE>    Package name to run the subcommand for
    -q, --quiet                Less output per occurrence
    -v, --verbose              More output per occurrence
        --workspace            Run the subcommand for all packages in the workspace
```
