# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.2](https://github.com/theseus-rs/file-type/compare/v0.8.1...v0.8.2) - 2025-03-27

### Fixed
- correct wikidata query to include executable file formats

## [0.8.1](https://github.com/theseus-rs/file-type/compare/v0.8.0...v0.8.1) - 2025-03-21

### Added
- update file types

### Other
- add custom compression tests

## [0.8.0](https://github.com/theseus-rs/file-type/compare/v0.7.6...v0.8.0) - 2025-03-15

### Other
- remove custom formats in favor of wikidata formats

## [0.7.6](https://github.com/theseus-rs/file-type/compare/v0.7.5...v0.7.6) - 2025-03-11

### Added
- enable docs for id and source type

## [0.7.5](https://github.com/theseus-rs/file-type/compare/v0.7.4...v0.7.5) - 2025-03-11

### Added
- update wikidata export to support file types without extensions

## [0.7.4](https://github.com/theseus-rs/file-type/compare/v0.7.3...v0.7.4) - 2025-03-07

### Added
- update iana, linguist and wikidata file types

## [0.7.3](https://github.com/theseus-rs/file-type/compare/v0.7.2...v0.7.3) - 2025-02-26

### Added
- update pronom definitions
- update wikidata definitions
- update linguist definitions

## [0.7.2](https://github.com/theseus-rs/file-type/compare/v0.7.1...v0.7.2) - 2025-02-20

### Added
- update file types
- update to Rust 2024 edition

## [0.7.1](https://github.com/theseus-rs/file-type/compare/v0.7.0...v0.7.1) - 2025-02-16

### Added
- update wikidata formats (add CBOR)

## [0.7.0](https://github.com/theseus-rs/file-type/compare/v0.6.0...v0.7.0) - 2025-02-10

### Added
- add no_std support

### Fixed
- suppress duplicate wikidata signatures
- address performance regression
- correct benchmark file path

### Other
- [**breaking**] remove tokio support
- refactor to use phf maps
- remove file types vector to reduce runtime memory utilization
- move file type document generation to test_data crate
- update test data generator to generate pronom test data
- create test data generation crate
