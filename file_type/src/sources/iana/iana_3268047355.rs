use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3268047355: FileType = FileType {
    file_format: &FileFormat {
        id: 3_268_047_355,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
