use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3274470052: FileType = FileType {
    file_format: &FileFormat {
        id: 3_274_470_052,
        source_type: SourceType::Iana,
        name: "uccs+cbor",
        extensions: &[],
        media_types: &["application/uccs+cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
