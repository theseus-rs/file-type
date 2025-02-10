use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3461787872: FileType = FileType {
    file_format: &FileFormat {
        id: 3_461_787_872,
        source_type: SourceType::Iana,
        name: "tm+json",
        extensions: &[],
        media_types: &["application/tm+json"],
        signatures: &[],
        related_formats: &[],
    },
};
