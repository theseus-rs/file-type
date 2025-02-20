use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2008268890: FileType = FileType {
    file_format: &FileFormat {
        id: 2_008_268_890,
        source_type: SourceType::Iana,
        name: "vnd.anki",
        extensions: &[],
        media_types: &["application/vnd.anki"],
        signatures: &[],
        related_formats: &[],
    },
};
