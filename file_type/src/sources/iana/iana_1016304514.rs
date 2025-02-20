use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1016304514: FileType = FileType {
    file_format: &FileFormat {
        id: 1_016_304_514,
        source_type: SourceType::Iana,
        name: "tamp-update-confirm",
        extensions: &[],
        media_types: &["application/tamp-update-confirm"],
        signatures: &[],
        related_formats: &[],
    },
};
