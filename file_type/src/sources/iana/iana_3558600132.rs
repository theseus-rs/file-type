use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3558600132: FileType = FileType {
    file_format: &FileFormat {
        id: 3_558_600_132,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.extended-properties+xml",
        extensions: &[],
        media_types: &["application/vnd.openxmlformats-officedocument.extended-properties+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
