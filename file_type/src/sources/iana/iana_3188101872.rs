use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3188101872: FileType = FileType {
    file_format: &FileFormat {
        id: 3_188_101_872,
        source_type: SourceType::Iana,
        name: "emf",
        extensions: &[],
        media_types: &["image/emf"],
        signatures: &[],
        related_formats: &[],
    },
};
