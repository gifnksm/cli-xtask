# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

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
[Unreleased]: https://github.com/gifnksm/cli-xtask/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/gifnksm/cli-xtask/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/gifnksm/cli-xtask/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/gifnksm/cli-xtask/compare/v0.1.4...v0.2.0
[0.1.4]: https://github.com/gifnksm/cli-xtask/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/gifnksm/cli-xtask/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/gifnksm/cli-xtask/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/gifnksm/cli-xtask/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/gifnksm/cli-xtask/commits/v0.1.0
