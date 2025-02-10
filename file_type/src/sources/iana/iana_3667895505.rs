use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3667895505: FileType = FileType {
    file_format: &FileFormat {
        id: 3_667_895_505,
        source_type: SourceType::Iana,
        name: "tamp-status-query",
        extensions: &[],
        media_types: &["application/tamp-status-query"],
        signatures: &[],
        related_formats: &[],
    },
};
