use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1292805617: FileType = FileType {
    file_format: &FileFormat {
        id: 1_292_805_617,
        source_type: SourceType::Iana,
        name: "alto-directory+json",
        extensions: &[],
        media_types: &["application/alto-directory+json"],
        signatures: &[],
        related_formats: &[],
    },
};
