use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1261083918: FileType = FileType {
    file_format: &FileFormat {
        id: 1_261_083_918,
        source_type: SourceType::Iana,
        name: "vnd.HandHeld-Entertainment+xml",
        extensions: &[],
        media_types: &["application/vnd.HandHeld-Entertainment+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
