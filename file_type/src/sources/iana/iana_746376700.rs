use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_746376700: FileType = FileType {
    file_format: &FileFormat {
        id: 746_376_700,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
