# `cargo-xtask-exec(1)`

```test
cargo-xtask-exec 
Run commands on all workspaces in the current directory and subdirectories

USAGE:
    cargo xtask exec [OPTIONS] <COMMAND> [COMMAND_OPTIONS]...

ARGS:
    <COMMAND>               Command to execute
    <COMMAND_OPTIONS>...    Arguments to pass to the command

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
