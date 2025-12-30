//! # `file_type`
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
//! Signature, extension and media type data are provided by:
//! * [Apache HTTPD](https://github.com/apache/httpd/blob/trunk/docs/conf/mime.types)
//! * [IANA](https://www.iana.org/assignments/media-types/media-types.xml)
//! * [Linguist](https://github.com/github-linguist/linguist/blob/main/lib/linguist/languages.yml)
//! * [The National Archives PRONOM](https://www.nationalarchives.gov.uk/pronom/)
//! * [Wikidata](https://www.wikidata.org/wiki/Wikidata:WikiProject_Informatics/Structures/File_formats/List)
//!
//! # Example
//!
//! Detect a file type from bytes:
//! ```
//! use file_type::FileType;
//!
//! let file_type = FileType::from_bytes(b"\xCA\xFE\xBA\xBE");
//! assert_eq!(file_type.name(), "Java class file");
//! assert_eq!(file_type.extensions(), vec!["class"]);
//! ```
//!
//! Retrieve a file type from an extension:
//! ```
//! use file_type::FileType;
//!
//! let file_types = FileType::from_extension("png");
//! let file_type = file_types.first().expect("file format");
//! assert_eq!(file_type.media_types(), vec!["image/png"]);
//! ```
//!
//! Retrieve a file type from a media type:
//! ```
//! use file_type::FileType;
//!
//! let file_types = FileType::from_media_type("image/png");
//! let file_type = file_types.first().expect("file format");
//! assert_eq!(file_type.extensions(), vec!["png"]);
//! ```
//!
//! ## Feature flags
//!
//! | Name       | Description                                                                                                                | Default? |
//! |------------|----------------------------------------------------------------------------------------------------------------------------|----------|
//! | `httpd`    | Enables [Apache HTTPD](https://github.com/apache/httpd/blob/trunk/docs/conf/mime.types) file types                         | No       |
//! | `iana`     | Enables [IANA](https://www.iana.org/assignments/media-types/media-types.xml) file types                                    | No       |
//! | `linguist` | Enables [Linguist](https://github.com/github-linguist/linguist/blob/main/lib/linguist/languages.yml) file types            | No       |
//! | `pronom`   | Enables [PRONOM](https://www.nationalarchives.gov.uk/PRONOM) file types                                                    | No       |
//! | `std`      | Enables support for the Rust standard library                                                                              | Yes      |
//! | `wikidata` | Enables [Wikidata](https://www.wikidata.org/wiki/Wikidata:WikiProject_Informatics/Structures/File_formats/List) file types | Yes      |
//!
//! ## Supported File Types
//!
//! [List of supported file types](https://github.com/theseus-rs/file-type/blob/main/FILETYPES.md)
//!
//! ## Safety
//!
//! This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
extern crate core;

mod error;
mod extensions;
mod file_type;
mod file_types;
#[doc(hidden)]
pub mod format;
mod media_types;
mod signatures;
pub mod sources;

pub use error::{Error, Result};
pub use file_type::FileType;
