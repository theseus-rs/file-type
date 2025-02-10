use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1930820961: FileType = FileType {
    file_format: &FileFormat {
        id: 1_930_820_961,
        source_type: SourceType::Iana,
        name: "fhir+xml",
        extensions: &[],
        media_types: &["application/fhir+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
