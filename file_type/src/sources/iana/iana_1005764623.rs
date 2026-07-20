use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1005764623: FileType = FileType {
    file_format: &FileFormat {
        id: 1_005_764_623,
        source_type: SourceType::Iana,
        name: "roughtime-server+json",
        extensions: &[],
        media_types: &["application/roughtime-server+json"],
        signatures: &[],
        related_formats: &[],
    },
};
