use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_79411734: FileType = FileType {
    file_format: &FileFormat {
        id: 79_411_734,
        source_type: SourceType::Iana,
        name: "svg+xml",
        extensions: &[],
        media_types: &["image/svg+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
