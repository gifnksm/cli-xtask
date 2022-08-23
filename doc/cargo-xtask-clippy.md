# `cargo-xtask-clippy(1)`

```test
cargo-xtask-clippy 
`cargo clippy` with options useful for tesing and continuous integration

USAGE:
    cargo xtask clippy [OPTIONS] [EXTRA_OPTIONS]...

ARGS:
    <EXTRA_OPTIONS>...    Options to pass to the `cargo clippy`

OPTIONS:
        --all-workspaces       Run the subcommand on all workspaces
        --each-feature         Run the subcommand with each feature enabled
        --exhaustive           Run the subcommand on all option combinations (workspaces, packages,
                               features if available)
    -h, --help                 Print help information
    -p, --package <PACKAGE>    Package name to run the subcommand for
    -q, --quiet                Less output per occurrence
    -v, --verbose              More output per occurrence
        --workspace            Run the subcommand for all packages in the workspace
```
