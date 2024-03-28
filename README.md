<!-- cargo-sync-rdme title [[ -->
# cli-xtask
<!-- cargo-sync-rdme ]] -->
<!-- cargo-sync-rdme badge [[ -->
[![Maintenance: passively-maintained](https://img.shields.io/badge/maintenance-passively--maintained-yellowgreen.svg?style=flat-square)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-badges-section)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/cli-xtask.svg?style=flat-square)](#license)
[![crates.io](https://img.shields.io/crates/v/cli-xtask.svg?logo=rust&style=flat-square)](https://crates.io/crates/cli-xtask)
[![docs.rs](https://img.shields.io/docsrs/cli-xtask.svg?logo=docs.rs&style=flat-square)](https://docs.rs/cli-xtask)
[![Rust: ^1.74.0](https://img.shields.io/badge/rust-^1.74.0-93450a.svg?logo=rust&style=flat-square)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field)
[![GitHub Actions: CI](https://img.shields.io/github/actions/workflow/status/gifnksm/cli-xtask/ci.yml.svg?label=CI&logo=github&style=flat-square)](https://github.com/gifnksm/cli-xtask/actions/workflows/ci.yml)
[![Codecov](https://img.shields.io/codecov/c/github/gifnksm/cli-xtask.svg?label=codecov&logo=codecov&style=flat-square)](https://codecov.io/gh/gifnksm/cli-xtask)
<!-- cargo-sync-rdme ]] -->
<!-- cargo-sync-rdme rustdoc [[ -->
A collection of utility functions and command line interfaces for
[cargo-xtask].

This crate provides the following utilities:

* **[`cargo xtask dist`]** and related subcommands
  * Builds a distributable tar.gz package for your bin crate.
* **[`cargo xtask lint`]** and related subcommands
  * Runs the lints for your bin/lib crate.
  * Integrated with [`rustdoc`], [`rustfmt`], [`clippy`],
    [`cargo-sync-rdme`], [`cargo-udeps`].
* **[`cargo xtask tidy`]** and related subcommands
  * Fixes the problems on your bin/lib crate.
  * Integrated with  [`rustfmt`], [`clippy`], [`cargo-sync-rdme`].
* **[`cargo xtask pre-release`]**
  * Checks if your bin/lib crate is ready for a release.
* **[`cargo xtask build`], [`clippy`](https://docs.rs/cli-xtask/latest/cli_xtask/subcommand/clippy/struct.Clippy.html), [`doc`](https://docs.rs/cli-xtask/latest/cli_xtask/subcommand/doc/struct.Doc.html), [`fmt`](https://docs.rs/cli-xtask/latest/cli_xtask/subcommand/fmt/struct.Fmt.html), [`test`](https://docs.rs/cli-xtask/latest/cli_xtask/subcommand/test/struct.Test.html)**
  * Runs the cargo commands with options useful for testing and continuous
    integration.
    * **`--all-workspaces`** - Runs the cargo commands for all workspaces.
    * **`--workspace`** - Runs the cargo commands for all packages in the
      workspace.
    * **`--each-features`** - Repeats to runs the cargo commands for each
      feature enabled.
    * **`--exhaustive`** - Same as `--all-workspaces --workspace --each-features`.
* **[`cargo xtask docsrs`]**
  * Builds the documentation for your lib crate with configuration for
    [docs.rs].
* **[`cargo xtask exec`]**
  * Runs a command in the gicontext of all workspaces.

## Usage

First, create an `xtask` crate following the [instructions on the
cargo-xtask website](https://github.com/matklad/cargo-xtask#defining-xtasks).

Then, run the following command to add `cli-xtask` to the dependencies.

* For bin crates:
  
  ````console
  cargo add -p xtask cli-xtask --features main,bin-crate
  ````
  
  If you want to use extra tools such as `cargo-sync-rdme` and
  `cargo-udeps`,     add the `bin-crate-extra` feature.
  
  ````console
  cargo add -p xtask cli-xtask --features main,bin-crate,bin-crate-extra
  ````

* For lib crates:
  
  ````console
  cargo add -p xtask cli-xtask --features main,lib-crate
  ````
  
  If you want to use extra tools such as `cargo-sync-rdme` and
  `cargo-udeps`,     add the `lib-crate-extra` feature.
  
  ````console
  cargo add -p xtask cli-xtask --features main,lib-crate,lib-crate-extra
  ````

Finally, edit `xtask/src/main.rs` as follows

````rust
use cli_xtask::{Result, Xtask};

fn main() -> Result<()> {
    <Xtask>::main()
}
````

Now you can run various workflows with `cargo xtask`.

## Customizing

If you want to remove the subcommands that are not useful for your project,
you can remove them by disabling the corresponding cargo features.
See the [Feature flags section](#feature-flags) for more information.

If you want to add the subcommands that are not included in this crate,
you can add them by creating a new data structure that implements the
[`clap::Subcommand`](https://docs.rs/clap_builder/4.5.2/clap_builder/derive/trait.Subcommand.html) and [`Run`](https://docs.rs/cli-xtask/latest/cli_xtask/trait.Run.html).
See [the documentation of `Xtask`](https://docs.rs/cli-xtask/latest/cli_xtask/command/struct.Xtask.html) for more
information.

## Feature flags

By using the features flags of cli-xtask, you can enable only the features
and commands you need. By default, all features are disabled.

The following section contains a list of available features:

### CLI features

* **`main`** - Enables \[`Xtask::main`\] function and
  \[`Xtask::main_with_config`\] function that are the premade entry point for
  the CLI.
* **`error-handler`** - Enables functions for error handling in
  [`error_handler`](https://docs.rs/cli-xtask/latest/cli_xtask/error_handler/index.html) module.
* **`logger`** - Enables functions for logging in [`logger`](https://docs.rs/cli-xtask/latest/cli_xtask/logger/index.html) module.

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

* **`subcommand-build`** - Enables [`cargo xtask build`].
* **`subcommand-clippy`** - Enables [`cargo xtask clippy`].
* **`subcommand-dist`** - Enables [`cargo xtask dist`].
* **`subcommand-dist-archive`** - Enables [`cargo xtask dist-archive`].
* **`subcommand-dist-build-bin`** - Enables [`cargo xtask dist-build-bin`].
* **`subcommand-dist-build-completion`** - Enables [`cargo xtask dist-build-completion`].
* **`subcommand-dist-build-doc`** - Enables [`cargo xtask dist-build-doc`].
* **`subcommand-dist-build-license`** - Enables [`cargo xtask dist-build-license`].
* **`subcommand-dist-build-man`** - Enables [`cargo xtask dist-build-man`].
* **`subcommand-dist-build-readme`** - Enables [`cargo xtask dist-build-readme`].
* **`subcommand-dist-clean`** - Enables [`cargo xtask dist-clean`].
* **`subcommand-doc`** - Enables [`cargo xtask doc`].
* **`subcommand-docsrs`** - Enables [`cargo xtask docsrs`].
* **`subcommand-exec`** - Enables [`cargo xtask exec`].
* **`subcommand-fmt`** - Enables [`cargo xtask fmt`].
* **`subcommand-lint`** - Enables [`cargo xtask lint`].
* **`subcommand-pre-release`** - Enables [`cargo xtask pre-release`].
* **`subcommand-test`** - Enables [`cargo xtask test`].
* **`subcommand-tidy`** - Enables [`cargo xtask tidy`].

The following features require third-party tools:

* **`subcommand-sync-rdme`** - Enables [`cargo xtask sync-rdme`]. Requires
  [`cargo-sync-rdme`] installed.
* **`subcommand-udeps`** - Enables [`cargo xtask udeps`]. Requires
  [`cargo-udeps`] installed.

### Other features

* **`archive`** - Enables [`archive`](https://docs.rs/cli-xtask/latest/cli_xtask/archive/index.html) module which provides the
  functionality to create the archive file for distribution.

## Minimum supported Rust version (MSRV)

The minimum supported Rust version is **Rust 1.74.0**.
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
[`cargo xtask dist`]: https://docs.rs/cli-xtask/latest/cli_xtask/subcommand/dist/struct.Dist.html
[`cargo xtask lint`]: https://docs.rs/cli-xtask/latest/cli_xtask/subcommand/lint/struct.Lint.html
[`rustdoc`]: https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html
[`rustfmt`]: https://github.com/rust-lang/rustfmt
[`clippy`]: https://github.com/rust-lang/rust-clippy
[`cargo-sync-rdme`]: https://github.com/gifnksm/cargo-sync-rdme
[`cargo-udeps`]: https://github.com/est31/cargo-udeps
[`cargo xtask tidy`]: https://docs.rs/cli-xtask/latest/cli_xtask/subcommand/tidy/struct.Tidy.html
[`cargo xtask pre-release`]: https://docs.rs/cli-xtask/latest/cli_xtask/subcommand/pre_release/struct.PreRelease.html
[`cargo xtask build`]: https://docs.rs/cli-xtask/latest/cli_xtask/subcommand/build/struct.Build.html
[`cargo xtask docsrs`]: https://docs.rs/cli-xtask/latest/cli_xtask/subcommand/docsrs/struct.Docsrs.html
[docs.rs]: https://docs.rs/
[`cargo xtask exec`]: https://docs.rs/cli-xtask/latest/cli_xtask/subcommand/exec/struct.Exec.html
[`cargo xtask clippy`]: https://docs.rs/cli-xtask/latest/cli_xtask/subcommand/clippy/struct.Clippy.html
[`cargo xtask dist-archive`]: https://docs.rs/cli-xtask/latest/cli_xtask/subcommand/dist_archive/struct.DistArchive.html
[`cargo xtask dist-build-bin`]: https://docs.rs/cli-xtask/latest/cli_xtask/subcommand/dist_build_bin/struct.DistBuildBin.html
[`cargo xtask dist-build-completion`]: https://docs.rs/cli-xtask/latest/cli_xtask/subcommand/dist_build_completion/struct.DistBuildCompletion.html
[`cargo xtask dist-build-doc`]: https://docs.rs/cli-xtask/latest/cli_xtask/subcommand/dist_build_doc/struct.DistBuildDoc.html
[`cargo xtask dist-build-license`]: https://docs.rs/cli-xtask/latest/cli_xtask/subcommand/dist_build_license/struct.DistBuildLicense.html
[`cargo xtask dist-build-man`]: https://docs.rs/cli-xtask/latest/cli_xtask/subcommand/dist_build_man/struct.DistBuildMan.html
[`cargo xtask dist-build-readme`]: https://docs.rs/cli-xtask/latest/cli_xtask/subcommand/dist_build_readme/struct.DistBuildReadme.html
[`cargo xtask dist-clean`]: https://docs.rs/cli-xtask/latest/cli_xtask/subcommand/dist_clean/struct.DistClean.html
[`cargo xtask doc`]: https://docs.rs/cli-xtask/latest/cli_xtask/subcommand/doc/struct.Doc.html
[`cargo xtask fmt`]: https://docs.rs/cli-xtask/latest/cli_xtask/subcommand/fmt/struct.Fmt.html
[`cargo xtask test`]: https://docs.rs/cli-xtask/latest/cli_xtask/subcommand/test/struct.Test.html
[`cargo xtask sync-rdme`]: https://docs.rs/cli-xtask/latest/cli_xtask/subcommand/sync_rdme/struct.SyncRdme.html
[`cargo xtask udeps`]: https://docs.rs/cli-xtask/latest/cli_xtask/subcommand/udeps/struct.Udeps.html
[LICENSE-APACHE]: https://github.com/gifnksm/cli-xtask/blob/main/LICENSE-APACHE
[LICENSE-MIT]: https://github.com/gifnksm/cli-xtask/blob/main/LICENSE-MIT
[CONTRIBUTING.md]: https://github.com/gifnksm/cli-xtask/blob/main/CONTRIBUTING.md
<!-- cargo-sync-rdme ]] -->
