# File Type

[![ci](https://github.com/theseus-rs/file-type/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/theseus-rs/file-type/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/file_type/badge.svg)](https://docs.rs/file_type)
[![Code Coverage](https://codecov.io/gh/theseus-rs/file-type/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/file-type)
[![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-file-type)
[![Latest version](https://img.shields.io/crates/v/file_type.svg)](https://crates.io/crates/file_type)
[![License](https://img.shields.io/crates/l/file_type)](https://github.com/theseus-rs/file-type#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

File Type is a library that uses file signatures and file extensions to determine the type of file.

Signature, extension and media type data are provided by:
* [Apache HTTPD](https://github.com/apache/httpd/blob/trunk/docs/conf/mime.types)
* [IANA](https://www.iana.org/assignments/media-types/media-types.xml)
* [Linguist](https://github.com/github-linguist/linguist/blob/main/lib/linguist/languages.yml)
* [The National Archives PRONOM](https://www.nationalarchives.gov.uk/pronom/)
* [Wikidata](https://www.wikidata.org/wiki/Wikidata:WikiProject_Informatics/Structures/File_formats/List)

If you need a format that is not supported, please provide your submission to the
[PRONOM](https://www.nationalarchives.gov.uk/pronom/submitinfo.htm) database and include the reference id when opening
a feature request for this project.

## Getting Started

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
| `iana`     | Enables [IANA](https://www.iana.org/assignments/media-types/media-types.xml) file types                                    | No       |
| `linguist` | Enables [Linguist](https://github.com/github-linguist/linguist/blob/main/lib/linguist/languages.yml) file types            | No       |
| `pronom`   | Enables [PRONOM](https://www.nationalarchives.gov.uk/PRONOM) file types                                                    | No       |
| `tokio`    | Enables using tokio for async                                                                                              | No       |
| `wikidata` | Enables [Wikidata](https://www.wikidata.org/wiki/Wikidata:WikiProject_Informatics/Structures/File_formats/List) file types | Yes      |

## Supported File Types

[List of supported file types](https://github.com/theseus-rs/file-type/blob/main/FILETYPES.md)

## Safety

These crates use `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

Data is licensed under the following licenses:

* The HTTPD definitions are provided by the Apache Software Foundation under the
[Apache License 2.0](https://github.com/apache/httpd/blob/trunk/LICENSE).
* The IANA definitions are provided by the Internet Assigned Numbers Authority under the
[Creative Commons Zero (CC0)](https://www.iana.org/help/licensing-terms) license.
* The PRONOM definitions are provided by The National Archives (UK) under the
[Open Government Licence](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/).
* The Wikidata definitions are provided by the Wikidata project under the
[Creative Commons Zero (CC0)](https://www.wikidata.org/wiki/Wikidata:Licensing) license.


## Alternatives

If this crate does not meet your requirements, you may want to consider the following alternatives:

* [infer](https://crates.io/crates/infer)
* [mime_guess](https://crates.io/crates/mime_guess)
* [file-format](https://crates.io/crates/file-format)

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
