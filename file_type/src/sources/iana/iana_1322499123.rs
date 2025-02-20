use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1322499123: FileType = FileType {
    file_format: &FileFormat {
        id: 1_322_499_123,
        source_type: SourceType::Iana,
        name: "xenc+xml",
        extensions: &[],
        media_types: &["application/xenc+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
