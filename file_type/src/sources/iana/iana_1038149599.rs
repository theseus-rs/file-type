use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1038149599: FileType = FileType {
    file_format: &FileFormat {
        id: 1_038_149_599,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
