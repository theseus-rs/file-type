use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_11854517: FileType = FileType {
    file_format: &FileFormat {
        id: 11_854_517,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.presentationml.presentation.main+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
