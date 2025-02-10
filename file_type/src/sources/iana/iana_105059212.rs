use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_105059212: FileType = FileType {
    file_format: &FileFormat {
        id: 105_059_212,
        source_type: SourceType::Iana,
        name: "ATXML",
        extensions: &[],
        media_types: &["application/ATXML"],
        signatures: &[],
        related_formats: &[],
    },
};
