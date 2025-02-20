use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_459332121: FileType = FileType {
    file_format: &FileFormat {
        id: 459_332_121,
        source_type: SourceType::Iana,
        name: "problem+xml",
        extensions: &[],
        media_types: &["application/problem+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
