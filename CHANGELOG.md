# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Windows support.
- Binary releases compiled using [trust](https://github.com/japaric/trust)
  are now available on the Releases page.

### Changed

- Release binaries are now built with link-time optimization enabled,
  resulting in smaller file sizes.

### Fixed

- The `COLORTERM` environment variable is now properly taken into account
  at run-time.

## 0.1.0 - 2018-08-21

- Initial versioned release.

[Unreleased]: https://github.com/Calinou/lagraph/compare/v0.1.0...HEAD
