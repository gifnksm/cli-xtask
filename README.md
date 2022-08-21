# cli-xtask

[![maintenance status: actively-developed](https://img.shields.io/badge/maintenance-actively--developed-yellowgreen.svg)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-badges-section)
[![license: MIT OR APACHE-2.0](https://img.shields.io/crates/l/cli-xtask.svg)](#license)
[![crates.io](https://img.shields.io/crates/v/cli-xtask.svg)](https://crates.io/crates/cli-xtask)
[![docs.rs](https://docs.rs/cli-xtask/badge.svg)](https://docs.rs/cli-xtask/)
[![rust 1.60.0+ badge](https://img.shields.io/badge/rust-1.60.0+-93450a.svg)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field)
[![Rust CI](https://github.com/gifnksm/cli-xtask/actions/workflows/ci.yml/badge.svg)](https://github.com/gifnksm/cli-xtask/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/gifnksm/cli-xtask/graph/badge.svg)](https://codecov.io/gh/gifnksm/cli-xtask)

<!-- cargo-rdme start -->

A collection of utility functions and command line interfaces for
[cargo-xtask].

This crate provides the following utilities:

* **`cargo xtask dist`** and related subcommands - Builds a distributable
  tar.gz package for your bin crate.
* **`cargo xtask lint`** and related subcommands - Runs the lints for your
  bin/lib crate.
  * Integrated with  [`rustfmt`], [`clippy`], [`cargo-rdme`],
    [`cargo-udeps`].

## Usage

First, create an `xtask` crate following the [instructions on the
cargo-xtask website][xtask-setup].

Then, run the following command to add `cli-xtask` to the dependencies.

* For bin crates:

    ```console
    cargo add -p xtask cli-xtask --features main,bin-crate
    ```

    If you want to use extra tools such as `cargo-rdme` and `cargo-udeps`,
    add the `bin-crate-extra` feature.

    ```console
    cargo add -p xtask cli-xtask --features main,bin-crate,bin-crate-extra
    ```

* For lib crates:

    ```console
    cargo add -p xtask cli-xtask --features main,lib-crate
    ```

    If you want to use extra tools such as `cargo-rdme` and `cargo-udeps`,
    add the `lib-crate-extra` feature.

    ```console
    cargo add -p xtask cli-xtask --features main,lib-crate,lib-crate-extra
    ```

Finally, edit `xtask/src/main.rs` as follows

```rust
use cli_xtask::{Result, Xtask};

fn main() -> Result<()> {
    <Xtask>::main()
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

## Customizing

If you want to remove the subcommands that are not useful for your project,
you can remove them by disabling the corresponding cargo features.
See the [Feature flags section](#feature-flags) for more information.

If you want to add the subcommands that are not included in this crate,
you can add them by creating a new data structure that implements the
`clap::Subcommand` and `Run`.
See the documentation of `Xtask` for more
information.

## Feature flags

By using the features flags of cli-xtask, you can enable only the features
and commands you need. By default, all features are disabled.

The following section contains a list of available features:

### CLI features

* **`main`** - Enables `main` function and
  `main_with_config` function that are the
  premade entry point for the CLI.
* **`error-handler`** - Enables functions for error handling in
  `error_handler` module.
* **`logger`** - Enables functions for logging in `logger`
  module.

### Subcommand features

There are two types of features that enable subcommands:

* **Combined features** - features that enable several useful subcommands at
  once, depending on the type of crate
* **Separated features** - features that enable each subcommand separately

#### Combined features

* **`bin-crate`**:- Enables useful subcommands for bin crates.
* **`lib-crate`** - Enables useful subcommands for lib crates.
* **`bin-crate-extra`** - Enables the additional subcommands useful for bin
  crates.
* **`lib-crate-extra`** - Enables the additional subcommands useful for lib
  crates.

The `{bin,lib}-crate` feature requires only the standard Rust tools that can
be installed with `rustup`. The `{bin,lib}-crate-extra` feature may require
third-party tools.

#### Separated features

The following features require only the standard Rust tools:

* **`subcommand-build`** - Enables `cargo xtask
  build`.
* **`subcommand-clippy`** - Enables `cargo xtask
  clippy`.
* **`subcommand-dist`** - Enables `cargo xtask
  dist`.
* **`subcommand-dist-archive`** - Enables `cargo xtask
  dist-archive`.
* **`subcommand-dist-build-bin`** - Enables `cargo xtask
  dist-build-bin`.
* **`subcommand-dist-build-completion`** - Enables `cargo xtask
  dist-build-completion`.
* **`subcommand-dist-build-doc`** - Enables `cargo xtask
  dist-build-doc`.
* **`subcommand-dist-build-license`** - Enables `cargo xtask
  dist-build-license`.
* **`subcommand-dist-build-man`** - Enables `cargo xtask
  dist-build-man`.
* **`subcommand-dist-build-readme`** - Enables `cargo xtask
  dist-build-readme`.
* **`subcommand-dist-clean`** - Enables `cargo xtask
  dist-clean`.
* **`subcommand-exec`** - Enables `cargo xtask
  exec`.
* **`subcommand-fmt`** - Enables `cargo xtask
  fmt`.
* **`subcommand-lint`** - Enables `cargo xtask
  lint`.
* **`subcommand-test`** - Enables `cargo xtask
  test`.

The following features require third-party tools:

* **`subcommand-rdme`** - Enables `cargo xtask
  rdme`. Requires [`cargo-rdme`] installed.
* **`subcommand-udeps`** - Enables `cargo xtask
  udeps`. Requires [`cargo-udeps`] installed.

### Other features

* **`archive`** - Enables `archive` module which provides
  the functionality to create the archive file for distribution.

## Minimum supported Rust version (MSRV)

The minimum supported Rust version is **Rust 1.60.0**.
At least the last 3 versions of stable Rust are supported at any given time.

While a crate is a pre-release status (0.x.x) it may have its MSRV bumped in
a patch release. Once a crate has reached 1.x, any MSRV bump will be
accompanied by a new minor version.

## License

This project is licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE] or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT] or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

See [CONTRIBUTING.md].

[cargo-xtask]: https://github.com/matklad/cargo-xtask
[`rustfmt`]: https://github.com/rust-lang/rustfmt
[`clippy`]: https://github.com/rust-lang/rust-clippy
[`cargo-rdme`]: https://github.com/orium/cargo-rdme
[`cargo-udeps`]: https://github.com/est31/cargo-udeps
[LICENSE-APACHE]: https://github.com/gifnksm/cli-xtask/blob/main/LICENSE-APACHE
[LICENSE-MIT]: https://github.com/gifnksm/cli-xtask/blob/main/LICENSE-MIT
[CONTRIBUTING.md]: https://github.com/gifnksm/cli-xtask/blob/main/CONTRIBUTING.md

<!-- cargo-rdme end -->
