use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1049349370: FileType = FileType {
    file_format: &FileFormat {
        id: 1_049_349_370,
        source_type: SourceType::Iana,
        name: "atomsvc+xml",
        extensions: &[],
        media_types: &["application/atomsvc+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
