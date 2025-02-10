use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3433762130: FileType = FileType {
    file_format: &FileFormat {
        id: 3_433_762_130,
        source_type: SourceType::Iana,
        name: "rfc+xml",
        extensions: &[],
        media_types: &["application/rfc+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
