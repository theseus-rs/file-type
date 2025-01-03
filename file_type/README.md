# file_type

[![ci](https://github.com/theseus-rs/file-type/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/theseus-rs/file-type/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/file_type/badge.svg)](https://docs.rs/file_type)
[![Code Coverage](https://codecov.io/gh/theseus-rs/file-type/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/file-type)
[![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-file-type)
[![Latest version](https://img.shields.io/crates/v/file_type.svg)](https://crates.io/crates/file_type)
[![License](https://img.shields.io/crates/l/file_type)](https://github.com/theseus-rs/file-type#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

## Getting Started

A file type.  The file type is determined by examining the file or bytes against known file
signatures and file extensions.

Additional information on PRONOM file types can be found at
[The National Archives](https://www.nationalarchives.gov.uk/pronom/)

# Example

Detect a Java class file from bytes:
```rust
use file_type::FileType;

let file_type = FileType::from_bytes(b"\xCA\xFE\xBA\xBE");
assert_eq!(file_type.id(), "x-fmt/415");
assert_eq!(file_type.name(), "Java Class File");
assert_eq!(file_type.media_types(), Vec::<String>::new());
assert_eq!(file_type.extensions(), vec!["class"]);
```

Detect text from bytes:
```rust
use file_type::FileType;

let file_type = FileType::from_bytes(b"hello, world\n");
assert_eq!(file_type.id(), "default/1");
assert_eq!(file_type.name(), "Text");
assert_eq!(file_type.media_types(), vec!["text/plain"]);
assert_eq!(file_type.extensions(), Vec::<String>::new());
```

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

AND

The PRONOM definitions are provided by The National Archives (UK) under the
[Open Government Licence](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

<a href="https://vscode.dev/redirect?url=vscode://ms-vscode-remote.remote-containers/cloneInVolume?url=https://github.com/theseus-rs/file-type">
<img
  src="https://img.shields.io/static/v1?label=VSCode%20Development%20Container&logo=visualstudiocode&message=Open&color=orange"
  alt="VSCode Development Container"
/>
</a>
<br/>
<a href="https://github.dev/theseus-rs/file-type">
<img
  src="https://img.shields.io/static/v1?label=GitHub%20Codespaces&logo=github&message=Open&color=orange"
  alt="GitHub Codespaces"
/>
</a>
