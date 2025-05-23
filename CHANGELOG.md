# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [0.10.1] - 2025-01-02

## [0.10.0] - 2024-12-27

## [0.9.0] - 2024-11-30

### Add

* Add default implementations of the conversion functions on the Run trait to simplify implementations
* Add `DistConfigBuilder::from_default_packages`
* (breaking change) Run commands for default workspace memmbers instead of root package if no package name specified

## [0.8.0] - 2023-09-24

### Changed

* (breaking change) return type of `cargo::build` is now `Result<impl Iteraotr, Error>`

## [0.7.1] - 2023-09-23

## [0.7.0] - 2023-09-17

### Fixed

* `cargo xtask dist-build-bin` now fails when `cargo build` fails

### Changed

* (breaking change) dist: Add cargo build option support
* Update dependencies
* Migrate to `chrono`

## [0.6.1] - 2022-12-03

### Fixed

* Add workaround for `cargo udeps` failure on windows

## [0.6.0] - 2022-10-07

### Changed

* (breaking change) Update clap requirement from 3.2.22 to 4.0.10

## [0.5.0] - 2022-09-16

### Added

* Add `Subcommand::selected()` method

### Fixed

* Revert "Removed a wrong default implementation of `Run::to_subcommands()`"

## [0.4.0] - 2022-09-16

### Fixed

* Removed a wrong default implementation of `Run::to_subcommands()`

## [0.3.0] - 2022-09-16

### Added

* (breaking change) `Run` trait now supports downcasting.

## [0.2.0] - 2022-09-15

### Removed

* (breaking change) `xtask rdme` is removed. Please use `xtask sync-rdme` instead.

### Added

* `xtask sync-rdme` is added

## [0.1.4] - 2022-09-14

### Fixed

* `xtask dist`: windows build now works again (binary name was wrong)

## [0.1.3] - 2022-09-06

* `xtask docsrs`: change rustdoc parameters to support hosting on GitHub Pages

## [0.1.2] - 2022-09-05

### Changed

* `xtask docsrs`: generate `index.html` to redirect root package's documentation

## [0.1.1] - 2022-09-04

### Fixed

* `xtask man`: handle spaces in command name correctly

## [0.1.0] - 2022-08-25

* First release

<!-- next-url -->
[Unreleased]: https://github.com/gifnksm/cli-xtask/compare/v0.10.1...HEAD
[0.10.1]: https://github.com/gifnksm/cli-xtask/compare/v0.10.0...v0.10.1
[0.10.0]: https://github.com/gifnksm/cli-xtask/compare/v0.9.0...v0.10.0
[0.9.0]: https://github.com/gifnksm/cli-xtask/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/gifnksm/cli-xtask/compare/v0.7.1...v0.8.0
[0.7.1]: https://github.com/gifnksm/cli-xtask/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/gifnksm/cli-xtask/compare/v0.6.1...v0.7.0
[0.6.1]: https://github.com/gifnksm/cli-xtask/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/gifnksm/cli-xtask/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/gifnksm/cli-xtask/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/gifnksm/cli-xtask/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/gifnksm/cli-xtask/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/gifnksm/cli-xtask/compare/v0.1.4...v0.2.0
[0.1.4]: https://github.com/gifnksm/cli-xtask/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/gifnksm/cli-xtask/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/gifnksm/cli-xtask/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/gifnksm/cli-xtask/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/gifnksm/cli-xtask/commits/v0.1.0
