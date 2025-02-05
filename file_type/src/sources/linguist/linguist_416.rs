use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_416: FileFormat = FileFormat {
    id: 416,
    source_type: SourceType::Linguist,
    name: "mupad",
    extensions: &["mu"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
