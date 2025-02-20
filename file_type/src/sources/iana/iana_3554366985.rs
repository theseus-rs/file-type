use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3554366985: FileType = FileType {
    file_format: &FileFormat {
        id: 3_554_366_985,
        source_type: SourceType::Iana,
        name: "dns+json",
        extensions: &[],
        media_types: &["application/dns+json"],
        signatures: &[],
        related_formats: &[],
    },
};
