use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2609693029: FileType = FileType {
    file_format: &FileFormat {
        id: 2_609_693_029,
        source_type: SourceType::Iana,
        name: "automationml-amlx+zip",
        extensions: &[],
        media_types: &["application/automationml-amlx+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
