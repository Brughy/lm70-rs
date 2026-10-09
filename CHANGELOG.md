# Changelog

All notable changes to this project will be documented in this file.
## [0.1.0] - 2026-09-19

### Added
- Platform-independent `no_std` LM70 driver using `embedded-hal` 1.0 `SpiDevice`.
- Signed 11-bit temperature conversion with 0.25 degrees Celsius resolution.
- Shutdown, continuous-conversion, and manufacturer/device ID operations.
- Register status validation and mock-based SPI protocol tests.
- English README, Rust API documentation, and Doxygen configuration.
[Unreleased]: https://github.com/Brughy/lm70-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Brughy/lm70-rs/releases/tag/v0.1.0
