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
assert_eq!(file_type.extensions(), vec!["class"]);
```

Detect the file type from a file:

```rust
use file_type::FileType;
use std::path::Path;

let file_path = Path::new("image.png");
let file_type = FileType::try_from_file(file_path).expect("file type not found");
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
| `wikidata` | Enables [Wikidata](https://www.wikidata.org/wiki/Wikidata:WikiProject_Informatics/Structures/File_formats/List) file types | Yes      |

## Supported File Types

[List of supported file types](https://github.com/theseus-rs/file-type/blob/main/FILETYPES.md)

## Safety

These crates use `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

## Performance

### Comparative performance determining file type from bytes

<a href="https://bencher.dev/perf/theseus-rs-file-type?lower_value=false&upper_value=false&lower_boundary=false&upper_boundary=false&x_axis=date_time&branches=e6bda651-ab44-4c6b-8db6-5b495b43ea40&testbeds=4927da7e-2d56-48e6-a579-d78b3787c104&benchmarks=bdf821ff-e1df-478f-923c-6dd28c4509e1%2C709db97a-d220-48b7-996a-2ee7cf2944bd%2Cfa2bef70-dfd4-4834-bbc9-eb3e30af67e5&measures=670fcc74-764a-40b6-8cd2-93076b6cc17d&start_time=1736357094394&tab=plots&plots_search=d92fa224-61b5-42b3-8643-c40fa14f8c11&key=true&reports_per_page=4&branches_per_page=8&testbeds_per_page=8&benchmarks_per_page=8&plots_per_page=8&reports_page=1&branches_page=1&testbeds_page=1&benchmarks_page=1&plots_page=1&utm_medium=share&utm_source=bencher&utm_content=img&utm_campaign=perf%2Bimg&utm_term=theseus-rs-file-type"><img src="https://api.bencher.dev/v0/projects/theseus-rs-file-type/perf/img?branches=e6bda651-ab44-4c6b-8db6-5b495b43ea40&heads=&testbeds=4927da7e-2d56-48e6-a579-d78b3787c104&benchmarks=bdf821ff-e1df-478f-923c-6dd28c4509e1%2C709db97a-d220-48b7-996a-2ee7cf2944bd%2Cfa2bef70-dfd4-4834-bbc9-eb3e30af67e5&measures=670fcc74-764a-40b6-8cd2-93076b6cc17d&start_time=1736357094394" title="theseus-rs/file-type" alt="theseus-rs/file-type - Bencher" /></a>

### Comparative performance determining file type from extension

<a href="https://bencher.dev/perf/theseus-rs-file-type?lower_value=false&upper_value=false&lower_boundary=false&upper_boundary=false&x_axis=date_time&branches=e6bda651-ab44-4c6b-8db6-5b495b43ea40&testbeds=4927da7e-2d56-48e6-a579-d78b3787c104&benchmarks=5ea43253-b368-4799-b6bd-a08556196456%2C2328df4a-c516-403c-bc84-2b7c7ee0a0fa&measures=670fcc74-764a-40b6-8cd2-93076b6cc17d&start_time=1736357094393&tab=plots&plots_search=f1500077-7e42-4ded-b658-8dd1f35205d1&key=true&reports_per_page=4&branches_per_page=8&testbeds_per_page=8&benchmarks_per_page=8&plots_per_page=8&reports_page=1&branches_page=1&testbeds_page=1&benchmarks_page=1&plots_page=1&utm_medium=share&utm_source=bencher&utm_content=img&utm_campaign=perf%2Bimg&utm_term=theseus-rs-file-type"><img src="https://api.bencher.dev/v0/projects/theseus-rs-file-type/perf/img?branches=e6bda651-ab44-4c6b-8db6-5b495b43ea40&heads=&testbeds=4927da7e-2d56-48e6-a579-d78b3787c104&benchmarks=5ea43253-b368-4799-b6bd-a08556196456%2C2328df4a-c516-403c-bc84-2b7c7ee0a0fa&measures=670fcc74-764a-40b6-8cd2-93076b6cc17d&start_time=1736357094393" title="theseus-rs/file-type" alt="theseus-rs/file-type - Bencher" /></a>

### Comparative performance determining file type from media type

<a href="https://bencher.dev/perf/theseus-rs-file-type?lower_value=false&upper_value=false&lower_boundary=false&upper_boundary=false&x_axis=date_time&branches=e6bda651-ab44-4c6b-8db6-5b495b43ea40&testbeds=4927da7e-2d56-48e6-a579-d78b3787c104&benchmarks=65468a2b-b06d-4c15-9491-694934dbd036%2C7d89adc9-2bf0-4449-9923-73e1ea4eabd8&measures=670fcc74-764a-40b6-8cd2-93076b6cc17d&start_time=1736357094389&tab=plots&plots_search=f564a523-53a7-491f-a650-7097249dc2cb&key=true&reports_per_page=4&branches_per_page=8&testbeds_per_page=8&benchmarks_per_page=8&plots_per_page=8&reports_page=1&branches_page=1&testbeds_page=1&benchmarks_page=1&plots_page=1&utm_medium=share&utm_source=bencher&utm_content=img&utm_campaign=perf%2Bimg&utm_term=theseus-rs-file-type"><img src="https://api.bencher.dev/v0/projects/theseus-rs-file-type/perf/img?branches=e6bda651-ab44-4c6b-8db6-5b495b43ea40&heads=&testbeds=4927da7e-2d56-48e6-a579-d78b3787c104&benchmarks=65468a2b-b06d-4c15-9491-694934dbd036%2C7d89adc9-2bf0-4449-9923-73e1ea4eabd8&measures=670fcc74-764a-40b6-8cd2-93076b6cc17d&start_time=1736357094389" title="theseus-rs/file-type" alt="theseus-rs/file-type - Bencher" /></a>

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

* [file-format](https://crates.io/crates/file-format)
* [infer](https://crates.io/crates/infer)
* [magic](https://crates.io/crates/magic)
* [mime_guess](https://crates.io/crates/mime_guess)

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
