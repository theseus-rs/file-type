# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## `file_type` - [0.2.2](https://github.com/theseus-rs/file-type/compare/file_type-v0.2.1...file_type-v0.2.2) - 2025-01-11

### Added
- update linguist definitions

### Fixed
- correct liquist extension lookup failures by removing the preceeding . from the extensions

### Other
- ignore tests failing on Windows

## `file_type` - [0.2.1](https://github.com/theseus-rs/file-type/compare/file_type-v0.2.0...file_type-v0.2.1) - 2025-01-08

### Fixed
- corrected classification sorting to select the most general file format

### Other
- Merge pull request [#35](https://github.com/theseus-rs/file-type/pull/35) from theseus-rs/update-linguist-file-formats
- ignore tests failing on Windows
- update crate documentation

## `file_type` - [0.2.0](https://github.com/theseus-rs/file-type/compare/file_type-v0.1.0...file_type-v0.2.0) - 2025-01-07

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

## `file_type` - [0.1.0](https://github.com/theseus-rs/file-type/compare/file_type-v0.0.5...file_type-v0.1.0) - 2025-01-05

### Fixed
- update default/1 to be binary, default/2 to be text and attempt to detect the default when no other file type could be determined

### Other
- correct lint error
- update documentation

## `file_type` - [0.0.5](https://github.com/theseus-rs/file-type/compare/file_type-v0.0.4...file_type-v0.0.5) - 2025-01-04

### Fixed
- update internal signature to require all byte sequences match

### Other
- reduce FileType memory allocations
- optimize file type evaluation by reducing map lookup and memory allocations
- add benchmarks
- update file type column header

## `file_type` - [0.0.4](https://github.com/theseus-rs/file-type/compare/file_type-v0.0.3...file_type-v0.0.4) - 2025-01-03

### Added
- add arrow, avro, duckdb, jsonl, and parquet file types

### Fixed
- update apache name format

### Other
- add supported file types to file_type crate readme

## `file_type` - [0.0.3](https://github.com/theseus-rs/file-type/compare/file_type-v0.0.2...file_type-v0.0.3) - 2025-01-03

### Other
- update readme with supported file types
- add FILE_TYPES.md and code to generate
- update format of data sources
- add data sources to documentation
- update test expectations

## `file_type` - [0.0.2](https://github.com/theseus-rs/file-type/compare/file_type-v0.0.1...file_type-v0.0.2) - 2025-01-03

### Added
- add apache httpd mime types

### Other
- update formatting
- limit xml file format data to only used elements
- update example documentation
- improve test coverage
- add file format tests

## `file_type` - [0.0.1](https://github.com/theseus-rs/file-type/compare/file_type-v0.0.0...file_type-v0.0.1) - 2025-01-02

### Other
- update builds to use ubuntu-latest
- update crate docs
- correct wasm32 builds
- correct crate docs
- correct badge links

## `file_type` - [0.0.0](https://github.com/theseus-rs/file-type/compare/1994670af4e90cad19e5768817d41f9d24b8f445...file_type-v0.0.0) - 2025-01-02

### Added
- initial commit

### Other
- update dependencies and remove unnecessary build actions
- correct deny.toml configuration
- update sync functions to use current thread
- remove release drafter
- update pronom_export readme
- add CHANGELOG.md
