# `cargo-xtask(1)`

```test
Rust project automation command

Usage: cargo xtask [OPTIONS] [COMMAND]

Commands:
  build                  `cargo build` with options useful for testing and continuous integration
  clippy                 `cargo clippy` with options useful for testing and continuous integration
  dist                   Build the artifacts and create the archive file for distribution
  dist-archive           Create the archive file for distribution
  dist-build             Build all artifacts for distribution
  dist-build-bin         Build the release binaries for distribution
  dist-build-completion  Build the shell completion files for distribution
  dist-build-doc         Build the documentation for distribution
  dist-build-license     Build the license files for distribution
  dist-build-man         Build the man pages for distribution
  dist-build-readme      Build the readme files for distribution
  dist-clean             Remove the artifacts and archives for distribution
  doc                    `cargo doc` with options useful for testing and continuous integration
  docsrs                 `cargo doc` with docs.rs specific options
  exec                   Run commands on all workspaces in the current directory and subdirectories
  fmt                    `cargo fmt` with options useful for testing and continuous integration
  lint                   Run lint commands at once
  pre-release            Run pre-release checks
  sync-rdme              `cargo sync-rdme` with options useful for testing and continuous integration
  test                   `cargo test` with options useful for testing and continuous integration
  tidy                   Fix the package problems
  udeps                  `cargo udeps` with options useful for testing and continuous integration
  help                   Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  More output per occurrence
  -q, --quiet...    Less output per occurrence
  -h, --help        Print help
```
