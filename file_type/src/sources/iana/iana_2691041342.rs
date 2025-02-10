use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2691041342: FileType = FileType {
    file_format: &FileFormat {
        id: 2_691_041_342,
        source_type: SourceType::Iana,
        name: "eat-ucs+cbor",
        extensions: &[],
        media_types: &["application/eat-ucs+cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
