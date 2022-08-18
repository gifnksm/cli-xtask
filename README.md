# cli-xtask

[![maintenance status: actively-developed](https://img.shields.io/badge/maintenance-actively--developed-yellowgreen.svg)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-badges-section)
[![license: MIT OR APACHE-2.0](https://img.shields.io/crates/l/cli-xtask.svg)](#license)
[![crates.io](https://img.shields.io/crates/v/cli-xtask.svg)](https://crates.io/crates/cli-xtask)
[![docs.rs](https://docs.rs/cli-xtask/badge.svg)](https://docs.rs/cli-xtask/)
[![rust 1.60.0+ badge](https://img.shields.io/badge/rust-1.60.0+-93450a.svg)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field)
[![Rust CI](https://github.com/gifnksm/cli-xtask/actions/workflows/ci.yml/badge.svg)](https://github.com/gifnksm/cli-xtask/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/gifnksm/cli-xtask/graph/badge.svg)](https://codecov.io/gh/gifnksm/cli-xtask)

<!-- cargo-rdme start -->

A number of utility functions and command line interfaces for [cargo-xtask] workflows.

This crate provides the following utilities:

* **`cargo xtask dist`** - Builds a distributable tar.gz package for your bin crate.
* **`cargo xtask lint`** - Runs the lints for your bin/lib crate.
  * Integrated with  [`rustfmt`], [`clippy`], [`cargo-rdme`], [`cargo-udeps`].

[cargo-xtask]: https://github.com/matklad/cargo-xtask
[`rustfmt`]: https://github.com/rust-lang/rustfmt
[`clippy`]: https://github.com/rust-lang/rust-clippy
[`cargo-rdme`]: https://github.com/orium/cargo-rdme
[`cargo-udeps`]: https://github.com/est31/cargo-udeps

## Usage

First, create an `xtask` crate following the [instructions on the cargo-xtask website][xtask-setup].

Then, run the following command to add `cli-xtask` to the dependencies.

* For bin crates:

    ```console
    cargo add -p xtask cli-xtask --features main,bin-crate
    ```

    If you want to extra tools such as `cargo-rdme` and `cargo-udeps`,
    add the `bin-crate-extra` feature.

    ```console
    cargo add -p xtask cli-xtask --features main,bin-crate,bin-crate-extra
    ```

* For lib crates:

    ```console
    cargo add -p xtask cli-xtask --features main,lib-crate
    ```

    If you want to extra tools such as `cargo-rdme` and `cargo-udeps`,
    add the `lib-crate-extra` feature.

    ```console
    cargo add -p xtask cli-xtask --features main,lib-crate,lib-crate-extra
    ```

Finally, edit `xtask/src/main.rs` as follows

```rust
fn main() -> cli_xtask::Result<()> {
    cli_xtask::main()?;
    Ok(())
}
```

Now you can run various workflows with `cargo xtask`.

```console
$ cargo xtask help
cargo-xtask
Rust project automation command

USAGE:
    cargo xtask [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -q, --quiet      Less output per occurrence
    -v, --verbose    More output per occurrence

SUBCOMMANDS:
    build                    Run `cargo build` on all workspaces in the current directory and
                                 subdirectories
    clippy                   Run `cargo clippy` on all workspaces in the current directory and
                                 subdirectories
    dist                     Create the archive file for distribution
    dist-archive             Create the archive file for distribution
    dist-build               Build all artifacts for distribution
    dist-build-bin           Build the release binaries dor distribution
    dist-build-completion    Build the shell completion files for distribution
    dist-build-doc           Build the documentation for distribution
    dist-build-license       Build the license files for distribution
    dist-build-man           Build the man pages for distribution
    dist-build-readme        Build the readme files for distribution
    dist-clean               Removes the dist artifacts
    fmt                      Run `cargo fmt` on all workspaces in the current directory and
                                 subdirectories
    help                     Print this message or the help of the given subcommand(s)
    lint                     Run all lint commands on all workspaces in the current directory
                                 and subdirectories
    rdme                     Run `cargo rdme` on all workspaces in the current directory and
                                 subdirectories
    test                     Run `cargo test` on all workspaces in the current directory and
                                 subdirectories
    udeps                    Run `cargo udeps` on all workspaces in the current directory and
                                 subdirectories
```

[xtask-setup]: https://github.com/matklad/cargo-xtask#defining-xtasks

## Feature flags

By using the features flags of cli-xtask, you can enable only the features and commands you need.
By default, all features are disabled.
The following is a list of available features

### Composite features

* **`bin-crate`**:- Enables useful features for bin crates. The included commands invoke only the standard Rust tools.
* **`bin-crate-extra`** - Enables the additional features useful for bin crates. The included commands may invoke third-party tools.
* **`lib-crate`** - Enables useful features for lib crates. The included commands invoke only the standard Rust tools.
* **`lib-crate-extra`** - Enables the additional features useful for lib crates. The included commands may invoke third-party tools.

### Individual features

* **`main`** - Enables `main` function which is the premade entry point for the CLI.
* **`args`** - Enables `args::Args` type which defines command line interface of the `xtask`.
* **`error-handler`** - Enables `install_error_handler` function which installs a `color-eyre` as an error/panic handler.
* **`logger`** - Enables `install_logger` function which installs a `tracing-subscriber` as a logger.
* **`archive`** - Enables `archive` module which provides the functionality to create the archive file for distribution.

### Subcommand features

* **`command-build`** - Enables `build` subcommand which invokes `cargo build` on all workspaces in the current directory and subdirectories.
* **`command-clippy`** - Enables `clippy` subcommand which invokes `cargo clippy` on all workspaces in the current directory and subdirectories.
* **`command-dist`** - Enables `dist` subcommand which builds the artifacts and creates the archive file for distribution.
  * **`command-dist-archive`** - Enables `dist-archive` subcommand which creates the archive file for distribution.
    This feature is enabled by `command-dist` feature.
  * **`command-dist-build-bin`** - Enables `dist-build-bin` subcommand which builds the release binaries for distribution.
    If enabled, release binaries are built and included in the archive file when the dist subcommand is executed.
  * **`command-dist-build-completion`** - Enables `dist-build-completion` subcommand which builds the shell completion files for distribution.
    If enabled, shell completion files are built and included in the archive file when the dist subcommand is executed.
  * **`command-dist-build-doc`** - Enables `dist-build-doc` subcommand which builds the documentation for distribution.
    If enabled, documentation are built and included in the archive file when the dist subcommand is executed.
  * **`command-dist-build-license`** - Enables `dist-build-license` subcommand which builds the license files for distribution.
    If enabled, license files are built and included in the archive file when the dist subcommand is executed.
  * **`command-dist-build-man`** - Enables `dist-build-man` subcommand which builds the man pages for distribution.
    If enabled, man pages are built and included in the archive file when the dist subcommand is executed.
  * **`command-dist-build-readme`** - Enables `dist-build-readme` subcommand which builds the readme files for distribution.
    If enabled, a readme files is built and included in the archive file when the dist subcommand is executed.
  * **`command-dist-clean`** - Enables `dist-clean` subcommand which removes the artifacts and archives for distribution.
* **`command-exec`** - Enables `exec` subcommand which invokes the given command on all workspaces in the current directory and subdirectories.
* **`command-fmt`** - Enables `fmt` subcommand which invokes `cargo fmt` on all workspaces in the current directory and subdirectories.
* **`command-lint`** - Enables `lint` subcommand which invokes all lint commands on all workspaces in the current directory and subdirectories.
* **`command-rdme`** - Enables `rdme` subcommand which invokes `cargo rdme` on all workspaces in the current directory and subdirectories.
* **`command-test`** - Enables `test` subcommand which invokes `cargo test` on all workspaces in the current directory and subdirectories.
* **`command-udeps`** - Enables `udeps` subcommand which invokes `cargo udeps` on all workspaces in the current directory and subdirectories.

## Minimum supported Rust version (MSRV)

The minimum supported Rust version is **Rust 1.60.0**.
At least the last 3 versions of stable Rust are supported at any given time.

While a crate is a pre-release status (0.x.x) it may have its MSRV bumped in a patch release.
Once a crate has reached 1.x, any MSRV bump will be accompanied by a new minor version.

## License

This project is licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE] or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT] or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md].

[LICENSE-APACHE]: https://github.com/gifnksm/cli-xtask/blob/main/LICENSE-APACHE
[LICENSE-MIT]: https://github.com/gifnksm/cli-xtask/blob/main/LICENSE-MIT
[CONTRIBUTING.md]: https://github.com/gifnksm/cli-xtask/blob/main/CONTRIBUTING.md

<!-- cargo-rdme end -->
