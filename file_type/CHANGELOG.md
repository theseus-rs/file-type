# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
