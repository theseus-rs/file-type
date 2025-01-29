# file_type

[![ci](https://github.com/theseus-rs/file-type/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/theseus-rs/file-type/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/file_type/badge.svg)](https://docs.rs/file_type)
[![Code Coverage](https://codecov.io/gh/theseus-rs/file-type/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/file-type)
[![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-file-type)
[![Latest version](https://img.shields.io/crates/v/file_type.svg)](https://crates.io/crates/file_type)
[![License](https://img.shields.io/crates/l/file_type)](https://github.com/theseus-rs/file-type#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

## Getting Started

File types are determined by examining a file or bytes against known file
signatures and file extensions.

Signature, extension and media type data are provided by:
* [The National Archives PRONOM](https://www.nationalarchives.gov.uk/pronom/)
* [Apache HTTPD](https://github.com/apache/httpd/blob/trunk/docs/conf/mime.types)
* [Linguist](https://github.com/github-linguist/linguist/blob/main/lib/linguist/languages.yml)

# Example

Detect the file type from bytes:
```rust
use file_type::FileType;

let file_type = FileType::from_bytes(b"\xCA\xFE\xBA\xBE");
assert_eq!(file_type.name(), "Java Class File");
assert_eq!(file_type.media_types(), Vec::<String>::new());
assert_eq!(file_type.extensions(), vec!["class"]);
```

Detect the file type from a file:
```rust
use file_type::FileType;
use std::path::Path;

#[tokio::main]
async fn main() {
    let file_path = Path::new("image.png");
    let file_type = FileType::try_from_file(file_path).await.expect("file type not found");
    assert_eq!(file_type.id(), "fmt/11");
    assert_eq!(file_type.name(), "Portable Network Graphics");
    assert_eq!(file_type.extensions(), vec!["png"]);
    assert_eq!(file_type.media_types(), vec!["image/png"]);
}
```

Detect the file type from a file synchronously:
```rust
use file_type::FileType;
use std::path::Path;

let file_path = Path::new("image.png");
let file_type = FileType::try_from_file_sync(file_path).expect("file type not found");
assert_eq!(file_type.id(), "fmt/11");
assert_eq!(file_type.name(), "Portable Network Graphics");
assert_eq!(file_type.extensions(), vec!["png"]);
assert_eq!(file_type.media_types(), vec!["image/png"]);
```

## Feature flags

| Name       | Description                                                                                                                | Default? |
|------------|----------------------------------------------------------------------------------------------------------------------------|----------|
| `custom`   | Enables custom file types                                                                                                  | Yes      |
| `httpd`    | Enables [Apache HTTPD](https://github.com/apache/httpd/blob/trunk/docs/conf/mime.types) file types                         | No       |
| `linguist` | Enables [Linguist](https://github.com/github-linguist/linguist/blob/main/lib/linguist/languages.yml) file types            | No       |
| `pronom`   | Enables [PRONOM](https://www.nationalarchives.gov.uk/PRONOM) file types                                                    | Yes      |
| `tokio`    | Enables using tokio for async                                                                                              | No       |
| `wikidata` | Enables [Wikidata](https://www.wikidata.org/wiki/Wikidata:WikiProject_Informatics/Structures/File_formats/List) file types | No       |

## Supported File Types

[List of supported file types](https://github.com/theseus-rs/file-type/blob/main/FILETYPES.md)

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
