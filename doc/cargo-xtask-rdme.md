# `cargo-xtask-rdme(1)`

```test
cargo-xtask-rdme 
`cargo rdme` with options useful for testing and continuous integration

USAGE:
    cargo xtask rdme [OPTIONS] [EXTRA_OPTIONS]...

ARGS:
    <EXTRA_OPTIONS>...    Options to pass to the `cargo rdme`

OPTIONS:
        --all-workspaces               Run the subcommand on all workspaces
    -e, --env <KEY>=<VALUE>            Environment variables to set for the subcommand
        --exclude-current-workspace    Run the subcommand on each workspace other than the current
                                       workspace
        --exhaustive                   Same as `--all-workspaces --workspace --each-feature`
    -h, --help                         Print help information
    -q, --quiet                        Less output per occurrence
    -v, --verbose                      More output per occurrence
```