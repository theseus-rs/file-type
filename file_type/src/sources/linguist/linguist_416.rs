use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_416: FileType = FileType {
    file_format: &FileFormat {
        id: 416,
        source_type: SourceType::Linguist,
        name: "mupad",
        extensions: &["mu"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
