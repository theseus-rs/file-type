use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3010191087: FileType = FileType {
    file_format: &FileFormat {
        id: 3_010_191_087,
        source_type: SourceType::Iana,
        name: "cloudevents+json",
        extensions: &[],
        media_types: &["application/cloudevents+json"],
        signatures: &[],
        related_formats: &[],
    },
};
