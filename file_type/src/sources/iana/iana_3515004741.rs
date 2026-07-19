use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3515004741: FileType = FileType {
    file_format: &FileFormat {
        id: 3_515_004_741,
        source_type: SourceType::Iana,
        name: "roughtime-malfeasance+json",
        extensions: &[],
        media_types: &["application/roughtime-malfeasance+json"],
        signatures: &[],
        related_formats: &[],
    },
};
