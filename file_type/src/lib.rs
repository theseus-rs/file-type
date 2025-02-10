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
//! assert_eq!(file_type.name(), "Java Class File");
//! assert_eq!(file_type.extensions(), vec!["class"]);
//! ```
//!
//! Detect the file type from a file:
//! ```rust,no_run
//! use file_type::FileType;
//! use std::path::Path;
//!
//! #[tokio::main]
//! async fn main() {
//!     let file_path = Path::new("image.png");
//!     let file_type = FileType::try_from_file(file_path).await.expect("file type not found");
//!     assert_eq!(file_type.extensions(), vec!["png"]);
//!     assert_eq!(file_type.media_types(), vec!["image/png"]);
//! }
//! ```
//!
//! Detect the file type from a file synchronously:
//! ```no_run
//! use file_type::FileType;
//! use std::path::Path;
//!
//! let file_path = Path::new("image.png");
//! let file_type = FileType::try_from_file_sync(file_path).expect("file type not found");
//! assert_eq!(file_type.extensions(), vec!["png"]);
//! assert_eq!(file_type.media_types(), vec!["image/png"]);
//! ```
//!
//! ## Feature flags
//!
//! | Name       | Description                                                                                                                | Default? |
//! |------------|----------------------------------------------------------------------------------------------------------------------------|----------|
//! | `custom`   | Enables custom file types                                                                                                  | Yes      |
//! | `httpd`    | Enables [Apache HTTPD](https://github.com/apache/httpd/blob/trunk/docs/conf/mime.types) file types                         | No       |
//! | `iana`     | Enables [IANA](https://www.iana.org/assignments/media-types/media-types.xml) file types                                    | No       |
//! | `linguist` | Enables [Linguist](https://github.com/github-linguist/linguist/blob/main/lib/linguist/languages.yml) file types            | No       |
//! | `pronom`   | Enables [PRONOM](https://www.nationalarchives.gov.uk/PRONOM) file types                                                    | No       |
//! | `tokio`    | Enables using tokio for async                                                                                              | No       |
//! | `wikidata` | Enables [Wikidata](https://www.wikidata.org/wiki/Wikidata:WikiProject_Informatics/Structures/File_formats/List) file types | Yes      |
//!
//! ## Supported File Types
//!
//! [List of supported file types](https://github.com/theseus-rs/file-type/blob/main/FILETYPES.md)
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
extern crate alloc;
extern crate core;

mod error;
mod file_type;
mod file_types;
#[doc(hidden)]
pub mod format;
pub mod sources;

pub use error::{Error, Result};
pub use file_type::FileType;
