use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1434422002: FileType = FileType {
    file_format: &FileFormat {
        id: 1_434_422_002,
        source_type: SourceType::Iana,
        name: "example",
        extensions: &[],
        media_types: &["image/example"],
        signatures: &[],
        related_formats: &[],
    },
};
