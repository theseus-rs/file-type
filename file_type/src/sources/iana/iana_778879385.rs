use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_778879385: FileType = FileType {
    file_format: &FileFormat {
        id: 778_879_385,
        source_type: SourceType::Iana,
        name: "jumbf",
        extensions: &[],
        media_types: &["application/jumbf"],
        signatures: &[],
        related_formats: &[],
    },
};
