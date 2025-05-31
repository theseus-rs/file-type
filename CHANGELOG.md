# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.7](https://github.com/theseus-rs/file-type/compare/v0.8.6...v0.8.7) - 2025-05-31

### Other
- update file formats

## [0.8.6](https://github.com/theseus-rs/file-type/compare/v0.8.5...v0.8.6) - 2025-05-15

### Other
- update to Rust 1.87.0
- update file formats

## [0.8.5](https://github.com/theseus-rs/file-type/compare/v0.8.4...v0.8.5) - 2025-05-06

### Added
- update file formats

### Other
- update changelog

## [0.8.4](https://github.com/theseus-rs/file-type/compare/v0.8.3...v0.8.4) - 2025-04-03

### Other
- update to Rust 1.86.0

## [0.8.3](https://github.com/theseus-rs/file-type/compare/v0.8.2...v0.8.3) - 2025-04-01

### Added
- update formats

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

## [0.6.0](https://github.com/theseus-rs/file-type/compare/v0.5.2...v0.6.0) - 2025-02-08

### Other
- [**breaking**]  reduce memory utilization by removing composite string ids
- update wikidata formats

## [0.5.2](https://github.com/theseus-rs/file-type/compare/v0.5.1...v0.5.2) - 2025-02-06

### Other
- optimize default file type determination
- reduce use of std library
- correct formatting
- run benchmarks on linux
- add magic crate benchmark
- refactor source code generation into a separate crate
- update examples for wikidata default
- remove end time parameter from performance reports

## [0.5.1](https://github.com/theseus-rs/file-type/compare/v0.5.0...v0.5.1) - 2025-02-05

### Other
- add performance graphs to readme
- update wikidata to be the default provider
- reducing memory allocation during signature evaluation

## [0.5.0](https://github.com/theseus-rs/file-type/compare/v0.4.1...v0.5.0) - 2025-02-03

### Added
- add IANA media types

### Other
- add comparison benchmarks
- [**breaking**] reduce FileType::file_format() visibility to pub(crate)
- improve test coverage
- [**breaking**] update pronom ids to be prefixed with pronom/ instead of fmt/ or x-fmt/
- optimize signature matching
- update FileType to use a reference to FileFormat to avoid unnecessary clone
- remove walkdir
- add wikidata to readme

## [0.4.1](https://github.com/theseus-rs/file-type/compare/v0.4.0...v0.4.1) - 2025-02-01

### Added
- remove dependencies

## [0.4.0](https://github.com/theseus-rs/file-type/compare/v0.3.1...v0.4.0) - 2025-01-29

### Added
- add wikidata file types
- add file type source feature flags

### Other
- remove jiff
- remove thiserror

## [0.3.1](https://github.com/theseus-rs/file-type/compare/v0.3.0...v0.3.1) - 2025-01-24

### Other
- add jai linguist definition

## [0.3.0](https://github.com/theseus-rs/file-type/compare/file_type-v0.2.4...file_type-v0.3.0) - 2025-01-18

### Added
- optimize signature matching

## [0.2.4](https://github.com/theseus-rs/file-type/compare/file_type-v0.2.3...file_type-v0.2.4) - 2025-01-17

### Added
- add rayon feature

## [0.2.3](https://github.com/theseus-rs/file-type/compare/file_type-v0.2.2...file_type-v0.2.3) - 2025-01-16

### Fixed
- update tokio to be optionally enabled by a feature

## [0.2.2](https://github.com/theseus-rs/file-type/compare/file_type-v0.2.1...file_type-v0.2.2) - 2025-01-11

### Added
- update linguist definitions

### Fixed
- correct liquist extension lookup failures by removing the preceeding . from the extensions

### Other
- ignore tests failing on Windows

## [0.2.1](https://github.com/theseus-rs/file-type/compare/file_type-v0.2.0...file_type-v0.2.1) - 2025-01-08

### Fixed
- corrected classification sorting to select the most general file format

### Other
- Merge pull request [#35](https://github.com/theseus-rs/file-type/pull/35) from theseus-rs/update-linguist-file-formats
- ignore tests failing on Windows
- update crate documentation

## [0.2.0](https://github.com/theseus-rs/file-type/compare/file_type-v0.1.0...file_type-v0.2.0) - 2025-01-07

### Fixed
- remove unused UnknownFileType

### Other
- minor readme update
- add links from the file type id to the format file
- optimize variable byte sequence matching by performing BOF/EOF matches first
- optimize signature classification
- Merge pull request [#29](https://github.com/theseus-rs/file-type/pull/29) from theseus-rs/optimize-classification
- minor refactor
- add xml and yaml tests
- update example documentation
- ignore tests failing on Windows
- ignore failing pronom classification tests
- add ignored test cases for pronom file classification failures
- add ignored test cases for pronom file classification failures

## [0.1.0](https://github.com/theseus-rs/file-type/compare/file_type-v0.0.5...file_type-v0.1.0) - 2025-01-05

### Fixed
- update default/1 to be binary, default/2 to be text and attempt to detect the default when no other file type could be determined

### Other
- correct lint error
- update documentation

## [0.0.5](https://github.com/theseus-rs/file-type/compare/file_type-v0.0.4...file_type-v0.0.5) - 2025-01-04

### Fixed
- update internal signature to require all byte sequences match

### Other
- reduce FileType memory allocations
- optimize file type evaluation by reducing map lookup and memory allocations
- add benchmarks
- update file type column header

## [0.0.4](https://github.com/theseus-rs/file-type/compare/file_type-v0.0.3...file_type-v0.0.4) - 2025-01-03

### Added
- add arrow, avro, duckdb, jsonl, and parquet file types

### Fixed
- update apache name format

### Other
- add supported file types to file_type crate readme

## [0.0.3](https://github.com/theseus-rs/file-type/compare/file_type-v0.0.2...file_type-v0.0.3) - 2025-01-03

### Other
- update readme with supported file types
- add FILE_TYPES.md and code to generate
- update format of data sources
- add data sources to documentation
- update test expectations

## [0.0.2](https://github.com/theseus-rs/file-type/compare/file_type-v0.0.1...file_type-v0.0.2) - 2025-01-03

### Added
- add apache httpd mime types

### Other
- update formatting
- limit xml file format data to only used elements
- update example documentation
- improve test coverage
- add file format tests

## [0.0.1](https://github.com/theseus-rs/file-type/compare/file_type-v0.0.0...file_type-v0.0.1) - 2025-01-02

### Other
- update builds to use ubuntu-latest
- update crate docs
- correct wasm32 builds
- correct crate docs
- correct badge links

## [0.0.0](https://github.com/theseus-rs/file-type/compare/1994670af4e90cad19e5768817d41f9d24b8f445...file_type-v0.0.0) - 2025-01-02

### Added
- initial commit

### Other
- update dependencies and remove unnecessary build actions
- correct deny.toml configuration
- update sync functions to use current thread
- remove release drafter
- update pronom_export readme
- add CHANGELOG.md
