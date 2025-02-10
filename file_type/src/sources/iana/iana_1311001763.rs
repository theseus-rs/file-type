use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1311001763: FileType = FileType {
    file_format: &FileFormat {
        id: 1_311_001_763,
        source_type: SourceType::Iana,
        name: "ohttp-res",
        extensions: &[],
        media_types: &["message/ohttp-res"],
        signatures: &[],
        related_formats: &[],
    },
};
