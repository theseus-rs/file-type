use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4036195416: FileType = FileType {
    file_format: &FileFormat {
        id: 4_036_195_416,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
