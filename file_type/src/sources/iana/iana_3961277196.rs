use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3961277196: FileType = FileType {
    file_format: &FileFormat {
        id: 3_961_277_196,
        source_type: SourceType::Iana,
        name: "timestamp-query",
        extensions: &[],
        media_types: &["application/timestamp-query"],
        signatures: &[],
        related_formats: &[],
    },
};
