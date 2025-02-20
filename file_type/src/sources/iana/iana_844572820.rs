use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_844572820: FileType = FileType {
    file_format: &FileFormat {
        id: 844_572_820,
        source_type: SourceType::Iana,
        name: "elm+xml",
        extensions: &[],
        media_types: &["application/elm+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
