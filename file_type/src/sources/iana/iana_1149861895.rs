use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1149861895: FileType = FileType {
    file_format: &FileFormat {
        id: 1_149_861_895,
        source_type: SourceType::Iana,
        name: "tamp-sequence-adjust",
        extensions: &[],
        media_types: &["application/tamp-sequence-adjust"],
        signatures: &[],
        related_formats: &[],
    },
};
