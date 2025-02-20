use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1460453601: FileType = FileType {
    file_format: &FileFormat {
        id: 1_460_453_601,
        source_type: SourceType::Iana,
        name: "mbms-envelope+xml",
        extensions: &[],
        media_types: &["application/mbms-envelope+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
