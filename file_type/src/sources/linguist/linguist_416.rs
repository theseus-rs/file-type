use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
