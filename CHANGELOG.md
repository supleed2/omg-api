# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.2.0] - 2023-08-30

### Added

- Address input parameter for fn `process` in `impl`s
- `Address` implementation for subcommands
- API `get` and `get_auth` functions in lib root
- Clap derived subcommand short aliases, using `visible_aliases`
- Derive `Debug` for subcommands in `/src`

### Changed

- fn `process` in `impl`s takes `&str` instead of `&Option<String>`

## [0.1.0] - 2023-05-08

### Added

- General project structure, [cargo.toml](Cargo.toml) and [license](LICENSE).
- Changelog file (this file) and reference in the [readme](README.md).
- Source files from [omg](https://crates.io/crates/omg) crate.

[unreleased]: https://github.com/supleed2/omg-api/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/supleed2/omg-api/releases/tag/v0.2.0
[0.1.0]: https://github.com/supleed2/omg-api/releases/tag/v0.1.0
