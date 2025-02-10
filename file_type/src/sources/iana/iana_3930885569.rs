use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3930885569: FileType = FileType {
    file_format: &FileFormat {
        id: 3_930_885_569,
        source_type: SourceType::Iana,
        name: "mixed",
        extensions: &[],
        media_types: &["multipart/mixed"],
        signatures: &[],
        related_formats: &[],
    },
};
