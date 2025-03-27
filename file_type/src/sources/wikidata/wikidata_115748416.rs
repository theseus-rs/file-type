use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_115748416: FileType = FileType {
    file_format: &FileFormat {
        id: 115_748_416,
        source_type: SourceType::Wikidata,
        name: "42.zip",
        extensions: &["zip"],
        media_types: &["application/zip"],
        signatures: &[],
        related_formats: &[],
    },
};
