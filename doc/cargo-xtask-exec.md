# `cargo-xtask-exec(1)`

```test
Run commands on all workspaces in the current directory and subdirectories

Usage: cargo xtask exec [OPTIONS] <COMMAND> [COMMAND_OPTIONS]...

Arguments:
  <COMMAND>             Command to execute
  [COMMAND_OPTIONS]...  Arguments to pass to the command

Options:
  -e, --env <KEY>=<VALUE>          Environment variables to set for the subcommand
  -v, --verbose...                 More output per occurrence
      --exhaustive                 Same as `--all-workspaces --workspace --each-feature`
  -q, --quiet...                   Less output per occurrence
      --all-workspaces             Run the subcommand on all workspaces
      --exclude-current-workspace  Run the subcommand on each workspace other than the current workspace
  -h, --help                       Print help information
```
