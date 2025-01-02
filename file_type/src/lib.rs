//! # file-type `FileType`
//!
//! [![Code Coverage](https://codecov.io/gh/theseus-rs/file-type/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/file-type)
//! [![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-file-type)
//! [![License](https://img.shields.io/crates/l/file_type)](https://github.com/theseus-rs/file-type#license)
//! [![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)
//!
//! ## Getting Started
//!
//! A file type.  The file type is determined by examining the file or bytes against known file
//! signatures and file extensions.
//!
//! Additional information on PRONOM file types can be found at
//! [The National Archives](https://www.nationalarchives.gov.uk/pronom/)
//!
//! # Example
//!
//! Detect a Java class file from bytes:
//! ```
//! use file_type::FileType;
//!
//! let file_type = FileType::from_bytes(b"\xCA\xFE\xBA\xBE");
//! assert_eq!(file_type.name(), "Java Class File");
//! assert_eq!(file_type.media_types(), Vec::<String>::new());
//! assert_eq!(file_type.extensions(), vec!["class".to_string()]);
//! ```
//!
//! Detect text from bytes:
//! ```
//! use file_type::FileType;
//!
//! let file_type = FileType::from_bytes(b"hello, world\n");
//! assert_eq!(file_type.name(), "Text");
//! assert_eq!(file_type.media_types(), vec!["text/plain".to_string()]);
//! assert_eq!(file_type.extensions(), Vec::<String>::new());
//! ```
//!
//! ## Safety
//!
//! This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

#![forbid(unsafe_code)]
#![forbid(clippy::allow_attributes)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]

mod error;
mod file_formats;
mod file_type;
pub mod pronom;

pub use error::{Error, Result};
pub use file_type::FileType;
