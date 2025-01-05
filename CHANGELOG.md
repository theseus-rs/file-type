# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
- add supported file types to file_type crate reamde

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
