use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1236622411: FileType = FileType {
    file_format: &FileFormat {
        id: 1_236_622_411,
        source_type: SourceType::Iana,
        name: "prs.bwtc32key",
        extensions: &[],
        media_types: &["application/prs.bwtc32key"],
        signatures: &[],
        related_formats: &[],
    },
};
