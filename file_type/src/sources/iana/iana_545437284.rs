use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_545437284: FileType = FileType {
    file_format: &FileFormat {
        id: 545_437_284,
        source_type: SourceType::Iana,
        name: "tamp-community-update-confirm",
        extensions: &[],
        media_types: &["application/tamp-community-update-confirm"],
        signatures: &[],
        related_formats: &[],
    },
};
