use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_151389392: FileType = FileType {
    file_format: &FileFormat {
        id: 151_389_392,
        source_type: SourceType::Iana,
        name: "alto-error+json",
        extensions: &[],
        media_types: &["application/alto-error+json"],
        signatures: &[],
        related_formats: &[],
    },
};
