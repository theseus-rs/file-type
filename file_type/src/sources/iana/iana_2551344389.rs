use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2551344389: FileType = FileType {
    file_format: &FileFormat {
        id: 2_551_344_389,
        source_type: SourceType::Iana,
        name: "elm+json",
        extensions: &[],
        media_types: &["application/elm+json"],
        signatures: &[],
        related_formats: &[],
    },
};
