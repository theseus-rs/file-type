use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_293173837: FileType = FileType {
    file_format: &FileFormat {
        id: 293_173_837,
        source_type: SourceType::Iana,
        name: "problem+json",
        extensions: &[],
        media_types: &["application/problem+json"],
        signatures: &[],
        related_formats: &[],
    },
};
