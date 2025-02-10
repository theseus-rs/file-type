use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1643328057: FileType = FileType {
    file_format: &FileFormat {
        id: 1_643_328_057,
        source_type: SourceType::Iana,
        name: "stix+json",
        extensions: &[],
        media_types: &["application/stix+json"],
        signatures: &[],
        related_formats: &[],
    },
};
