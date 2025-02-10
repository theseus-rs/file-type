use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3665539775: FileType = FileType {
    file_format: &FileFormat {
        id: 3_665_539_775,
        source_type: SourceType::Iana,
        name: "fhir+json",
        extensions: &[],
        media_types: &["application/fhir+json"],
        signatures: &[],
        related_formats: &[],
    },
};
