use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_39470048: FileType = FileType {
    file_format: &FileFormat {
        id: 39_470_048,
        source_type: SourceType::Iana,
        name: "pdf",
        extensions: &[],
        media_types: &["application/pdf"],
        signatures: &[],
        related_formats: &[],
    },
};
